#![ cfg(all( feature = "threadpool", feature = "spawn_handle" )) ]

// ✔ pass a     ThreadPool  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<ThreadPool> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &ThreadPool  to a function that takes exec: `&dyn SpawnHandleOs`
//
mod common;

use
{
	common           :: { *                        } ,
	futures          :: { channel::mpsc, StreamExt } ,
	futures_executor :: { ThreadPool               } ,
};

// pass a ThreadPool to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn test_spawn_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = ThreadPool::new().expect( "create threadpool" );


	let result = block_on( async move
	{
		increment_spawn_handle( 4, exec, tx ).await;

		rx.next().await
	});


	assert_eq!( 5u8, result.expect( "Some" ) );
}


// pass an Arc<ThreadPool> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn test_spawn_handle_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = ThreadPool::new().expect( "create threadpool" );


	let result = block_on( async move
	{
		increment_spawn_handle( 4, Arc::new(exec), tx ).await;

		rx.next().await
	});


	assert_eq!( 5u8, result.expect( "Some" ) );
}


// pass a ThreadPool to a function that takes exec: `&dyn SpawnHandleOs`
//
#[ test ]
//
fn test_spawn_handle_os()
{
	let exec = ThreadPool::new().expect( "create threadpool" );


	let result = block_on( async move
	{
		increment_spawn_handle_os( 4, &exec ).await
	});


	assert_eq!( 5u8, result );
}
