#![ cfg( feature = "tokio_ct" ) ]

// Tested:
//
// ✔ pass a &mut TokioCt to a function that takes exec: `&mut impl Spawn`
// ✔ pass a &mut TokioCt to a function that takes exec: `&mut impl LocalSpawn`
// ✔ pass a      TokioCt to a function that takes exec: `impl Spawn      + Clone`
// ✔ pass a      TokioCt to a function that takes exec: `impl LocalSpawn + Clone`
// ✔ test spawn_handle
// ✔ test spawn_handle_local
//
mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::{ mpsc, oneshot }, executor::block_on, StreamExt } ,
	std             :: { rc::Rc                                                    } ,
};


// pass a &mut TokioCt to a function that takes exec: `&mut impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::new();

	increment( 4, &mut exec, tx );


	let result = exec.run();

		assert!( result.is_ok() );


	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut TokioCt to a function that takes exec: `&mut impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::new();

	increment_local( 4, &mut exec, tx );


	let result = exec.run();

		assert!( result.is_ok() );


	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut TokioCt to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_with_clone()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::new();

	increment_by_value( 4, exec.clone(), tx );


	let result = exec.run();

		assert!( result.is_ok() );


	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut TokioCt to a function that takes exec: `impl LocalSpawn + Clone`
//
#[ test ]
//
fn test_spawn_with_clone_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::new();

	increment_by_value_local( 4, exec.clone(), tx );


	let result = exec.run();

		assert!( result.is_ok() );


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
	let mut exec = TokioCt::new();

	let fut = async move
	{
		rx.await.expect( "Some" )
	};

	let join_handle = exec.spawn_handle( fut ).expect( "spawn" );

	tx.send( 5u8 ).expect( "send" );

	exec.run().expect( "run tokio_ct exec" );

		assert_eq!( 5u8, block_on( join_handle ) );
}


// test spawn_handle_local
//
#[ test ]
//
fn test_spawn_with_local_handle()
{
	let (tx, rx) = oneshot::channel();
	let mut exec = TokioCt::new();

	let fut = async move
	{
		rx.await.expect( "Some" )
	};

	let join_handle = exec.spawn_handle_local( fut ).expect( "spawn" );

	// Use Rc to make sure we get a !Send output.
	//
	tx.send( Rc::new( 5u8 ) ).expect( "send" );

	exec.run().expect( "run tokio_ct exec" );

		assert_eq!( Rc::new( 5u8 ), block_on( join_handle ) );
}
