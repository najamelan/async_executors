#[ allow(unused_imports) ] // some imports are conditional on features
//
use
{
	std         :: { future::Future } ,
	std         :: { task::{ Poll, Context }, pin::Pin                      } ,
	futures_util:: { future::{ AbortHandle, Aborted, RemoteHandle }, ready  } ,
};


#[ cfg( feature = "async_global" ) ]
//
use async_global_executor::{ Task as AsyncGlobalTask };

#[ cfg( feature = "async_std" ) ]
//
use async_std_crate::{ task::JoinHandle as AsyncStdJoinHandle };

#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
//
use tokio::{ task::JoinHandle as TokioJoinHandle };


/// A framework agnostic BlockingJoinHandle type.
/// You can't cancel a blocking task
//
#[ derive( Debug ) ]
//
#[ must_use = "JoinHandle will cancel your future when dropped." ]
//
pub struct BlockingJoinHandle<T> { inner: InnerJh<T> }

impl<T> BlockingJoinHandle<T> {
	fn new(inner: InnerJh<T>) -> Self {
		Self {
			inner
		}
	}

	#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
	/// Make a tokio_jh
	pub fn tokio(handle: TokioJoinHandle<T>  ) -> Self {
		Self::new(InnerJh::Tokio { handle })
	}

	/// Make a async_std handle
	pub fn pthread(handle  : std::thread::JoinHandle<T> , done: futures_channel::oneshot::Receiver<()>) -> Self {
		Self::new(InnerJh::Pthread { handle: Some(handle), done })
	}
}
#[ derive(Debug) ] #[ allow(dead_code) ]
enum InnerJh<T>
{
	/// Wrapper around tokio JoinHandle.
	//
	#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
	//
	Tokio
	{
		handle  : TokioJoinHandle<T> ,
	},

	Pthread
	{
		handle: Option<std::thread::JoinHandle<T>>,
		done: futures_channel::oneshot::Receiver<()>
	},
}



impl<T> BlockingJoinHandle<T>
{
	/// TODO: Joins this handle blocking the thread
	#[allow(dead_code)]
	fn join_blocking( self )
	{
		todo!()
	}
}



impl<T: 'static> Future for BlockingJoinHandle<T>
{
	type Output = T;

	fn poll( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Self::Output>
	{
		match &mut self.get_mut().inner
		{
			#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
			//
			InnerJh::Tokio{ handle, .. } =>
			{
				match ready!( Pin::new( handle ).poll( cx ) )
				{
					Ok (t) => Poll::Ready( t ),

					Err(e) =>
					{
						panic!( "Task has been canceled. Are you dropping the executor to early? Error: {}", e );
					}
				}
			}

			InnerJh::Pthread { handle, done } => {
				match done.try_recv() {
					Ok(Some(())) => {
						Poll::Ready(handle.take().unwrap().join().unwrap())
					}
					Ok(None) => {
						Poll::Pending
					}
					Err(_canceled) => {
						Poll::Ready(handle.take().expect("You cannot join twice").join().expect("Task panicked"))
						// This should not return because we know the task has been canceled due to
						// 1) panicking 2) dropping the oneshot::Sender accidentally (which is a bug)
					}
				}
			}
		}
	}
}



impl<T> Drop for BlockingJoinHandle<T>
{
	fn drop( &mut self )
	{

	}
}
