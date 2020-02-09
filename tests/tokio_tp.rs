#![ cfg( feature = "tokio_tp" ) ]

// Tested:
//
// ✔ pass a     TokioHandle  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<TokioHandle> to a function that takes exec: `impl Spawn`
// ✔ pass a     TokioHandle  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<TokioHandle> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a     TokioHandle  to a function that takes exec: `impl SpawnHandleNative`
// ✔ pass a builder with some config set.
//
mod common;

use
{
	common          :: * ,
	futures         :: { channel::{ mpsc, oneshot }, StreamExt } ,
	tokio::runtime  :: { Builder                               } ,
	std             :: { convert::TryFrom                      } ,
};


// pass a TokioHandle to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut wrap     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec     = wrap.handle();

	increment( 4, exec, tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioHandle to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn test_spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut wrap     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec     = wrap.handle();

	increment_ref( 4, &exec, tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioHandle to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut wrap     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec     = wrap.handle();

	increment( 4, &exec, tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioHandle to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut wrap     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec     = wrap.handle();

	increment_clone( 4, &exec, tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a Arc<TokioHandle> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn test_spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut wrap     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec     = wrap.handle();

	increment( 4, Arc::new(exec), tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a TokioHandle to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut wrap     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec     = wrap.handle();


	let result = wrap.block_on( async move
	{
		increment_spawn_handle( 4, exec, tx ).await;

		rx.next().await
	});


	assert_eq!( 5u8, result.expect( "Some" ) );
}


// pass an Arc<TokioHandle> to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut wrap     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec     = wrap.handle();


	let result = wrap.block_on( async move
	{
		increment_spawn_handle( 4, Arc::new(exec), tx ).await;

		rx.next().await
	});


	assert_eq!( 5u8, result.expect( "Some" ) );
}


// pass a TokioHandle to a function that takes exec: `impl SpawnHandleNative`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_native()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut wrap     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec     = wrap.handle();


	let result = wrap.block_on( async move
	{
		increment_spawn_handle_native( 4, exec, tx ).await;

		rx.next().await
	});


	assert_eq!( 5u8, result.expect( "Some" ) );
}


// pass a builder with some config set.
//
#[ test ]
//
fn test_build_name_thread()
{
	let (tx, rx) = oneshot::channel();

	let mut wrap = TokioTp::try_from( Builder::new().thread_name( "test_thread" ) ).expect( "create tokio threadpool" );
	let     exec = wrap.handle();

	let task = async move
	{
		let name = std::thread::current().name().expect( "some name" ).to_string();
		tx.send( name ).expect( "send on oneshot" );
	};

	exec.spawn( task ).expect( "spawn" );

	wrap.block_on( async
	{
		assert_eq!( rx.await.expect( "read channel" ), "test_thread" );

	});
}

