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

impl<T> Future for JoinHandle<T>
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


#[derive(Debug)]
//
pub(crate) enum Inner<T>
{
	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd(async_std_crate::task::JoinHandle<T>),

	_Phantom( PhantomData<T>)
}

impl<T> Unpin for Inner<T> {}


impl<T> Future for Inner<T>
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

			Inner::_Phantom(_) => { unreachable!() }
		}
	}
}
