//! Provides TokioTp executor specific functionality.
//
use
{
	crate::{ import::*, JoinHandle, SpawnHandle },

	tokio_executor::
	{
		Executor,

		thread_pool::
		{
			ThreadPool as TokioTpExec    ,
			Spawner    as TokioTpSpawner ,
		},
	},

	futures::task::SpawnExt
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



impl Spawn for TokioTp
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



impl SpawnHandle for TokioTp
{
	fn spawn_handle<T: 'static + Send>( &mut self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		// Even though the tokio threadpool has a JoinHandle, we use a oneshot::channel here because
		// the JoinHandle requires return types to be Send, which gives trouble if we want to use our
		// JoinHandle impl for current thread executors.
		// TODO: does this affect performance?
		//
		let (tx, rx) = oneshot::channel();

		let task = async move
		{
			let t = fut.await;
			let _ = tx.send(t);
		};

		self.spawn( task );

		Ok( rx.into() )
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



impl Spawn for TokioTpHandle
{
	fn spawn_obj( &mut self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn( future );
		Ok(())
	}
}
