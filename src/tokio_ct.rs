//! Provides TokioCt executor specific functionality.

use crate :: { import::* };


/// An executor that uses [tokio::runtime::current_thread::Runtime]
//
#[ derive( Debug ) ]
//
pub struct TokioCt
{
	exec: TokioCtExec,
}



impl TokioCt
{
	/// Create a new TokioCt from an [Config](crate::Config) configuration.
	//
	pub fn new() -> Self
	{
		Self { exec: TokioCtExec::new() }
	}


	/// Run all spawned futures to completion. Note that this does nothing for the threadpool,
	/// but if you are using a local pool, you will need to run this or futures will not be polled.
	/// This blocks the current thread.
	//
	pub fn run( &mut self ) -> Result< (), TokioRunError >
	{
		self.exec.run()
	}


	// pub fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

	// 	-> Result< Box< dyn Future< Output=T > + Send + 'static + Unpin >, Error >

	// {
	// 	let (fut, handle) = fut.remote_handle();

	// 	self.exec.spawn_local( fut )?;
	// 	Ok(Box::new( handle ))
	// }



	// pub fn spawn_handle_local<T: 'static + Send>( &self, fut: impl Future< Output=T > + 'static )

	// 	-> Result< Box< dyn Future< Output=T > + 'static + Unpin >, Error >
	// {
	// 	let (fut, handle) = fut.remote_handle();

	// 	self.exec.spawn_local( fut )?;
	// 	Ok(Box::new( handle ))
	// }
}



impl futures::task::LocalSpawn for TokioCt
{
	fn spawn_local_obj( &mut self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.exec.spawn( future );
		Ok(())
	}
}




impl futures::task::Spawn for TokioCt
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.exec.spawn( future );
		Ok(())
	}
}

