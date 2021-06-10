#![ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ]

// Tested:
//
// ✔ pass a     Bindgen  to a function that takes exec:  `impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec:  `impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec:  `impl Spawn + Clone`
// ✔ pass a Arc<Bindgen> to a function that takes exec:  `impl Spawn`
// ✔ pass a     Bindgen  to a function that takes exec:  `impl SpawnHandle`
// ✔ pass a Arc<Bindgen> to a function that takes exec:  `impl SpawnHandle`
// ✔ pass a    &Bindgen  to a function that takes exec:  `&dyn SpawnHandle`
//
// ✔ pass a     Bindgen  to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a    &Bindgen  to a function that takes exec: `&impl LocalSpawn`
// ✔ pass a    &Bindgen  to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a    &Bindgen  to a function that takes exec:  `impl LocalSpawn + Clone`
// ✔ pass a  Rc<Bindgen> to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a     Bindgen  to a function that takes exec:  `impl LocalSpawnHandle`
// ✔ pass a  Rc<Bindgen> to a function that takes exec:  `impl LocalSpawnHandle`
// ✔ pass a    &Bindgen  to a function that takes exec:  `&dyn LocalSpawnHandle`
//
// ✔ pass a Bindgen to a function that requires a YieldNow.
// ✔ pass a Bindgen to a function that requires a Timer.
// ✔ Verify Bindgen does not implement Timer when feature is not enabled.
// ✔ Verify Timeout future.
//
mod common;

use
{
	common            :: { *                        } ,
	futures           :: { channel::mpsc, StreamExt } ,
	wasm_bindgen_test :: { *                        } ,
};

wasm_bindgen_test_configure!( run_in_browser );


// pass a Bindgen to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment( 4, exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `&impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_ref( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `impl Spawn + Clone`
//
#[ wasm_bindgen_test ]
//
fn spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_clone( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a Arc<Bindgen> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ wasm_bindgen_test ]
//
fn spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment( 4, Arc::new(exec), tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a Bindgen to a function that takes exec: `impl SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle()
{
	let exec = Bindgen::default();

	let fut = async move
	{
		let result = increment_spawn_handle( 4, exec ).await;

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass an Arc<Bindgen> to a function that takes exec: `impl SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_arc()
{
	let exec = Bindgen::default();

	let fut = async move
	{
		let result = increment_spawn_handle( 4, Arc::new(exec) ).await;

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `&dyn SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_os()
{
	let exec = Bindgen::default();

	let fut = async move
	{
		let result = increment_spawn_handle_os( 4, &exec ).await;

		assert_eq!( 5u8, result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


//----------------------Local



// pass a Bindgen to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_local( 4, exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `&impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_ref_local( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_local( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `impl Spawn + Clone`
//
#[ wasm_bindgen_test ]
//
fn spawn_clone_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_clone_local( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass a Arc<Bindgen> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ wasm_bindgen_test ]
//
fn spawn_clone_with_arc_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_local( 4, Arc::new(exec), tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass a Bindgen to a function that takes exec: `impl SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_local()
{
	let exec = Bindgen::default();

	let fut = async move
	{
		let result = increment_spawn_handle_local( 4, exec ).await;

		assert_eq!( 5u8, *result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass an Arc<Bindgen> to a function that takes exec: `impl SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_arc_local()
{
	let exec = Bindgen::default();


	let fut = async move
	{
		let result = increment_spawn_handle_local( 4, Arc::new(exec) ).await;

		assert_eq!( 5u8, *result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `&dyn SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_os_local()
{
	let exec = Bindgen::default();

	let fut = async move
	{
		let result = increment_spawn_handle_local_os( 4, &exec ).await;

		assert_eq!( 5u8, *result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}



// pass a Bindgen to a function that requires a YieldNow.
//
#[ wasm_bindgen_test ]
//
fn yield_run_subtask_first()
{
	let task = async{ try_yield_now( Bindgen ).await.expect( "yield_now" ); };

	Bindgen.spawn_local( task ).expect( "spawn" );
}



// pass a Bindgen to a function that requires a YieldNow.
//
#[ wasm_bindgen_test ]
//
fn yield_run_subtask_last()
{
	let task = async{ without_yield_now( Bindgen ).await.expect( "yield_now" ); };

	Bindgen.spawn_local( task ).expect( "spawn" );
}




// pass an Bindgen to a function that requires a Timer.
//
#[ cfg( feature = "timer" ) ]
//
#[ wasm_bindgen_test ]
//
fn timer_should_wake_local()
{
	Bindgen.spawn_local( timer_should_wake_up_local( Bindgen ) ).expect( "spawn" );
}



// Verify timeout future.
//
#[ cfg( feature = "timer" ) ]
//
#[ wasm_bindgen_test ]
//
fn run_timeout()
{
	Bindgen.spawn_local( timeout( Bindgen ) ).expect( "spawn" );
}



// Verify timeout future.
//
#[ cfg( feature = "timer" ) ]
//
#[ wasm_bindgen_test ]
//
fn run_dont_timeout()
{
	Bindgen.spawn_local( dont_timeout( Bindgen ) ).expect( "spawn" );
}



// Verify Bindgen does not implement Timer when feature is not enabled.
//
#[ cfg(not( feature = "timer" )) ]
//
#[ test ]
//
fn no_feature_no_timer()
{
	static_assertions::assert_not_impl_any!( Bindgen: Timer );
}
