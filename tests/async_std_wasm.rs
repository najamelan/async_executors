#![ cfg(all( feature = "async_std", target_arch = "wasm32" )) ]

// Tested:
//
// ✔ pass a     AsyncStd  to a function that takes exec:  `impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `impl Spawn`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `impl Spawn + Clone`
// ✔ pass a Arc<AsyncStd> to a function that takes exec:  `impl Spawn`
// ✔ pass a     AsyncStd  to a function that takes exec:  `impl SpawnHandle`
// ✔ pass a Arc<AsyncStd> to a function that takes exec:  `impl SpawnHandle`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `&dyn SpawnHandle`
//
//
// ✔ pass a     AsyncStd  to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a    &AsyncStd  to a function that takes exec: `&impl LocalSpawn`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `impl LocalSpawn + Clone`
// ✔ pass a  Rc<AsyncStd> to a function that takes exec:  `impl LocalSpawn`
// ✔ pass a     AsyncStd  to a function that takes exec:  `impl LocalSpawnHandle`
// ✔ pass a  Rc<AsyncStd> to a function that takes exec:  `impl LocalSpawnHandle`
// ✔ pass a    &AsyncStd  to a function that takes exec:  `&dyn LocalSpawnHandle`
//
mod common;

use
{
	common            :: { *                        } ,
	futures           :: { channel::mpsc, StreamExt } ,
	wasm_bindgen_test :: { *                        } ,
};

wasm_bindgen_test_configure!( run_in_browser );



// pass a AsyncStd to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	increment( 4, AsyncStd, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn( fut ).expect( "spawn future" );
}


// pass a &AsyncStd to a function that takes exec: `&impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	increment_ref( 4, &AsyncStd, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn( fut ).expect( "spawn future" );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	increment( 4, &AsyncStd, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn( fut ).expect( "spawn future" );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn + Clone`
//
#[ wasm_bindgen_test ]
//
fn spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	increment_clone( 4, &AsyncStd, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn( fut ).expect( "spawn future" );
}


// pass a Arc<AsyncStd> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ wasm_bindgen_test ]
//
fn spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );

	increment( 4, Arc::new(AsyncStd), tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn( fut ).expect( "spawn future" );
}


// pass a AsyncStd to a function that takes exec: `impl SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle()
{
	let fut = async move
	{
		let result = increment_spawn_handle( 4, AsyncStd ).await;

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn( fut ).expect( "spawn future" );
}


// pass an Arc<AsyncStd> to a function that takes exec: `impl SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_arc()
{
	let fut = async move
	{
		let result = increment_spawn_handle( 4, Arc::new(AsyncStd) ).await;

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn( fut ).expect( "spawn future" );
}


// pass a &AsyncStd to a function that takes exec: `&dyn SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_os()
{
	let fut = async move
	{
		let result = increment_spawn_handle_os( 4, &AsyncStd ).await;

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}



//----------------------Local



// pass a AsyncStd to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	increment_local( 4, AsyncStd, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}


// pass a &AsyncStd to a function that takes exec: `&impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	increment_ref_local( 4, &AsyncStd, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn`
//
#[ wasm_bindgen_test ]
//
fn spawn_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	increment_local( 4, &AsyncStd, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}


// pass a &AsyncStd to a function that takes exec: `impl Spawn + Clone`
//
#[ wasm_bindgen_test ]
//
fn spawn_clone_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	increment_clone_local( 4, &AsyncStd, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}


// pass a Arc<AsyncStd> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ wasm_bindgen_test ]
//
fn spawn_clone_with_arc_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	increment_local( 4, Arc::new(AsyncStd), tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}


// pass a AsyncStd to a function that takes exec: `impl SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_local()
{
	let fut = async move
	{
		let result = increment_spawn_handle_local( 4, AsyncStd ).await;

		assert_eq!( 5u8, *result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}


// pass an Arc<AsyncStd> to a function that takes exec: `impl SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_arc_local()
{
	let fut = async move
	{
		let result = increment_spawn_handle_local( 4, Arc::new(AsyncStd) ).await;

		assert_eq!( 5u8, *result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}


// pass a &AsyncStd to a function that takes exec: `&dyn SpawnHandle`
//
#[ wasm_bindgen_test ]
//
fn spawn_handle_os_local()
{
	let fut = async move
	{
		let result = increment_spawn_handle_local_os( 4, &AsyncStd ).await;

		assert_eq!( 5u8, *result );
	};

	AsyncStd.spawn_local( fut ).expect( "spawn future" );
}

