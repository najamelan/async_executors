use
{
	crate        :: { SpawnHandle, LocalSpawnHandle, JoinHandle,               } ,
	futures_task :: { FutureObj, LocalFutureObj, Spawn, LocalSpawn, SpawnError } ,
	futures_util :: { future::abortable                                        } ,

	async_std_crate as async_std,
};


/// An executor that spawns tasks on async-std. In contrast to the other executors, this one
/// is not self contained, because async-std does not provide an API that allows that,
/// so the threadpool is global.
///
/// It works on Wasm.
//
#[ derive( Copy, Clone, Default ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "async_std" )) ) ]
//
pub struct AsyncStd;

impl AsyncStd
{
	/// Create a new AsyncStd wrapper, forwards to `Default::default`.
	///
	pub fn new() -> Self
	{
		Self::default()
	}


	/// Wrapper around [async_std::task::block_on](::async_std_crate::task::block_on()). This is not available on Wasm
	/// as Wasm does not have threads and you're not allowed to block the only thread you have.
	//
	#[cfg(not(target_os = "unknown"))]
	#[ cfg_attr( nightly, doc(cfg(not( target_os = "unknown" ))) ) ]
	//
	pub fn block_on<F: std::future::Future>(future: F) -> F::Output
	{
		async_std::task::block_on( future )
	}
}



#[ cfg( target_arch = "wasm32" ) ]
//
impl Spawn for AsyncStd
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_std::task::spawn_local( future );

		Ok(())
	}
}



#[ cfg(not( target_arch = "wasm32" )) ]
//
impl Spawn for AsyncStd
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_std::task::spawn( future );

		Ok(())
	}
}



#[ cfg( not(target_arch = "wasm32") ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for AsyncStd
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, a_handle) = abortable( future );
		let handle          = async_std::task::spawn( fut );

		Ok( JoinHandle::async_std(handle, a_handle) )
	}
}



// async-std only exposes local_spawn on wasm.
//
#[ cfg( target_arch = "wasm32" ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for AsyncStd
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, a_handle) = abortable( future );
		let handle          = async_std::task::spawn_local( fut );

		Ok( JoinHandle::async_std(handle, a_handle) )

	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for AsyncStd
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, a_handle) = abortable( future );
		let handle = async_std::task::spawn_local( fut );

		Ok( JoinHandle::async_std(handle, a_handle))
	}
}



impl LocalSpawn for AsyncStd
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = async_std::task::spawn_local( future );

		Ok(())
	}
}



impl crate::YieldNow for AsyncStd {}



#[ cfg( not(target_arch = "wasm32") ) ]
//
impl crate::SpawnBlocking for AsyncStd
{
	fn spawn_blocking<F, R>( &self, f: F ) -> crate::BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	         R: Send + 'static                 ,
	{
		let handle = async_std::task::spawn_blocking( f );

		crate::BlockingHandle::async_std( handle )
	}
}



impl std::fmt::Debug for AsyncStd
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "AsyncStd executor" )
	}
}



/// Signal io can be used on this executor.
//
#[ cfg(all( not(target_arch = "wasm32"), feature="async_std_tokio" )) ]
//
#[ cfg_attr( nightly, doc(cfg(all( not(target_arch = "wasm32"), feature="async_std_tokio" ))) ) ]
//
impl crate::TokioIo for AsyncStd {}






#[ cfg(not( target_arch = "wasm32" )) ]
//
impl crate::Timer for AsyncStd
{
	fn sleep( &self, dur: std::time::Duration ) -> futures_core::future::BoxFuture<'static, ()>
	{
		Box::pin( async_std::task::sleep(dur) )
	}
}





// On wasm async_std future is not Send, so use futures-timer.
//
#[ cfg(all( target_arch = "wasm32", feature = "timer" )) ]
//
impl crate::Timer for AsyncStd
{
	fn sleep( &self, dur: std::time::Duration ) -> futures_core::future::BoxFuture<'static, ()>
	{
		Box::pin( futures_timer::Delay::new(dur) )
	}
}


