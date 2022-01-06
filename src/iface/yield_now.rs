use
{
	std::{ future::Future, pin::Pin, task::{ Context, Poll } } ,
	blanket::blanket,
};

/// Trait indicating that tasks can yield to the executor. This put's
/// the current task at the back of the schedulers queue, giving other
/// tasks a chance to run.
///
/// In practice for most executors this just returns a future that will,
/// the first time it is polled, wake up the waker and then return Pending.
///
/// The problem with using the executors native implementation is that they
/// generally return an opaque future we would have to box.
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
#[must_use = "YieldNowFut doesn't do anything unless polled or awaited."]
//
#[ derive( Debug, Copy, Clone ) ]
//
pub struct YieldNowFut
{
	pub(crate) done: bool,
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
