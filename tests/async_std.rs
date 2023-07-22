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
// ✔ pass an AsyncStd to a function that requires a SpawnBlocking.
// ✔ pass an AsyncStd to a function that requires an object safe SpawnBlocking.
// ✔ pass an AsyncStd to a function that requires a Timer.
// ✔ Verify tokio_io works when the async_std_tokio feature is enabled.
// ✔ Verify tokio_io doesn't work when the async_std_tokio feature is not enabled.
// ✔ Verify Timeout future.
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
};


// pass a AsyncStd to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd;

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
	let exec         = AsyncStd;

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
	let exec         = AsyncStd;

	#[allow(clippy::needless_borrow)]
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
	let exec         = AsyncStd;

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
	let exec         = AsyncStd;

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
	let exec   = AsyncStd;
	let result = AsyncStd::block_on( increment_spawn_handle( 4, exec ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<AsyncStd> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle_arc()
{
	let exec   = AsyncStd;
	let result = AsyncStd::block_on( increment_spawn_handle( 4, Arc::new(exec) ) );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn spawn_handle_os()
{
	let exec   = AsyncStd;
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
#[ test ]
//
fn join_handle_abort()
{
	let (tx , rx) = oneshot::channel::<()>();

	let join_handle = AsyncStd.spawn_handle( async move
	{
		let _notify = DropNotify{ tx: Some(tx) };

		// This will never end.
		//
		#[allow(clippy::let_unit_value)]
		let () = futures::future::pending().await;

	}).expect( "spawn task" );


	AsyncStd::block_on( async
	{
		// Don't drop the handle before the task is scheduled by the executor.
		//
		Delay::new( Duration::from_millis(10) ).await;

		drop( join_handle );

		// This should not deadlock.
		//
		assert!( rx.await.is_ok() );
	})
}


// Joinhandle::detach does not aborts the task.
//
#[ test ]
//
fn join_handle_detach()
{
	let (out_tx , out_rx) = oneshot::channel::<()>();
	let (in_tx  , in_rx ) = oneshot::channel::<()>();

	let join_handle = AsyncStd.spawn_handle( async move
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
	assert!( AsyncStd::block_on(out_rx).is_ok() );
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
		#[allow(clippy::needless_borrow)]
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
fn no_feature_no_timer()
{
	AsyncStd::block_on( timer_should_wake_up_local( AsyncStd ) );
}



// pass an AsyncStd to a function that requires a Timer.
//
#[ test ]
//
fn run_timeout()
{
	AsyncStd::block_on( timeout( AsyncStd ) );
}



// pass an AsyncStd to a function that requires a Timer.
//
#[ test ]
//
fn run_dont_timeout()
{
	AsyncStd::block_on( dont_timeout( AsyncStd ) );
}



// pass an AsyncStd to a function that requires a Timer.
//
#[ test ]
//
fn spawn_blocking() -> DynResult<()>
{
	AsyncStd::block_on( blocking( AsyncStd ) )
}



// pass an AsyncStd to a function that requires a SpawnBlocking.
//
#[ test ]
//
fn spawn_blocking_void() -> DynResult<()>
{
	AsyncStd::block_on( blocking_void( &AsyncStd ) )
}



// Verify tokio_io works when the async_std_tokio feature is enabled.
//
#[ cfg(all( not(target_arch = "wasm32"), feature = "async_std_tokio" )) ]
//
#[ test ]
//
fn tokio_io() -> DynResult<()>
{
	AsyncStd::block_on( tokio_io::tcp( AsyncStd ) )
}


// Verify tokio_io doesn't work when the async_std_tokio feature is not enabled.
//
#[ cfg(all( not(target_arch = "wasm32"), not(feature = "async_std_tokio") )) ]
//
#[ test ] #[ should_panic ]
//
fn no_tokio_io()
{
	let test = async
	{
		let _ = tokio_io::socket_pair().await.expect( "socket_pair" );
	};

	AsyncStd::block_on( test );
}
