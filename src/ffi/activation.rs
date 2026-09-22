use super::*;
use std::collections::HashMap;

struct FunctionOverload<'a, 'f> {
    descriptor: cxx::SharedPtr<FunctionDescriptor<'a>>,
    implementation: cxx::UniquePtr<Function<'f>>,
}

fn build_function_overloads<'a, 'f: 'a>(
    function_registry: &rust::function::FunctionRegistry<'f>,
    function_bindings: &rust::function::FunctionBindings<'f>,
    arena: &'a Arena,
    descriptor_pool: &'a DescriptorPool,
    mut wrap: impl FnMut(rust::function::Function<'f>) -> cxx::UniquePtr<Function<'f>>,
) -> Result<HashMap<String, Vec<FunctionOverload<'a, 'f>>>, rust::Error> {
    // Validate full argument types and call style before constructing descriptors.
    for (name, b_overloads) in function_bindings.entries() {
        let Some(reg) = function_registry.find(name) else {
            return Err(rust::Error::invalid_argument(format!(
                "binding '{name}' has no compile-time declaration"
            )));
        };
        for be in b_overloads.entries() {
            if reg.find(be.member(), &be.inner().arguments()).is_none() {
                let id = be.inner().function_type().id(name, be.member());
                return Err(rust::Error::invalid_argument(format!(
                    "binding '{name}' overload '{id}' has no compile-time declaration"
                )));
            }
        }
    }

    let mut functions = HashMap::new();
    for (name, reg_overloads) in function_registry.entries() {
        let mut ols = Vec::new();
        let binding = function_bindings.find(name);
        for entry in reg_overloads.entries() {
            let member = entry.member();
            let id = entry.inner().decl().id(name, member);
            let impl_fn = binding
                .and_then(|b| b.find(member, entry.inner().decl().arguments()))
                .map(|be| be.inner().clone())
                .or_else(|| entry.inner().r#impl().cloned());
            let Some(impl_fn) = impl_fn else {
                continue;
            };
            let arg_types = entry
                .inner()
                .decl()
                .arguments()
                .iter()
                .map(|t| type_from_rust(t, arena, descriptor_pool))
                .collect::<Result<Vec<_>, _>>()?;
            let descriptor = FunctionDescriptor::new_shared_with_overload_id(
                name, &id, member, &arg_types, true,
            );
            let implementation = wrap(impl_fn);
            ols.push(FunctionOverload {
                descriptor,
                implementation,
            });
        }
        functions.insert(name.to_string(), ols);
    }

    Ok(functions)
}

/// Invokes one Rust implementation selected by cel-cpp overload resolution.
struct FfiFunctionImpl<'f> {
    r#impl: rust::function::Function<'f>,
}

impl<'f> FfiFunctionImpl<'f> {
    fn new(r#impl: rust::function::Function<'f>) -> Self {
        Self { r#impl }
    }
}

impl<'f> FfiFunction for FfiFunctionImpl<'f> {
    fn invoke<'a>(
        &self,
        args: Span<'_, Value<'a>>,
        descriptor_pool: &'a DescriptorPool,
        message_factory: &'a MessageFactory,
        arena: &'a Arena,
    ) -> Result<cxx::UniquePtr<Value<'a>>, Status> {
        let rust_args = args
            .into_iter()
            .map(|arg| value_to_rust(arg, arena, descriptor_pool, message_factory))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| Status::from(&e))?;

        let result = self.r#impl.call(rust_args);

        #[cfg(feature = "async")]
        let result = result.into_result().ok_or_else(|| {
            Status::failed_precondition("asynchronous function requires asynchronous evaluation")
        })?;

        let value = result
            .map(|value| value_from_rust(&value, arena, descriptor_pool, message_factory))
            .map_err(|e| Status::from(&e))?;
        Ok(value)
    }
}

pub(crate) struct FfiActivationImpl<'a, 'f> {
    variables: HashMap<String, rust::variable::VariableBinding<'f>>,
    functions: HashMap<String, Vec<FunctionOverload<'a, 'f>>>,
}

impl<'a, 'f: 'a> FfiActivationImpl<'a, 'f> {
    /// Builds descriptors for declared overloads with an implementation.
    /// Bindings override registry implementations when their argument types and
    /// call style match; bindings without a matching declaration are rejected.
    ///
    /// Descriptors borrow type data from `arena`; the activation must not
    /// outlive it.
    pub(crate) fn new(
        function_registry: &rust::function::FunctionRegistry<'f>,
        variable_registry: &rust::variable::VariableRegistry,
        function_bindings: &rust::function::FunctionBindings<'f>,
        variable_bindings: &rust::variable::VariableBindings<'f>,
        arena: &'a Arena,
        descriptor_pool: &'a DescriptorPool,
    ) -> Result<Self, rust::Error> {
        let functions = build_function_overloads(
            function_registry,
            function_bindings,
            arena,
            descriptor_pool,
            |implementation| Function::new(FfiFunctionImpl::new(implementation)),
        )?;

        let mut variables = HashMap::new();
        for (name, _) in variable_registry.entries() {
            let binding = variable_bindings.find(name);
            if let Some(binding) = binding {
                variables.insert(name.to_string(), binding.clone());
            }
        }

        Ok(FfiActivationImpl {
            variables,
            functions,
        })
    }
}

