use super::*;
use std::collections::HashMap;

struct FunctionOverload<'f> {
    descriptor: cxx::SharedPtr<FunctionDescriptor>,
    implementation: cxx::UniquePtr<Function<'f>>,
}

struct FfiFunctionImpl<'f> {
    descriptor: cxx::SharedPtr<FunctionDescriptor>,
    overloads: Vec<rust::function::Function<'f>>,
}

impl<'f> FfiFunctionImpl<'f> {
    fn new(
        descriptor: cxx::SharedPtr<FunctionDescriptor>,
        overloads: Vec<rust::function::Function<'f>>,
    ) -> Self {
        Self {
            descriptor,
            overloads,
        }
    }
}

impl<'f> FfiFunction for FfiFunctionImpl<'f> {
    fn invoke<'a>(
        &self,
        args: Span<'_, Value<'a>>,
        descriptor_pool: &'a DescriptorPool,
        message_factory: &'a MessageFactory,
        arena: &'a Arena,
        overload_id: Span<'_, cxx::CxxString>,
    ) -> Result<cxx::UniquePtr<Value<'a>>, Status> {
        for overload in self.overloads.iter() {
            let target_id: String = overload.function_type().id(
                &self.descriptor.name().to_string_lossy(),
                self.descriptor.receiver_style(),
            );
            if !overload_id
                .iter()
                .any(|id| id.as_bytes() == target_id.as_bytes())
            {
                continue;
            }

            let rust_args = args
                .into_iter()
                .map(|arg| value_to_rust(arg, arena, descriptor_pool, message_factory))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| error_from_rust(&e))?;

            let result = overload.call(rust_args);

            #[cfg(feature = "async")]
            let result = result.expect_result("should not be a future");

            let value = result
                .map(|value| value_from_rust(&value, arena, descriptor_pool, message_factory))
                .map_err(|e| error_from_rust(&e))?;

            return Ok(value);
        }
        Err(Status::new(
            StatusCode::InvalidArgument,
            "No matching overload found",
        ))
    }
}

pub(crate) struct FfiActivationImpl<'f> {
    variables: HashMap<String, rust::variable::VariableBinding<'f>>,
    functions: HashMap<String, Vec<FunctionOverload<'f>>>,
}

impl<'f> FfiActivationImpl<'f> {
    pub(crate) fn new<'a>(
        function_registry: &rust::function::FunctionRegistry<'f>,
        variable_registry: &rust::variable::VariableRegistry,
        function_bindings: &rust::function::FunctionBindings<'f>,
        variable_bindings: &rust::variable::VariableBindings<'f>,
    ) -> FfiActivationImpl<'f>
    where
        'f: 'a,
    {
        let mut functions = HashMap::new();
        for (name, overloads) in function_registry.entries() {
            let mut ols: Vec<FunctionOverload<'f>> = Vec::new();
            for kind_overload in overloads.entries() {
                let member = kind_overload.member();
                let types = kind_overload.argument_kinds();
                let descriptor = FunctionDescriptor::new_shared(name, member, types, true);
                let mut type_overloads = Vec::new();
                for type_overload in kind_overload.entries() {
                    if let Some(fn_impl) = type_overload.r#impl() {
                        type_overloads.push(fn_impl.clone());
                    }
                }
                let implementation =
                    Function::new(FfiFunctionImpl::new(descriptor.clone(), type_overloads));
                ols.push(FunctionOverload {
                    descriptor,
                    implementation,
                });
            }
            functions.insert(name.to_string(), ols);

            let binding = function_bindings.find(name);
            if let Some(binding) = binding {
                let mut ols = Vec::new();
                for kind_overload in binding.entries() {
                    let member = kind_overload.member();
                    let types = kind_overload.argument_kinds();
                    let descriptor = FunctionDescriptor::new_shared(name, member, types, true);
                    let type_overloads = kind_overload.entries().cloned().collect::<Vec<_>>();
                    let implementation =
                        Function::new(FfiFunctionImpl::new(descriptor.clone(), type_overloads));
                    ols.push(FunctionOverload {
                        descriptor,
                        implementation,
                    });
                }
                functions.insert(name.to_string(), ols);
            }
        }

        let mut variables = HashMap::new();
        for (name, _) in variable_registry.entries() {
            let binding = variable_bindings.find(name);
            if let Some(binding) = binding {
                variables.insert(name.to_string(), binding.clone());
            }
        }

        FfiActivationImpl {
            variables,
            functions,
        }
    }
}

