#![ allow(dead_code) ]
//
pub use
{
	futures         :: { FutureExt, SinkExt, channel::{ mpsc::Sender, oneshot }, executor::block_on      } ,
	futures::task   :: { LocalSpawnExt, SpawnExt, LocalSpawn, Spawn                                      } ,
	std             :: { convert::TryFrom                                                                } ,
	std             :: { sync::{ Arc, atomic::{ AtomicBool, Ordering::SeqCst } }, rc::Rc, time::Duration } ,
	async_executors :: { *                                                                               } ,
};


pub type DynResult<T>       = Result< T, Box<dyn std::error::Error + Send + Sync> >;
pub type DynResultNoSend<T> = Result< T, Box<dyn std::error::Error> >;


#[ cfg(not( target_arch = "wasm32" )) ]
//
pub mod tokio_io
{
	use
	{
		tokio::net :: { TcpListener, TcpStream      } ,
		tokio::io  :: { AsyncReadExt, AsyncWriteExt } ,

		super::*,
	};

	/// Creates a connected pair of sockets.
	/// Uses tokio tcp stream. This will only work if the reactor is running.
	//
	pub async fn socket_pair() -> DynResult< (TcpStream, TcpStream) >
	{
		// port 0 = let the OS choose
		//
		let listener = TcpListener::bind("127.0.0.1:0").await?;
		let stream1  = TcpStream::connect(listener.local_addr()?).await?;
		let stream2  = listener.accept().await?.0;

		Ok( (stream1, stream2) )
	}


	pub async fn tcp( exec: impl SpawnHandle< DynResult<()> > + TokioIo ) -> DynResult<()>
	{
		let test = async
		{
			let (mut one, mut two) = socket_pair().await?;

			one.write_u8( 5 ).await?;

			assert_eq!( 5, two.read_u8().await? );

			Ok(())
		};

		exec.spawn_handle( test )?.await
	}
}



async fn sum( a: u8, b: u8, mut tx: Sender<u8> )
{
	let res = tx.send( a + b ).await;

		assert!( res.is_ok() );
}


async fn sum_local( a: u8, b: u8, mut tx: Sender<u8> )
{
	// make sure we have something !Send in here.
	//
	let a = Rc::new(a);

	let res = tx.send( *a + b ).await;

		assert!( res.is_ok() );
}


async fn sum_handle( a: u8, b: u8 ) -> u8
{
	a + b
}


async fn sum_handle_local( a: u8, b: u8 ) -> Rc<u8>
{
	// Return something !Send
	//
	Rc::new( a + b )
}


