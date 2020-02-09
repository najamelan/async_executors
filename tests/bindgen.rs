#![ cfg( feature = "bindgen" ) ]

// Tested:
//
// ✔ pass a     Bindgen  to a function that takes exec: `impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec: `impl Spawn`
// ✔ pass a    &Bindgen  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<Bindgen> to a function that takes exec: `impl Spawn`
// ✔ pass a     Bindgen  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<Bindgen> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a     Bindgen  to a function that takes exec: `impl SpawnHandleNative`
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


// // pass a Bindgen to a function that takes exec: `impl SpawnHandle`
// //
// #[ cfg( feature = "spawn_handle" ) ]
// //
// #[ wasm_bindgen_test ]
// //
// fn test_spawn_handle()
// {
// 	let (tx, mut rx) = mpsc::channel( 1 );
// 	let exec         = Bindgen::default();


// 	let result = block_on( async move
// 	{
// 		increment_spawn_handle( 4, exec, tx ).await;

// 		rx.next().await
// 	});


// 	assert_eq!( 5u8, result.expect( "Some" ) );
// }


// // pass an Arc<Bindgen> to a function that takes exec: `impl SpawnHandle`
// //
// #[ cfg( feature = "spawn_handle" ) ]
// //
// #[ wasm_bindgen_test ]
// //
// fn test_spawn_handle_arc()
// {
// 	let (tx, mut rx) = mpsc::channel( 1 );
// 	let exec         = Bindgen::default();


// 	let result = block_on( async move
// 	{
// 		increment_spawn_handle( 4, Arc::new(exec), tx ).await;

// 		rx.next().await
// 	});


// 	assert_eq!( 5u8, result.expect( "Some" ) );
// }


// // pass a Bindgen to a function that takes exec: `impl SpawnHandleNative`
// //
// #[ cfg( feature = "spawn_handle" ) ]
// //
// #[ wasm_bindgen_test ]
// //
// fn test_spawn_handle_native()
// {
// 	let (tx, mut rx) = mpsc::channel( 1 );
// 	let exec         = Bindgen::default();


// 	let result = block_on( async move
// 	{
// 		increment_spawn_handle_native( 4, exec, tx ).await;

// 		rx.next().await
// 	});


// 	assert_eq!( 5u8, result.expect( "Some" ) );
// }
