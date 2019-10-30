//! Provides TokioTp executor specific functionality.
//
use
{
	crate::import::*,

	tokio_executor::
	{
		Executor,

		thread_pool::
		{
			ThreadPool as TokioTpExec    ,
			Spawner    as TokioTpSpawner ,
		},
	},
};


/// An executor that uses [tokio_executor::thread_pool::ThreadPool]
//
#[ derive( Debug ) ]
//
pub struct TokioTp
{
	exec: TokioTpExec,
}



impl TokioTp
{
	/// Create a new TokioTp.
	//
	pub fn new() -> Self
	{
		Self::default()
	}


	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn and LocalSpawn traits.
	//
	pub fn handle( &self ) -> TokioTpHandle
	{
		TokioTpHandle::new( self.exec.spawner().clone() )
	}


	// pub fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

	// 	-> Result< Box< dyn Future< Output=T > + Send + 'static + Unpin >, Error >

	// {
	// 	let (fut, handle) = fut.remote_handle();

	// 	self.exec.spawn_local( fut )?;
	// 	Ok(Box::new( handle ))
	// }
}



impl Default for TokioTp
{
	fn default() -> Self
	{
		Self { exec: TokioTpExec::new() }
	}
}


impl From<TokioTpExec> for TokioTp
{
	fn from( exec: TokioTpExec ) -> Self
	{
		Self { exec }
	}
}



impl futures::task::Spawn for TokioTp
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// Tokio ThreadPool also has a spawn function which returns a JoinHandle, and when not
		// keeping the JoinHandle, the task is immediately dropped, so it's important we use the
		// spawn function from the Executor trait.
		//
		// Impl in tokio is actually infallible, so no point in converting the error type.
		//
		let _ = <&TokioTpExec as Executor>::spawn( &mut &self.exec, Box::pin( future ) );

		Ok(())
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
pub struct TokioTpHandle
{
	spawner : TokioTpSpawner,
}


impl TokioTpHandle
{
	pub(crate) fn new( spawner: TokioTpSpawner ) -> Self
	{
		Self { spawner }
	}
}



impl futures::task::Spawn for TokioTpHandle
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn( future );
		Ok(())
	}
}
