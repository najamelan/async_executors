//! Provides TokioCt executor specific functionality.
//
use
{
	crate::{ import::*, JoinHandle, SpawnHandle, LocalSpawnHandle } ,

	tokio::runtime::{ Builder, Runtime, Handle as TokioRtHandle } ,

	// tokio_executor::
	// {
	// 	SpawnError    as TokioSpawnError ,

	// 	current_thread::
	// 	{
	// 		CurrentThread as TokioCtExec     ,
	// 		Handle        as TokioCtSpawner  ,
	// 		RunError      as TokioRunError   ,
	// 	},
	// },

	futures::task::{ SpawnExt, LocalSpawnExt } ,

	std::{ marker::PhantomData, sync::{ Arc, Mutex } },
};



/// An executor that uses [tokio_executor::current_thread::CurrentThread]
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioCt
{
	exec   : Arc<Mutex< Runtime >>,
	spawner: TokioCtHandle        ,
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
		TokioCtSendHandle::new( self.exec.lock().expect( "lock tokio_ct executor" ).handle().clone() )
	}
}




impl TryFrom<Builder> for TokioCt
{
	type Error = std::io::Error;

	fn try_from( mut builder: Builder ) -> Result<Self, Self::Error>
	{
		let exec = builder.basic_scheduler().build()?;
		let spawner = TokioCtHandle::new( exec.handle().clone() );

		Ok( Self { exec: Arc::new( Mutex::new( exec )), spawner } )
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
	fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

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
	fn spawn_handle_local<T: 'static>( &self, fut: impl Future< Output=T > + 'static )

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
	// We will solve this by crating a Send
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
		// This is necessary because tokio does not provide a clonable handle that can spawn !Send futures.
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

