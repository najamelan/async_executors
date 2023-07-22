#![ cfg(all( not(target_arch = "wasm32"), feature = "tokio_tp" )) ]
//
// Tested:
//
// ✔ build a TokioTp from a currently entered runtime.
// ✔ build a TokioTp from a handle.
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
// ✔ pass a TokioTp to a function that requires a SpawnBlocking.
// ✔ pass a TokioTp to a function that requires an object safe SpawnBlocking.
// ✔ pass a TokioTp to a function that requires a Timer.
// ✔ Verify TokioTp does not implement Timer when feature is not enabled.
// ✔ Verify Timeout future.
//
// ✔ Verify tokio_io         works when the tokio_io feature is     enabled.
// ✔ Verify tokio_io doesn't work  when the tokio_io feature is not enabled.
//
// ✔ Joinhandle::detach allows task to keep running.
//
mod common;

use
{
	common  :: { *                                     } ,
	futures :: { channel::{ mpsc, oneshot }, StreamExt } ,
	std     :: { convert::TryFrom                      } ,
	tokio   :: { runtime::{ Builder, Handle }          } ,
};


// pass a TokioTp to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = TokioTp::new().expect( "create tokio threadpool" );

	increment( 4, exec.clone(), tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = TokioTp::new().expect( "create tokio threadpool" );

	increment_ref( 4, &exec, tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = TokioTp::new().expect( "create tokio threadpool" );

	increment( 4, &exec, tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = TokioTp::new().expect( "create tokio threadpool" );

	increment_clone( 4, &exec, tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a Arc<TokioTp> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = TokioTp::new().expect( "create tokio threadpool" );

	increment( 4, Arc::new( exec.clone() ), tx );

	let result = exec.block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a TokioTp to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle()
{
	let exec = TokioTp::new().expect( "create tokio threadpool" );

	let result = exec.block_on( increment_spawn_handle( 4, exec.clone() ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<TokioTp> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle_arc()
{
	let exec = TokioTp::new().expect( "create tokio threadpool" );

	let result = exec.block_on( increment_spawn_handle( 4, Arc::new( exec.clone() ) ) );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn spawn_handle_os()
{
	let exec = TokioTp::new().expect( "create tokio threadpool" );

	let result = exec.block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}


// pass a builder with some config set.
//
#[ test ]
//
fn build_name_thread()
{
	let (tx, rx) = oneshot::channel();

	let rt = Builder::new_multi_thread()
		.thread_name( "test_thread" )
		.build()
		.expect( "tokio builder" )
	;

	let exec = TokioTp::try_from( rt ).expect( "create TokioTp from builder" );

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


// build from current.
//
#[ tokio::test(flavor = "multi_thread") ]
//
async fn build_from_current()
{
	let (tx, rx) = oneshot::channel();

	let exec = TokioTp::try_current().expect( "create TokioTp from current" );

	let task = async move
	{
		let name = std::thread::current().name().expect( "some name" ).to_string();
		tx.send( name ).expect( "send on oneshot" );
	};

	exec.spawn( task ).expect( "spawn" );

	assert_eq!( rx.await.expect( "read channel" ), "tokio-runtime-worker" );
}



// build from a `Handle`.
//
#[ tokio::test(flavor = "multi_thread") ]
//
async fn build_from_handle()
{
	let (tx, rx) = oneshot::channel();
	let handle = Handle::current();

	let exec = TokioTp::try_from( handle ).expect( "create TokioTp from current" );

	let task = async move
	{
		let name = std::thread::current().name().expect( "some name" ).to_string();
		tx.send( name ).expect( "send on oneshot" );
	};

	exec.spawn( task ).expect( "spawn" );

	assert_eq!( rx.await.expect( "read channel" ), "tokio-runtime-worker" );
}



// Joinhandle::detach allows task to keep running.
//
#[ test ]
//
fn join_handle_detach()
{
	let exec         = TokioTp::new().expect( "create tokio threadpool" );

	let (in_tx , in_rx ) = oneshot::channel();
	let (out_tx, out_rx) = oneshot::channel();


	let in_join_handle = exec.spawn_handle( async move
	{
		let content = in_rx.await.expect( "receive on in" );

		out_tx.send( content ).expect( "send on out" );

	}).expect( "spawn task" );


	in_join_handle.detach();

	exec.block_on( async move
	{
		in_tx.send( 5u8 ).expect( "send on in" );

		assert_eq!( out_rx.await, Ok(5) );
	});
}



// pass an TokioTp to a function that requires a Timer.
//
#[ cfg(any( feature="timer", feature="tokio_timer" )) ]
//
#[ test ]
//
fn timer_should_wake()
{
	let exec = TokioTp::new().expect( "create tokio current thread" );

	exec.block_on( timer_should_wake_up( exec.clone() ) );
}



// pass an TokioTp to a function that requires a Timer.
//
#[ cfg(any( feature="timer", feature="tokio_timer" )) ]
//
#[ test ]
//
fn run_timeout()
{
	let exec = &TokioTp::new().expect( "create tokio current thread" );

	exec.block_on( timeout( exec ) );
}



// pass an TokioTp to a function that requires a Timer.
//
#[ cfg(any( feature="timer", feature="tokio_timer" )) ]
//
#[ test ]
//
fn run_dont_timeout()
{
	let exec = &TokioTp::new().expect( "create tokio current thread" );

	exec.block_on( dont_timeout( exec ) );
}



// Verify TokioTp does not implement Timer when feature is not enabled.
//
#[ cfg(not(any( feature="timer", feature="tokio_timer" ))) ]
//
#[ test ]
//
fn no_feature_no_timer()
{
	static_assertions::assert_not_impl_any!( TokioTp: Timer );
}



// pass a TokioTp to a function that requires SpawnBlocking.
//
#[ test ]
//
fn spawn_blocking() -> DynResult<()>
{
	let exec = &TokioTp::new()?;

	exec.block_on( blocking( exec ) )
}



// pass a TokioTp to a function that requires an object safe SpawnBlocking.
//
#[ test ]
//
fn spawn_blocking_void() -> DynResult<()>
{
	let exec = &TokioTp::new()?;

	exec.block_on( blocking_void( exec ) )
}



// Verify tokio_io works when the tokio_io feature is enabled.
//
#[ cfg( feature = "tokio_io" ) ]
//
#[ test ]
//
fn tokio_io() -> DynResult<()>
{
	let exec = TokioTp::new()?;

	exec.block_on( tokio_io::tcp( exec.clone() ) )
}


// Verify tokio_io doesn't work when the tokio_io feature is not enabled.
//
#[ cfg(not( feature = "tokio_io" )) ]
//
#[ test ] #[ should_panic ]
//
fn no_tokio_io()
{
	let exec = TokioTp::new().expect( "create tokio threadpool" );

	let test = async
	{
		let _ = tokio_io::socket_pair().await.expect( "socket_pair" );
	};

	exec.block_on( test );
}
