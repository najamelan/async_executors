use
{
	crate :: { import::* } ,
};


/// We currently only support a global AsyncStd threadpool. In principle this is the only supported
/// executor that allows full control. We could expose an interface that allows users to control
/// the lifetime and scope of a AsyncStd threadpool.
//
#[ derive( Clone ) ]
//
pub struct AsyncStd {}


impl AsyncStd
{
	/// Create a new AsyncStd threadpool.
	//
	pub fn new() -> Self
	{
		Self{}
	}



	/// Obtain a handle to this executor that can easily be cloned and that implements
	/// Spawn the trait.
	//
	pub fn handle( &self ) -> AsyncStd
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



impl futures::task::Spawn for AsyncStd
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		async_std_crate::task::spawn( future );

		Ok(())
	}
}



impl std::fmt::Debug for AsyncStd
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "AsyncStd threadpool" )
	}
}
