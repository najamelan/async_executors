//! Provides TokioCt executor specific functionality.
//
use
{
	crate::import::*,

	tokio_executor::
	{
		SpawnError    as TokioSpawnError ,

		current_thread::
		{
			CurrentThread as TokioCtExec     ,
			Handle        as TokioCtSpawner  ,
			RunError      as TokioRunError   ,
		},
	},


	std::marker::PhantomData,
};



/// An executor that uses [tokio_executor::current_thread::CurrentThread]
//
#[ derive( Debug ) ]
//
pub struct TokioCt
{
	exec: TokioCtExec,
}



impl TokioCt
{
	/// Create a new TokioCt.
	//
	pub fn new() -> Self
	{
		Self::default()
	}


	/// Run all spawned futures to completion. Note that this does nothing for the threadpool,
	/// but if you are using a local pool, you will need to run this or futures will not be polled.
	/// This blocks the current thread.
	//
	pub fn run( &mut self ) -> Result< (), TokioRunError >
	{
		self.exec.run()
	}


	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn and LocalSpawn traits.
	//
	pub fn handle( &self ) -> TokioCtHandle
	{
		TokioCtHandle::new( self.exec.handle() )
	}


	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn trait.
	///
	/// Note that this handle is `Send` and can be sent to another thread to spawn tasks on the
	/// current executor, but as such, tasks are required to be `Send`. See [handle] for `!Send` futures.
	//
	pub fn send_handle( &self ) -> TokioCtSendHandle
	{
		TokioCtSendHandle::new( self.exec.handle() )
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



impl Default for TokioCt
{
	fn default() -> Self
	{
		Self { exec: TokioCtExec::new() }
	}
}


impl From<TokioCtExec> for TokioCt
{
	fn from( exec: TokioCtExec ) -> Self
	{
		Self { exec }
	}
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



//------------------------------------------------------------------------ SendHandle
//
//
/// A handle to this localpool that can easily be cloned and that implements
/// Spawn and LocalSpawn traits.
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioCtSendHandle
{
	spawner: TokioCtSpawner,
}


impl TokioCtSendHandle
{
	pub(crate) fn new( spawner: TokioCtSpawner ) -> Self
	{
		Self { spawner }
	}
}


impl futures::task::Spawn for TokioCtSendHandle
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn( future ).map_err( tok_to_fut_spawn_error )
	}
}




//------------------------------------------------------------------------ Handle
//
//
/// A handle to this localpool that can easily be cloned and that implements
/// Spawn and LocalSpawn traits.
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioCtHandle
{
	spawner : TokioCtSpawner,

	// This handle must not be Send. We want to be able to impl LocalSpawn for it, but tokio does not
	// provide us with the API to do so as their handle is Send and requires Send on the futures.
	//
	// We will solve this by crating a Send
	//
	_no_send: PhantomData<*mut fn()> ,
}


impl TokioCtHandle
{
	pub(crate) fn new( spawner: TokioCtSpawner ) -> Self
	{
		Self { spawner, _no_send: PhantomData::default() }
	}
}




impl futures::task::LocalSpawn for TokioCtHandle
{
	fn spawn_local_obj( &mut self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We transform the LocalFutureObj into a FutureObj. Just magic!
		//
		// This is safe because TokioCtHandle is not Send, so it can never venture to another thread than the
		// current_thread executor it's created from.
		//
		// This is necessary because tokio does not provide a clonable handle that can spawn !Send futures.
		//
		let fut;

		unsafe
		{
			fut = future.into_future_obj();
		}

		self.spawner.spawn( fut ).map_err( tok_to_fut_spawn_error )
	}
}




impl futures::task::Spawn for TokioCtHandle
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn( future ).map_err( tok_to_fut_spawn_error )
	}
}



/// Convert a tokio SpawnError to a futures-rs SpawnError.
///
/// The tokio error can also be `at_capacity`, but the futures one only supports shutdown,
/// so we haven't much choice here.
//
fn tok_to_fut_spawn_error( _e: TokioSpawnError  ) -> FutSpawnErr
{
	FutSpawnErr::shutdown()
}
