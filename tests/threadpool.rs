#![ cfg(all( feature = "threadpool", feature = "spawn_handle" )) ]

// ✔ pass a     ThreadPool  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<ThreadPool> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &ThreadPool  to a function that takes exec: `&dyn SpawnHandle`
//
mod common;

use
{
	common           :: { *          } ,
	futures_executor :: { ThreadPool } ,
};

// pass a ThreadPool to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn test_spawn_handle()
{
	let exec   = ThreadPool::new().expect( "create threadpool" );
	let result = block_on( increment_spawn_handle( 4, exec ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<ThreadPool> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn test_spawn_handle_arc()
{
	let exec   = ThreadPool::new().expect( "create threadpool" );
	let result = block_on( increment_spawn_handle( 4, Arc::new(exec) ) );

	assert_eq!( 5u8, result );
}


// pass a ThreadPool to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn test_spawn_handle_os()
{
	let exec   = ThreadPool::new().expect( "create threadpool" );
	let result = block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}
