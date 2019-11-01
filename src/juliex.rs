use
{
	crate :: { import::* } ,
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


impl Juliex
{
	/// Create a new Juliex threadpool.
	//
	pub fn new() -> Self
	{
		Self
		{
			pool: juliex_crate::ThreadPool::new()
		}
	}



	/// Obtain a handle to this executor that can easily be cloned and that implements
	/// Spawn the trait.
	//
	pub fn handle( &self ) -> Juliex
	{
		self.clone()
	}


	// pub(crate) fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

	// 	-> Result< Box< dyn Future< Output=T > + Send + 'static + Unpin >, Error >

	// {
	// 	let (fut, handle) = fut.remote_handle();

	// 	self.spawn( fut )?;
	// 	Ok(Box::new( handle ))
	// }



	// pub(crate) fn spawn_handle_local<T: 'static + Send>( &self, _: impl Future< Output=T > + 'static )

	// 	-> Result< Box< dyn Future< Output=T > + 'static + Unpin >, Error >

	// {
	// 	Err( ErrorKind::SpawnLocalOnThreadPool.into() )
	// }
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



impl futures::task::Spawn for Juliex
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.pool.spawn( future );

		Ok(())
	}
}



impl std::fmt::Debug for Juliex
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "Juliex threadpool" )
	}
}
