#[ allow(unused_imports) ]
//
use
{
	futures_util :: { future::{ FutureExt, abortable }, task::SpawnExt                    } ,
	futures_task :: { SpawnError, FutureObj                                               } ,
	crate        :: { JoinHandle                                                          } ,
	std          :: { pin::Pin, future::Future, sync::{ Arc, atomic::AtomicBool }, rc::Rc } ,
};


/// Let's you spawn and get a [JoinHandle] to await the output of a future.
///
/// This trait works much like the [`Spawn`](futures_task::Spawn) trait from the futures library.
/// It takes a [`FutureObj`] so we can hopefully make it `no_std` compatible when needed. This
/// also allows it to be object safe.  For convenience, there is [`SpawnHandleExt`] which allows you
/// to spawn a generic future directly without having to manually make the [`FutureObj`].
///
/// [`SpawnHandleExt`] is automatically implemented but must be in scope, so this works:
///
/// ```rust
/// use async_executors::{ SpawnHandle, SpawnHandleExt };
///
/// async fn need_exec( exec: impl SpawnHandle<()> )
/// {
///    let join_handle = exec.spawn_handle( async {} ).expect( "spawn" );
///
///    join_handle.await;
/// }
/// ```
///
/// and so does this:
///
/// ```rust
/// use async_executors::{ SpawnHandle, SpawnHandleExt };
///
/// async fn need_exec( exec: Box< dyn SpawnHandle<()> > )
/// {
///    let join_handle = exec.spawn_handle( async {} ).expect( "spawn" );
///
///    join_handle.await;
/// }
/// ```
///
/// One inconvenience of it having to be object safe is that the trait needs to be generic over the
/// output parameter. This can be annoying if you need an executor that can spawn futures with different
/// output parameters. Normally you should always be able to know which ones you need. If not
/// you will have to make the type that stores the executor generic over the output type as well.
///
/// So to enable several output types you can use the
/// [following workaround](https://github.com/najamelan/async_executors/tree/master/examples/spawn_handle_multi.rs).
//
pub trait SpawnHandle<Out: 'static + Send>
{
	/// Spawn a future and return a [`JoinHandle`] that can be awaited for the output of the future.
	//
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>;
}

/// Convenience trait for passing in a generic future to [`SpawnHandle`]. Much akin to `Spawn` and `SpawnExt` in the
/// futures library.
//
pub trait SpawnHandleExt<Out: 'static + Send> : SpawnHandle<Out>
{
	/// Spawn a future and return a [JoinHandle] that can be awaited for the output of the future.
	//
	fn spawn_handle( &self, future: impl Future<Output = Out> + Send + 'static ) -> Result<JoinHandle<Out>, SpawnError>;
}


impl<T, Out> SpawnHandleExt<Out> for T

	where T  : SpawnHandle<Out> + ?Sized ,
	      Out: 'static + Send            ,
{
	fn spawn_handle( &self, future: impl Future<Output = Out> + Send + 'static ) -> Result<JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle_obj( FutureObj::new(future.boxed()) )
	}
}


impl<T: ?Sized, Out> SpawnHandle<Out> for Box<T> where T: SpawnHandle<Out>, Out: 'static + Send
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_obj( future )
	}
}


impl<T: ?Sized, Out> SpawnHandle<Out> for Arc<T> where T: SpawnHandle<Out>, Out: 'static + Send
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_obj( future )
	}
}


impl<T: ?Sized, Out> SpawnHandle<Out> for Rc<T> where T: SpawnHandle<Out>, Out: 'static + Send
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_obj( future )
	}
}


impl<T, Out> SpawnHandle<Out> for &T where T: SpawnHandle<Out>, Out: 'static + Send
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_obj( future )
	}
}



impl<T, Out> SpawnHandle<Out> for &mut T where T: SpawnHandle<Out>, Out: 'static + Send
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_obj( future )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for futures_executor::LocalSpawner
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, handle) = future.remote_handle();

		self.spawn( fut )?;

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



#[ cfg( feature = "threadpool" ) ]
//
impl<Out: 'static + Send> SpawnHandle<Out> for futures_executor::ThreadPool
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, handle) = future.remote_handle();

		self.spawn( fut )?;

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}


