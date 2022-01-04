#![ cfg( feature = "glommio" ) ]
//
// Using glommio requires increasing the rlimit_memlock.
// See: https://github.com/DataDog/glommio/issues/1
//
// Tested:
//
// ✔ pass a     GlommioCt  to a function that takes exec: `impl Spawn`
// ✔ pass a    &GlommioCt  to a function that takes exec: `&impl Spawn`
// ✔ pass a    &GlommioCt  to a function that takes exec: `impl Spawn`
// ✔ pass a    &GlommioCt  to a function that takes exec: `impl Spawn + Clone`
// ✔ pass a Arc<GlommioCt> to a function that takes exec: `impl Spawn`
// ✔ pass a     GlommioCt  to a function that takes exec: `impl SpawnHandle`
// ✔ pass a Arc<GlommioCt> to a function that takes exec: `impl SpawnHandle`
// ✔ pass a    &GlommioCt  to a function that takes exec: `&dyn SpawnHandle`
//
// ✔ pass a    GlommioCt  to a function that takes exec: `impl LocalSpawn`
// ✔ pass a   &GlommioCt  to a function that takes exec: `&impl LocalSpawn`
// ✔ pass a   &GlommioCt  to a function that takes exec: `impl LocalSpawn`
// ✔ pass a   &GlommioCt  to a function that takes exec: `impl LocalSpawn + Clone`
// ✔ pass a Rc<GlommioCt> to a function that takes exec: `impl LocalSpawn`
// ✔ pass a    GlommioCt  to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a Rc<GlommioCt> to a function that takes exec: `impl LocalSpawnHandle`
// ✔ pass a   &GlommioCt  to a function that takes exec: `&dyn LocalSpawnHandle`

// ✔ pass an GommioCt to a function that requires a Timer.
// ✔ Verify Timeout future.
//
// ✔ Joinhandle::detach allows task to keep running.
// - Test cpu pinning.
// - What happens if we make a nested call to block_on
// - What happens if we call exec constructor again inside block_on.
//
mod common;

use
{
	common        :: * ,
	futures       :: { channel::{ mpsc }, StreamExt } ,
	glommio_crate :: { LocalExecutorBuilder         } ,
	std           :: { rc::Rc                       } ,
};



