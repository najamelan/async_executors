#![ cfg(all( feature = "threadpool", feature = "spawn_handle" )) ]

// ✔ pass a     ThreadPool  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<ThreadPool> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &ThreadPool  to a function that takes exec: `&dyn SpawnHandle`
//
// ✔ Joinhandle::detach allows task to keep running.
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



// Joinhandle::detach allows task to keep running.
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_join_handle_detach()
{
	let exec   = ThreadPool::new().expect( "create threadpool" );

	let (in_tx , in_rx ) = oneshot::channel();
	let (out_tx, out_rx) = oneshot::channel();


	let in_join_handle = exec.spawn_handle( async move
	{
		let content = in_rx.await.expect( "receive on in" );

		out_tx.send( content ).expect( "send on out" );

	}).expect( "spawn task" );


	in_join_handle.detach();

	futures::executor::block_on( async move
	{
		in_tx.send( 5u8 ).expect( "send on in" );

		assert_eq!( out_rx.await, Ok(5) );
	});
}
