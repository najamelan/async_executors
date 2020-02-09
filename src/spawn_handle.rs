use
{
	futures_util :: { future::{ FutureExt, RemoteHandle }, task::{ SpawnExt, LocalSpawnExt } } ,
	futures_task :: { SpawnError                                                             } ,
	std::future  :: { Future                                                                 } ,
};


/// An extension trait that automatically returns you a RemoteHandle to await the
/// output of a future.
//
pub trait SpawnHandle : SpawnExt
{
	/// Spawn a future and return a RemoteHandle that can be awaited for the output of the future.
	//
	fn spawn_handle<Fut, T>(&self, future: Fut) -> Result<RemoteHandle<T>, SpawnError>

		where Fut: Future<Output = T> + Send + 'static,
		      T  : Send + 'static
	{
		let (fut, handle) = future.remote_handle();

		self.spawn( fut )?;

		Ok( handle )
	}
}


/// An extension trait that automatically returns you a RemoteHandle to await the
/// output of a future.
//
pub trait LocalSpawnHandle : LocalSpawnExt
{
	/// Spawn a future and return a RemoteHandle that can be awaited for the output of the future.
	//
	fn spawn_handle_local<Fut, T>(&self, future: Fut) -> Result<RemoteHandle<T>, SpawnError>

		where Fut: Future<Output = T> + 'static,
		      T  : Send + 'static
	{
		let (fut, handle) = future.remote_handle();

		self.spawn_local( fut )?;

		Ok( handle )
	}
}


impl<T> SpawnHandle for T

	where T: SpawnExt
{}


impl<T> LocalSpawnHandle for T

	where T: LocalSpawnExt
{}
