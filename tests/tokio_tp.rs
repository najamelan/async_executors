#![ cfg( feature = "tokio_tp" ) ]

// Tested:
//
// ✔ pass a     TokioTp  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioTp  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &TokioTp  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioTp  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<TokioTp> to a function that takes exec: `impl Spawn`
// ✔ pass a     TokioTp  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<TokioTp> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &TokioTp  to a function that takes exec: `&dyn SpawnHandleOs`
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


// pass a TokioTp to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     ex2      = exec.clone();

	increment( 4, ex2, tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn test_spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	increment_ref( 4, &exec, tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	increment( 4, &exec, tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	increment_clone( 4, &exec, tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a Arc<TokioTp> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn test_spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     ex2      = exec.clone();

	increment( 4, Arc::new(ex2), tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a TokioTp to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle()
{
	let mut exec     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     ex2      = exec.clone();

	let result = exec.block_on( increment_spawn_handle( 4, ex2 ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<TokioTp> to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_arc()
{
	let mut exec     = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     ex2      = exec.clone();

	let result = exec.block_on( increment_spawn_handle( 4, Arc::new(ex2) ) );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `&dyn SpawnHandleOs`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_os()
{
	let mut exec = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     ex2  = exec.clone();

	let result = exec.block_on( increment_spawn_handle_os( 4, &ex2 ) );

	assert_eq!( 5u8, result );
}


// pass a builder with some config set.
//
#[ test ]
//
fn test_build_name_thread()
{
	let (tx, rx) = oneshot::channel();

	let mut exec = TokioTp::try_from( Builder::new().thread_name( "test_thread" ) ).expect( "create tokio threadpool" );

	let task = async move
	{
		let name = std::thread::current().name().expect( "some name" ).to_string();
		tx.send( name ).expect( "send on oneshot" );
	};

	exec.spawn( task ).expect( "spawn" );

	exec.block_on( async
	{
		assert_eq!( rx.await.expect( "read channel" ), "test_thread" );

	});
}

