#![ cfg( feature = "tokio_ct" ) ]

// Tested:
//
// ✔ pass a     TokioCt  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioCt  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &TokioCt  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioCt  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<TokioCt> to a function that takes exec: `impl Spawn`
// ✔ pass a     TokioCt  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<TokioCt> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &TokioCt  to a function that takes exec: `&dyn SpawnHandle`
//
// ✔ pass a    TokioCt  to a function that takes exec: `impl LocalSpawn`
// ✔ pass a   &TokioCt  to a function that takes exec: `&impl LocalSpawn`
// ✔ pass a   &TokioCt  to a function that takes exec: `impl LocalSpawn`
// ✔ pass a   &TokioCt  to a function that takes exec: `impl LocalSpawn + Clone`
// ✔ pass a Rc<TokioCt> to a function that takes exec: `impl LocalSpawn`
// ✔ pass a    TokioCt  to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a Rc<TokioCt> to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a   &TokioCt  to a function that takes exec: `&dyn LocalSpawnHandle`
//
// ✔ make sure handle() works from within spawned task.
//
mod common;

use
{
	common          :: * ,
	futures         :: { channel::{ mpsc }, StreamExt } ,
	tokio::runtime  :: { Builder                      } ,
	std             :: { convert::TryFrom, rc::Rc     } ,
};


// pass a TokioCt to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2      = exec.clone();

	let res = exec.block_on( async move
	{
		increment( 4, ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioCt to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn test_spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2      = exec.clone();

	let res = exec.block_on( async move
	{
		increment_ref( 4, &ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioCt to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn test_spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2      = exec.clone();

	let res = exec.block_on( async move
	{
		increment( 4, &ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioCt to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2      = exec.clone();

	let res = exec.block_on( async move
	{
		increment_clone( 4, &ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a Arc<TokioCt> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn test_spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2      = exec.clone();

	let res = exec.block_on( async move
	{
		increment_clone( 4, Arc::new(ex2), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a TokioCt to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle()
{
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( increment_spawn_handle( 4, ex2 ) );

	assert_eq!( 5u8, res );
}


// pass an Arc<TokioCt> to a function that takes exec: `impl SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_arc()
{
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( increment_spawn_handle( 4, Arc::new(ex2) ) );

	assert_eq!( 5u8, res );
}



// pass a &TokioCt to a function that takes exec: `&dyn SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_os()
{
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     ex2  = exec.clone();

	let result = exec.block_on( increment_spawn_handle_os( 4, &ex2 ) );

	assert_eq!( 5u8, result );
}


// ------------------ Local
//


// pass a TokioCt to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( async move
	{
		increment_local( 4, ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioCt to a function that takes exec: `&impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( async move
	{
		increment_ref_local( 4, &ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioCt to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn test_spawn_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( async move
	{
		increment_local( 4, &ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a &TokioCt to a function that takes exec: `impl LocalSpawn + Clone`
//
#[ test ]
//
fn test_spawn_clone_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( async move
	{
		increment_clone_local( 4, &ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a Arc<TokioCt> to a function that takes exec: `impl LocalSpawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn test_spawn_clone_with_rc_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( async move
	{
		increment_clone_local( 4, Rc::new(ex2), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}


// pass a TokioCt to a function that takes exec: `impl LocalSpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_local()
{
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( increment_spawn_handle_local( 4, ex2 ) );

	assert_eq!( 5u8, *res );
}


// pass an Rc<TokioCt> to a function that takes exec: `impl LocalSpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_rc_local()
{
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let     ex2  = exec.clone();

	let res = exec.block_on( increment_spawn_handle_local( 4, Rc::new(ex2) ) );

	assert_eq!( 5u8, *res );
}



// pass a &TokioCt to a function that takes exec: `&dyn LocalSpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_local_os()
{
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     ex2  = exec.clone();

	let result = exec.block_on( increment_spawn_handle_os( 4, &ex2 ) );

	assert_eq!( 5u8, result );
}



// make sure handle() works from within spawned task
//
#[ test ]
//
fn test_handle()
{
	let (mut tx, mut rx) = mpsc::channel( 1 );

	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let     ex2  = exec.clone();

	let task = async move
	{
		let handle = ex2.handle();

		let inner = async move
		{
			std::thread::spawn( move ||
			{
				handle.spawn( async move { tx.send( 5u8 ).await.expect( "send on tx" ); } ).expect( "spawn on handle" );
			});
		};

		ex2.spawn( inner ).expect( "spawn inner" );

		rx.next().await.expect( "wait on rx" )
	};

	let result = exec.block_on( task );

	assert_eq!( 5u8, result );
}

