use
{
	crate :: { import::*, TokioHandle } ,
	std   :: { rc::Rc, cell::RefCell  } ,
	tokio :: { task::LocalSet         } ,
};


/// An executor that uses a [tokio::runtime::Runtime] with the [basic scheduler](tokio::runtime::Builder::basic_scheduler).
/// Can spawn `!Send` futures.
///
/// You must make sure that calls to `spawn` and `spawn_local` happen in async context, withing a task running
/// on [TokioCt::block_on].
///
/// You can obtain a wrapper to `tokio::runtime::handle` through [TokioCt::handle]. That can be used to send a future
/// from another thread to run on the `TokioCt` executor.
///
/// ## Unwind Safety.
///
/// When a future spawned on this wrapper panics, the thread will unwind until the block_on, not above.
///
/// You must only spawn futures to this API that are unwind safe. Tokio will wrap the task running from block_on in
/// [std::panic::AssertUnwindSafe] and wrap the poll invocation with [std::panic::catch_unwind].
///
/// They reason that this is fine because they require `Send + 'static` on the task. As far
/// as I can tell this is wrong. Unwind safety can be circumvented in several ways even with
/// `Send + 'static` (eg. `parking_lot::Mutex` is `Send + 'static` but `!UnwindSafe`).
///
/// You should make sure that if your future panics, no code that lives on after the top level task has
/// unwound, nor any destructors called during the unwind can observe data in an inconsistent state.
///
/// Note that these are logic errors, not related to the class of problems that cannot happen
/// in safe rust (memory safety, undefined behavior, unsoundness, data races, ...). See the relevant
/// [catch_unwind RFC](https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md)
/// and it's discussion threads for more info as well as the documentation in [std::panic::UnwindSafe]
/// for more information.
//
#[ derive( Debug, Clone ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "tokio_ct" )) ) ]
//
pub struct TokioCt
{
	pub(crate) exec  : Rc<RefCell< Runtime  >> ,
	pub(crate) local : Rc<         LocalSet  > ,

	// We keep one handy, because users might pass this into a task they run with block_on, which
	// borrows the exec field. So we shouldn't need to borrow when handle is called, otherwise
	// the refcell will panic.
	//
	pub(crate) handle: TokioRtHandle,
}



impl TokioCt
{
	/// This is the entry point for this executor. You must call spawn from within a future that is running through `block_on`.
	/// Once this call returns, no remaining tasks shall be polled anymore. However the tasks stay in the executor,
	/// so if you make a second call to `block_on` with a new task, the older tasks will start making progress again.
	///
	/// For simplicity, it's advised to just create top level task that you run through `block_on` and make sure your
	/// program is done when it returns.
	//
	pub fn block_on< F: Future >( &mut self, f: F ) -> F::Output
	{
		self.exec.borrow_mut().block_on( self.local.run_until( f ) )
	}

	/// Obtain a handle to this executor that can easily be cloned and that implements the
	/// Spawn trait.
	///
	/// Note that this handle is `Send` and can be sent to another thread to spawn tasks on the
	/// current executor, but as such, tasks are required to be `Send`.
	//
	pub fn handle( &self ) -> TokioHandle
	{
		TokioHandle::new( self.handle.clone() )
	}
}



impl TryFrom<&mut Builder> for TokioCt
{
	type Error = std::io::Error;

	fn try_from( builder: &mut Builder ) -> Result<Self, Self::Error>
	{
		let exec  = builder.basic_scheduler().build()?;
		let local = LocalSet::new();

		Ok( Self
		{
			 handle   : exec.handle().clone()          ,
			 exec     : Rc::new( RefCell::new(exec ) ) ,
			 local    : Rc::new( local               ) ,
		})
	}
}



impl Spawn for TokioCt
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.local.spawn_local( future );

		Ok(())
	}
}



impl LocalSpawn for TokioCt
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.local.spawn_local( future );

		Ok(())
	}
}



#[ cfg(test) ]
//
mod tests
{
	use super::*;

	// It's important that this is not Send, as we allow spawning !Send futures on it.
	//
	static_assertions::assert_not_impl_any!( TokioCt: Send, Sync );
}
