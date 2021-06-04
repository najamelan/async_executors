#![ cfg(all( feature = "async_global", not(target_os = "unknown") )) ]
//
// Tested:
//
// ✔ pass a     AsyncGlobal  to a function that takes exec: `impl Spawn`
// ✔ pass a    &AsyncGlobal  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &AsyncGlobal  to a function that takes exec: `impl Spawn`
// ✔ pass a    &AsyncGlobal  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<AsyncGlobal> to a function that takes exec: `impl Spawn`
// ✔ pass a     AsyncGlobal  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<AsyncGlobal> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &AsyncGlobal  to a function that takes exec: `&dyn SpawnHandle`
//
// ✔ pass a     AsyncGlobal  to a function that takes exec: `impl LocalSpawn`
// ✔ pass a    &AsyncGlobal  to a function that takes exec: `&impl LocalSpawn`
// ✔ pass a    &AsyncGlobal  to a function that takes exec: `impl LocalSpawn`
// ✔ pass a    &AsyncGlobal  to a function that takes exec: `impl LocalSpawn + Clone`
// ✔ pass a Arc<AsyncGlobal> to a function that takes exec: `impl LocalSpawn`.
// ✔ pass a     AsyncGlobal  to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass an Rc<AsyncGlobal> to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a    &AsyncGlobal  to a function that takes exec: `&dyn LocalSpawnHandle`
//
// ✔ pass an AsyncGlobal to a function that requires a Timer.
// ✔ Verify AsyncGlobal does not implement Timer when feature is not enabled.
// ✔ Verify tokio_io works        when the async_global_tokio feature is enabled.
// ✔ Verify tokio_io doesn't work when the async_global_tokio feature is not enabled.
//
// ✔ Joinhandle::detach allows task to keep running.
// ✔ Joinhandle::drop aborts the task.
//
mod common;

use
{
	common        :: { *                        } ,
	futures       :: { channel::mpsc, StreamExt } ,
	std           :: { time::Duration           } ,
	futures_timer :: { Delay                    } ,
};


