use
{
	crate :: { import::* } ,
	std   :: { future::Future } ,
};


/// An executor that spawns tasks on async-std. In contrast to the other executors, this one
/// is not self contained, because async-std does not provide an API that allows that.
/// So the threadpool is global.
//
#[ derive( Copy, Clone, Default ) ]
//
pub struct AsyncStd;

impl AsyncStd
{
	/// Wrapper around [::async_std::task::block_on].
	//
	pub fn block_on<F: Future>(future: F) -> F::Output
	{
		async_std_crate::task::block_on( future )
	}
}



impl Spawn for AsyncStd
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
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
