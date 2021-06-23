#[ allow(unused_imports) ] // some imports are conditional on features
//
use
{
	std         :: { future::Future, sync::atomic::{ AtomicBool, Ordering } } ,
	std         :: { task::{ Poll, Context }, pin::Pin                      } ,
	futures_util:: { future::{BoxFuture, AbortHandle, Aborted, RemoteHandle }, ready  } ,
	super       :: *,
};


/// A framework agnostic BlockingHandle type. This is returned by [`SpawnBlocking`](crate::SpawnBlocking).
/// Await this handle for the output of the task. You can't cancel a blocking task once it has started running.
/// If you drop this after the task starts running, it will just detach and let the task run in the background.
//
#[ derive( Debug ) ]
//
pub struct BlockingHandle<T>( InnerBh<T> );


impl<T> BlockingHandle<T>
{
	/// Make a wrapper around [`tokio::task::JoinHandle`].
	//
	#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
	//
	pub fn tokio( handle: TokioJoinHandle<T> ) -> Self
	{
		Self( InnerBh::Tokio(handle) )
	}


	/// Make a wrapper around [`async_global_executor::Task`].
	//
	#[ cfg( feature = "async_global" ) ]
	//
	pub fn async_global( task: BoxFuture<'static, T> ) -> Self
	{
		Self( InnerBh::AsyncGlobal(task) )
	}


	/// Make a wrapper around [`async_std::task::JoinHandle`](async_std_crate::task::JoinHandle).
	//
	#[ cfg( feature = "async_std" ) ]
	//
	pub fn async_std( handle: AsyncStdJoinHandle<T> ) -> Self
	{
		Self( InnerBh::AsyncStd(handle) )
	}

	/// Make a wrapper around
	pub fn std_thread(handle: std::thread::JoinHandle<T>, alive: std::sync::Weak<()>) -> Self {
		Self(InnerBh::StdThread(Some(handle), alive))
	}
}



#[ allow(dead_code) ]
//
enum InnerBh<T>
{
	/// Wrapper around tokio BlockingHandle.
	//
	#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
	//
	Tokio( TokioJoinHandle<T> ),

	/// Wrapper around AsyncStd BlockingHandle.
	//
	#[ cfg( feature = "async_global" ) ]
	//
	AsyncGlobal( BoxFuture<'static, T> ),

	/// Wrapper around AsyncStd BlockingHandle.
	//
	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd( AsyncStdJoinHandle<T> ),

	/// Wrapper around std::thread::JoinHandle
	StdThread( Option<std::thread::JoinHandle<T>>, std::sync::Weak<()> ),

	Phantom(std::marker::PhantomData< fn()->T >),
}



impl<T: 'static> Future for BlockingHandle<T>
{
	type Output = T;

	fn poll( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Self::Output>
	{
		match &mut self.0
		{
			#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
			//
			InnerBh::Tokio(handle) =>
			{
				match ready!( Pin::new( handle ).poll( cx ) )
				{
					Ok(t)  => Poll::Ready( t ),

					Err(e) => panic!( "Task has been canceled or has panicked. \
						Are you dropping the executor to early? Error: {}", e ),
				}
			}

			#[ cfg( feature = "async_std"    ) ] InnerBh::AsyncStd   ( handle ) => Pin::new( handle ).poll( cx ) ,
			#[ cfg( feature = "async_global" ) ] InnerBh::AsyncGlobal( task   ) => Pin::new( task   ).poll( cx ) ,

			InnerBh::StdThread(handle, alive) => {
				if let Some(_) = alive.upgrade() {
					// FIXME: use a proper waking mechanism
					cx.waker().wake_by_ref();
					Poll::Pending
				} else {
					match handle.take().expect("Can only be joined once").join() {
						Ok(ok) => Poll::Ready(ok),
						Err(panicked) => {
							std::panic::resume_unwind(panicked)
						}
					}
				}
			}
			InnerBh::Phantom(_) => unreachable!(),
		}
	}
}



impl<T> std::fmt::Debug for InnerBh<T>
{
	fn fmt( &self,	f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "InnerBh" )
	}
}
