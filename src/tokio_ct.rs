//! Provides TokioCt executor specific functionality.
//
use
{
	crate          :: { import::*                                 } ,
	tokio::runtime :: { Builder, Runtime, Handle as TokioRtHandle } ,
	std            :: { marker::PhantomData, future::Future       } ,
};



/// An executor that uses [tokio_executor::current_thread::CurrentThread]
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
	//
	pub fn send_handle( &self ) -> TokioCtSendHandle
	{
		TokioCtSendHandle::new( self.exec.handle().clone() )
	}

	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn and SpawnLocal traits.
	///
	/// Note that this handle is `!Send` and cannot be sent to another thread. It allows spawning
	/// futures that are !Send.
	//
	pub fn handle( &self ) -> TokioCtHandle
	{
		TokioCtHandle::new( self.exec.handle().clone() )
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
	spawner: TokioRtHandle,
}


impl TokioCtSendHandle
{
	pub(crate) fn new( spawner: TokioRtHandle ) -> Self
	{
		Self { spawner }
	}
}


impl Spawn for TokioCtSendHandle
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn( future );
		Ok(())
	}
}



//------------------------------------------------------------------------ Handle
//
/// A handle to this localpool that can easily be cloned and that implements
/// Spawn and LocalSpawn traits.
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioCtHandle
{
	spawner : TokioRtHandle,

	// This handle must not be Send. We want to be able to impl LocalSpawn for it, but tokio does not
	// provide us with the API to do so as their handle is Send and requires Send on the futures.
	//
	_no_send: PhantomData<*mut fn()> ,
}


impl TokioCtHandle
{
	pub(crate) fn new( spawner: TokioRtHandle ) -> Self
	{
		Self { spawner, _no_send: PhantomData::default() }
	}
}




impl LocalSpawn for TokioCtHandle
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We transform the LocalFutureObj into a FutureObj. Just magic!
		//
		// This is safe because TokioCtHandle is not Send, so it can never venture to another thread than the
		// current_thread executor it's created from.
		//
		// This is necessary because tokio does not provide a handle that can spawn !Send futures.
		//
		let fut;

		unsafe
		{
			fut = future.into_future_obj();
		}

		self.spawner.spawn( fut );
		Ok(())
	}
}


impl Spawn for TokioCtHandle
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn( future );
		Ok(())
	}
}

