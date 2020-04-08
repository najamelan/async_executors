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
// ✔ pass a    &TokioTp  to a function that takes exec: `&dyn SpawnHandle`
// ✔ pass a builder with some config set.
//
// ✔ Joinhandle::detach allows task to keep running.
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
	let exec         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	increment( 4, exec.clone(), tx );

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
	let exec         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

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
	let exec         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

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
	let exec         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

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
	let exec         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	increment( 4, Arc::new( exec.clone() ), tx );

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
	let exec = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	let result = exec.block_on( increment_spawn_handle( 4, exec.clone() ) );

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
	let exec = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	let result = exec.block_on( increment_spawn_handle( 4, Arc::new( exec.clone() ) ) );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `&dyn SpawnHandle`
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_spawn_handle_os()
{
	let exec = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	let result = exec.block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}


// pass a builder with some config set.
//
#[ test ]
//
fn test_build_name_thread()
{
	let (tx, rx) = oneshot::channel();

	let exec = TokioTp::try_from( Builder::new().thread_name( "test_thread" ) ).expect( "create tokio threadpool" );

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



// Joinhandle::detach allows task to keep running.
//
#[ cfg( feature = "spawn_handle" ) ]
//
#[ test ]
//
fn test_join_handle_detach()
{
	let wrap         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
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
