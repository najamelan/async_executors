//! Provides ThreadPool executor specific functionality.

use
{
	crate   :: { import::*, JoinHandle, SpawnHandle        } ,
	futures :: { executor::{ ThreadPool as FutThreadPool } } ,
};


/// An executor that uses [futures 0.3 ThreadPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/executor/struct.ThreadPool.html).
///
/// This does not implement [`Default`] because the constructor of the futures ThreadPool is fallible. Thus
/// we have a `new()` function that returns a [`Result`].
//
#[ derive( Debug, Clone ) ]
//
pub struct ThreadPool
{
	pool: FutThreadPool,
}



impl ThreadPool
{
	/// Create a new ThreadPool. This operation is fallible if the OS fails to spawn
	/// threads. I haven't yet figured out which io::ErrorKind this will throw as it's
	/// not documented in std.
	//
	pub fn new() -> Result< Self, futures::io::Error >
	{
		Ok( Self { pool: FutThreadPool::new()? } )
	}
}


impl From<FutThreadPool> for ThreadPool
{
	/// Create a new ThreadPool from an existing ThreadPool from the futures library. This allows
	/// you to use the ThreadPoolBuilder to change the default configuration.
	//
	fn from( pool: FutThreadPool ) -> Self
	{
		Self { pool }
	}
}



impl Spawn for ThreadPool
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.pool.spawn_obj( future )
	}
}


impl SpawnHandle for ThreadPool
{
	fn spawn_handle<T: 'static + Send>( &mut self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		let (tx, rx) = oneshot::channel();

		let task = async move
		{
			let t = fut.await;
			let _ = tx.send(t);
		};

		self.pool.spawn_ok( task );

		Ok( rx.into() )
	}
}

