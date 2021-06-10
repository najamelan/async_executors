pub(crate) mod blocking_handle    ;
pub(crate) mod join_handle        ;
pub(crate) mod local_spawn_handle ;
pub(crate) mod spawn_blocking     ;
pub(crate) mod spawn_handle       ;
pub(crate) mod timer              ;
pub(crate) mod yield_now          ;

pub use blocking_handle    ::*;
pub use join_handle        ::*;
pub use local_spawn_handle ::*;
pub use spawn_blocking     ::*;
pub use spawn_handle       ::*;
pub use timer              ::*;
pub use yield_now          ::*;



#[ cfg( feature = "async_global" ) ]
//
use async_global_executor::{ Task as AsyncGlobalTask };

#[ cfg( feature = "async_std" ) ]
//
use async_std_crate::{ task::JoinHandle as AsyncStdJoinHandle };

#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
//
use tokio::{ task::JoinHandle as TokioJoinHandle };



/// Trait indicating that tokio IO can be used with the executor that
/// implements it. Currently this can be enabled through features on [`TokioCt`](crate::TokioCt),
/// [`TokioTp`](crate::TokioTp), [`AsyncGlobal`](crate::AsyncGlobal) and [`AsyncStd`](crate::AsyncStd).
///
/// This means a tokio reactor will be running and that the network types from tokio (eg. `TcpStream`) will work.
//
pub trait TokioIo {}
