#![ cfg( feature = "bindgen" )]

// Tested:
//
// ✔ pass a &mut Bindgen to a function that takes exec: `&mut impl Spawn`
// ✔ pass a &mut Bindgen to a function that takes exec: `&mut impl LocalSpawn`
// ✔ pass a      Bindgen to a function that takes exec: `impl Spawn      + Clone`
// ✔ pass a      Bindgen to a function that takes exec: `impl LocalSpawn + Clone`
//
mod common;

use
{
	common            :: { *                                     } ,
	async_executors   :: { *                                     } ,
	wasm_bindgen_test :: { *                                     } ,
	futures           :: { channel::mpsc, StreamExt } ,
};

wasm_bindgen_test_configure!(run_in_browser);

// pass a &mut Bindgen to a function that takes exec: `&mut impl Spawn`
//
#[wasm_bindgen_test]
//
fn test_spawn()
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


// pass a &mut Bindgen to a function that takes exec: `&mut impl LocalSpawn`
//
#[wasm_bindgen_test]
//
fn test_spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_local( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &mut Bindgen to a function that takes exec: `impl Spawn + Clone`
//
#[wasm_bindgen_test]
//
fn test_spawn_with_clone()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_by_value( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &mut Bindgen to a function that takes exec: `impl LocalSpawn + Clone`
//
#[wasm_bindgen_test]
//
fn test_spawn_with_clone_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = Bindgen::default();

	increment_by_value_local( 4, &exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}
