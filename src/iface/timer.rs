use
{
	std::{ time::Duration, future::Future, task::{ Poll, Context }, pin::Pin },
	pin_project::pin_project,
};

/// Represents the fact that an executor has timer functionality.
///
//  Implementation:
//  - for tokio: use tokio when tokio_time feature is enabled, futures-timer otherwise.
//  - for async-global-executor: use futures-timer.
//  - for glommio: has own timer that can't be turned off. But we don't use it because
//    it's not Send.
//  - for bindgen: use futures-timer
//  - for async-std: has a timer that cannot be turned off. Isn't Send on Wasm.
//
//  The trait needs to be available inconditionally, as a library must be able
//  to depend on it without specifying a backend.
//
#[ blanket::blanket( derive( Ref, Mut, Rc, Arc, Box ) ) ]
//
pub trait Timer
{
	/// Future returned by sleep(). On wasm isn't required to be `Send` for now.
	/// Mainly async-std's sleep future isn't `Send` on Wasm.
	//
	#[ cfg( target_arch = "wasm32") ]
	//
	type SleepFuture: Future<Output=()> + 'static;

	/// Future returned by sleep().
	//
	#[ cfg(not( target_arch = "wasm32" )) ]
	//
	type SleepFuture: Future<Output=()> + Send + 'static;


	/// Future that resolves after a given duration.
	//
	#[ must_use = "sleep() returns a future, which does nothing unless awaited" ]
	//
	fn sleep( &self, dur: Duration ) -> Self::SleepFuture;
}




// The following code was taken from tor-rtcompat https://gitlab.torproject.org/tpo/core/arti/-/blob/main/tor-rtcompat/src/timer.rs
// This is licenced: "MIT OR Apache-2.0".
//



/// An extension trait on [`Timer`] for timeouts and clock delays.
//
pub trait TimerExt: Timer
{
	/// Wrap a [`Future`] with a timeout.
	///
	/// The output of the new future will be the returned value of
	/// `future` if it completes within `duration`.  Otherwise, it
	/// will be `Err(TimeoutError)`.
	///
	/// # Limitations
	///
	/// This uses [`Timer::sleep`] for its timer, and is
	/// subject to the same limitations.
	//
	#[ must_use = "timeout() returns a future, which does nothing unless awaited." ]
	//
	fn timeout<F: Future>( &self, duration: Duration, future: F ) -> Timeout<F, Self::SleepFuture>
	{
		let sleep_future = self.sleep( duration );

		Timeout { future, sleep_future }
	}
}


impl<T: Timer> TimerExt for T {}


/// An error value given when a function times out.
///
/// This value is generated when the timeout from
/// [`TimerExt::timeout`] expires before the provided future
/// is ready.
//
#[ derive( Copy, Clone, Debug, Eq, PartialEq ) ]
//
#[allow(clippy::exhaustive_structs)]
//
pub struct TimeoutError;


impl std::error::Error for TimeoutError {}


impl std::fmt::Display for TimeoutError
{
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
	{
		write!( f, "Timeout expired" )
	}
}


impl From<TimeoutError> for std::io::Error
{
	fn from( err: TimeoutError ) -> std::io::Error
	{
		std::io::Error::new( std::io::ErrorKind::TimedOut, err )
	}
}


/// A timeout returned by [`TimerExt::timeout`].
//
#[pin_project]
//
#[ derive(Debug) ]
//
pub struct Timeout<T, S>
{
	/// The future we want to execute.
	//
	#[pin] future: T,

	/// The future implementing the timeout.
	//
	#[pin] sleep_future: S,
}



impl<T, S> Future for Timeout<T, S>

	where T: Future              ,
	      S: Future<Output = ()> ,

{
	type Output = Result< T::Output, TimeoutError >;


	fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
	{
		let this = self.project();


		if let Poll::Ready(x) = this.future.poll(cx)
		{
			return Poll::Ready(Ok(x));
		}


		match this.sleep_future.poll(cx)
		{
			Poll::Pending   => Poll::Pending                    ,
			Poll::Ready(()) => Poll::Ready( Err(TimeoutError) ) ,
		}
	}
}

