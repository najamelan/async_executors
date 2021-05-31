pub(crate) mod spawn_handle       ;
pub(crate) mod local_spawn_handle ;
pub(crate) mod join_handle        ;

pub use spawn_handle       ::*;
pub use local_spawn_handle ::*;
pub use join_handle        ::*;


pub(crate) mod timer;
pub use timer::*;

/// Trait indicating that glommio IO can be used with the executor that
/// implements it. Thus when a library requests a `impl Spawn + GlommioIo` it is
/// not executor agnostic but it will still benefit from the `Spawn` implementation.
///
/// This means that the network types from glommio (eg. `TcpStream`) will work.
//
pub trait GlommioIo {}


/// Trait indicating that tokio IO can be used with the executor that
/// implements it. Thus when a library requests a `impl Spawn + TokioIo` it is
/// not executor agnostic but it will still benefit from the `Spawn` implementation.
///
/// This means that the network types from tokio (eg. `TcpStream`) will work.
/// This crate turns on both the `net` and `process` features of tokio when
/// the `tokio_reactor` feature is enabled.
//
pub trait TokioIo {}


/// Trait indicating that async-io can be used with the executor that
/// implements it. Thus when a library requests a `impl Spawn + AsyncIo` it is
/// not executor agnostic but it will still benefit from the `Spawn` implementation.
///
/// This means that the network types from async-std (eg. `TcpStream`) will work.
/// This crate turns on both the `net` and `process` features of tokio when
/// the `tokio_reactor` feature is enabled.
//
pub trait AsyncIo {}
