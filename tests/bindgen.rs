#![ cfg( feature = "bindgen" )]

// Tested:
//
// ✔ pass a &mut Bindgen to a function that takes exec: `&mut impl SpawnExt`
// ✔ pass a &mut Bindgen to a function that takes exec: `&mut impl LocalSpawnExt`
// ✔ pass a      Bindgen to a function that takes exec: `impl SpawnExt      + Clone`
// ✔ pass a      Bindgen to a function that takes exec: `impl LocalSpawnExt + Clone`
//
mod common;

use
{
	common            :: { *                        } ,
	async_executors   :: { *                        } ,
	wasm_bindgen_test :: { *                        } ,
	futures           :: { StreamExt, channel::mpsc } ,
};

wasm_bindgen_test_configure!(run_in_browser);

// pass a &mut Bindgen to a function that takes exec: `&mut impl SpawnExt`
//
#[wasm_bindgen_test]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment( 4, &mut exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &mut Bindgen to a function that takes exec: `&mut impl LocalSpawnExt`
//
#[wasm_bindgen_test]
//
fn test_spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment_local( 4, &mut exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &mut Bindgen to a function that takes exec: `impl LocalSpawnExt + Clone`
//
#[wasm_bindgen_test]
//
fn test_spawn_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment_by_value( 4, exec.handle(), tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// pass a &mut Bindgen to a function that takes exec: `impl LocalSpawnExt + Clone`
//
#[wasm_bindgen_test]
//
fn test_spawn_handle_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment_by_value_local( 4, exec.handle(), tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}
