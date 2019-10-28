#![ cfg( feature = "tokio_ct" ) ]

// Tested:
//
// ✔ pass a &mut TokioCt to a function that takes exec: `&mut impl SpawnExt`
// ✔ pass a &mut TokioCt to a function that takes exec: `&mut impl LocalSpawnExt`

mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::mpsc, executor::block_on, StreamExt },
};


// pass a &mut TokioCt to a function that takes exec: `&mut impl SpawnExt`
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


// pass a &mut TokioCt to a function that takes exec: `&mut impl LocalSpawnExt`
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
