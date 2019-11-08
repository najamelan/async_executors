//! Provides TokioTp executor specific functionality.
//
use
{
	crate::{ import::*, JoinHandle, SpawnHandle },

	tokio_executor::
	{
		thread_pool::
		{
			ThreadPool as TokioTpExec    ,
			Spawner    as TokioTpSpawner ,
		},
	},

	futures::task::SpawnExt,

	std :: { sync::{ Arc, Mutex } },
};


/// An executor that uses [tokio_executor::thread_pool::ThreadPool]
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioTp
{
	exec   : Arc<Mutex< TokioTpExec >> ,
	spawner: TokioTpSpawner            ,
}


impl Default for TokioTp
{
	fn default() -> Self
	{
		let exec = TokioTpExec::new();
		let spawner = exec.spawner().clone();

		Self { exec: Arc::new( Mutex::new( exec )), spawner }
	}
}


impl From<TokioTpExec> for TokioTp
{
	fn from( exec: TokioTpExec ) -> Self
	{
		let spawner = exec.spawner().clone();

		Self { exec: Arc::new( Mutex::new( exec )), spawner }
	}
}



impl Spawn for TokioTp
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// Impl in tokio is actually infallible, so no point in converting the error type.
		//
		self.spawner.spawn( future );

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

		// impl in tokio is infallible.
		//
		let _ = self.spawn( task );

		Ok( rx.into() )
	}
}