// pass a AsyncGlobal to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncGlobal::default();

	increment( 4, exec, tx );

	let result = AsyncGlobal::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncGlobal to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncGlobal::default();

	increment_ref( 4, &exec, tx );

	let result = AsyncGlobal::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncGlobal to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncGlobal::default();

	increment( 4, &exec, tx );

	let result = AsyncGlobal::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &AsyncGlobal to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncGlobal::default();

	increment_clone( 4, &exec, tx );

	let result = AsyncGlobal::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a Arc<AsyncGlobal> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncGlobal::default();

	increment( 4, Arc::new(exec), tx );

	let result = AsyncGlobal::block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a AsyncGlobal to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle()
{
	let exec   = AsyncGlobal::default();
	let result = AsyncGlobal::block_on( increment_spawn_handle( 4, exec ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<AsyncGlobal> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle_arc()
{
	let exec   = AsyncGlobal::default();
	let result = AsyncGlobal::block_on( increment_spawn_handle( 4, Arc::new(exec) ) );

	assert_eq!( 5u8, result );
}


// pass a AsyncGlobal to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn spawn_handle_os()
{
	let exec   = AsyncGlobal::default();
	let result = AsyncGlobal::block_on( increment_spawn_handle_os( 4, &exec ) );

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
	let exec      = AsyncGlobal::default();
	let (tx , rx) = oneshot::channel::<()>();

	let join_handle = exec.spawn_handle( async move
	{
		let _notify = DropNotify{ tx: Some(tx) };

		// This will never end.
		// TODO: Replace with the std version when that is merged in stable.
		//
		let () = futures::future::pending().await;

	}).expect( "spawn task" );

	AsyncGlobal::block_on( async move
	{
		// Don't drop the handle before the task is scheduled by the executor.
		//
		Delay::new( Duration::from_millis(10) ).await;

		drop( join_handle );

		// This should not deadlock.
		//
		assert!( rx.await.is_ok() );
	});
}


// Joinhandle::detach does not aborts the task.
//
#[ test ]
//
fn join_handle_detach()
{
	let exec              = AsyncGlobal::default();
	let (out_tx , out_rx) = oneshot::channel::<()>();
	let (in_tx  , in_rx ) = oneshot::channel::<()>();

	let join_handle = exec.spawn_handle( async move
	{
		in_rx.await.expect( "receive in" );

		out_tx.send(()).expect( "send out" );

	}).expect( "spawn task" );


	AsyncGlobal::block_on( async move
	{
		// This will drop the handle.
		//
		join_handle.detach();

		// When commenting out this line, the next one does hang.
		//
		in_tx.send(()).expect( "send in" );

		// This should not deadlock.
		//
		assert!( out_rx.await.is_ok() );
	});
}


// ------------------ Local
//


// pass a AsyncGlobal to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncGlobal::block_on( async
	{
		increment_local( 4, AsyncGlobal, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &AsyncGlobal to a function that takes exec: `&impl LocalSpawn`
//
#[ test ]
//
fn spawn_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncGlobal::block_on( async
	{
		increment_ref_local( 4, &AsyncGlobal, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &AsyncGlobal to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn spawn_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncGlobal::block_on( async
	{
		increment_local( 4, &AsyncGlobal, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &AsyncGlobal to a function that takes exec: `impl LocalSpawn + Clone`
//
#[ test ]
//
fn spawn_clone_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncGlobal::block_on( async
	{
		increment_clone_local( 4, &AsyncGlobal, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}




// pass a Arc<AsyncGlobal> to a function that takes exec: `impl LocalSpawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn spawn_clone_with_rc_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	let res = AsyncGlobal::block_on( async
	{
		increment_clone_local( 4, Rc::new( AsyncGlobal ), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a AsyncGlobal to a function that takes exec: `impl LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_local()
{
	let res = AsyncGlobal::block_on( increment_spawn_handle_local( 4, AsyncGlobal ) );

	assert_eq!( 5u8, *res );
}


// pass an Rc<AsyncGlobal> to a function that takes exec: `impl LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_rc_local()
{
	let res = AsyncGlobal::block_on( increment_spawn_handle_local( 4, Rc::new( AsyncGlobal ) ) );

	assert_eq!( 5u8, *res );
}



// pass a &AsyncGlobal to a function that takes exec: `&dyn LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_local_os()
{
	let result = AsyncGlobal::block_on( increment_spawn_handle_os( 4, &AsyncGlobal ) );

	assert_eq!( 5u8, result );
}



// pass an AsyncGlobal to a function that requires a Timer.
//
#[ cfg( feature = "timer" ) ]
//
#[ test ]
//
fn timer_should_wake()
{
	AsyncGlobal::block_on( timer_should_wake_up( AsyncGlobal ) );
}



// pass an AsyncGlobal to a function that requires a Timer.
//
#[ cfg( feature = "timer" ) ]
//
#[ test ]
//
fn timer_should_wake_local()
{
	AsyncGlobal::block_on( timer_should_wake_up_local( AsyncGlobal ) );
}



// pass an AsyncGlobal to a function that requires a Timer.
//
#[ test ]
//
fn spawn_blocking() -> DynResult<()>
{
	AsyncGlobal::block_on( blocking( AsyncGlobal ) )
}



// Verify AsyncGlobal does not implement Timer when feature is not enabled.
//
#[ cfg(not( feature = "timer" )) ]
//
#[ test ]
//
fn no_feature_no_timer()
{
	static_assertions::assert_not_impl_any!( AsyncGlobal: Timer );
}



// Verify tokio_io works when the async_global_tokio feature is enabled.
//
#[ cfg(all( not(target_arch = "wasm32"), feature = "async_global_tokio" )) ]
//
#[ test ]
//
fn tokio_io() -> DynResult<()>
{
	AsyncGlobal::block_on( tokio_io::tcp( AsyncGlobal ) )
}



// Verify tokio_io doesn't work when the async_global_tokio feature is not enabled.
//
#[ cfg(all( not(target_arch = "wasm32"), not(feature = "async_global_tokio") )) ]
//
#[ test ] #[ should_panic ]
//
fn no_tokio_io()
{
	let test = async
	{
		let _ = tokio_io::socket_pair().await.expect( "socket_pair" );
	};

	AsyncGlobal::block_on( test );
}
