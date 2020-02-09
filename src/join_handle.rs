use
{
	crate:: { import::* } ,
	std  :: { future::Future, convert::From } ,
};


#[ cfg( feature = "async_std" ) ]
//
use async_std_crate::{ task::JoinHandle as AsJoinHandle };

#[ cfg( feature = "tokio_tp" ) ]
//
use tokio::{ task::JoinHandle as TokioJoinHandle };



/// A framework agnostic JoinHandle type.
//
#[ derive( Debug ) ]
//
pub enum JoinHandle<T>
{
	/// Wrapper around tokio JoinHandle.
	//
	#[ cfg( feature = "tokio_tp" ) ]
	//
	Tokio( tokio::task::JoinHandle<T> ),

	/// Wrapper around AsyncStd JoinHandle.
	//
	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd( AsJoinHandle<T> ),

	/// Wrapper around futures RemoteHandle.
	//
	ThreadPool( futures_util::future::RemoteHandle<T> ),
}


impl<T: 'static + Send> Future for JoinHandle<T>
{
	type Output = T;

	fn poll( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Self::Output>
	{
		match self.get_mut()
		{
			#[ cfg( feature = "tokio_tp"  ) ] JoinHandle::Tokio( handle ) =>
			{
				use futures_util::ready;

				match ready!( Pin::new( handle ).poll( cx ) )
				{
					Ok (t) => Poll::Ready( t ),
					Err(e) => panic!( "{}", e ),
				}
			}

			#[ cfg( feature = "async_std" ) ] JoinHandle::AsyncStd  ( handle ) => Pin::new( handle ).poll( cx ),
			                                  JoinHandle::ThreadPool( handle ) => Pin::new( handle ).poll( cx ),
		}
	}
}


#[ cfg( feature = "async_std" ) ]
//
impl<T> From<AsJoinHandle<T>> for JoinHandle<T>
{
	fn from( handle: AsJoinHandle<T> ) -> Self
	{
		JoinHandle::AsyncStd( handle )
	}
}


#[ cfg( feature = "tokio_tp" ) ]
//
impl<T> From<TokioJoinHandle<T>> for JoinHandle<T>
{
	fn from( handle: TokioJoinHandle<T> ) -> Self
	{
		JoinHandle::Tokio( handle )
	}
}


