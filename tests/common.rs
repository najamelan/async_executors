
pub use
{
	futures :: { SinkExt, task::{ LocalSpawnExt, SpawnExt }, channel::mpsc::Sender },
};

// A function that takes a generic executor and spawns a task.
//
pub fn increment( a: u8, exec: &mut impl SpawnExt, tx: Sender<u8> )
{
	let res = exec.spawn( sum( a, 1, tx ) );
	assert!( res.is_ok() );
}

// A function that takes a generic executor and spawns a task.
//
pub fn increment_local( a: u8, exec: &mut impl LocalSpawnExt, tx: Sender<u8> )
{
	let res = exec.spawn_local( sum( a, 1, tx ) );
	assert!( res.is_ok() );
}


async fn sum( a: u8, b: u8, mut tx: Sender<u8> )
{
	let res = tx.send( a + b ).await;
	assert!( res.is_ok() );
}
