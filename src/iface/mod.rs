pub(crate) mod spawn_handle       ;
pub(crate) mod local_spawn_handle ;
pub(crate) mod join_handle        ;
pub(crate) mod yield_now          ;

pub use spawn_handle       ::*;
pub use local_spawn_handle ::*;
pub use join_handle        ::*;
pub use yield_now          ::*;

pub(crate) mod timer;
pub use timer::*;


/// Trait indicating that tokio IO can be used with the executor that
/// implements it. Currently this can be enabled through features on [`TokioCt`](crate::TokioCt),
/// [`TokioTp`](crate::TokioTp), [`AsyncGlobal`](crate::AsyncGlobal) and [`AsyncStd`](crate::AsyncStd).
///
/// This means a tokio reactor will be running and that the network types from tokio (eg. `TcpStream`) will work.
//
pub trait TokioIo {}
