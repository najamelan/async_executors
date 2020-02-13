use
{
	crate       :: { import::*                                              } ,
	std         :: { future::Future, sync::atomic::{ AtomicBool, Ordering } } ,
	futures_util:: { future::{ AbortHandle, Aborted }                       } ,
};


#[ cfg( feature = "async_std" ) ]
//
use async_std_crate::{ task::JoinHandle as AsJoinHandle };

#[ cfg( feature = "tokio_tp" ) ]
//
use tokio::{ task::JoinHandle as TokioJoinHandle };



/// A framework agnostic JoinHandle type. Cancels the future on dropping the handle.
/// You can call [`JoinHandle::detach`] to leave the future running when dropping the handle.
//
#[ derive( Debug ) ]
//
#[ must_use = "JoinHandle will cancel your future when dropped." ]
//
pub enum JoinHandle<T>
{
	/// Wrapper around tokio JoinHandle.
	//
	#[ cfg( feature = "tokio_tp" ) ]
	//
	Tokio
	{
		#[doc(hidden)] handle  : TokioJoinHandle<Result<T, Aborted>> ,
		#[doc(hidden)] a_handle: AbortHandle                         ,
		#[doc(hidden)] detached: AtomicBool                          ,
	},

	/// Wrapper around AsyncStd JoinHandle.
	//
	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd
	{
		#[doc(hidden)] handle  : AsJoinHandle<Result<T, Aborted>> ,
		#[doc(hidden)] a_handle: AbortHandle                      ,
		#[doc(hidden)] detached: AtomicBool                       ,
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
		match &mut self
		{
			#[ cfg( feature = "tokio_tp"  ) ] JoinHandle::Tokio{ ref detached, .. } =>
			{
				detached.store( true, Ordering::Relaxed );
			}

			#[ cfg( feature = "async_std" ) ] JoinHandle::AsyncStd{ ref detached, .. } =>
			{
				detached.store( true, Ordering::Relaxed );
			}

			JoinHandle::RemoteHandle( handle ) =>
			{
				if let Some(rh) = handle.take() { rh.forget() };
			}
		}
	}
}


// Even if T is not Unpin, JoinHandle still is.
//
impl<T> Unpin for JoinHandle<T> {}


impl<T> Future for JoinHandle<T>
{
	type Output = T;

	fn poll( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Self::Output>
	{
		match self.get_mut()
		{

			#[ cfg( feature = "tokio_tp" ) ] JoinHandle::Tokio{ handle, .. } =>
			{
				match futures_util::ready!( Pin::new( handle ).poll( cx ) )
				{
					Ok (t) => Poll::Ready( t.expect( "future panicked" ) ),
					Err(_) => unreachable!( "future shouldn't be aborted" ),
				}
			}


			#[ cfg( feature = "async_std" ) ] JoinHandle::AsyncStd{ handle, .. } =>
			{
				match futures_util::ready!( Pin::new( handle ).poll( cx ) )
				{
					Ok (t) => Poll::Ready( t ),
					Err(_) => unreachable!(),
				}
			}


			JoinHandle::RemoteHandle( ref mut handle ) => Pin::new( handle ).as_pin_mut().expect( "no polling after detach" ).poll( cx ),
		}
	}
}


impl<T> Drop for JoinHandle<T>
{
	fn drop( &mut self )
	{
		match self
		{
			#[ cfg( feature = "tokio_tp"  ) ] JoinHandle::Tokio{ a_handle, detached, .. } =>

				if detached.load( Ordering::Relaxed ) { a_handle.abort() },


			#[ cfg( feature = "async_std" ) ] JoinHandle::AsyncStd { a_handle, detached, .. } =>

				if detached.load( Ordering::Relaxed ) { a_handle.abort() },


			JoinHandle::RemoteHandle( _ ) => {},
		};
	}
}
