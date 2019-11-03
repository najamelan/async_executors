#![ cfg( feature = "threadpool" ) ]

// Tested:
//
// ✔ pass a &mut ThreadPool to a function that takes exec: `&mut impl Spawn`
// ✔ pass a      ThreadPool to a function that takes exec: `impl Spawn + Clone`
// ✔ test spawn_handle
//
mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::{ mpsc, oneshot }, executor::block_on, StreamExt },
};


// pass a &mut ThreadPool to a function that takes exec: `&mut impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = ThreadPool::new().expect( "create threadpool" );

	increment( 4, &mut exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut ThreadPool to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_with_clone()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = ThreadPool::new().expect( "create threadpool" );

	increment_by_value( 4, &mut exec, tx );

	let result = block_on( rx.next() );

		assert_eq!( Some( 5u8 ), result );
}


// test spawn_handle
//
#[ test ]
//
fn test_spawn_with_handle()
{
	let (tx, rx) = oneshot::channel();
	let mut exec = ThreadPool::new().expect( "create threadpool" );

	let fut = async move
	{
		rx.await.expect( "Some" )
	};

	let join_handle = exec.spawn_handle( fut ).expect( "spawn" );

	tx.send( 5 ).expect( "send" );

		assert_eq!( 5u8, block_on( join_handle ) );
}


