#![ cfg( feature = "tokio_ct" ) ]

// Tested:
//
// ✔ pass a     TokioHandle  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<TokioHandle> to a function that takes exec: `impl Spawn`
// ✔ pass a     TokioHandle  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<TokioHandle> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &TokioHandle  to a function that takes exec: `&dyn SpawnHandleOs`
//
// ✔ pass a    TokioLocalHandle  to a function that takes exec: `impl LocalSpawn`
// ✔ pass a   &TokioLocalHandle  to a function that takes exec: `&impl LocalSpawn`
// ✔ pass a   &TokioLocalHandle  to a function that takes exec: `impl LocalSpawn`
// ✔ pass a   &TokioLocalHandle  to a function that takes exec: `impl LocalSpawn + Clone`
// ✔ pass a Rc<TokioLocalHandle> to a function that takes exec: `impl LocalSpawn`
// ✔ pass a    TokioLocalHandle  to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a Rc<TokioLocalHandle> to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a   &TokioLocalHandle  to a function that takes exec: `&dyn LocalSpawnHandleOs`
//
mod common;

use
{
	common          :: * ,
	futures         :: { channel::{ mpsc }, StreamExt } ,
	tokio::runtime  :: { Builder                      } ,
	std             :: { convert::TryFrom, rc::Rc     } ,
};


// pass a TokioHandle to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.handle();

	let res = exec.block_on( async move
	{
		increment( 4, spawner, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioHandle to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn test_spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.handle();

	let res = exec.block_on( async move
	{
		increment_ref( 4, &spawner, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioHandle to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.handle();

	let res = exec.block_on( async move
	{
		increment( 4, &spawner, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioHandle to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.handle();

	let res = exec.block_on( async move
	{
		increment_clone( 4, &spawner, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a Arc<TokioHandle> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn test_spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.handle();

	let res = exec.block_on( async move
	{
		increment_clone( 4, Arc::new(spawner), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
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
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.handle();

	let res = exec.block_on( async move
	{
		increment_spawn_handle( 4, spawner, tx ).await;

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
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
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.handle();

	let res = exec.block_on( async move
	{
		increment_spawn_handle( 4, Arc::new(spawner), tx ).await;

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a &TokioHandle to a function that takes exec: `&dyn SpawnHandleOs`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_os()
{
	let mut wrap = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec = wrap.handle();


	let result = wrap.block_on( async move
	{
		increment_spawn_handle_os( 4, &exec ).await
	});


	assert_eq!( 5u8, result );
}


// ------------------ Local
//


// pass a TokioLocalHandle to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.local_handle();

	let res = exec.block_on( async move
	{
		increment_local( 4, spawner, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioLocalHandle to a function that takes exec: `&impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.local_handle();

	let res = exec.block_on( async move
	{
		increment_ref_local( 4, &spawner, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioLocalHandle to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.local_handle();

	let res = exec.block_on( async move
	{
		increment_local( 4, &spawner, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioLocalHandle to a function that takes exec: `impl LocalSpawn + Clone`
//
#[ test ]
//
fn test_spawn_clone_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.local_handle();

	let res = exec.block_on( async move
	{
		increment_clone_local( 4, &spawner, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a Arc<TokioLocalHandle> to a function that takes exec: `impl LocalSpawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn test_spawn_clone_with_rc_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.local_handle();

	let res = exec.block_on( async move
	{
		increment_clone_local( 4, Rc::new(spawner), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a TokioLocalHandle to a function that takes exec: `impl LocalSpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_local()
{
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.local_handle();

	let res = exec.block_on( async move
	{
		increment_spawn_handle_local( 4, spawner ).await
	});

	assert_eq!( 5u8, *res );
}


// pass an Rc<TokioLocalHandle> to a function that takes exec: `impl LocalSpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_rc_local()
{
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     spawner  = exec.local_handle();

	let res = exec.block_on( async move
	{
		increment_spawn_handle_local( 4, Rc::new(spawner) ).await
	});

	assert_eq!( 5u8, *res );
}



// pass a &TokioLocalHandle to a function that takes exec: `&dyn LocalSpawnHandleOs`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_local_os()
{
	let mut wrap = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     exec = wrap.local_handle();


	let result = wrap.block_on( async move
	{
		increment_spawn_handle_os( 4, &exec ).await
	});


	assert_eq!( 5u8, result );
}