// pass a GlommioCt to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );
	let ex2          = exec.clone();

	let res = exec.block_on( async
	{
		increment( 4, ex2, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a &GlommioCt to a function that takes exec: `&impl Spawn`
//
#[ test ]
//
fn spawn_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment_ref( 4, &exec, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a &GlommioCt to a function that takes exec: `impl Spawn`
//
#[ test ]
//
fn spawn_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment( 4, &exec, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a &GlommioCt to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn spawn_clone_with_ref()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment_clone( 4, &exec, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a Arc<GlommioCt> to a function that takes exec: `impl Spawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn spawn_clone_with_arc()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment_clone( 4, Arc::new( exec.clone() ), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a GlommioCt to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle()
{
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( increment_spawn_handle( 4, exec.clone() ) );

	assert_eq!( 5u8, res );
}



// pass an Arc<GlommioCt> to a function that takes exec: `impl SpawnHandle`
//
#[ test ]
//
fn spawn_handle_arc()
{
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( increment_spawn_handle( 4, Arc::new( exec.clone() ) ) );

	assert_eq!( 5u8, res );
}



// pass a &GlommioCt to a function that takes exec: `&dyn SpawnHandle`
//
#[ test ]
//
fn spawn_handle_os()
{
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let result = exec.block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}



// spawn a large number of tasks.
//
#[ test ]
//
fn spawn_handle_many()
{
	let builder      = LocalExecutorBuilder::new();
	let exec         = &GlommioCt::new( builder ).expect( "create exec" );

	let _result = exec.block_on( async move
	{
		let amount  = 1000;
		let mut rxs = Vec::with_capacity( amount );

		for i in 0..amount
		{
			let (mut tx, rx) = mpsc::channel(3);

			rxs.push( rx.fold(0, |_,i| futures::future::ready(i)) );

			exec.spawn_local( async move { tx.send(i).await.unwrap(); } ).unwrap();
		}

		futures::future::join_all( rxs ).await;
	});
}



// ------------------ Local
//


// pass a GlommioCt to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn spawn_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment_local( 4, exec.clone(), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a &GlommioCt to a function that takes exec: `&impl LocalSpawn`
//
#[ test ]
//
fn spawn_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment_ref_local( 4, &exec, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a &GlommioCt to a function that takes exec: `impl LocalSpawn`
//
#[ test ]
//
fn spawn_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment_local( 4, &exec, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a &GlommioCt to a function that takes exec: `impl LocalSpawn + Clone`
//
#[ test ]
//
fn spawn_clone_with_ref_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment_clone_local( 4, &exec, tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a Arc<GlommioCt> to a function that takes exec: `impl LocalSpawn`.
// Possible since futures 0.3.2.
//
#[ test ]
//
fn spawn_clone_with_rc_local()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( async
	{
		increment_clone_local( 4, Rc::new( exec.clone() ), tx );

		rx.next().await.expect( "Some" )
	});

	assert_eq!( 5u8, res );
}



// pass a GlommioCt to a function that takes exec: `impl LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_local()
{
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( increment_spawn_handle_local( 4, exec.clone() ) );

	assert_eq!( 5u8, *res );
}



// pass an Rc<GlommioCt> to a function that takes exec: `impl LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_rc_local()
{
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let res = exec.block_on( increment_spawn_handle_local( 4, Rc::new( exec.clone() ) ) );

	assert_eq!( 5u8, *res );
}



// pass a &GlommioCt to a function that takes exec: `&dyn LocalSpawnHandle`
//
#[ test ]
//
fn spawn_handle_local_os()
{
	let builder      = LocalExecutorBuilder::new();
	let exec         = GlommioCt::new( builder ).expect( "create exec" );

	let result = exec.block_on( increment_spawn_handle_os( 4, &exec ) );

	assert_eq!( 5u8, result );
}



// Joinhandle::detach allows task to keep running.
//
#[ test ]
//
fn join_handle_detach()
{
	let builder      = LocalExecutorBuilder::new();
	let exec         = &GlommioCt::new( builder ).expect( "create exec" );

	let (in_tx , in_rx ) = oneshot::channel();
	let (out_tx, out_rx) = oneshot::channel();


	exec.block_on( async move
	{
		let handle = exec.spawn_handle( async move
		{
			let content = in_rx.await.expect( "receive on in" );

			out_tx.send( content ).expect( "send on out" );

		}).expect( "spawn task" );

		// This moves out handle and drops it.
		//
		handle.detach();

		in_tx.send( 5u8 ).expect( "send on in" );

			assert_eq!( out_rx.await, Ok(5) );
	});
}



// pass a GlommioCt to a function that requires a YieldNow.
//
#[ test ]
//
fn yield_run_subtask_first() -> DynResultNoSend<()>
{
	let builder = LocalExecutorBuilder::new()

		.preempt_timer( std::time::Duration::from_millis(20) )
	;

	let exec = &GlommioCt::new( builder )?;

	exec.block_on( try_yield_now_glommio( exec ) ).map_err( |e| e as Box<dyn std::error::Error> )
}



// pass a GlommioCt to a function that requires a YieldNow.
//
#[ test ]
//
fn yield_run_subtask_last() -> DynResultNoSend<()>
{
	let builder = LocalExecutorBuilder::new()

		.preempt_timer( std::time::Duration::from_millis(20) )
	;

	let exec = GlommioCt::new( builder )?;

	exec.block_on( without_yield_now_glommio( exec.clone() ) ).map_err( |e| e as Box<dyn std::error::Error> )
}


// Use same exec to run this function as you pass in. This is different from the one for other
// executors as it will sleep because glommio does not yield unless the task has been running for some time.
//
pub async fn try_yield_now_glommio( exec: impl SpawnHandle<()> + YieldNow ) -> DynResult<()>
{
	let flag  = Arc::new( AtomicBool::new( false ) );
	let flag2 = flag.clone();

	let task = async move
	{
		flag2.store( true, SeqCst );
	};

	let handle = exec.spawn_handle( task )?;

	// glommio will only yield if we have been running sufficiently long.
	//
	std::thread::sleep( Duration::from_millis( 20 ) );


	exec.yield_now().await;

	// by now task should have run because of the yield_now.
	//
	assert!( flag.load(SeqCst) );

	handle.await;

	Ok(())
}


// Glommio immediately polls a spawned task. Hence this test doesnt work unless we try to do the
// yield_now in the subtask. Otherwise the subtask runs during the spawn call.
//
async fn without_yield_now_glommio( exec: impl LocalSpawnHandle<()> + YieldNow + Clone + 'static ) -> DynResult<()>
{
	let flag  = Arc::new( AtomicBool::new( false ) );
	let flag2 = flag.clone();
	let exec2 = exec.clone();

	let task = async move
	{
		// Glommio also doesn't yield unless the task has been running for some time.
		//
		std::thread::sleep( Duration::from_millis( 20 ) );

		exec2.yield_now().await;

		flag2.store( true, SeqCst );
	};

	let handle = exec.spawn_handle_local( task )?;

	// spawned task should not have run yet.
	//
	assert!( !flag.load(SeqCst) );

	handle.await;

	Ok(())
}



// pass an GommioCt to a function that requires a Timer.
//
#[ cfg( feature = "timer" ) ]
//
#[ test ]
//
fn timer_should_wake_local()
{
	let exec = GlommioCt::new( LocalExecutorBuilder::new() ).expect( "create exec" );
	exec.block_on( timer_should_wake_up_local( exec.clone() ) );
}



// pass an GlommioCt to a function that requires a Timer.
//
#[ cfg( feature = "timer" ) ]
//
#[ test ]
//
fn run_timeout()
{
	let exec = &GlommioCt::new( LocalExecutorBuilder::new() ).expect( "create exec" );

	exec.block_on( timeout( exec ) );
}



// pass an GlommioCt to a function that requires a Timer.
//
#[ cfg( feature = "timer" ) ]
//
#[ test ]
//
fn run_dont_timeout()
{
	let exec = &GlommioCt::new( LocalExecutorBuilder::new() ).expect( "create exec" );

	exec.block_on( dont_timeout( exec ) );
}
