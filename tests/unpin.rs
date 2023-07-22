#![ cfg(all( nightly, not(target_os = "unknown"), any( feature = "threadpool", feature = "async_std" ) )) ]
#![ feature( negative_impls )]

// Tested:
//
// ✔ What happens to joinhandle if T is Unpin. The handle is unpin even if T isn't,
//   even without a manual unpin implementation.
//
mod common;

use common::*;


#[ derive( PartialEq, Eq, Debug ) ]
//
struct Boon;

impl !Unpin for Boon {}


fn assert_unpin<T: Unpin>( _t: &T ) {}




#[ cfg( feature = "threadpool" ) ]
//
use futures::executor::ThreadPool;

// the handle is unpin even if T isn't
//
#[ test ] #[ cfg( feature = "threadpool" ) ]
//
fn unpin_handle()
{
	let exec = ThreadPool::new().expect( "create threadpool" );

	let join_handle = exec.spawn_handle( async { Boon } ).expect( "spawn" );

	assert_unpin( &join_handle );
	let t = block_on( join_handle );

	assert_eq!( Boon, t );
}


// the handle is unpin even if T isn't
//
#[ test ] #[ cfg( feature = "async_std" ) ]
//
fn unpin_handle_async_std()
{
	let join_handle = AsyncStd.spawn_handle( async { Boon } ).expect( "spawn" );

	assert_unpin( &join_handle );
	let t = AsyncStd::block_on( join_handle );

	assert_eq!( Boon, t );
}
