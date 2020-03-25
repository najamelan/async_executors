//! Provides TokioTp executor specific functionality.
//
use
{
	crate       :: { import::*, TokioHandle } ,
	parking_lot :: { Mutex                  } ,
	std         :: { sync::Arc              } ,
};


/// An executor that uses [tokio::runtime::Runtime].
///
/// ## Drop order.
///
/// TokioTp bundles an `Arc<Mutex<tokio::runtime::Runtime>>` with a `tokio::runtime::handle`.
/// Doing so has some nice properties. The type behaves similarly to other wrapped executors in
/// this crate. It implements all the spawn traits directly and is self contained. That means
/// you can pass it to an API and holding the type means it's valid. If we give out just a
/// `tokio::runtime::handle`, it can only be used to spawn tasks as long as the `Runtime` is
/// alive.
///
/// However, a new problem arises. `Runtime` should never be dropped from async context. Since we
/// use a reference counted `Runtime`, the last one actually invokes drop, and if that last one is
/// in async context, it panics the thread. If you pass a clone into some async task and that tasks
/// is not properly synchronized, it might outlive the code in non-async context that spawned it.
/// Now drop happens in async context and boom.
///
/// To solve this you can either make sure all tasks are properly synchronized (eg. await `JoinHandle`s
/// so no tasks containing an executor outlive the parent), or hand out [TokioHandle] which can be
/// obtained from [`TokioTp::handle`] and which implements all required traits to spawn.
///
/// ## Unwind Safety.
///
/// You must only spawn futures to this API that are unwind safe. Tokio will wrap it in
/// [std::panic::AssertUnwindSafe] and wrap the poll invocation with [std::panic::catch_unwind].
///
/// They reason that this is fine because they require `Send + 'static` on the future. As far
/// as I can tell this is wrong. Unwind safety can be circumvented in several ways even with
/// `Send + 'static` (eg. `parking_lot::Mutex` is `Send + 'static` but `!UnwindSafe`).
///
/// You should make sure that if your future panics, no code that lives on after the spawned task has
/// unwound, nor any destructors called during the unwind can observe data in an inconsistent state.
///
/// Note that these are logic errors, not related to the class of problems that cannot happen
/// in safe rust (memory safety, undefined behavior, unsoundness, data races, ...). See the relevant
/// [catch_unwind RFC](https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md)
/// and it's discussion threads for more info as well as the documentation in [std::panic::UnwindSafe].
//
#[ derive( Debug, Clone ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "tokio_tp" )) ) ]
//
pub struct TokioTp
{
	pub(crate) exec  : Arc< Mutex<Runtime> >,
	pub(crate) handle: TokioRtHandle        ,
}



impl TokioTp
{
	/// Wrapper around [Runtime::block_on].
	//
	pub fn block_on< F: Future >( &mut self, f: F ) -> F::Output
	{
		self.exec.lock().block_on( f )
	}

	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn trait.
	///
	/// This handle only works as long as the parent executor is still alive.
	//
	pub fn handle( &self ) -> TokioHandle
	{
		TokioHandle::new( self.handle.clone() )
	}
}



impl TryFrom<&mut Builder> for TokioTp
{
	type Error = std::io::Error;

	fn try_from( builder: &mut Builder ) -> Result<Self, Self::Error>
	{
		let exec = builder.threaded_scheduler().build()?;

		Ok( Self
		{
			handle: exec.handle().clone()       ,
			exec  : Arc::new( Mutex::new(exec) ),
		})
	}
}


impl Spawn for TokioTp
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.handle.spawn( future );

		Ok(())
	}
}
