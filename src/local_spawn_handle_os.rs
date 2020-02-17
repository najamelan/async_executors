#[ allow(unused_imports) ]
//
use
{
	futures_util :: { task::{ LocalSpawn, SpawnError }, future::FutureExt } ,
	crate        :: { LocalSpawnHandle, import::*                         } ,
	std          :: { pin::Pin, future::Future, sync::Arc, rc::Rc         } ,
};


/// Object safe version of [crate::SpawnHandle]. This allows you to take it as a param
/// and store it. It incurs some overhead, since the future needs to be boxed and executors
/// will box it again to queue it.
///
/// It also implies you have to choose an Out type.
//
pub trait LocalSpawnHandleOs<Out: 'static>
{
	/// Spawn a future and return a RemoteHandle that can be awaited for the output of the future.
	//
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>;
}



impl<T: ?Sized, Out> LocalSpawnHandleOs<Out> for Box<T> where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}



impl<T: ?Sized, Out> LocalSpawnHandleOs<Out> for Arc<T> where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}


impl<T: ?Sized, Out> LocalSpawnHandleOs<Out> for Rc<T> where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}


impl<T, Out> LocalSpawnHandleOs<Out> for &T where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}


impl<T, Out> LocalSpawnHandleOs<Out> for &mut T where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		(**self).spawn_handle_local_os( future )
	}
}


#[ cfg( feature = "tracing" ) ]
//
impl<T, Out> LocalSpawnHandleOs<Out> for Instrumented<T> where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		let fut = future.instrument( self.span().clone() );

		self.inner().spawn_handle_local_os( fut.boxed_local() )
	}
}


#[ cfg( feature = "tracing" ) ]
//
impl<T, Out> LocalSpawnHandleOs<Out> for WithDispatch<T> where T: LocalSpawnHandleOs<Out>, Out: 'static
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		let fut = self.with_dispatch(future);

		self.inner().spawn_handle_local_os( fut.boxed_local() )
	}
}



#[ cfg(any( feature = "tokio_ct" )) ]
//
impl<Out: 'static> LocalSpawnHandleOs<Out> for crate::TokioCt
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle_local( future )
	}
}



#[ cfg( feature = "bindgen" ) ]
//
impl<Out: 'static> LocalSpawnHandleOs<Out> for crate::Bindgen
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle_local( future )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl<Out: 'static> LocalSpawnHandleOs<Out> for futures_executor::LocalSpawner
{
	fn spawn_handle_local_os( &self, future: Pin<Box< dyn Future<Output = Out> >> ) -> Result<crate::JoinHandle<Out>, SpawnError>
	{
		self.spawn_handle_local( future )
	}
}
