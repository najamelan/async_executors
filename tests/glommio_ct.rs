#![ cfg( feature = "glommio" ) ]

// ✔ pass a     GlommioCt  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<GlommioCt> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &GlommioCt  to a function that takes exec: `&dyn SpawnHandle`
//
// ✔ Joinhandle::detach allows task to keep running.
//
mod common;

use
{
	common           :: { *          } ,
};

// pass a GlommioCt to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle()
{
	let exec   = GlommioCt::new("unnamed", None);
	let result = exec.block_on( increment_spawn_handle( 4, &exec ) );

	assert_eq!( 5u8, result );
}


// pass an Arc<GlommioCt> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle_arc()
{
	let exec   = Arc::new(GlommioCt::new("unnamed", None));
	let result = exec.block_on( increment_spawn_handle( 4, Arc::clone(&exec) ) );

	assert_eq!( 5u8, result );
}


// pass a GlommioCt to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn spawn_handle_os()
{
	let exec   = GlommioCt::new("unnamed", None);
	let result = exec.block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}



// Joinhandle::detach allows task to keep running.
//
#[ test ]
//
fn join_handle_detach()
{
	let exec   = GlommioCt::new("unnamed", None);

	let (in_tx , in_rx ) = oneshot::channel();
	let (out_tx, out_rx) = oneshot::channel();


	let exec = &exec;
	exec.block_on( async move
	{
		let in_join_handle = exec.spawn_handle( async move
		{
			let content = in_rx.await.expect( "receive on in" );

			out_tx.send( content ).expect( "send on out" );

		}).expect( "spawn task" );


		in_join_handle.detach();
		in_tx.send( 5u8 ).expect( "send on in" );

		assert_eq!( out_rx.await, Ok(5) );
	});
}
