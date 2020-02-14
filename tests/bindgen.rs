#![ cfg( feature = "bindgen" ) ]

// Tested:
//
// ✔ pass a     Bindgen  to a function that takes exec:  `impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec:  `impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec:  `impl Spawn + Clone`
// ✔ pass a Arc<Bindgen> to a function that takes exec:  `impl Spawn`
// ✔ pass a     Bindgen  to a function that takes exec:  `impl SpawnHandle`
// ✔ pass a Arc<Bindgen> to a function that takes exec:  `impl SpawnHandle`
// ✔ pass a    &Bindgen  to a function that takes exec:  `&dyn SpawnHandleOs`
//
// ✔ pass a     Bindgen  to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a    &Bindgen  to a function that takes exec: `&impl LocalSpawn`
// ✔ pass a    &Bindgen  to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a    &Bindgen  to a function that takes exec:  `impl LocalSpawn + Clone`
// ✔ pass a  Rc<Bindgen> to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a     Bindgen  to a function that takes exec:  `impl LocalSpawnHandle`
// ✔ pass a  Rc<Bindgen> to a function that takes exec:  `impl LocalSpawnHandle`
// ✔ pass a    &Bindgen  to a function that takes exec:  `&dyn LocalSpawnHandleOs`
//
mod common;

use
{
	common            :: { *                        } ,
	futures           :: { channel::mpsc, StreamExt } ,
	wasm_bindgen_test :: { *                        } ,
};


// pass a Bindgen to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn test_spawn()
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
fn test_spawn_ref()
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
fn test_spawn_with_ref()
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
fn test_spawn_clone_with_ref()
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
fn test_spawn_clone_with_arc()
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
#[ cfg( feature = "spawn_handle" ) ]
//
#[ wasm_bindgen_test ]
//
fn test_spawn_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();


	let fut = async move
	{
		increment_spawn_handle( 4, exec, tx ).await;

		assert_eq!( 5u8, rx.next().await.expect( "Some" ) );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass an Arc<Bindgen> to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ wasm_bindgen_test ]
//
fn test_spawn_handle_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();


	let fut = async move
	{
		increment_spawn_handle( 4, Arc::new(exec), tx ).await;

		assert_eq!( 5u8, rx.next().await.expect( "Some" ) );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `&dyn SpawnHandleOs`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ wasm_bindgen_test ]
//
fn test_spawn_handle_os()
{
	let exec = Bindgen::default();
	let ex2  = exec.clone();

	let fut = async move
	{
		let result = increment_spawn_handle_os( 4, &ex2 ).await;

		assert_eq!( 5u8, result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


//----------------------Local



// pass a Bindgen to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn test_spawn_local()
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
fn test_spawn_ref_local()
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
fn test_spawn_with_ref_local()
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
fn test_spawn_clone_with_ref_local()
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
fn test_spawn_clone_with_arc_local()
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
#[ cfg( feature = "spawn_handle" ) ]
//
#[ wasm_bindgen_test ]
//
fn test_spawn_handle_local()
{
	let exec         = Bindgen::default();

	let fut = async move
	{
		let result = increment_spawn_handle_local( 4, exec ).await;

		assert_eq!( 5u8, *result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass an Arc<Bindgen> to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ wasm_bindgen_test ]
//
fn test_spawn_handle_arc_local()
{
	let exec         = Bindgen::default();


	let fut = async move
	{
		let result = increment_spawn_handle_local( 4, Arc::new(exec) ).await;

		assert_eq!( 5u8, *result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}


// pass a &Bindgen to a function that takes exec: `&dyn SpawnHandleOs`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ wasm_bindgen_test ]
//
fn test_spawn_handle_os_local()
{
	let exec = Bindgen::default();
	let ex2  = exec.clone();

	let fut = async move
	{
		let result = increment_spawn_handle_local_os( 4, &ex2 ).await;

		assert_eq!( 5u8, *result );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}

