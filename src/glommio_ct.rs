use std::future::Future;
use glommio_crate::{LocalExecutorBuilder};
use futures_task::{FutureObj, Spawn, SpawnError, LocalSpawn};
use crate::{SpawnHandle, JoinHandle, InnerJh, LocalSpawnHandle};
use futures_util::future::LocalFutureObj;

/// A simple glommio runtime builder
#[derive(Debug)]
pub struct GlommioCtBuilder {
    binding: Option<usize>,
    name: String,
}

impl GlommioCtBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self { binding: None, name: "unnamed".to_string() }
    }

    /// Sets the new executor's affinity to the provided CPU.  The largest `cpu`
    /// value [supported] by libc is 1023.
    ///
    /// [supported]: https://man7.org/linux/man-pages/man2/sched_setaffinity.2.html#NOTES
    pub fn pin_to_cpu(&mut self, cpu: usize) {
        self.binding = Some(cpu);
    }

    fn get_builder(&self) -> LocalExecutorBuilder {
        let mut builder = LocalExecutorBuilder::new().name(&self.name);
        if let Some(binding) = self.binding {
            builder = builder.pin_to_cpu(binding);
        }
        builder
    }

    /// block on the given future
    pub fn block_on<Fut, Out: 'static>(&self, future: Fut) -> Out
        where Fut: Future<Output=Out>
    {
        self.get_builder().make().expect("Cannot make a local executor").run(
            future
        )
    }
}

impl Spawn for GlommioCtBuilder {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        GlommioCt::new().spawn_obj(future)
    }
}

impl<Out: Send + 'static> SpawnHandle<Out> for GlommioCtBuilder {
    fn spawn_handle_obj(&self, future: FutureObj<'static, Out>) -> Result<JoinHandle<Out>, SpawnError> {
        let (tx, rx) = futures::channel::oneshot::channel();
        let handle = self.get_builder().spawn(move || async move {
            tx.send(future.await)
        }).expect("Cannot spawn an OS thread");

        Ok(JoinHandle {
            inner: InnerJh::Glommio {
                handle: Some(handle),
                result: rx,
            }
        })
    }
}
impl LocalSpawn for GlommioCtBuilder {
    fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError> {
        GlommioCt::new().spawn_local_obj(future)
    }
}


impl<Out: Send + 'static> LocalSpawnHandle<Out> for GlommioCtBuilder {
    fn spawn_handle_local_obj(&self, future: LocalFutureObj<'static, Out>) -> Result<JoinHandle<Out>, SpawnError> {
        GlommioCt::new().spawn_handle_local_obj(future)
    }
}


/// Glommio Local Executor
#[derive(Debug, Copy, Clone)]
pub struct GlommioCt {}

impl GlommioCt {
    /// new Glommio Local Executor
    pub fn new() -> Self {
        Self {}
    }
}

impl LocalSpawn for GlommioCt {
    fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError> {
        glommio_crate::Task::local(future).detach();
        Ok(())
    }
}


impl<Out: Send + 'static> LocalSpawnHandle<Out> for GlommioCt {
    fn spawn_handle_local_obj(&self, future: LocalFutureObj<'static, Out>) -> Result<JoinHandle<Out>, SpawnError> {
        let (tx, rx) = futures::channel::oneshot::channel();

        let _task = glommio_crate::Task::local(async { let _ = tx.send(future.await); }).detach();
        Ok(JoinHandle {
            inner: InnerJh::Glommio {
                handle: None,
                result: rx,
            }
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
        self.spawn_handle_local_obj(LocalFutureObj::from(future))
    }
}