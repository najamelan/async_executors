#![ cfg(all( feature = "localpool", feature = "spawn_handle" )) ]

// Tested:
//
// ✔ pass a     LocalSpawner  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<LocalSpawner> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &LocalSpawner  to a function that takes exec: `&dyn SpawnHandleOs`
//
// ✔ pass a    LocalSpawner  to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a Rc<LocalSpawner> to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a   &LocalSpawner  to a function that takes exec: `&dyn LocalSpawnHandleOs`
//
mod common;

use
{
	common           :: * ,
	futures_executor :: { LocalPool                    } ,
	std              :: { rc::Rc                       } ,
};


// pass a LocalSpawner to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn test_spawn_handle()
{
	let mut exec     = LocalPool::new();
	let     spawner  = exec.spawner();

	let res = exec.run_until( increment_spawn_handle( 4, spawner ) );

	assert_eq!( 5u8, res );
}


// pass an Arc<LocalSpawner> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn test_spawn_handle_arc()
{
	let mut exec     = LocalPool::new();
	let     spawner  = exec.spawner();

	let res = exec.run_until( increment_spawn_handle( 4, Arc::new(spawner) ) );

	assert_eq!( 5u8, res );
}



// pass a &LocalSpawner to a function that takes exec: `&dyn SpawnHandleOs`
//
#[ test ]
//
fn test_spawn_handle_os()
{
	let mut wrap = LocalPool::new();
	let     exec = wrap.spawner();

	let result = wrap.run_until( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}


// ------------------ Local
//


// pass a LocalSpawner to a function that takes exec: `impl LocalSpawnHandle`
//
#[ test ]
//
fn test_spawn_handle_local()
{
	let mut exec     = LocalPool::new();
	let     spawner  = exec.spawner();

	let res = exec.run_until( increment_spawn_handle_local( 4, spawner ) );

	assert_eq!( 5u8, *res );
}


// pass an Rc<LocalSpawner> to a function that takes exec: `impl LocalSpawnHandle`
//
#[ test ]
//
fn test_spawn_handle_rc_local()
{
	let mut exec     = LocalPool::new();
	let     spawner  = exec.spawner();

	let res = exec.run_until( increment_spawn_handle_local( 4, Rc::new(spawner) ) );

	assert_eq!( 5u8, *res );
}



// pass a &LocalSpawner to a function that takes exec: `&dyn LocalSpawnHandleOs`
//
#[ test ]
//
fn test_spawn_handle_local_os()
{
	let mut wrap = LocalPool::new();
	let     exec = wrap.spawner();

	let result = wrap.run_until( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}