impl<'a, 'f: 'a> FfiActivation<'a> for FfiActivationImpl<'a, 'f> {
    fn find_variable<'v>(
        &self,
        name: &str,
        descriptor_pool: &'v DescriptorPool,
        message_factory: &'v MessageFactory,
        arena: &'v Arena,
    ) -> Result<Option<cxx::UniquePtr<Value<'v>>>, Status> {
        let variable = self.variables.get(name);
        match variable {
            None => Ok(None),
            Some(rust::variable::VariableBinding::Value((_value_type, value))) => Ok(Some(
                value_from_rust(value, arena, descriptor_pool, message_factory),
            )),
            Some(rust::variable::VariableBinding::Provider(provider)) => {
                let res = provider.call(vec![]);

                #[cfg(feature = "async")]
                let res = res.into_result().ok_or_else(|| {
                    Status::failed_precondition(
                        "asynchronous provider requires asynchronous evaluation",
                    )
                })?;

                match res {
                    Ok(ref value) => Ok(Some(value_from_rust(
                        value,
                        arena,
                        descriptor_pool,
                        message_factory,
                    ))),
                    Err(e) => Err(Status::from(e)),
                }
            }
        }
    }

    fn find_function_overloads<'this>(
        &'this self,
        name: &str,
    ) -> Vec<FunctionOverloadReference<'this, 'this, 'a>> {
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

    /// Async single-impl adapter; mirrors `FfiFunctionImpl` but wraps the impl
    /// in an `AbortableFunctionRunner` so async futures can be cancelled.
    struct FfiAsyncFunctionImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        r#impl: AbortableFunctionRunner<'f, R>,
    }

    impl<'f, R> FfiAsyncFunctionImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        pub(crate) fn new(
            r#impl: rust::function::Function<'f>,
            abortable: Arc<Mutex<Abortable>>,
        ) -> Self {
            Self {
                r#impl: AbortableFunctionRunner::new(r#impl, abortable),
            }
        }
    }

    impl<'f, R> FfiFunction for FfiAsyncFunctionImpl<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        fn invoke<'a>(
            &self,
            args: Span<'_, Value<'a>>,
            descriptor_pool: &'a DescriptorPool,
            message_factory: &'a MessageFactory,
            arena: &'a Arena,
        ) -> Result<cxx::UniquePtr<Value<'a>>, Status> {
            let rust_args = args
                .into_iter()
                .map(|arg| value_to_rust(arg, arena, descriptor_pool, message_factory))
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| Status::from(e))?;

            let result = self.r#impl.call(rust_args);

            let value = result
                .map(|value| value_from_rust(&value, arena, descriptor_pool, message_factory))
                .map_err(|e| Status::from(e))?;
            Ok(value)
        }
    }

    enum AbortableVariableBinding<'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        Value(rust::Value),
        Provider(AbortableFunctionRunner<'f, R>),
    }

    pub(crate) struct FfiAsyncActivationImpl<'a, 'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        variables: HashMap<String, AbortableVariableBinding<'f, R>>,
        functions: HashMap<String, Vec<FunctionOverload<'a, 'f>>>,
    }

    impl<'a, 'f: 'a, R> FfiAsyncActivationImpl<'a, 'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        /// Async mirror of `FfiActivationImpl::new`; see that fn for the
        /// per-overload / binding-validation semantics.
        pub(crate) fn new(
            function_registry: &rust::function::FunctionRegistry<'f>,
            variable_registry: &rust::variable::VariableRegistry,
            function_bindings: &rust::function::FunctionBindings<'f>,
            variable_bindings: &rust::variable::VariableBindings<'f>,
            arena: &'a Arena,
            descriptor_pool: &'a DescriptorPool,
            abortable: Arc<Mutex<Abortable>>,
        ) -> Result<Self, rust::Error> {
            let functions = build_function_overloads(
                function_registry,
                function_bindings,
                arena,
                descriptor_pool,
                |implementation| {
                    Function::new(FfiAsyncFunctionImpl::<R>::new(
                        implementation,
                        abortable.clone(),
                    ))
                },
            )?;

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
                                AbortableVariableBinding::Provider(AbortableFunctionRunner::new(
                                    provider.clone(),
                                    abortable.clone(),
                                )),
                            );
                        }
                    }
                }
            }

            Ok(FfiAsyncActivationImpl {
                variables,
                functions,
            })
        }
    }

    impl<'a, 'f: 'a, R> FfiActivation<'a> for FfiAsyncActivationImpl<'a, 'f, R>
    where
        R: rust::r#async::BlockingRunner,
    {
        fn find_variable<'v>(
            &self,
            name: &str,
            descriptor_pool: &'v DescriptorPool,
            message_factory: &'v MessageFactory,
            arena: &'v Arena,
        ) -> Result<Option<cxx::UniquePtr<Value<'v>>>, Status> {
            let variable = self.variables.get(name);
            match variable {
                None => Ok(None),
                Some(AbortableVariableBinding::Value(value)) => Ok(Some(value_from_rust(
                    value,
                    arena,
                    descriptor_pool,
                    message_factory,
                ))),
                Some(AbortableVariableBinding::Provider(runner)) => match runner.call(Vec::new()) {
                    Ok(value) => Ok(Some(value_from_rust(
                        &value,
                        arena,
                        descriptor_pool,
                        message_factory,
                    ))),
                    Err(e) => Err(Status::from(e)),
                },
            }
        }

        fn find_function_overloads<'this>(
            &'this self,
            name: &str,
        ) -> Vec<FunctionOverloadReference<'this, 'this, 'a>> {
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

        fn call(&self, args: Vec<rust::Value>) -> Result<rust::Value, rust::Error> {
            match self.function.call(args) {
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
