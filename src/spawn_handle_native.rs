use
{
	futures_util :: { task::{ SpawnExt, SpawnError } } ,
	std          :: { future::Future                 } ,
};


/// An extension trait that automatically returns you a RemoteHandle to await the
/// output of a future.
//
pub trait SpawnHandleNative : SpawnExt
{
	/// Spawn a future and return a RemoteHandle that can be awaited for the output of the future.
	//
	fn spawn_handle_native<Fut, T>(&self, future: Fut) -> Result<crate::JoinHandle<T>, SpawnError>

		where Fut: Future<Output = T> + 'static + Send,
		      T  : 'static + Send
	;
}



#[ cfg( feature = "async_std" ) ]
//
impl SpawnHandleNative for crate::async_std::AsyncStd
{
	fn spawn_handle_native<Fut, T>(&self, future: Fut) -> Result<crate::JoinHandle<T>, SpawnError>

		where Fut: Future<Output = T> + 'static + Send,
		      T  : 'static + Send

	{
		Ok(crate::JoinHandle::from( async_std_crate::task::spawn( future ) ))
	}
}



#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
//
impl SpawnHandleNative for crate::TokioHandle
{
	fn spawn_handle_native<Fut, T>(&self, future: Fut) -> Result<crate::JoinHandle<T>, SpawnError>

		where Fut: Future<Output = T> + 'static + Send,
		      T  : 'static + Send

	{
		Ok(crate::JoinHandle::from( self.spawner.spawn( future ) ))
	}
}


