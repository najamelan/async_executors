#[ allow(unused_imports) ]
//
use
{
	crate       :: { import::*, remote_handle::RemoteHandle                 } ,
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
/// You can call [`detach`](JoinHandle::detach) to leave the future running when dropping the handle.
///
/// This leverages the performance gains from the native join handles compared to
/// [RemoteHandle](futures_util::future::RemoteHandle) where possible.
///
/// It does wrap futures in [Abortable](futures_util::future::Abortable) where needed as
/// [tokio] and [async-std](async_std_crate) don't support canceling out of the box.
///
/// # Panics
///
/// There is an inconsistency between executors when it comes to a panicking task.
/// Generally we unwind the thread on which the handle is awaited when a task panics,
/// but async-std will also let the executor thread unwind. No `catch_unwind` was added to
/// bring async-std in line with the other executors here.
///
/// Awaiting the JoinHandle can also panic if you drop the executor before it completes.
//
#[ derive( Debug ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "spawn_handle" )) ) ]
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
	RemoteHandle( Option<RemoteHandle<T>> ),
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
				// only other use of this is in Drop impl and we consume self here,
				// so there cannot be any race as this does not sync things across threads,
				// hence Relaxed ordering.
				//
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
					// expect: it returns futures::future::Aborted, but we hold this abortable and
					// only expose the abort through being dropped, so this should be unreachable.
					//
					Ok (t) => Poll::Ready( t.expect( "task aborted" ) ),

					//
					Err(e) =>
					{
						dbg!(e);
						panic!( "Task has been canceled. Are you dropping the executor to early?" );
					}
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
	// see reasoning about Relaxed atomic in detach().
	//
	fn drop( &mut self )
	{
		match &mut self.inner
		{
			#[ cfg(any( feature = "tokio_tp", feature = "tokio_ct" )) ]
			//
			InnerJh::Tokio{ a_handle, detached, .. } =>

				if !detached.load( Ordering::Relaxed ) { a_handle.abort() },


			#[ cfg( feature = "async_std" ) ] InnerJh::AsyncStd { a_handle, detached, .. } =>

				if !detached.load( Ordering::Relaxed ) { a_handle.abort() },


			InnerJh::RemoteHandle( _ ) => {},
		};
	}
}
