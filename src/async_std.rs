use
{
	crate        :: { SpawnHandle, LocalSpawnHandle, JoinHandle, join_handle::InnerJh } ,
	futures_task :: { FutureObj, LocalFutureObj, Spawn, LocalSpawn, SpawnError        } ,
	std          :: { sync::atomic::AtomicBool                                        } ,
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
		async_std_crate::task::block_on( future )
	}
}



#[ cfg( target_arch = "wasm32" ) ]
//
impl Spawn for AsyncStd
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_std_crate::task::spawn_local( future );

		Ok(())
	}
}



#[ cfg(not( target_arch = "wasm32" )) ]
//
impl Spawn for AsyncStd
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		async_std_crate::task::spawn( future );

		Ok(())
	}
}



#[ cfg( not(target_arch = "wasm32") ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for AsyncStd
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::AsyncStd
		{
			handle  : Some( async_std_crate::task::spawn( future ) ) ,
			detached: AtomicBool::new( false )                       ,
		}})
	}
}



#[ cfg( target_arch = "wasm32" ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for AsyncStd
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, a_handle) = abortable( future );

		Ok( JoinHandle{ inner: InnerJh::AsyncStd
		{
			handle  : Some( async_std_crate::task::spawn( future ) ) ,
			detached: AtomicBool::new( false )                       ,
			a_handle                                                 ,
		}})
	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for AsyncStd
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		Ok( JoinHandle{ inner: InnerJh::AsyncStd
		{
			handle  : Some( async_std_crate::task::spawn_local( future ) ) ,
			detached: AtomicBool::new( false )                             ,
		}})
	}
}



impl LocalSpawn for AsyncStd
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = async_std_crate::task::spawn_local( future );

		Ok(())
	}
}


impl std::fmt::Debug for AsyncStd
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "AsyncStd threadpool" )
	}
}
