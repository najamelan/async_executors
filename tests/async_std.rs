#![ cfg( feature = "async_std" ) ]

// Tested:
//
// ✔ pass a &mut AsyncStd to a function that takes exec: `&mut impl SpawnExt`
// ✔ pass a      AsyncStd to a function that takes exec: `impl SpawnExt + Clone`
//
mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::mpsc, executor::block_on, StreamExt },
};


// pass a &mut AsyncStd to a function that takes exec: `&mut impl SpawnExt`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = AsyncStd::new();

	increment( 4, &mut exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut AsyncStd to a function that takes exec: `impl LocalSpawnExt + Clone`
//
#[ test ]
//
fn test_spawn_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec = AsyncStd::new();

	increment_by_value( 4, exec.handle(), tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}

