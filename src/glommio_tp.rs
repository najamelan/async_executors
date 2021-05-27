use std::future::Future;
use glommio_crate::{LocalExecutorBuilder, LocalExecutorPoolBuilder};
use futures_task::{FutureObj, Spawn, SpawnError, LocalSpawn};
use crate::{SpawnHandle, JoinHandle, InnerJh, LocalSpawnHandle, GlommioCt};
use futures_util::future::LocalFutureObj;
use std::ops::Range;

/// A simple glommio runtime builder
#[derive(Debug)]
pub struct GlommioTpBuilder {
    threads: usize,
    name: String,
    pin_to_cpu: Option<Range<usize>>
}

impl GlommioTpBuilder {
    /// Create a new builder
    pub fn new(threads: usize) -> Self {
        Self {
            threads,
            name: "unnamed".to_string(),
            pin_to_cpu: None
        }
    }

    /// block on the given future
    pub fn block_on<Fut, Out: 'static>(&self, future: Fut) -> Out
        where Fut: Future<Output=Out>
    {
        let range = self.pin_to_cpu.unwrap_or(0..self.threads);
        for (thread, cpu_id) in (0..self.threads).zip(range) {
            let mut builder = LocalExecutorBuilder::new().name(&format!("{}-{}", self.name, thread));
            if self.pin_to_cpu.is_some() {
                builder = builder.pin_to_cpu(cpu_id);
            }
            builder.spawn(&future)
        }
    }
}

impl Spawn for GlommioTpBuilder {
    fn spawn_obj(&self, future: FutureObj<'static, ()>) -> Result<(), SpawnError> {
        self.get_builder().spawn(move || future).expect("Cannot spawn an OS thread");
        Ok(())
    }
}

impl<Out: Send + 'static> SpawnHandle<Out> for GlommioTpBuilder {
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

impl LocalSpawn for GlommioTpBuilder {
    fn spawn_local_obj(&self, future: LocalFutureObj<'static, ()>) -> Result<(), SpawnError> {
        GlommioCt::new().spawn_local_obj(future)
    }
}


impl<Out: Send + 'static> LocalSpawnHandle<Out> for GlommioTpBuilder {
    fn spawn_handle_local_obj(&self, future: LocalFutureObj<'static, Out>) -> Result<JoinHandle<Out>, SpawnError> {
        GlommioCt::new().spawn_handle_local_obj(future)
    }
}
