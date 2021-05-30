use
{
	crate        :: { SpawnHandle, LocalSpawnHandle, JoinHandle                } ,
	futures_task :: { FutureObj, LocalFutureObj, Spawn, LocalSpawn, SpawnError } ,
};


/// An executor that spawns tasks on async-global-executor. In contrast to the other executors, this one
/// is not self contained, because async-global-executor does not provide an API that allows that,
/// so the threadpool is global.
///
/// It works on Wasm.
//
#[ derive( Copy, Clone, Default ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "async_global" )) ) ]
//
pub struct AsyncGlobal;

impl AsyncGlobal
{
	/// Create a new AsyncGlobal wrapper, forwards to `Default::default`.
	///
	pub fn new() -> Self
	{
		Self::default()
	}


	/// Wrapper around [async_global_executor::block_on]. This is not available on Wasm
	/// as Wasm does not have threads and you're not allowed to block the only thread you have.
	//
	// TODO: is target_arch = "wasm32"  not a better way to express this?
	//
	#[cfg(not(target_os = "unknown"))]
	#[ cfg_attr( nightly, doc(cfg(not( target_os = "unknown" ))) ) ]
	//
	pub fn block_on<F: std::future::Future>(future: F) -> F::Output
	{
		async_global_executor::block_on( future )
	}
}



#[ cfg( target_arch = "wasm32" ) ]
//
impl Spawn for AsyncGlobal
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_global_executor::spawn_local( future ).detach();

		Ok(())
	}
}



#[ cfg(not( target_arch = "wasm32" )) ]
//
impl Spawn for AsyncGlobal
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_global_executor::spawn( future ).detach();

		Ok(())
	}
}



#[ cfg( not(target_arch = "wasm32") ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for AsyncGlobal
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::AsyncGlobal
		{
			task: Some( async_global_executor::spawn(future) ),
		}})
	}
}



#[ cfg( target_arch = "wasm32" ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for AsyncGlobal
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::AsyncGlobal
		{
			task: Some( async_global_executor::spawn_local(future) ),
		}})
	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for AsyncGlobal
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::AsyncGlobal
		{
			task: Some( async_global_executor::spawn_local(future) ),
		}})
	}
}



impl LocalSpawn for AsyncGlobal
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		let _ = async_global_executor::spawn_local( future ).detach();

		Ok(())
	}
}


impl std::fmt::Debug for AsyncGlobal
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "AsyncGlobal executor" )
	}
}
