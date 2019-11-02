//! Provides localpool executor specific functionality.

use
{
	crate   :: { import::*, JoinHandle, SpawnHandle, LocalSpawnHandle  } ,
	futures :: { task::{ SpawnExt, LocalSpawnExt }                     } ,
	futures :: { executor::{ LocalPool as FutLocalPool, LocalSpawner } } ,
};


/// An executor that uses [futures 0.3 LocalPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.19/futures/executor/struct.LocalPool.html).
//
#[ derive( Debug ) ]
//
pub struct LocalPool
{
	pool   : FutLocalPool,
	spawner: LocalSpawner,
}



impl LocalPool
{
	/// Create a new LocalPool.
	//
	pub fn new() -> Self
	{
		Self::default()
	}


	/// Obtain a handle to this executor that can easily be cloned and that implements
	/// Spawn and LocalSpawn traits.
	//
	pub fn handle( &self ) -> LocalPoolHandle
	{
		LocalPoolHandle::new( self.spawner.clone() )
	}


	/// Run all spawned futures to completion. Note that this does nothing for the threadpool,
	/// but if you are using a local pool, you will need to run this or futures will not be polled.
	/// This blocks the current thread.
	//
	pub fn run( &mut self )
	{
		self.pool.run()
	}


	///
	//
	pub fn run_until<F: Future>( &mut self, future: F ) -> <F as Future>::Output
	{
		self.pool.run_until( future )
	}


	///
	//
	pub fn try_run_one( &mut self ) -> bool
	{
		self.pool.try_run_one()
	}


	///
	//
	pub fn run_until_stalled( &mut self )
	{
		self.pool.run_until_stalled()
	}
}



impl Default for LocalPool
{
	fn default() -> Self
	{
		let pool    = FutLocalPool::new();
		let spawner = pool.spawner();

		Self { pool, spawner }
	}
}


impl From<FutLocalPool> for LocalPool
{
	fn from( pool: FutLocalPool ) -> Self
	{
		Self { spawner: pool.spawner(), pool }
	}
}


impl LocalSpawn for LocalPool
{
	fn spawn_local_obj( &mut self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn_local_obj( future )
	}
}




impl Spawn for LocalPool
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn_obj( future )
	}
}


impl SpawnHandle for LocalPool
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

		self.spawner.spawn( task )?;

		Ok( rx.into() )
	}
}


impl LocalSpawnHandle for LocalPool
{
	fn spawn_handle_local<T: 'static>( &mut self, fut: impl Future< Output=T > + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		let (tx, rx) = oneshot::channel();

		let task = async move
		{
			let t = fut.await;
			let _ = tx.send(t);
		};

		self.spawner.spawn_local( task )?;

		Ok( rx.into() )
	}
}



/// A handle to this localpool that can easily be cloned and that implements
/// Spawn and LocalSpawn traits.
//
#[ derive( Debug, Clone ) ]
//
pub struct LocalPoolHandle
{
	spawner: LocalSpawner,
}


impl LocalPoolHandle
{
	pub(crate) fn new( spawner: LocalSpawner ) -> Self
	{
		Self { spawner }
	}
}




impl LocalSpawn for LocalPoolHandle
{
	fn spawn_local_obj( &mut self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn_local_obj( future )
	}
}




impl Spawn for LocalPoolHandle
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn_obj( future )
	}
}

