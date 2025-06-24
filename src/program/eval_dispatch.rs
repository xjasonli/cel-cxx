use super::inner::ProgramInner;
use crate::*;
use std::sync::Arc;

pub trait EvalDispatch<'f, A, Afm> {
    type Output;
    fn eval(self, program: Arc<ProgramInner<'f>>, activation: A) -> Self::Output;
}

#[allow(missing_debug_implementations)]
pub struct EvalDispatcher<Fm, Rm> {
    _fn_marker: std::marker::PhantomData<Fm>,
    _rt_marker: std::marker::PhantomData<Rm>,
}

impl<Fm, Rm> Default for EvalDispatcher<Fm, Rm> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Fm, Rm> EvalDispatcher<Fm, Rm> {
    pub fn new() -> Self {
        Self {
            _fn_marker: std::marker::PhantomData,
            _rt_marker: std::marker::PhantomData,
        }
    }
}

// Implement RunDispatch for Dispatcher<()>
impl<'f, A, Rm> EvalDispatch<'f, A, ()> for EvalDispatcher<(), Rm>
where
    Rm: RuntimeMarker,
    A: ActivationInterface<'f, ()>,
{
    type Output = Result<Value, Error>;

    fn eval(self, program: Arc<ProgramInner<'f>>, activation: A) -> Self::Output {
        program.eval_sync(activation)
    }
}

// Implement RunDispatch for Dispatcher<Async>
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
const _: () = {
    use crate::r#async::*;
    use futures::future::BoxFuture;

    impl<'f, A, Rm, Afm> EvalDispatch<'f, A, Afm> for EvalDispatcher<Async, Rm>
    where
        Rm: Runtime,
        <Rm::ScopedSpawner as async_scoped::spawner::Spawner<()>>::FutureOutput: Send,
        A: ActivationInterface<'f, Afm>,
        Afm: FnMarker,
    {
        type Output = BoxFuture<'f, Result<Value, Error>>;

        fn eval(self, program: Arc<ProgramInner<'f>>, activation: A) -> Self::Output {
            program.eval_async::<Rm, A, Afm>(activation)
        }
    }
};
