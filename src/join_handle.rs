use crate::import::*;

/// A handle that awaits the result of a task.
//
#[derive(Debug)]
//
pub struct JoinHandle<T>
{
	inner: Inner<T>
}

// unsafe impl<T> Send for JoinHandle<T> {}
// unsafe impl<T> Sync for JoinHandle<T> {}


impl<T> JoinHandle<T>
{
	#[ allow(dead_code) ]
	//
	pub(crate) fn new( inner: Inner<T> ) -> JoinHandle<T>
	{
		JoinHandle{ inner }
	}
}

impl<T: 'static> Future for JoinHandle<T>
{
	type Output = T;

	fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
	{
		Pin::new( &mut self.inner ).poll(cx)
	}
}


#[ cfg( feature = "async_std" ) ]
//
impl<T> From< async_std_crate::task::JoinHandle<T> > for JoinHandle<T>
{
	fn from( inner: async_std_crate::task::JoinHandle<T> ) -> Self
	{
		Self::new( Inner::AsyncStd(inner) )
	}
}


#[ cfg(any( feature = "bindgen", feature = "juliex", feature = "threadpool", feature = "tokio_tp", feature = "tokio_ct", feature = "localpool" )) ]
//
impl<T> From< oneshot::Receiver<T> > for JoinHandle<T>
{
	fn from( inner: oneshot::Receiver<T> ) -> Self
	{
		Self::new( Inner::Oneshot(inner) )
	}
}


#[derive(Debug)]
//
pub(crate) enum Inner<T>
{
	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd( async_std_crate::task::JoinHandle<T> ),

	#[ cfg(any( feature = "bindgen", feature = "juliex", feature = "threadpool", feature = "tokio_tp", feature = "tokio_ct", feature = "localpool" )) ]
	//
	Oneshot( oneshot::Receiver<T> ),

	_Phantom( PhantomData<T>)
}

impl<T> Unpin for Inner<T> {}


// This currently forwards panics from the spawned tasks.
//
impl<T: 'static > Future for Inner<T>
{
	type Output = T;

	#[ allow( unused_variables ) ]
	//
	fn poll( self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<Self::Output>
	{
		match self.get_mut()
		{
			#[ cfg( feature = "async_std" ) ]
			//
			Inner::AsyncStd( inner ) =>
			{
				Pin::new( inner ).poll(cx)
			}


			#[ cfg(any( feature = "bindgen", feature = "juliex", feature = "threadpool", feature = "tokio_ct", feature = "tokio_tp", feature = "localpool" )) ]
			//
			Inner::Oneshot( inner ) =>
			{
				match ready!( Pin::new( inner ).poll(cx) )
				{
					Ok(val) => val.into(),

					// The oneshot channel can return a cancelled error, but since we don't
					// provide any other way to cancel it than dropping the joinhandle, it
					// just means the task has panicked.
					//
					Err(_)  => panic!( "The spawned task has panicked." )
				}
			}

			Inner::_Phantom(_) => { unreachable!() }
		}
	}
}
