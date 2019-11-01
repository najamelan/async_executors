
pub use
{
	futures :: { SinkExt, task::{ LocalSpawnExt, SpawnExt, LocalSpawn, Spawn }, channel::mpsc::Sender },
};

// A function that takes a generic executor and spawns a task.
//
pub fn increment( a: u8, exec: &mut impl Spawn, tx: Sender<u8> )
{
	let res = exec.spawn( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}

// A function that takes a generic executor and spawns a task.
//
#[ cfg(any( feature = "localpool", feature = "tokio_ct", feature = "bindgen" )) ]
//
pub fn increment_local( a: u8, exec: &mut impl LocalSpawn, tx: Sender<u8> )
{
	let res = exec.spawn_local( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}

// A function that takes a generic executor by value, clones it and spawns a task.
//
pub fn increment_by_value( a: u8, exec: impl Spawn + Clone, tx: Sender<u8> )
{
	let mut second = exec.clone();

	let res = second.spawn( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}

// A function that takes a generic executor by value, clones it and spawns a task.
//
#[ cfg(any( feature = "localpool", feature = "tokio_ct", feature = "bindgen" )) ]
//
pub fn increment_by_value_local( a: u8, exec: impl LocalSpawn + Clone, tx: Sender<u8> )
{
	let mut second = exec.clone();

	let res = second.spawn_local( sum( a, 1, tx ) );

		assert!( res.is_ok() );
}


async fn sum( a: u8, b: u8, mut tx: Sender<u8> )
{
	let res = tx.send( a + b ).await;

		assert!( res.is_ok() );
}
