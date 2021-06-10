//! This example demonstrates how you can use Tokio io primitives on an
//! agnostic executor. Use the `TokioIo` trait to request an executor
//! that runs tasks in the context of a tokio reactor.
//!
//! The features `async_global_tokio` and `async_std_tokio` will turn on the
//! corresponding features on the underlying libraries.
//!
//! The `tokio_io` feature will enable `net` and `process` on tokio and make
//! sure the executor builder has `enable_reactor()`.
//!
//! Run with:
//!
//! cargo run --example tokio_io --features "tokio_ct, async_global_tokio, async_std_tokio, tokio_io"
//!
//! See the API docs for detailed explanation of how the features interact.
//!
//! Expected output:
//!
//! Testing tokio::net::TcpStream spawned from async-global-executor
//! Testing tokio::net::TcpStream spawned from async-std
//! Testing tokio::net::TcpStream spawned from tokio current thread
//
use
{
	async_executors :: { AsyncGlobal, AsyncStd, TokioCtBuilder, TokioIo, SpawnHandle, SpawnHandleExt } ,
	trait_set       :: { trait_set                                                                   } ,
	tokio::net      :: { TcpListener, TcpStream                                                      } ,
};


pub type DynResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync> >;


/// Creates a connected pair of sockets.
/// Uses tokio tcp stream. This will only work if the reactor is running.
//
pub async fn socket_pair() -> DynResult< (TcpStream, TcpStream) >
{
	// port 0 = let the OS choose
	//
	let listener = TcpListener::bind("127.0.0.1:0").await?;
	let stream1  = TcpStream::connect(listener.local_addr()?).await?;
	let stream2  = listener.accept().await?.0;

	Ok( (stream1, stream2) )
}


trait_set!
{
	pub trait LibExec = SpawnHandle<()> + TokioIo;
}


async fn lib_function( exec_name: &str, exec: impl LibExec ) -> DynResult<()>
{
	println!( "Testing tokio::net::TcpStream spawned from {}", exec_name );

	let test = async
	{
		// This will panic if the tokio reactor is not running.
		//
		let _ = socket_pair().await.expect( "socket_pair" );
	};

	exec.spawn_handle( test )?.await;

	Ok(())
}


fn main() -> DynResult<()>
{
	AsyncGlobal::block_on( lib_function( "async-global-executor", AsyncGlobal ) )?;
	AsyncStd   ::block_on( lib_function( "async-std"            , AsyncStd    ) )?;

	let tokio_ct = &TokioCtBuilder::new().build()?;

	tokio_ct.block_on( lib_function( "tokio current thread", tokio_ct.clone() ) )?;

	Ok(())
}
