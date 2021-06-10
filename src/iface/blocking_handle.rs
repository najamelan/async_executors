#[ allow(unused_imports) ] // some imports are conditional on features
//
use
{
	std         :: { future::Future, sync::atomic::{ AtomicBool, Ordering } } ,
	std         :: { task::{ Poll, Context }, pin::Pin                      } ,
	futures_util:: { future::{ AbortHandle, Aborted, RemoteHandle }, ready  } ,
	super       :: *,
};


#[ cfg( feature = "async_global" ) ]
//
type BoxedFut<T> = Pin<Box< dyn Future<Output=T> + Send >>;


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
	pub fn async_global( task: BoxedFut<T> ) -> Self
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
	AsyncGlobal( BoxedFut<T> ),

	/// Wrapper around AsyncStd BlockingHandle.
	//
	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd( AsyncStdJoinHandle<T> ),

	Phantom(std::marker::PhantomData< fn()->T >),
}



impl<T: 'static> Future for BlockingHandle<T>
{
	type Output = T;

	fn poll( self: Pin<&mut Self>, _cx: &mut Context<'_> ) -> Poll<Self::Output>
	{
		match &mut self.get_mut().0
		{
			#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
			//
			InnerBh::Tokio(handle) =>
			{
				match ready!( Pin::new( handle ).poll( _cx ) )
				{
					Ok(t)  => Poll::Ready( t ),

					Err(e) => panic!( "Task has been canceled or has panicked. \
						Are you dropping the executor to early? Error: {}", e ),
				}
			}

			#[ cfg( feature = "async_std"    ) ] InnerBh::AsyncStd   ( handle ) => Pin::new( handle ).poll( _cx ) ,
			#[ cfg( feature = "async_global" ) ] InnerBh::AsyncGlobal( task   ) => Pin::new( task   ).poll( _cx ) ,

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
