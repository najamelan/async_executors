use
{
	std::{ time::Duration, future::Future },
};

/// Represents the fact that an executor has timer functionality.
//
pub trait Sleep
{
	/// Future returned by sleep().
	//
	type SleepFuture: Future<Output=()> + Send + 'static;

	/// Future that resolves after a given duration.
	//
	#[ must_use = "sleep() returns a future, which does nothing unless awaited" ]
	//
	fn sleep( &self, dur: Duration ) -> Self::SleepFuture;
}
