#![ cfg( feature = "localpool" ) ]

// Tested:
//
// ✔ pass a &mut LocalPool to a function that takes exec: `&mut impl SpawnExt`
// ✔ pass a &mut LocalPool to a function that takes exec: `&mut impl LocalSpawnExt`

mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::mpsc, executor::block_on, StreamExt },
};


// pass a &mut LocalPool to a function that takes exec: `&mut impl SpawnExt`
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


// pass a &mut LocalPool to a function that takes exec: `&mut impl LocalSpawnExt`
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
