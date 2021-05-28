use futures_task::{FutureObj, LocalSpawn, Spawn, SpawnError};
use futures_util::future::LocalFutureObj;
use glommio_crate::{LocalExecutor, LocalExecutorBuilder};
use std::future::Future;
use std::rc::Rc;
use crate::{LocalSpawnHandle, SpawnHandle, InnerJh, JoinHandle};

/// A simple glommio runtime builder
#[derive(Debug)]
pub struct GlommioCt {
    executor: Rc<LocalExecutor>,
}

impl GlommioCt {
    /// new Glommio Local Executor
    pub fn new(name: &str, cpu_set: Option<usize>) -> Self {
        let mut builder = LocalExecutorBuilder::new().name(&name);
        if let Some(binding) = cpu_set {
            builder = builder.pin_to_cpu(binding);
        }
        let executor = builder.make().unwrap();
        Self {
            executor: Rc::new(executor),
        }
    }
    /// execute the code until completion
    pub fn block_on<F: Future>(&self, future: F) -> <F as Future>::Output {
        self.executor.run(future)
    }
}


impl LocalSpawn for GlommioCt {
    fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError> {
        glommio_crate::Task::local(future).detach();
        Ok(())
    }
}


impl<Out: 'static> LocalSpawnHandle<Out> for GlommioCt {
    fn spawn_handle_local_obj(
        &self,
        future: LocalFutureObj<'static, Out>,
    ) -> Result<JoinHandle<Out>, SpawnError> {

        Ok(JoinHandle {
            inner: InnerJh::Glommio{ task: Some(glommio_crate::Task::local(future)), handle: None },
        })
    }
}

impl Spawn for GlommioCt {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        self.spawn_local_obj(LocalFutureObj::from(future))
    }
}

impl<Out: Send + 'static> SpawnHandle<Out> for GlommioCt {
    fn spawn_handle_obj(&self, future: FutureObj<'static, Out>) -> Result<JoinHandle<Out>, SpawnError> {

        Ok(JoinHandle {
            inner: InnerJh::Glommio{ task: Some(glommio_crate::Task::local(future)), handle: None },
        })
    }
}