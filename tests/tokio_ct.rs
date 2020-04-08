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
// ✔ handle() works from within spawned task.
// ✔ we can spawn without being in a future running on block_on.
// ✔ Joinhandle::detach allows task to keep running.
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
	let exec         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let ex2          = exec.clone();

	let res = exec.block_on( async
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
	let exec         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment_ref( 4, &exec, tx );

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
	let exec         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment( 4, &exec, tx );

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
	let exec         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment_clone( 4, &exec, tx );

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
	let exec     = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment_clone( 4, Arc::new( exec.clone() ), tx );

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
	let exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( increment_spawn_handle( 4, exec.clone() ) );

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
	let exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( increment_spawn_handle( 4, Arc::new( exec.clone() ) ) );

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
	let exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	let result = exec.block_on( increment_spawn_handle_os( 4, &exec ) );

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
	let exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment_local( 4, exec.clone(), tx );

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
	let exec         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment_ref_local( 4, &exec, tx );

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
	let exec         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment_local( 4, &exec, tx );

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
	let exec         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment_clone_local( 4, &exec, tx );

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
	let exec         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( async
	{
		increment_clone_local( 4, Rc::new( exec.clone() ), tx );

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
	let exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( increment_spawn_handle_local( 4, exec.clone() ) );

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
	let exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let res = exec.block_on( increment_spawn_handle_local( 4, Rc::new( exec.clone() ) ) );

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
	let exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let result = exec.block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}



// make sure handle() works from within spawned task.
//
#[ test ]
//
fn test_handle()
{
	let (mut tx, mut rx) = mpsc::channel( 1 );
	let exec             = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	let task = async
	{
		let handle = exec.handle();

		let inner = async
		{
			std::thread::spawn( move ||
			{
				handle.spawn( async move { tx.send( 5u8 ).await.expect( "send on tx" ); } ).expect( "spawn on handle" );
			});
		};

		exec.spawn( inner ).expect( "spawn inner" );

		rx.next().await.expect( "wait on rx" )
	};

	let result = exec.block_on( task );

	assert_eq!( 5u8, result );
}



// make sure we can spawn without being in a future running on block_on.
//
#[ test ]
//
fn test_spawn_outside_block_on()
{
	let (mut tx, mut rx) = mpsc::channel( 1 );

	let exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );

	exec.spawn( async move
	{
		tx.send( "hello" ).await.expect( "send hello" );

	}).expect( "spawn" );

	let result = exec.block_on( async move
	{
		rx.next().await.expect( "receive hello" )
	});

	assert_eq!( "hello", result );
}



// Joinhandle::detach allows task to keep running.
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_join_handle_detach()
{
	let wrap         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio current thread" );
	let exec         = wrap.handle();

	let (in_tx , in_rx ) = oneshot::channel();
	let (out_tx, out_rx) = oneshot::channel();


	let in_join_handle = exec.spawn_handle( async move
	{
		let content = in_rx.await.expect( "receive on in" );

		out_tx.send( content ).expect( "send on out" );

	}).expect( "spawn task" );


	in_join_handle.detach();

	wrap.block_on( async move
	{
		in_tx.send( 5u8 ).expect( "send on in" );

		assert_eq!( out_rx.await, Ok(5) );
	});
}
