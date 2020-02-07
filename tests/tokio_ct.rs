#![ cfg( feature = "tokio_ct" ) ]

// Tested:
//
// ✔ pass a &mut TokioCt to a function that takes exec: `&mut impl Spawn`
// ✔ pass a &mut TokioCt to a function that takes exec: `&mut impl LocalSpawn`
// ✔ pass a      TokioCt to a function that takes exec: `impl Spawn      + Clone`
// ✔ pass a      TokioCt to a function that takes exec: `impl LocalSpawn + Clone`
//
mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::mpsc, StreamExt } ,
	std             :: { convert::TryFrom         } ,
	tokio::runtime  :: { Builder                  } ,
};


// pass a &mut TokioCt to a function that takes exec: `&mut impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let spawner = exec.send_handle();

	exec.block_on( async move
	{
		increment( 4, &spawner, tx );

		assert_eq!( 5u8, rx.next().await.expect( "Some" ) );
	});
}


// pass a &mut TokioCt to a function that takes exec: `&mut impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let spawner = exec.handle();

	exec.block_on( async move
	{
		increment_local( 4, &spawner, tx );

		assert_eq!( 5u8, rx.next().await.expect( "Some" ) );
	});
}


// pass a &mut TokioCt to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_with_clone()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let mut spawner = exec.send_handle();

	exec.block_on( async move
	{
		increment_by_value( 4, &mut spawner, tx );

		assert_eq!( 5u8, rx.next().await.expect( "Some" ) );
	});
}


// pass a &mut TokioCt to a function that takes exec: `impl LocalSpawn + Clone`
//
#[ test ]
//
fn test_spawn_with_clone_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let spawner = exec.handle();

	exec.block_on( async move
	{
		increment_by_value_local( 4, &spawner, tx );

		assert_eq!( 5u8, rx.next().await.expect( "Some" ) );
	});
}
