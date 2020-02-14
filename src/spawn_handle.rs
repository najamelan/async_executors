#[ allow(unused_imports) ]
//
use
{
	futures_util :: { task::{ SpawnExt, SpawnError }, future::{ FutureExt, abortable } } ,
	std          :: { future::Future, sync::{ Arc, atomic::AtomicBool }, rc::Rc        } ,
};


/// Let's you spawn and get a [JoinHandle] to await the output of a future.
///
/// This trait is not object safe, see [SpawnHandleOs] for a best effort object safe one.
//
pub trait SpawnHandle
{
	/// Spawn a future and return a [JoinHandle] that can be awaited for the output of the future.
	//
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send
	;
}


impl<T> SpawnHandle for Arc<T> where T: SpawnHandle
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}


impl<T> SpawnHandle for Rc<T> where T: SpawnHandle
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}


impl<T> SpawnHandle for &T where T: SpawnHandle
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}


impl<T> SpawnHandle for &mut T where T: SpawnHandle
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}




#[ cfg( feature = "async_std" ) ]
//
impl SpawnHandle for crate::async_std::AsyncStd
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, a_handle) = abortable( future );

		Ok( crate::JoinHandle{ inner: crate::join_handle::InnerJh::AsyncStd
		{
			handle  : async_std_crate::task::spawn(fut) ,
			detached: AtomicBool::new(false)            ,
			a_handle                                    ,
		}})
	}
}



#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
//
impl SpawnHandle for crate::TokioHandle
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, a_handle) = abortable( future );

		Ok( crate::JoinHandle{ inner: crate::join_handle::InnerJh::Tokio
		{
			handle  : self.spawner.spawn(fut) ,
			detached: AtomicBool::new(false)  ,
			a_handle                          ,
		}})
	}
}



#[ cfg(any( feature = "tokio_ct" )) ]
//
impl SpawnHandle for crate::TokioLocalHandle
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, a_handle) = abortable( future );

		Ok( crate::JoinHandle{ inner: crate::join_handle::InnerJh::Tokio
		{
			handle  : self.spawner.spawn(fut) ,
			detached: AtomicBool::new(false)  ,
			a_handle                          ,
		}})
	}
}



#[ cfg( feature = "bindgen" ) ]
//
impl SpawnHandle for crate::Bindgen
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, handle) = future.remote_handle();
		wasm_bindgen_futures::spawn_local(fut);

		Ok( crate::JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl SpawnHandle for futures_executor::LocalSpawner
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let handle = self.spawn_with_handle( future )?;

		Ok( crate::JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



#[ cfg( feature = "threadpool" ) ]
//
impl SpawnHandle for futures_executor::ThreadPool
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<crate::JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let handle = self.spawn_with_handle( future )?;

		Ok( crate::JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}


