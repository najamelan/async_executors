use
{
	crate :: { import::*, JoinHandle, SpawnHandle } ,
};


/// We currently only support a global juliex threadpool. In principle this is the only supported
/// executor that allows full control. We could expose an interface that allows users to control
/// the lifetime and scope of a juliex threadpool.
//
#[ derive( Clone ) ]
//
pub struct Juliex
{
	pool: juliex_crate::ThreadPool
}



impl Default for Juliex
{
	fn default() -> Self
	{
		Self
		{
			pool: juliex_crate::ThreadPool::new()
		}
	}
}



impl From<juliex_crate::ThreadPool> for Juliex
{
	/// Create a new ThreadPool from an existing juliex ThreadPool.
	//
	fn from( pool: juliex_crate::ThreadPool ) -> Self
	{
		Self { pool }
	}
}



impl Spawn for Juliex
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.pool.spawn( future );

		Ok(())
	}
}



impl SpawnHandle for Juliex
{
	fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		let (tx, rx) = oneshot::channel();

		let task = async move
		{
			let t = fut.await;
			let _ = tx.send(t);
		};

		self.pool.spawn( task );

		Ok( rx.into() )
	}
}



impl std::fmt::Debug for Juliex
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "Juliex threadpool" )
	}
}
