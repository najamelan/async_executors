#[ allow(unused_imports) ]
//
use
{
	crate       :: { import::*                                              } ,
	std         :: { future::Future, sync::atomic::{ AtomicBool, Ordering } } ,
	futures_util:: { future::{ AbortHandle, Aborted }                       } ,
};


#[ cfg( feature = "async_std" ) ]
//
use async_std_crate::{ task::JoinHandle as AsyncStdJoinHandle };

#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
//
use tokio::{ task::JoinHandle as TokioJoinHandle };


/// A framework agnostic JoinHandle type. Cancels the future on dropping the handle.
/// You can call [`JoinHandle::detach`] to leave the future running when dropping the handle.
//
#[ derive( Debug ) ]
//
#[ must_use = "JoinHandle will cancel your future when dropped." ]
//
pub struct JoinHandle<T> { pub(crate) inner: InnerJh<T> }


#[ derive(Debug) ] #[ allow(dead_code) ]
//
pub(crate) enum InnerJh<T>
{
	/// Wrapper around tokio JoinHandle.
	//
	#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
	//
	Tokio
	{
		handle  : TokioJoinHandle<Result<T, Aborted>> ,
		a_handle: AbortHandle                         ,
		detached: AtomicBool                          ,
	},

	/// Wrapper around AsyncStd JoinHandle.
	//
	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd
	{
		handle  : AsyncStdJoinHandle<Result<T, Aborted>> ,
		a_handle: AbortHandle                            ,
		detached: AtomicBool                             ,
	},

	/// Wrapper around futures RemoteHandle.
	//
	RemoteHandle( Option<futures_util::future::RemoteHandle<T>> ),
}


impl<T> JoinHandle<T>
{
	/// Drops this handle without canceling the underlying future.
	///
	/// This method can be used if you want to drop the handle, but let the execution continue.
	//
	pub fn detach( mut self )
	{
		match &mut self.inner
		{
			#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
			//
			InnerJh::Tokio{ ref detached, .. } =>
			{
				detached.store( true, Ordering::Relaxed );
			}

			#[ cfg( feature = "async_std" ) ] InnerJh::AsyncStd{ ref detached, .. } =>
			{
				detached.store( true, Ordering::Relaxed );
			}

			InnerJh::RemoteHandle( handle ) =>
			{
				if let Some(rh) = handle.take() { rh.forget() };
			}
		}
	}
}


// Even if T is not Unpin, JoinHandle still is.
//
impl<T> Unpin for JoinHandle<T> {}


impl<T: 'static> Future for JoinHandle<T>
{
	type Output = T;

	fn poll( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Self::Output>
	{
		match &mut self.get_mut().inner
		{
			#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
			//
			InnerJh::Tokio{ handle, .. } =>
			{
				match futures_util::ready!( Pin::new( handle ).poll( cx ) )
				{
					Ok (t) => Poll::Ready( t.expect( "task panicked" ) ),
					Err(_) => unreachable!( "task shouldn't be aborted" ),
				}
			}


			#[ cfg( feature = "async_std" ) ] InnerJh::AsyncStd{ handle, .. } =>
			{
				match futures_util::ready!( Pin::new( handle ).poll( cx ) )
				{
					Ok (t) => Poll::Ready( t ),
					Err(_) => unreachable!(),
				}
			}


			InnerJh::RemoteHandle( ref mut handle ) => Pin::new( handle ).as_pin_mut().expect( "no polling after detach" ).poll( cx ),
		}
	}
}


impl<T> Drop for JoinHandle<T>
{
	fn drop( &mut self )
	{
		match &mut self.inner
		{
			#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
			//
			InnerJh::Tokio{ a_handle, detached, .. } =>

				if detached.load( Ordering::Relaxed ) { a_handle.abort() },


			#[ cfg( feature = "async_std" ) ] InnerJh::AsyncStd { a_handle, detached, .. } =>

				if detached.load( Ordering::Relaxed ) { a_handle.abort() },


			InnerJh::RemoteHandle( _ ) => {},
		};
	}
}
