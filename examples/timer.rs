//! This example demonstrates how to use an agnostic timer.
//! When you specify the Timer trait, the client must give you an executor
//! that supports timer operations.
//!
//! In this example, async-std draws it's timer from async-io and when not
//! on wasm there currently is no way to turn it off.
//!
//! TokioCt can get it's timer from tokio with the tokio_timer feature, or
//! from futures-timer if the timer feature is enabled. As we need to specify
//! the required features for examples in Cargo.toml, you'll have to enable both.
//! In reality you only need to choose one.
//!
//! cargo run --example timer --features "notwasm tokio_timer timer async_std tokio_ct"
//!
//! See the API docs for detailed explanation of how the features interact.
//!
//! Expected output:
//!
//! async-std: running for 1s.
//! async-std: running for 2s.
//! async-std: running for 3s.
//! async-std: running for 4s.
//! async-std: running for 5s.
//! tokio current thread: running for 1s.
//! tokio current thread: running for 2s.
//! tokio current thread: running for 3s.
//! tokio current thread: running for 4s.
//! tokio current thread: running for 5s.
//
use
{
	async_executors :: { AsyncStd, TokioCt, Timer, SpawnHandle } ,
	trait_set       :: { trait_set                             } ,
	std             :: { time::Duration                        } ,
};

pub type DynResult<T> = Result<T, Box<dyn std::error::Error> >;


trait_set!
{
	pub trait LibExec = SpawnHandle<()> + Timer;
}


async fn lib_function( exec_name: &str, exec: impl LibExec )
{
	for i in 1..6
	{
		exec.sleep( Duration::from_secs(1) ).await;
		println!( "{}: running for {}s.", exec_name, i );
	}
}


fn main() -> DynResult<()>
{
	AsyncStd::block_on( lib_function( "async-std", AsyncStd ) );

	let tokio_ct = TokioCt::new()?;

	tokio_ct.block_on( lib_function( "tokio current thread", tokio_ct.clone() ) );

	Ok(())
}
