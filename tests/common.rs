#![ allow(dead_code) ]
//
pub use
{
	futures         :: { FutureExt, SinkExt, channel::{ mpsc::Sender, oneshot }, executor::block_on } ,
	futures::task   :: { LocalSpawnExt, SpawnExt, LocalSpawn, Spawn                                 } ,
	std             :: { sync::Arc, rc::Rc, time::Duration                                          } ,
	async_executors :: { *                                                                          } ,
};


pub type DynError = Box<dyn std::error::Error + Send + Sync>;


#[ cfg(any( feature = "tokio_io", feature = "async_global_tokio", feature = "async_std_tokio" )) ]
//
pub mod tokio_io
{
	use
	{
		tokio::net::{ TcpListener, TcpStream },
		super::*,
	};

	/// Creates a connected pair of sockets.
	///
	/// This is similar to UnixStream::socket_pair, but works on windows too.
	//
	pub async fn socket_pair() -> Result<(TcpStream, TcpStream), DynError>
	{
		// port 0 = let the OS choose
		//
		let listener = TcpListener::bind("127.0.0.1:0").await?;
		let stream1  = TcpStream::connect(listener.local_addr()?).await?;
		let stream2  = listener.accept().await?.0;

		Ok( (stream1, stream2) )
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
#[ allow(dead_code) ]
//
pub async fn increment_spawn_handle_os( a: u8, exec: &dyn SpawnHandle<u8> ) -> u8
{
	exec.spawn_handle( sum_handle( a, 1 ).boxed() ).expect( "spawn handle" ).await
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
