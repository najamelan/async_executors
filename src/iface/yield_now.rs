use
{
	std::{ future::Future, pin::Pin, task::{ Context, Poll } } ,
	blanket::blanket,
};

/// Trait indicating that tasks can yield to the executor. This put's
/// the current task at the back of the schedulers queue, giving other
/// tasks a chance to run.
//
#[ blanket( derive(Ref, Mut, Arc, Rc, Box) ) ]
//
pub trait YieldNow
{
	/// Await this future in order to yield to the executor.
	//
	fn yield_now( &self ) -> YieldNowFut
	{
		YieldNowFut{ done: false }
	}
}



/// Future returned by [`YieldNow::yield_now`].
//
#[ derive( Debug, Copy, Clone ) ]
//
pub struct YieldNowFut
{
	done: bool,
}


impl Future for YieldNowFut
{
	type Output = ();

	fn poll( mut self: Pin<&mut Self>, cx: &mut Context<'_> ) -> Poll<()>
	{
		if self.done
		{
			return Poll::Ready(());
		}

		self.done = true;
		cx.waker().wake_by_ref();
		Poll::Pending
	}
}