impl<'f> FfiActivation<'f> for FfiActivationImpl<'f> {
    fn find_variable<'a>(
        &self,
        name: &str,
        descriptor_pool: &'a DescriptorPool,
        message_factory: &'a MessageFactory,
        arena: &'a Arena,
    ) -> Result<Option<cxx::UniquePtr<Value<'a>>>, Status> {
        let variable = self.variables.get(name);
        match variable {
            None => Ok(None),
            Some(rust::variable::VariableBinding::Value((_value_type, value))) => Ok(Some(
                value_from_rust(value, arena, descriptor_pool, message_factory),
            )),
            Some(rust::variable::VariableBinding::Provider(provider)) => {
                let res = provider.call(vec![]);

                #[cfg(feature = "async")]
                let res = res.expect_result("should not be a future");

                match res {
                    Ok(ref value) => Ok(Some(value_from_rust(
                        value,
                        arena,
                        descriptor_pool,
                        message_factory,
                    ))),
                    Err(e) => Err(Status::new(
                        StatusCode::Internal,
                        format!("Failed to call provider: {}", e).as_str(),
                    )),
                }
            }
        }
    }

    fn find_function_overloads<'this>(
        &'this self,
        name: &str,
    ) -> Vec<FunctionOverloadReference<'this, 'f>> {
        let overloads = self.functions.get(name);
        match overloads {
            None => {
                vec![]
            }
            Some(overloads) => overloads
                .iter()
                .map(|overload| {
                    FunctionOverloadReference::new(&overload.descriptor, &overload.implementation)
                })
                .collect(),
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub(crate) mod async_activation {
    use super::*;
    use crate::r#async::{abort::Abortable, BlockingRunner};
    use std::sync::{Arc, Mutex};

    struct FfiAsyncFunctionImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        descriptor: cxx::SharedPtr<FunctionDescriptor>,
        overloads: Vec<AbortableFunctionRunner<'f, R>>,
    }

    impl<'f, R> FfiAsyncFunctionImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        pub(crate) fn new(
            descriptor: cxx::SharedPtr<FunctionDescriptor>,
            overloads: Vec<rust::function::Function<'f>>,
            abortable: Arc<Mutex<Abortable>>,
        ) -> Self {
            let overloads = overloads
                .iter()
                .map(|overload| AbortableFunctionRunner::new(overload.clone(), abortable.clone()))
                .collect::<Vec<_>>();
            Self {
                descriptor,
                overloads,
            }
        }
    }

    impl<'f, R> FfiFunction for FfiAsyncFunctionImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        #[allow(unused_variables)]
        fn invoke<'a>(
            &self,
            args: Span<'_, Value<'a>>,
            descriptor_pool: &'a DescriptorPool,
            message_factory: &'a MessageFactory,
            arena: &'a Arena,
            overload_id: Span<'_, cxx::CxxString>,
        ) -> Result<cxx::UniquePtr<Value<'a>>, Status> {
            for overload in self.overloads.iter() {
                let target_id: String = overload.function_impl().function_type().id(
                    &self.descriptor.name().to_string_lossy(),
                    self.descriptor.receiver_style(),
                );
                if !overload_id
                    .iter()
                    .any(|id| id.as_bytes() == target_id.as_bytes())
                {
                    continue;
                }

                let rust_args = args
                    .into_iter()
                    .map(|arg| value_to_rust(arg, arena, descriptor_pool, message_factory))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| error_from_rust(&e))?;

                let result = overload.call(rust_args);

                let value = result
                    .map(|value| value_from_rust(&value, arena, descriptor_pool, message_factory))
                    .map_err(|e| error_from_rust(&e))?;
                return Ok(value);
            }

            Err(Status::new(
                StatusCode::InvalidArgument,
                "No matching async overload found",
            ))
        }
    }

    enum AbortableVariableBinding<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        Value(rust::Value),
        Provider(AbortableProviderRunner<'f, R>),
    }

    pub(crate) struct FfiAsyncActivationImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        variables: HashMap<String, AbortableVariableBinding<'f, R>>,
        functions: HashMap<String, Vec<FunctionOverload<'f>>>,
        _marker: std::marker::PhantomData<R>,
    }

    impl<'f, R> FfiAsyncActivationImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        pub(crate) fn new(
            function_registry: &rust::function::FunctionRegistry<'f>,
            variable_registry: &rust::variable::VariableRegistry,
            function_bindings: &rust::function::FunctionBindings<'f>,
            variable_bindings: &rust::variable::VariableBindings<'f>,
            abortable: Arc<Mutex<Abortable>>,
        ) -> FfiAsyncActivationImpl<'f, R> {
            let mut functions = HashMap::new();
            for (name, overloads) in function_registry.entries() {
                let mut ols: Vec<FunctionOverload<'f>> = Vec::new();
                for kind_overload in overloads.entries() {
                    let member = kind_overload.member();
                    let types = kind_overload.argument_kinds();
                    let descriptor = FunctionDescriptor::new_shared(name, member, types, true);
                    let mut type_overloads = Vec::new();
                    for type_overload in kind_overload.entries() {
                        if let Some(fn_impl) = type_overload.r#impl() {
                            type_overloads.push(fn_impl.clone());
                        }
                    }
                    let implementation = Function::new(FfiAsyncFunctionImpl::<R>::new(
                        descriptor.clone(),
                        type_overloads,
                        abortable.clone(),
                    ));
                    ols.push(FunctionOverload {
                        descriptor,
                        implementation,
                    });
                }
                functions.insert(name.to_string(), ols);

                let binding = function_bindings.find(name);
                if let Some(binding) = binding {
                    let mut ols = Vec::new();
                    for kind_overload in binding.entries() {
                        let member = kind_overload.member();
                        let types = kind_overload.argument_kinds();
                        let descriptor = FunctionDescriptor::new_shared(name, member, types, true);
                        let type_overloads = kind_overload.entries().cloned().collect::<Vec<_>>();
                        let implementation = Function::new(FfiAsyncFunctionImpl::<R>::new(
                            descriptor.clone(),
                            type_overloads,
                            abortable.clone(),
                        ));
                        ols.push(FunctionOverload {
                            descriptor,
                            implementation,
                        });
                    }
                    functions.insert(name.to_string(), ols);
                }
            }

            let mut variables = HashMap::new();
            for (name, _) in variable_registry.entries() {
                let binding = variable_bindings.find(name);
                if let Some(binding) = binding {
                    match binding {
                        rust::variable::VariableBinding::Value((_value_type, value)) => {
                            variables.insert(
                                name.to_string(),
                                AbortableVariableBinding::Value(value.clone()),
                            );
                        }
                        rust::variable::VariableBinding::Provider(provider) => {
                            variables.insert(
                                name.to_string(),
                                AbortableVariableBinding::Provider(AbortableProviderRunner::new(
                                    provider.clone(),
                                    abortable.clone(),
                                )),
                            );
                        }
                    }
                }
            }

            FfiAsyncActivationImpl {
                variables,
                functions,
                _marker: std::marker::PhantomData,
            }
        }
    }

    impl<'f, R> FfiActivation<'f> for FfiAsyncActivationImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        fn find_variable<'a>(
            &self,
            name: &str,
            descriptor_pool: &'a DescriptorPool,
            message_factory: &'a MessageFactory,
            arena: &'a Arena,
        ) -> Result<Option<cxx::UniquePtr<Value<'a>>>, Status> {
            let variable = self.variables.get(name);
            match variable {
                None => Ok(None),
                Some(AbortableVariableBinding::Value(value)) => Ok(Some(value_from_rust(
                    value,
                    arena,
                    descriptor_pool,
                    message_factory,
                ))),
                Some(AbortableVariableBinding::Provider(runner)) => match runner.call() {
                    Ok(value) => Ok(Some(value_from_rust(
                        &value,
                        arena,
                        descriptor_pool,
                        message_factory,
                    ))),
                    Err(e) => Err(Status::new(
                        StatusCode::Internal,
                        format!("Failed to call provider: {}", e).as_str(),
                    )),
                },
            }
        }

        fn find_function_overloads<'this>(
            &'this self,
            name: &str,
        ) -> Vec<FunctionOverloadReference<'this, 'f>> {
            let overloads = self.functions.get(name);
            match overloads {
                None => {
                    vec![]
                }
                Some(overloads) => overloads
                    .iter()
                    .map(|overload| {
                        FunctionOverloadReference::new(
                            &overload.descriptor,
                            &overload.implementation,
                        )
                    })
                    .collect(),
            }
        }
    }

    #[derive(Debug, Clone)]
    struct AbortableFunctionRunner<'f, R: BlockingRunner> {
        function: rust::function::Function<'f>,
        abortable: Arc<Mutex<Abortable>>,
        _marker: std::marker::PhantomData<R>,
    }

    impl<'f, R: BlockingRunner> AbortableFunctionRunner<'f, R> {
        fn new(function: rust::function::Function<'f>, abortable: Arc<Mutex<Abortable>>) -> Self {
            Self {
                function,
                abortable,
                _marker: std::marker::PhantomData,
            }
        }

        fn function_impl(&self) -> &rust::function::Function<'f> {
            &self.function
        }

        fn call(&self, args: Vec<rust::Value>) -> Result<rust::Value, rust::Error> {
            match self.function.call(args) {
                rust::MaybeFuture::Result(result) => result,
                rust::MaybeFuture::Future(fut) => {
                    run_until_aborted::<R>(fut, self.abortable.clone())
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    struct AbortableProviderRunner<'f, R: BlockingRunner> {
        provider: rust::function::Function<'f>,
        abortable: Arc<Mutex<Abortable>>,
        _marker: std::marker::PhantomData<R>,
    }

    impl<'f, R: BlockingRunner> AbortableProviderRunner<'f, R> {
        fn new(provider: rust::function::Function<'f>, abortable: Arc<Mutex<Abortable>>) -> Self {
            Self {
                provider,
                abortable,
                _marker: std::marker::PhantomData,
            }
        }

        fn call(&self) -> Result<rust::Value, rust::Error> {
            match self.provider.call(vec![]) {
                rust::MaybeFuture::Result(result) => result,
                rust::MaybeFuture::Future(fut) => {
                    run_until_aborted::<R>(fut, self.abortable.clone())
                }
            }
        }
    }

    fn run_until_aborted<'f, R: BlockingRunner>(
        mut fut: futures::future::BoxFuture<'f, Result<rust::Value, rust::Error>>,
        abortable: Arc<Mutex<Abortable>>,
    ) -> Result<rust::Value, rust::Error> {
        use futures::FutureExt;

        let mut abortable_guard = abortable.lock().expect("abortable lock failed");
        let mut abortable = std::pin::Pin::new(&mut *abortable_guard).fuse();
        let mut fut = fut.as_mut().fuse();
        let fut = async move {
            futures::select! {
                _ = abortable => {
                    Err(rust::Error::cancelled("aborted by the caller"))
                }
                result = fut => {
                    result
                }
            }
        };
        R::block_on(fut)
    }
}
