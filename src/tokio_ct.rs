use
{
	crate        :: { SpawnHandle, LocalSpawnHandle, JoinHandle, join_handle::InnerJh      } ,
	std          :: { sync::Arc, future::Future, sync::atomic::AtomicBool } ,
	tokio        :: { task::LocalSet, runtime::{  Runtime }            } ,
	futures_task :: { FutureObj, LocalFutureObj, Spawn, LocalSpawn, SpawnError                          } ,
	futures_util :: { future::abortable                                                                 } ,
};


/// An executor that uses a [`tokio::runtime::Runtime`] with the [basic scheduler](tokio::runtime::Builder::basic_scheduler)
/// and a [`tokio::task::LocalSet`]. Can spawn `!Send` futures.
///
/// You can obtain a wrapper to [`tokio::runtime::Handle`] through [`TokioCt::handle`]. That can be used to send a future
/// from another thread to run on the `TokioCt` executor.
///
/// ## Creation of the runtime
///
/// You create the wrapper through the [`TryFrom`] impl for [`tokio::runtime::Builder`]. This allows you to configure
/// the tokio runtime that will be used. Setting `threaded_scheduler` on it will be void and overwritten. `core_threads`
/// also makes no sense. You can choose any other configuration, like whether to have a reactor and a timer.
///
/// ```
/// // Make sure to set the `tokio_ct` feature on async_executors. The
/// // following example also requires the feature `spawn_handle`.
/// //
/// use
/// {
///    async_executors :: { TokioCt, TokioCtBuilder, LocalSpawnHandleExt } ,
///    tokio           :: { runtime::Builder             } ,
///    std             :: { rc::Rc     } ,
/// };
///
/// let exec = TokioCtBuilder::new().build().expect( "create tokio runtime" );
///
/// // block_on takes a &self, so if you need to `async move`,
/// // just clone it for use inside the async block.
/// //
/// exec.block_on( async
/// {
///    let not_send = async { let rc = Rc::new(()); };
///
///    // We can spawn !Send futures here.
///    //
///    let join_handle = exec.spawn_handle_local( not_send ).expect( "spawn" );
///
///    join_handle.await;
/// });
///```
///
/// ## Unwind Safety.
///
/// When a future spawned on this wrapper panics, the thread will unwind until the `block_on`, not above.
///
/// You must only spawn futures to this API that are unwind safe. Tokio will wrap the task running from `block_on` in
/// [`std::panic::AssertUnwindSafe`] and wrap the poll invocation with [`std::panic::catch_unwind`].
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
/// and it's discussion threads for more info as well as the documentation of [std::panic::UnwindSafe]
/// for more information.
///
//
#[ derive( Debug, Clone ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "tokio_ct" )) ) ]
//
pub struct TokioCt
{
	pub(crate) exec  : Arc< Runtime  > ,
	pub(crate) local : Arc< LocalSet > ,

	// We keep one handy, because users might pass this into a task they run with block_on, which
	// borrows the exec field. So we shouldn't need to borrow when handle is called, otherwise
	// the RefCell will panic.
	//
	// pub(crate) handle: TokioRtHandle,
}



impl TokioCt
{
	/// This is the entry point for this executor. Once this call returns, no remaining tasks shall be polled anymore.
	/// However the tasks stay in the executor, so if you make a second call to `block_on` with a new task, the older
	/// tasks will start making progress again.
	///
	/// For simplicity, it's advised to just create top level task that you run through `block_on` and make sure your
	/// program is done when it returns.
	///
	/// ## Panics
	///
	/// This function will panic if it is called from an async context, including but not limited to making a nested
	/// call.
	//
	pub fn block_on< F: Future >( &self, f: F ) -> F::Output
	{
		self.exec.block_on( self.local.run_until( f ) )
	}
}


impl Spawn for TokioCt
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.local.spawn_local( future );

		Ok(())
	}
}



impl LocalSpawn for TokioCt
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.local.spawn_local( future );

		Ok(())
	}
}



impl<Out: 'static + Send> SpawnHandle<Out> for TokioCt
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, a_handle) = abortable( future );

		Ok( JoinHandle{ inner: InnerJh::Tokio
		{
			handle  : self.exec.spawn( fut ) ,
			detached: AtomicBool::new( false ) ,
			a_handle                           ,
		}})
	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for TokioCt
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, a_handle) = abortable( future );

		Ok( JoinHandle{ inner: InnerJh::Tokio
		{
			handle  : self.local.spawn_local( fut ) ,
			detached: AtomicBool::new( false )      ,
			a_handle                                ,
		}})

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
