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
// ✔ pass a     AsyncStd  to a function that takes exec: `impl SpawnHandleNative`
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
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn test_spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment_ref( 4, &exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, &exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment_clone( 4, &exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a Arc<AsyncStd> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn test_spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, Arc::new(exec), tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();


	let result = block_on( async move
	{
		increment_spawn_handle( 4, exec, tx ).await;

		rx.next().await
	});


	assert_eq!( 5u8, result.expect( "Some" ) );
}


// pass an Arc<AsyncStd> to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();


	let result = block_on( async move
	{
		increment_spawn_handle( 4, Arc::new(exec), tx ).await;

		rx.next().await
	});


	assert_eq!( 5u8, result.expect( "Some" ) );
}


// pass a AsyncStd to a function that takes exec: `impl SpawnHandleNative`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_native()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();


	let result = block_on( async move
	{
		increment_spawn_handle_native( 4, exec, tx ).await;

		rx.next().await
	});


	assert_eq!( 5u8, result.expect( "Some" ) );
}
