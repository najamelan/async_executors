//! This example shows how you can yield to an executor. In this case we can count on
//! the other task that is waiting to run first. Obviously that will only work on a
//! single threaded executor.
//!
//! In any case yield_now will allow other tasks to run if they are waiting.
//!
//
use
{
	async_executors :: { TokioCtBuilder, SpawnHandle, SpawnHandleExt, YieldNow } ,
	std             :: { sync::{ atomic::{ AtomicBool, Ordering::SeqCst }, Arc } }
};


pub type DynResult<T> = Result< T, Box<dyn std::error::Error + Send + Sync> >;



// Use same exec to run this function as you pass in.
//
pub async fn lib_function( exec: impl SpawnHandle<()> + YieldNow ) -> DynResult<()>
{
	let flag  = Arc::new( AtomicBool::new( false ) );
	let flag2 = flag.clone();

	let task = async move
	{
		flag2.store( true, SeqCst );
		println!( "I am subtask" );
	};

	let handle = exec.spawn_handle( task )?;

	println!( "I am yielding" );
	exec.yield_now().await;

	// by now task should have run because of the yield_now.
	//
	assert!( flag.load(SeqCst) );

	handle.await;

	Ok(())
}



fn main() -> DynResult<()>
{
	let exec = TokioCtBuilder::new().build().expect( "create tokio threadpool" );

	exec.block_on( lib_function( &exec ) )
}
