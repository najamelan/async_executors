#![ cfg( feature = "bindgen" )]

// Tested:
//
// ✔ pass a &mut Bindgen to a function that takes exec: `&mut impl Spawn`
// ✔ pass a &mut Bindgen to a function that takes exec: `&mut impl LocalSpawn`
// ✔ pass a      Bindgen to a function that takes exec: `impl Spawn      + Clone`
// ✔ pass a      Bindgen to a function that takes exec: `impl LocalSpawn + Clone`
// ✔ test spawn_handle
// ✔ test spawn_handle_local
//
mod common;

use
{
	common            :: { *                                     } ,
	async_executors   :: { *                                     } ,
	wasm_bindgen_test :: { *                                     } ,
	futures           :: { channel::{ mpsc, oneshot }, StreamExt } ,
	std               :: { rc::Rc                                } ,
};

wasm_bindgen_test_configure!(run_in_browser);

// pass a &mut Bindgen to a function that takes exec: `&mut impl Spawn`
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


// pass a &mut Bindgen to a function that takes exec: `&mut impl LocalSpawn`
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


// pass a &mut Bindgen to a function that takes exec: `impl Spawn + Clone`
//
#[wasm_bindgen_test]
//
fn test_spawn_with_clone()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = Bindgen::new();

	increment_by_value( 4, &mut exec, tx );

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
	let mut exec     = Bindgen::new();

	increment_by_value_local( 4, &mut exec, tx );

	let fut = async move
	{
		let result = rx.next().await.expect( "Some" );

		assert_eq!( 5u8, result );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// test spawn_handle
//
#[ wasm_bindgen_test ]
//
fn test_spawn_with_handle()
{
	let (tx, rx) = oneshot::channel();
	let mut exec = Bindgen::new();

	let fut = async move
	{
		rx.await.expect( "Some" )
	};

	let join_handle = exec.spawn_handle( fut ).expect( "spawn" );

	tx.send( 5u8 ).expect( "send" );


	let fut = async move
	{
		assert_eq!( 5u8, join_handle.await );
	};

	exec.spawn( fut ).expect( "spawn future" );
}


// test spawn_handle_local
//
#[ wasm_bindgen_test ]
//
fn test_spawn_with_local_handle()
{
	let (tx, rx) = oneshot::channel();
	let mut exec = Bindgen::new();

	let fut = async move
	{
		rx.await.expect( "Some" )
	};

	let join_handle = exec.spawn_handle_local( fut ).expect( "spawn" );

	// Use Rc to make sure we get a !Send output.
	//
	tx.send( Rc::new( 5u8 ) ).expect( "send" );

	let fut = async move
	{
		assert_eq!( Rc::new( 5u8 ), join_handle.await );
	};

	exec.spawn_local( fut ).expect( "spawn future" );
}
