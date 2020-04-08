#![ cfg(all( nightly, feature = "spawn_handle" )) ]
#![ feature( negative_impls )]

// Tested:
//
// ✔ What happens to joinhandle if T is Unpin. The handle is unpin even if T isn't,
//   even without a manual unpin implementation.
//
mod common;

use
{
	common  :: { *                    } ,
	futures :: { executor::ThreadPool } ,
};


#[ derive( PartialEq, Eq, Debug ) ]
//
struct Boon;

impl !Unpin for Boon {}


fn assert_unpin<T: Unpin>( _t: &T ) {}


// the handle is unpin even if T isn't
//
#[ test ]
//
fn unpin_handle()
{
	let exec = ThreadPool::new().expect( "create threadpool" );

	let join_handle = exec.spawn_handle( async { Boon } ).expect( "spawn" );

	assert_unpin( &join_handle );
	let t = AsyncStd::block_on( join_handle );

	assert_eq!( Boon, t );
}


// the handle is unpin even if T isn't
//
#[ test ]
//
fn unpin_handle_async_std()
{
	let exec = AsyncStd::default();

	let join_handle = exec.spawn_handle( async { Boon } ).expect( "spawn" );

	assert_unpin( &join_handle );
	let t = AsyncStd::block_on( join_handle );

	assert_eq!( Boon, t );
}
