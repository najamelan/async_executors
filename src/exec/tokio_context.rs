use std::sync::atomic::AtomicBool;

use futures_task::{FutureObj, SpawnError};

use crate::{JoinHandle, iface::InnerJh};


/// An executor that uses the current [`tokio::runtime::Runtime`] to spawn futures.
///
/// Useful for use inside apps that use [`tokio::main`]
#[derive(Debug, Clone, Copy, Default)]
#[ cfg_attr( nightly, doc(cfg( feature = "tokio_context" )) ) ]
pub struct TokioContext {}


impl TokioContext {
    /// Constructs a TokioContext executor
    pub fn new() -> Self {
        TokioContext {}
    }
}

impl<Out: Send + 'static> crate::SpawnHandle<Out> for TokioContext {
    fn spawn_handle_obj(
        &self,
        future: FutureObj<'static, Out>,
    ) -> Result<JoinHandle<Out>, SpawnError> {
        Ok(JoinHandle {
            inner: InnerJh::Tokio {
                handle: tokio::task::spawn(future),
                detached: AtomicBool::new(false)
            }
        })
    }
}
