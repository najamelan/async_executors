#![ cfg( feature = "tokio_tp" ) ]

// Tested:
//
// ✔ pass a &mut TokioTp to a function that takes exec: `&mut impl Spawn`
// ✔ pass a      TokioTp to a function that takes exec: `impl Spawn + Clone`
// ✔ test spawn_handle
//
mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::{ mpsc, oneshot }, executor::block_on, StreamExt },
};


// pass a &mut TokioTp to a function that takes exec: `&mut impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioTp::new();

	increment( 4, &mut exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut TokioTp to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec = TokioTp::new();

	increment_by_value( 4, exec.handle(), tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// test spawn_handle
//
#[ test ]
//
fn test_spawn_with_handle()
{
	let (tx, rx) = oneshot::channel();
	let mut exec = TokioTp::new();

	let fut = async move
	{
		rx.await.expect( "Some" )
	};

	let join_handle = exec.spawn_handle( fut ).expect( "spawn" );

	tx.send( 5 ).expect( "send" );

		assert_eq!( 5u8, block_on( join_handle ) );
}


