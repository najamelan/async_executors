#[ allow(unused_imports) ]
//
use
{
	futures_util :: { task::{ Spawn, SpawnError }, future::FutureExt } ,
	crate        :: { import::*, SpawnHandle, JoinHandle             } ,
	std          :: { pin::Pin, future::Future, sync::Arc, rc::Rc    } ,
};


/// Object safe version of [SpawnHandle]. This allows you to take it as a param
/// and store it. It incurs some overhead, since the future needs to be boxed and executors
/// will box it again to queue it.
///
/// It also implies you have to choose an Out type.
//
#[ cfg_attr( feature = "docs", doc(cfg( feature = "spawn_handle" )) ) ]
//
pub trait SpawnHandleOs<Out: 'static + Send>
{
	/// Spawn a future and return a RemoteHandle that can be awaited for the output of the future.
	//
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>;
}


impl<T: ?Sized, Out> SpawnHandleOs<Out> for Box<T> where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}


impl<T: ?Sized, Out> SpawnHandleOs<Out> for Arc<T> where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}


impl<T: ?Sized, Out> SpawnHandleOs<Out> for Rc<T> where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}


impl<T, Out> SpawnHandleOs<Out> for &T where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}


impl<T, Out> SpawnHandleOs<Out> for &mut T where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}


#[ cfg( feature = "tracing" ) ]
//
impl<T, Out> SpawnHandleOs<Out> for Instrumented<T> where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let fut = future.instrument( self.span().clone() );

		self.inner().spawn_handle_os( fut.boxed() )
	}
}


#[ cfg( feature = "tracing" ) ]
//
impl<T, Out> SpawnHandleOs<Out> for WithDispatch<T> where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let fut = self.with_dispatch( future );

		self.inner().spawn_handle_os( fut.boxed() )
	}
}



#[ cfg( feature = "async_std" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for crate::async_std::AsyncStd
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg( feature = "tokio_tp" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for crate::TokioTp
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg( feature = "tokio_ct" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for crate::TokioCt
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg( feature = "bindgen" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for crate::Bindgen
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for futures_executor::LocalSpawner
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg( feature = "threadpool" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for futures_executor::ThreadPool
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}


