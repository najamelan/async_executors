#![ cfg( feature = "localpool" ) ]

// Tested:
//
// ✔ pass a &mut LocalPool to a function that takes exec: `&mut impl Spawn`
// ✔ pass a &mut LocalPool to a function that takes exec: `&mut impl LocalSpawn`
// ✔ pass a      LocalPool to a function that takes exec: `impl Spawn      + Clone`
// ✔ pass a      LocalPool to a function that takes exec: `impl LocalSpawn + Clone`
//
mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::mpsc, executor::block_on, StreamExt },
};


// pass a &mut LocalPool to a function that takes exec: `&mut impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = LocalPool::new();

	increment( 4, &mut exec, tx );
	exec.run();

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut LocalPool to a function that takes exec: `&mut impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = LocalPool::new();

	increment_local( 4, &mut exec, tx );
	exec.run();

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut LocalPool to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_from_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = LocalPool::new();

	increment_by_value( 4, exec.handle(), tx );
	exec.run();

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut LocalPool to a function that takes exec: `impl LocalSpawn + Clone`
//
#[ test ]
//
fn test_spawn_from_handle_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = LocalPool::new();

	increment_by_value_local( 4, exec.handle(), tx );
	exec.run();

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}