// A function that takes a generic executor and spawns a task.
//
pub fn increment( a: u8, exec: impl Spawn, tx: Sender<u8> )
{
	let res = exec.spawn( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}


// A function that takes a generic executor and spawns a task.
//
#[ allow(dead_code) ] // gives warning when testing all executors at once.
//
pub fn increment_local( a: u8, exec: impl LocalSpawn, tx: Sender<u8> )
{
	let res = exec.spawn_local( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}


// A function that takes a generic executor and spawns a task.
//
pub fn increment_ref( a: u8, exec: &impl Spawn, tx: Sender<u8> )
{
	let res = exec.spawn( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}


// A function that takes a generic executor and spawns a task.
//
#[ allow(dead_code) ] // gives warning when testing all executors at once.
//
pub fn increment_ref_local( a: u8, exec: &impl LocalSpawn, tx: Sender<u8> )
{
	let res = exec.spawn_local( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}


// A function that takes a generic executor by value, clones it and spawns a task.
//
pub fn increment_clone( a: u8, exec: impl Spawn + Clone, tx: Sender<u8> )
{
	let second = exec.clone();
	drop( exec );

	let res = second.spawn( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}


// A function that takes a generic executor by value, clones it and spawns a task.
//
#[ allow(dead_code) ] // gives warning when testing all executors at once.
//
pub fn increment_clone_local( a: u8, exec: impl LocalSpawn + Clone, tx: Sender<u8> )
{
	let second = exec.clone();
	drop( exec );

	let res = second.spawn_local( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}


// A function that takes a generic executor and spawns a task.
//
#[ allow(dead_code) ]
//
pub async fn increment_spawn_handle( a: u8, exec: impl SpawnHandle<u8> ) -> u8
{
	exec.spawn_handle( sum_handle( a, 1 ) ).expect( "spawn handle" ).await
}


// A function that takes a trait object and spawns a task.
//
#[ allow(dead_code) ]
//
pub async fn increment_spawn_handle_os( a: u8, exec: &dyn SpawnHandle<u8> ) -> u8
{
	exec.spawn_handle( sum_handle( a, 1 ).boxed() ).expect( "spawn handle" ).await
}



// A function that takes a generic executor and spawns a task.
//
#[ allow(dead_code) ] // gives warning when testing all executors at once.
//
pub async fn increment_spawn_handle_local( a: u8, exec: impl LocalSpawnHandle<Rc<u8>> ) -> Rc<u8>
{
	exec.spawn_handle_local( sum_handle_local( a, 1 ) ).expect( "spawn handle" ).await
}

// A function that takes a trait object and spawns a task.
//
#[ allow(dead_code) ] // gives warning when testing all executors at once.
//
pub async fn increment_spawn_handle_local_os( a: u8, exec: &dyn LocalSpawnHandle<Rc<u8>> )-> Rc<u8>
{
	exec.spawn_handle_local( sum_handle_local( a, 1 ).boxed() ).expect( "spawn handle" ).await
}




// Verify timers work by making sure a shorter timer ends before the longer one.
// On top of that this shouldn't hang.
//
#[ cfg(not( target_arch = "wasm32" )) ]
//
pub async fn timer_should_wake_up( exec: impl SpawnHandle<()> + Clone + Timer + Send + Sync + 'static )
{
	let ex2    = exec.clone();
	let handle = exec.spawn_handle( async move
	{
		let fast = ex2.sleep( Duration::from_millis( 1) ).fuse();
		let slow = ex2.sleep( Duration::from_millis(80) ).fuse();

		futures_util::pin_mut!( fast );
		futures_util::pin_mut!( slow );

		let res = futures_util::select!
		{
			_ = slow => false,
			_ = fast => true,
		};

		assert!( res );

	}).expect( "spawn_handle" );

	handle.await;
}



// Verify timers work by making sure a shorter timer ends before the longer one.
// On top of that this shouldn't hang.
//
// This one is needed for executors that aren't Send, because we pass the executor into
// the spawned task and ThreadPools don't implement spawning !Send futures.
//
pub async fn timer_should_wake_up_local( exec: impl LocalSpawnHandle<()> + Clone + Timer + 'static )
{
	let ex2    = exec.clone();
	let handle = exec.spawn_handle_local( async move
	{
		let fast = ex2.sleep( Duration::from_millis( 1) ).fuse();
		let slow = ex2.sleep( Duration::from_millis(80) ).fuse();

		futures_util::pin_mut!( fast );
		futures_util::pin_mut!( slow );

		let res = futures_util::select!
		{
			_ = slow => false,
			_ = fast => true,
		};

		assert!( res );

	}).expect( "spawn_handle" );

	handle.await;
}




// Use timeout.
//
pub async fn timeout( exec: impl Timer )
{
	let fut = exec.sleep  ( Duration::from_millis(80)      );
	let fut = exec.timeout( Duration::from_millis(20), fut );

	assert!( fut.await.is_err() );
}


// Use timeout.
//
pub async fn dont_timeout( exec: impl Timer )
{
	let fut = exec.sleep  ( Duration::from_millis(20)      );
	let fut = exec.timeout( Duration::from_millis(80), fut );

	assert!( fut.await.is_ok() );
}



// Use same exec to run this function as you pass in.
//
pub async fn try_yield_now( exec: impl SpawnHandle<()> + YieldNow ) -> DynResult<()>
{
	let flag  = Arc::new( AtomicBool::new( false ) );
	let flag2 = flag.clone();

	let task = async move
	{
		flag2.store( true, SeqCst );
	};

	let handle = exec.spawn_handle( task )?;

	exec.yield_now().await;

	// by now task should have run because of the yield_now.
	//
	assert!( flag.load(SeqCst) );

	handle.await;

	Ok(())
}



// Use same exec to run this function as you pass in.
//
pub async fn without_yield_now( exec: impl SpawnHandle<()> + YieldNow ) -> DynResult<()>
{
	let flag  = Arc::new( AtomicBool::new( false ) );
	let flag2 = flag.clone();

	let task = async move
	{
		flag2.store( true, SeqCst );
	};

	let handle = exec.spawn_handle( task )?;

	// spawned task should not have run yet.
	//
	assert!( !flag.load(SeqCst) );

	handle.await;

	Ok(())
}



// Use same exec to run this function as you pass in.
//
pub async fn blocking( exec: impl SpawnBlocking<()> ) -> DynResult<()>
{
	let flag  = Arc::new( AtomicBool::new( false ) );
	let flag2 = flag.clone();

	let task = move ||
	{
		// blocking work
		//
		std::thread::sleep( Duration::from_millis( 5 ) );

		flag2.store( true, SeqCst );
	};

	let handle = exec.spawn_blocking( task );

	handle.await;

	assert!( flag.load(SeqCst) );

	Ok(())
}



// Use same exec to run this function as you pass in. This tests for
// the possibility of an object safe SpawnBlocking.
//
pub async fn blocking_void( exec: &dyn SpawnBlocking<()> ) -> DynResult<()>
{
	let flag  = Arc::new( AtomicBool::new( false ) );
	let flag2 = flag.clone();

	let task = move ||
	{
		// blocking work
		//
		std::thread::sleep( Duration::from_millis( 5 ) );

		flag2.store( true, SeqCst );
	};

	let handle = exec.spawn_blocking_dyn( Box::new(task) );

	handle.await;

	assert!( flag.load(SeqCst) );

	Ok(())
}
