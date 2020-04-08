#![ cfg(any( feature = "tokio_ct", feature = "tokio_tp" )) ]

// Tested:
//
// ✔ pass a     TokioHandle  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `impl Spawn`
// ✔ pass a    &TokioHandle  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<TokioHandle> to a function that takes exec: `impl Spawn`
// ✔ pass a     TokioHandle  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<TokioHandle> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &TokioHandle  to a function that takes exec: `&dyn SpawnHandle`
// ✔ pass a builder with some config set.
//
mod common;

use
{
	common          :: * ,
	futures         :: { channel::{ mpsc }, StreamExt } ,
	tokio::runtime  :: { Builder                      } ,
	std             :: { convert::TryFrom             } ,
};


// pass a TokioTp to a function that takes exec: `impl Spawn`
//
#[ test ]
//
#[ cfg( feature = "tokio_tp" ) ]
//
fn spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let wrap         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec         = wrap.handle();

	increment( 4, exec, tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
#[ cfg( feature = "tokio_tp" ) ]
//
fn spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let wrap         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec         = wrap.handle();

	increment_ref( 4, &exec, tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `impl Spawn`
//
#[ test ]
//
#[ cfg( feature = "tokio_tp" ) ]
//
fn spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let wrap         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec         = wrap.handle();

	increment( 4, &exec, tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a &TokioTp to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
#[ cfg( feature = "tokio_tp" ) ]
//
fn spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let wrap         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec         = wrap.handle();

	increment_clone( 4, &exec, tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a Arc<TokioTp> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
#[ cfg( feature = "tokio_tp" ) ]
//
fn spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let wrap         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec         = wrap.handle();

	increment( 4, Arc::new(exec), tx );

	let result = wrap.block_on( rx.next() ).expect( "Some" );

	assert_eq!( 5u8, result );
}


// pass a TokioTp to a function that takes exec: `impl SpawnHandle`
//
#[ cfg(all( feature = "spawn_handle", feature = "tokio_tp" )) ]
//
#[ test ]
//
fn spawn_handle()
{
	let wrap = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec = wrap.handle();

	let result = wrap.block_on( increment_spawn_handle( 4, exec ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<TokioTp> to a function that takes exec: `impl SpawnHandle`
//
#[ cfg(all( feature = "spawn_handle", feature = "tokio_tp" )) ]
//
#[ test ]
//
fn spawn_handle_arc()
{
	let wrap = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec = wrap.handle();

	let result = wrap.block_on( increment_spawn_handle( 4, Arc::new(exec) ) );

	assert_eq!( 5u8, result );
}


// pass a AsyncStd to a function that takes exec: `&dyn SpawnHandle`
//
#[ cfg(all( feature = "spawn_handle", feature = "tokio_tp" )) ]
//
#[ test ]
//
fn spawn_handle_os()
{
	let wrap = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec = wrap.handle();

	let result = wrap.block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}


// use a TokioHandle after dropping the executor.
//
#[ test ]
//
#[ cfg( feature = "tokio_tp" ) ]
//
fn spawn_drop_exec_tp()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let wrap         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec         = wrap.handle();

	drop( wrap );

	increment( 4, exec, tx );

	let result = futures::executor::block_on( rx.next() );

	assert!( result.is_none() );
}


// use a TokioHandle after dropping the executor.
//
#[ test ]
//
#[ cfg( feature = "tokio_ct" ) ]
//
fn spawn_drop_exec_ct()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let wrap         = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let exec         = wrap.handle();

	drop( wrap );

	increment( 4, exec, tx );

	let result = futures::executor::block_on( rx.next() );

	assert!( result.is_none() );
}

