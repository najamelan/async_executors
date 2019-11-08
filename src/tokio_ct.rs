//! Provides TokioCt executor specific functionality.
//
use
{
	crate::{ import::*, JoinHandle, SpawnHandle, LocalSpawnHandle } ,

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

	futures::task::{ SpawnExt, LocalSpawnExt } ,

	std::{ marker::PhantomData, sync::{ Arc, Mutex } },
};



/// An executor that uses [tokio_executor::current_thread::CurrentThread]
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioCt
{
	exec   : Arc<Mutex< TokioCtExec >>,
	spawner: TokioCtHandle            ,
}



impl TokioCt
{
	/// Run all spawned futures to completion. Note that this does nothing for the threadpool,
	/// but if you are using a local pool, you will need to run this or futures will not be polled.
	/// This blocks the current thread.
	//
	pub fn run( &mut self ) -> Result< (), TokioRunError >
	{
		self.exec.lock().expect( "lock tokio_ct executor" ).run()
	}


	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn trait.
	///
	/// Note that this handle is `Send` and can be sent to another thread to spawn tasks on the
	/// current executor, but as such, tasks are required to be `Send`. See [handle] for `!Send` futures.
	//
	pub fn send_handle( &self ) -> TokioCtSendHandle
	{
		TokioCtSendHandle::new( self.exec.lock().expect( "lock tokio_ct executor" ).handle() )
	}
}



impl Default for TokioCt
{
	fn default() -> Self
	{
		let exec = TokioCtExec::new();
		let spawner = TokioCtHandle::new( exec.handle() );

		Self { exec: Arc::new( Mutex::new( exec )), spawner }
	}
}


impl From<TokioCtExec> for TokioCt
{
	fn from( exec: TokioCtExec ) -> Self
	{
		let spawner = TokioCtHandle::new( exec.handle() );

		Self { exec: Arc::new( Mutex::new( exec )), spawner }
	}
}



impl LocalSpawn for TokioCt
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn_local_obj( future )
	}
}




impl Spawn for TokioCt
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		self.spawner.spawn_obj( future )
	}
}


impl SpawnHandle for TokioCt
{
	fn spawn_handle<T: 'static + Send>( &mut self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		let (tx, rx) = oneshot::channel();

		let task = async move
		{
			let t = fut.await;
			let _ = tx.send(t);
		};

		self.spawn( task ).unwrap();

		Ok( rx.into() )
	}
}


impl LocalSpawnHandle for TokioCt
{
	fn spawn_handle_local<T: 'static>( &mut self, fut: impl Future< Output=T > + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >

	{
		let (tx, rx) = oneshot::channel();

		let task = async move
		{
			let t = fut.await;
			let _ = tx.send(t);
		};

		self.spawn_local( task ).unwrap();

		Ok( rx.into() )
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


impl Spawn for TokioCtSendHandle
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
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




impl LocalSpawn for TokioCtHandle
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
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




impl Spawn for TokioCtHandle
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
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
