#![ cfg( feature = "threadpool" ) ]
//
// ✔ pass a     ThreadPool  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<ThreadPool> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &ThreadPool  to a function that takes exec: `&dyn SpawnHandle`
//
// ✔ Joinhandle::detach allows task to keep running.
//
// ✔ pass an ThreadPool to a function that requires a Timer.
// ✔ Verify ThreadPool does not implement Timer when feature is not enabled.
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
fn spawn_handle()
{
	let exec   = ThreadPool::new().expect( "create threadpool" );
	let result = block_on( increment_spawn_handle( 4, exec ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<ThreadPool> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle_arc()
{
	let exec   = ThreadPool::new().expect( "create threadpool" );
	let result = block_on( increment_spawn_handle( 4, Arc::new(exec) ) );

	assert_eq!( 5u8, result );
}


// pass a ThreadPool to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn spawn_handle_os()
{
	let exec   = ThreadPool::new().expect( "create threadpool" );
	let result = block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}



// Joinhandle::detach allows task to keep running.
//
#[ test ]
//
fn join_handle_detach()
{
	let exec = ThreadPool::new().expect( "create threadpool" );

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



// pass an ThreadPool to a function that requires a Timer.
//
#[ cfg( feature = "timer" ) ]
//
#[ test ]
//
fn timer_should_wake()
{
	let exec = ThreadPool::new().expect( "create threadpool" );

	block_on( timer_should_wake_up( exec ) );
}



// Verify ThreadPool does not implement Timer when feature is not enabled.
//
#[ cfg(not( feature = "timer" )) ]
//
#[ test ]
//
fn no_feature_no_timer()
{
	static_assertions::assert_not_impl_any!( ThreadPool: Timer );
}
