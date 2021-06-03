#![ cfg( feature = "localpool" ) ]
//
// Tested:
//
// ✔ pass a     LocalSpawner  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<LocalSpawner> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &LocalSpawner  to a function that takes exec: `&dyn SpawnHandle`
//
// ✔ pass a    LocalSpawner  to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a Rc<LocalSpawner> to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a   &LocalSpawner  to a function that takes exec: `&dyn LocalSpawnHandle`
//
// ✔ pass an LocalPool to a function that requires a Timer.
// ✔ Verify LocalPool    does not implement Timer when feature is not enabled.
// ✔ Verify LocalSpawner does not implement Timer when feature is not enabled.
//
mod common;

use
{
	common           :: { *         } ,
	futures_executor :: { LocalPool } ,
	std              :: { rc::Rc    } ,
};


// pass a LocalSpawner to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle()
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
fn spawn_handle_arc()
{
	let mut exec     = LocalPool::new();
	let     spawner  = exec.spawner();

	let res = exec.run_until( increment_spawn_handle( 4, Arc::new(spawner) ) );

	assert_eq!( 5u8, res );
}



// pass a &LocalSpawner to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn spawn_handle_os()
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
fn spawn_handle_local()
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
fn spawn_handle_rc_local()
{
	let mut exec     = LocalPool::new();
	let     spawner  = exec.spawner();

	let res = exec.run_until( increment_spawn_handle_local( 4, Rc::new(spawner) ) );

	assert_eq!( 5u8, *res );
}



// pass a &LocalSpawner to a function that takes exec: `&dyn LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_local_os()
{
	let mut wrap = LocalPool::new();
	let     exec = wrap.spawner();

	let result = wrap.run_until( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}



// pass a LocalSpawner to a function that requires a YieldNow.
//
#[ test ]
//
fn yield_run_subtask_first() -> DynResult<()>
{
	let mut wrap = LocalPool::new();
	let     exec = wrap.spawner();

	wrap.run_until( try_yield_now( exec ) )
}



// pass a LocalSpawner to a function that requires a YieldNow.
//
#[ test ]
//
fn yield_run_subtask_last() -> DynResult<()>
{
	let mut wrap = LocalPool::new();
	let     exec = wrap.spawner();

	wrap.run_until( without_yield_now( exec ) )
}



// pass an LocalPool to a function that requires a Timer.
//
#[ cfg( feature = "timer" ) ]
//
#[ test ]
//
fn timer_should_wake()
{
	let mut wrap = LocalPool::new();
	let     exec = wrap.spawner();

	wrap.run_until( timer_should_wake_up_local( exec ) );
}



// Verify LocalPool does not implement Timer when feature is not enabled.
//
#[ cfg(not( feature = "timer" )) ]
//
#[ test ]
//
fn no_feature_no_timer()
{
	static_assertions::assert_not_impl_any!( LocalPool   : Timer );
	static_assertions::assert_not_impl_any!( LocalSpawner: Timer );
}

