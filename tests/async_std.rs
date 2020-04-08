#![ cfg( feature = "async_std" ) ]

// Tested:
//
// ✔ pass a     AsyncStd  to a function that takes exec: `impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<AsyncStd> to a function that takes exec: `impl Spawn`
// ✔ pass a     AsyncStd  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<AsyncStd> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &AsyncStd  to a function that takes exec: `&dyn SpawnHandle`
//
// ✔ Joinhandle::detach allows task to keep running.
// ✔ Joinhandle::drop aborts the task.
//
mod common;

use
{
	common  :: { *                        } ,
	futures :: { channel::mpsc, StreamExt } ,
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
#[ cfg( feature = "spawn_handle" ) ]
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
#[ cfg( feature = "spawn_handle" ) ]
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
#[ cfg( feature = "spawn_handle" ) ]
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
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn join_handle_abort()
{
	let exec      = AsyncStd::default();
	let (tx , rx) = oneshot::channel::<()>();
	let notify    = DropNotify{ tx: Some(tx) };

	// it's important we move in notify.
	//
	let in_join_handle = exec.spawn_handle( async move
	{
		let _my_notify = notify;

		// This will never end.
		// TODO: Replace with the std version when that is merged in stable.
		//
		let () = futures::future::pending().await;

	}).expect( "spawn task" );


	drop( in_join_handle );

	// This should not deadlock.
	//
	AsyncStd::block_on( async
	{
		assert!( rx.await.is_ok() );
	})
}
