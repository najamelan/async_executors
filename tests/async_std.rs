#![ cfg( feature = "async_std" ) ]

// Tested:
//
// ✔ pass a &mut AsyncStd to a function that takes exec: `&mut impl Spawn`
// ✔ pass a      AsyncStd to a function that takes exec: `impl Spawn + Clone`
//
mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::mpsc, executor::block_on, StreamExt },
};


// pass a &mut AsyncStd to a function that takes exec: `&mut impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment( 4, &exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut AsyncStd to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_with_clone()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = AsyncStd::default();

	increment_by_value( 4, &exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}

