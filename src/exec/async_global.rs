use
{
	crate        :: { SpawnHandle, LocalSpawnHandle, JoinHandle                } ,
	futures_task :: { FutureObj, LocalFutureObj, Spawn, LocalSpawn, SpawnError } ,

	async_global_executor as async_global,
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
	#[ cfg(not( target_os = "unknown")) ]
	//
	#[ cfg_attr( nightly, doc(cfg(not( target_os = "unknown" ))) ) ]
	//
	pub fn block_on<F: std::future::Future>(future: F) -> F::Output
	{
		async_global::block_on( future )
	}
}


#[ cfg( target_arch = "wasm32" ) ]
//
impl Spawn for AsyncGlobal
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_global::spawn_local( future ).detach();

		Ok(())
	}
}



#[ cfg(not( target_arch = "wasm32" )) ]
//
impl Spawn for AsyncGlobal
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_global::spawn( future ).detach();

		Ok(())
	}
}



#[ cfg( not(target_arch = "wasm32") ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for AsyncGlobal
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let handle = async_global::spawn( future );

		Ok( JoinHandle::async_global(handle) )
	}
}



#[ cfg( target_arch = "wasm32" ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for AsyncGlobal
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let handle = async_global::spawn_local( future );

		Ok( JoinHandle::async_global(handle) )
	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for AsyncGlobal
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let handle = async_global::spawn_local( future );

		Ok( JoinHandle::async_global(handle) )
	}
}



impl LocalSpawn for AsyncGlobal
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_global::spawn_local( future ).detach();

		Ok(())
	}
}


impl crate::YieldNow for AsyncGlobal {}



#[ cfg( not(target_arch = "wasm32") ) ]
//
impl<R: Send + 'static> crate::SpawnBlocking<R> for AsyncGlobal
{
	fn spawn_blocking<F>( &self, f: F ) -> crate::BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	{
		let handle = async_global::spawn_blocking( f );

		crate::BlockingHandle::async_global( Box::pin( handle ) )
	}


	fn spawn_blocking_dyn( &self, f: Box< dyn FnOnce()->R + Send > ) -> crate::BlockingHandle<R>
	{
		self.spawn_blocking( f )
	}
}




impl std::fmt::Debug for AsyncGlobal
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "AsyncGlobal executor" )
	}
}



/// Signal io can be used on this executor.
//
#[ cfg(all( not(target_arch = "wasm32"), feature = "async_global_io" )) ]
//
#[ cfg_attr( nightly, doc(cfg(all( not(target_arch = "wasm32"), feature = "async_global_io" ))) ) ]
//
impl crate::AsyncIo for AsyncGlobal {}


/// Signal io can be used on this executor.
//
#[ cfg(all( not(target_arch = "wasm32"), feature = "async_global_tokio" )) ]
//
#[ cfg_attr( nightly, doc(cfg(all( not(target_arch = "wasm32"), feature = "async_global_tokio" ))) ) ]
//
impl crate::TokioIo for AsyncGlobal {}



#[ cfg( feature = "timer" ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "timer", feature = "async_global" ))) ) ]
//
impl crate::Timer for AsyncGlobal
{
	fn sleep( &self, dur: std::time::Duration ) -> futures_core::future::BoxFuture<'static, ()>
	{
		Box::pin( futures_timer::Delay::new( dur ) )
	}
}




