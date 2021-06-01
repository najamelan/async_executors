#[ allow(unused_imports) ]
//
use
{
	futures_task :: { SpawnError, LocalFutureObj                                          } ,
	futures_util :: { task::{ LocalSpawnExt }, future::{ FutureExt, abortable }           } ,
	crate        :: { JoinHandle                                                          } ,
	std          :: { pin::Pin, future::Future, sync::{ Arc, atomic::AtomicBool }, rc::Rc } ,
	blanket      :: { blanket                                                             } ,
};


/// This is similar to [`SpawnHandle`](crate::SpawnHandle) except that it allows spawning `!Send` futures. Please see
/// the docs on [`SpawnHandle`](crate::SpawnHandle).
//
#[ blanket(derive( Ref, Mut, Box, Arc, Rc )) ]
//
pub trait LocalSpawnHandle<Out: 'static>
{
	/// Spawn a future and return a [`JoinHandle`] that can be awaited for the output of the future.
	//
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>;
}


/// Lets you spawn a !Send future and get a [JoinHandle] to await the output of a future.
//
pub trait LocalSpawnHandleExt<Out: 'static> : LocalSpawnHandle<Out>
{
	/// Convenience trait for passing in a generic future to [`LocalSpawnHandle`]. Much akin to `LocalSpawn` and `LocalSpawnExt` in the
	/// futures library.
	//
	fn spawn_handle_local( &self, future: impl Future<Output = Out> + 'static ) -> Result<JoinHandle<Out>, SpawnError>;
}


impl<T, Out> LocalSpawnHandleExt<Out> for T

	where T  : LocalSpawnHandle<Out> + ?Sized ,
	      Out: 'static                        ,
{
	fn spawn_handle_local( &self, future: impl Future<Output = Out> + 'static ) -> Result<JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle_local_obj( LocalFutureObj::new(future.boxed_local()) )
	}
}
