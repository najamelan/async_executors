#![ cfg(all( feature = "async_std", not(target_os = "unknown") )) ]

// Tested:
//
// ✔ pass a     AsyncStd  to a function that takes exec:  `impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `impl Spawn + Clone`
// ✔ pass a Arc<AsyncStd> to a function that takes exec:  `impl Spawn`
// ✔ pass a     AsyncStd  to a function that takes exec:  `impl SpawnHandle`
// ✔ pass a Arc<AsyncStd> to a function that takes exec:  `impl SpawnHandle`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `&dyn SpawnHandle`
//
// ✔ pass a     AsyncStd  to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `&impl LocalSpawn`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `impl LocalSpawn + Clone`
// ✔ pass a Arc<AsyncStd> to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a     AsyncStd  to a function that takes exec:  `impl LocalSpawnHandle`
// ✔ pass a Arc<AsyncStd> to a function that takes exec:  `impl LocalSpawnHandle`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `&dyn LocalSpawnHandle`
//
// ✔ pass an AsyncStd to a function that requires a Timer.
//
// ✔ Joinhandle::detach allows task to keep running.
// ✔ Joinhandle::drop aborts the task.
//
mod common;

use
{
	common          :: { *                        } ,
	futures         :: { channel::mpsc, StreamExt } ,
	std             :: { time::Duration           } ,
	futures_timer   :: { Delay                    } ,
	async_std_crate as async_std,
};


// pass a AsyncStd to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, exec, tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment_ref( 4, &exec, tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, &exec, tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment_clone( 4, &exec, tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a Arc<AsyncStd> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, Arc::new(exec), tx );

	let result = AsyncStd::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle()
{
	let exec   = AsyncStd::default();
	let result = AsyncStd::block_on( increment_spawn_handle( 4, exec ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<AsyncStd> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle_arc()
{
	let exec   = AsyncStd::default();
	let result = AsyncStd::block_on( increment_spawn_handle( 4, Arc::new(exec) ) );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn spawn_handle_os()
{
	let exec   = AsyncStd::default();
	let result = AsyncStd::block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}


struct DropNotify
{
	tx: Option<oneshot::Sender<()>>,
}

impl Drop for DropNotify
{
	fn drop( &mut self )
	{
		self.tx.take().unwrap().send(()).expect( "notify drop" );
	}
}


// Joinhandle::drop aborts the task.
// Make sure that a task that is currently waiting for it's waker to be woken up
// get's dropped when JoinHandle is dropped.
//
#[ async_std::test ]
//
async fn join_handle_abort()
{
	let exec      = AsyncStd::default();
	let (tx , rx) = oneshot::channel::<()>();

	let join_handle = exec.spawn_handle( async move
	{
		let _notify = DropNotify{ tx: Some(tx) };

		// This will never end.
		// TODO: Replace with the std version when that is merged in stable.
		//
		let () = futures::future::pending().await;

	}).expect( "spawn task" );

	// Don't drop the handle before the task is scheduled by the executor.
	//
	Delay::new( Duration::from_millis(10) ).await;

	drop( join_handle );

	// This should not deadlock.
	//
	assert!( rx.await.is_ok() );
}


// Joinhandle::detach does not aborts the task.
//
#[ async_std::test ]
//
async fn join_handle_detach()
{
	let exec              = AsyncStd::default();
	let (out_tx , out_rx) = oneshot::channel::<()>();
	let (in_tx  , in_rx ) = oneshot::channel::<()>();

	let join_handle = exec.spawn_handle( async move
	{
		in_rx.await.expect( "receive in" );

		out_tx.send(()).expect( "send out" );

	}).expect( "spawn task" );


	// This will drop the handle.
	//
	join_handle.detach();

	// When commenting out this line, the next one does hang.
	//
	in_tx.send(()).expect( "send in" );

	// This should not deadlock.
	//
	assert!( out_rx.await.is_ok() );
}


// ------------------ Local
//


// pass a AsyncStd to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncStd::block_on( async
	{
		increment_local( 4, AsyncStd, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &AsyncStd to a function that takes exec: `&impl LocalSpawn`
//
#[ test ]
//
fn spawn_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncStd::block_on( async
	{
		increment_ref_local( 4, &AsyncStd, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &AsyncStd to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn spawn_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncStd::block_on( async
	{
		increment_local( 4, &AsyncStd, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &AsyncStd to a function that takes exec: `impl LocalSpawn + Clone`
//
#[ test ]
//
fn spawn_clone_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncStd::block_on( async
	{
		increment_clone_local( 4, &AsyncStd, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}




// pass a Arc<AsyncStd> to a function that takes exec: `impl LocalSpawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn spawn_clone_with_rc_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncStd::block_on( async
	{
		increment_clone_local( 4, Rc::new( AsyncStd ), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a AsyncStd to a function that takes exec: `impl LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_local()
{
	let res = AsyncStd::block_on( increment_spawn_handle_local( 4, AsyncStd ) );

	assert_eq!( 5u8, *res );
}


// pass an Rc<AsyncStd> to a function that takes exec: `impl LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_rc_local()
{
	let res = AsyncStd::block_on( increment_spawn_handle_local( 4, Rc::new( AsyncStd ) ) );

	assert_eq!( 5u8, *res );
}



// pass a &AsyncStd to a function that takes exec: `&dyn LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_local_os()
{
	let result = AsyncStd::block_on( increment_spawn_handle_os( 4, &AsyncStd ) );

	assert_eq!( 5u8, result );
}



// pass an AsyncStd to a function that requires a Timer.
//
#[ test ]
//
fn timer_should_wake()
{
	AsyncStd::block_on( timer_should_wake_up( AsyncStd ) );
}



// pass an AsyncStd to a function that requires a Timer.
//
#[ test ]
//
fn timer_should_wake_local()
{
	AsyncStd::block_on( timer_should_wake_up_local( AsyncStd ) );
}


// Verify tokio_io works.
//
#[ cfg( feature = "async_std_tokio" ) ]
//
#[ test ]
//
fn tokio_io() -> Result<(), DynError >
{
	use tokio::io::{ AsyncReadExt, AsyncWriteExt };

	let test = async
	{
		let (mut one, mut two) = tokio_io::socket_pair().await.expect( "socket_pair" );

		one.write_u8( 5 ).await.expect( "write tokio io" );

		assert_eq!( 5, two.read_u8().await.expect( "read tokio io" ) );
	};

	AsyncStd::block_on( test );

	Ok(())
}
