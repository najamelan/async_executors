use
{
	crate :: { import::*, JoinHandle, SpawnHandle } ,
};


/// An executor that spawns tasks on async-std. In contrast to the other executors, this one
/// is not self contained, because async-std does not provide an API that allows that.
/// So the threadpool is global.
//
#[ derive( Clone ) ]
//
pub struct AsyncStd {}


impl AsyncStd
{
	/// Create a new AsyncStd executor (note that this is a NOOP because async-std only has
	/// a global spawn function).
	//
	pub fn new() -> Self
	{
		Self{}
	}



	/// Obtain a handle to this executor that can easily be cloned and that implements
	/// Spawn the trait. This one is zero-size.
	//
	pub fn handle( &self ) -> AsyncStd
	{
		self.clone()
	}
}



impl Spawn for AsyncStd
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		async_std_crate::task::spawn( future );

		Ok(())
	}
}



impl SpawnHandle for AsyncStd
{
	fn spawn_handle<T: 'static + Send>( &mut self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		Ok( async_std_crate::task::spawn( fut ).into() )
	}
}



impl std::fmt::Debug for AsyncStd
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "AsyncStd threadpool" )
	}
}
