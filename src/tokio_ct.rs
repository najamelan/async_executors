//! Provides TokioCt executor specific functionality.
//
use
{
	crate          :: { import::*, TokioHandle, TokioLocalHandle } ,
	tokio::runtime :: { Builder, Runtime                         } ,
	std            :: { marker::PhantomData                      } ,
};



/// An executor that uses a [tokio::runtime::Runtime] with the [basic scheduler](tokio::runtime::Builder::basic_scheduler).
//
#[ derive( Debug ) ]
//
pub struct TokioCt
{
	exec: Runtime,

	// This must not be Send. We allow creating TokioCtHandle which must be in the same thread
	// as the runtime, so the runtime needs to be pinned to the thread it is created in.
	//
	_no_send: PhantomData<*mut fn()> ,
}



impl TokioCt
{
	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn trait.
	///
	/// Note that this handle is `Send` and can be sent to another thread to spawn tasks on the
	/// current executor, but as such, tasks are required to be `Send`. See [handle] for `!Send` futures.
	///
	/// __Please read the documentation for [TokioHandle] about unwind safety.__
	//
	pub fn handle( &self ) -> TokioHandle
	{
		TokioHandle::new( self.exec.handle().clone() )
	}

	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn and SpawnLocal traits.
	///
	/// Note that this handle is `!Send` and cannot be sent to another thread. It allows spawning
	/// futures that are !Send.
	///
	/// __Please read the documentation for [TokioLocalHandle] about unwind safety.__
	//
	pub fn local_handle( &self ) -> TokioLocalHandle
	{
		TokioLocalHandle::new( self.exec.handle().clone() )
	}

	/// This is the entry point for this executor. You must call spawn on the handle from within a future that is run with block_on.
	//
	pub fn block_on< F: Future >( &mut self, f: F ) -> F::Output
	{
		self.exec.block_on( f )
	}
}




impl TryFrom<&mut Builder> for TokioCt
{
	type Error = std::io::Error;

	fn try_from( builder: &mut Builder ) -> Result<Self, Self::Error>
	{
		let exec = builder.basic_scheduler().build()?;

		Ok( Self
		{
			 exec                  ,
			_no_send : PhantomData ,
		})
	}
}


#[ cfg(test) ]
//
mod tests
{
	use super::*;

	static_assertions::assert_not_impl_any!( TokioCt: Send, Sync );
}
