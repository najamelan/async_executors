//! Provides TokioTp executor specific functionality.
//
use
{
	crate          :: { import::*, tokio_handle::TokioHandle } ,
	tokio::runtime :: { Builder, Runtime                     } ,
};


/// An executor that uses [tokio_executor::thread_pool::ThreadPool]
//
#[ derive( Debug ) ]
//
pub struct TokioTp
{
	exec: Runtime,
}


impl TokioTp
{
	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn trait.
	///
	/// Note that this handle is `Send` and can be sent to another thread to spawn tasks on the
	/// current executor, but as such, tasks are required to be `Send`. See [handle] for `!Send` futures.
	//
	pub fn handle( &self ) -> TokioHandle
	{
		TokioHandle::new( self.exec.handle().clone() )
	}


	/// This is the entry point for this executor. You must call spawn on the handle from within a future that is run with block_on.
	//
	pub fn block_on< F: Future >( &mut self, f: F ) -> F::Output
	{
		self.exec.block_on( f )
	}
}



impl TryFrom<&mut Builder> for TokioTp
{
	type Error = std::io::Error;

	fn try_from( builder: &mut Builder ) -> Result<Self, Self::Error>
	{
		let exec    = builder.threaded_scheduler().build()?;

		Ok( Self { exec } )
	}
}
