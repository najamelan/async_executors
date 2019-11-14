#![ cfg( feature = "localpool" ) ]

// Tested:
//
// ✔ recover return value
// ✔ JoinHandle<T>: Send where T: Send
// ✔ dropping JoinHandle does not cancel task.
// ✔ test spawn_handle
//
mod common;

use
{
	async_executors :: * ,
	futures         :: { channel::{ mpsc, oneshot }, executor::block_on, StreamExt },
	std             :: thread,
};


// recover return value
//
#[ test ]
//
fn recover_return_value()
{
	let (tx, rx) = oneshot::channel();
	let mut exec = LocalPool::default();

	let task = async move
	{
		rx.await.expect( "Some" )
	};

	let handle = exec.spawn_handle( task ).expect( "spawn" );

	tx.send( 5u8 ).expect( "send 5" );

	exec.run();

	assert_eq!( 5u8, block_on( handle ) );
}


// JoinHandle<T>: Send where T: Send
//
#[ test ]
//
fn send_handle()
{
	let (tx, rx) = oneshot::channel();
	let mut exec = LocalPool::default();

	let task = async move
	{
		rx.await.expect( "Some" )
	};

	let handle = exec.spawn_handle( task ).expect( "spawn" );

	tx.send( 5u8 ).expect( "send 5" );

	exec.run();

	assert_eq!( 5u8, block_on( handle ) );
}
