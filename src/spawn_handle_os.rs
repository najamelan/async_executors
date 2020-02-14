#[ allow(unused_imports) ]
//
use
{
	futures_util :: { task::{ Spawn, SpawnError }                 } ,
	crate        :: { SpawnHandle                                 } ,
	std          :: { pin::Pin, future::Future, sync::Arc, rc::Rc } ,
};


/// Object safe version of [crate::SpawnHandle]. This allows you to take it as a param
/// and store it. It incurs some overhead, since the future needs to be boxed and executors
/// will box it again to queue it.
///
/// It also implies you have to choose an Out type.
//
pub trait SpawnHandleOs<Out: 'static + Send> : Spawn
{
	/// Spawn a future and return a RemoteHandle that can be awaited for the output of the future.
	//
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>;
}


impl<T, Out> SpawnHandleOs<Out> for Arc<T> where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}


impl<T, Out> SpawnHandleOs<Out> for Rc<T> where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}


impl<T, Out> SpawnHandleOs<Out> for &T where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}


impl<T, Out> SpawnHandleOs<Out> for &mut T where T: SpawnHandleOs<Out>, Out: 'static + Send
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_os( future )
	}
}



#[ cfg( feature = "async_std" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for crate::async_std::AsyncStd
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for crate::TokioHandle
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg(any( feature = "tokio_ct" )) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for crate::TokioLocalHandle
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg( feature = "bindgen" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for crate::Bindgen
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for futures_executor::LocalSpawner
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}



#[ cfg( feature = "threadpool" ) ]
//
impl<Out: 'static + Send> SpawnHandleOs<Out> for futures_executor::ThreadPool
{
	fn spawn_handle_os( &self, future: Pin<Box< dyn Future<Output = Out> + Send >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle( future )
	}
}


