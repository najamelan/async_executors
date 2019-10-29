//! Provides ThreadPool executor specific functionality.

use
{
	crate   :: { import::*                                 } ,
	futures :: { executor::{ ThreadPool as FutThreadPool } } ,
};


/// An executor that uses [futures 0.3 ThreadPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/executor/struct.ThreadPool.html).
//
#[ derive( Debug, Clone ) ]
//
pub struct ThreadPool
{
	pool: FutThreadPool,
}



impl ThreadPool
{
	/// Create a new ThreadPool from an existing ThreadPool from the futures library. This allows
	/// you to use the ThreadPoolBuilder to change the default configuration. If you just want
	/// default configuration, use [`Default::default`].
	//
	pub fn new() -> Result< Self, futures::io::Error >
	{
		Ok( Self { pool: FutThreadPool::new()? } )
	}




	/// Obtain a handle to this executor that can easily be cloned and that implements
	/// Spawn the trait.
	//
	pub fn handle( &self ) -> ThreadPool
	{
		self.clone()
	}


	/// Spawn a future, keeping a handle to await it's completion and recover the returned value.
	/// Dropping the handle cancels the future instantly.
	//
	pub fn spawn_handle<T: 'static + Send>( &mut self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< Box< dyn Future< Output=T > + Send + 'static + Unpin >, FutSpawnErr >

	{
		let (fut, handle) = fut.remote_handle();

		self.spawn( fut )?;
		Ok(Box::new( handle ))
	}
}



impl futures::task::Spawn for ThreadPool
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.pool.spawn_obj( future )
	}
}


impl From<FutThreadPool> for ThreadPool
{
	fn from( pool: FutThreadPool ) -> Self
	{
		Self { pool }
	}
}

