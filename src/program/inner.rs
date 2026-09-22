use crate::*;
use ouroboros::self_referencing;
use std::sync::Arc;

#[self_referencing]
#[derive(Debug)]
pub struct ProgramInner<'f> {
    env: Arc<EnvInner<'f>>,

    ffi_ctx: ffi::Ctx,

    #[borrows(env, ffi_ctx)]
    #[covariant]
    ffi_program: (cxx::UniquePtr<ffi::Program<'this, 'f>>, ValueType),
}

impl<'f> ProgramInner<'f> {
    pub fn new_from_env_source<S: AsRef<[u8]>>(
        env: Arc<EnvInner<'f>>,
        source: S,
    ) -> Result<Self, Error> {
        let ctx = env.ctx().clone_with_new_arena();
        ProgramInnerTryBuilder {
            env,
            ffi_ctx: ctx,
            ffi_program_builder: |env: &Arc<EnvInner<'f>>, ffi_ctx: &ffi::Ctx| {
                let mut result = env
                    .compiler()
                    .compile(source.as_ref(), None)
                    .map_err(|ffi_status| Error::from(&ffi_status))?;
                if !result.is_valid() {
                    let error = result.format_error();
                    return Err(Error::invalid_argument(error));
                }

                let ast = result
                    .pin_mut()
                    .release_ast()
                    .map_err(|ffi_status| Error::from(&ffi_status))?;
                let return_type =
                    ffi::type_to_rust(&ast.return_type(ffi_ctx.descriptor_pool(), ffi_ctx.arena()));

                let program = env
                    .runtime()
                    .create_program(ast)
                    .map_err(|ffi_status| Error::from(&ffi_status))?;
                Ok((program, return_type))
            },
        }
        .try_build()
    }

    pub fn env(&self) -> &Arc<EnvInner<'f>> {
        self.with_env(|v| v)
    }

    pub fn ffi_ctx(&self) -> &ffi::Ctx {
        self.with_ffi_ctx(|v| v)
    }

    pub fn ffi_program<'this>(&'this self) -> &'this ffi::Program<'this, 'f> {
        self.with_ffi_program(|v| &v.0)
    }

    pub fn return_type(&self) -> &ValueType {
        self.with_ffi_program(|v| &v.1)
    }

    pub fn eval_sync<'a, A>(self: Arc<Self>, activation: A) -> Result<Value, Error>
    where
        'f: 'a,
        A: ActivationInterface<'f, ()> + 'a,
    {
        let eval_ctx = self.ffi_ctx().clone_with_new_arena();
        let ffi_activation = ffi::Activation::new(ffi::FfiActivationImpl::new(
            self.env().function_registry(),
            self.env().variable_registry(),
            activation.functions(),
            activation.variables(),
            eval_ctx.arena(),
            eval_ctx.descriptor_pool(),
        )?);
        let ffi_value = self
            .ffi_program()
            .evaluate(
                eval_ctx.arena(),
                Some(eval_ctx.message_factory()),
                &ffi_activation,
            )
            .map_err(|ffi_status| Error::from(&ffi_status))?;
        if ffi_value.is_null() {
            return Err(Error::unknown("evaluation returned null"));
        }
        let value = ffi::value_to_rust(
            &ffi_value,
            eval_ctx.arena(),
            eval_ctx.descriptor_pool(),
            eval_ctx.message_factory(),
        )?;
        if value.is_error() && !self.return_type().is_error() {
            return Err(value.expect_error("expect error but got value"));
        }
        Ok(value)
    }

    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn eval_async<Rm, A, Afm>(
        self: Arc<Self>,
        activation: A,
    ) -> futures::future::BoxFuture<'f, Result<Value, Error>>
    where
        'f: 'static,
        Rm: crate::r#async::Runtime,
        <Rm::ScopedSpawner as async_scoped::spawner::Spawner<()>>::FutureOutput: Send,
        A: ActivationInterface<'f, Afm>,
        Afm: FnMarker,
    {
        let eval_ctx = self.ffi_ctx().clone_with_new_arena();
        let (abort_handle, abortable) = crate::r#async::abort::abortable();
        let function_bindings = activation.functions().clone();
        let variable_bindings = activation.variables().clone();

        let this = self.clone();
        let closure = move || -> Result<Value, Error> {
            // Build the descriptors after moving the context into the worker,
            // so their arena borrow stays within the evaluation's lifetime.
            let ffi_activation =
                ffi::Activation::new(ffi::async_activation::FfiAsyncActivationImpl::<
                    Rm::BlockingRunner,
                >::new(
                    this.env().function_registry(),
                    this.env().variable_registry(),
                    &function_bindings,
                    &variable_bindings,
                    eval_ctx.arena(),
                    eval_ctx.descriptor_pool(),
                    Arc::new(std::sync::Mutex::new(abortable)),
                )?);
            let ffi_value = this
                .ffi_program()
                .evaluate(
                    eval_ctx.arena(),
                    Some(eval_ctx.message_factory()),
                    &ffi_activation,
                )
                .map_err(|ffi_status| Error::from(ffi_status))?;
            if ffi_value.is_null() {
                return Err(Error::unknown("evaluation returned null"));
            }
            let value = ffi::value_to_rust(
                &ffi_value,
                eval_ctx.arena(),
                eval_ctx.descriptor_pool(),
                eval_ctx.message_factory(),
            )?;
            Ok(value)
        };

        let result = Arc::new(std::sync::Mutex::new(None));
        let worker_result = result.clone();
        use async_scoped::spawner::FuncSpawner;
        let worker = Rm::ScopedSpawner::default().spawn_func(move || {
            let r = closure();
            *worker_result.lock().unwrap() = Some(r);
        });

        let this = self.clone();
        let fut = async move {
            let _abort_handle = abort_handle;

            let _ = worker.await;
            let value = result
                .lock()
                .expect("result lock failed")
                .take()
                .unwrap_or(Err(Error::internal(
                    "async evaluation did not return a value",
                )))?;
            if value.is_error() && !this.return_type().is_error() {
                return Err(value.expect_error("expect error but got value"));
            }
            Ok(value)
        };
        Box::pin(fut)
    }
}
