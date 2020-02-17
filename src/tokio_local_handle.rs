use
{
	crate          :: { import::*               } ,
	tokio::runtime :: { Handle as TokioRtHandle } ,
	std            :: { marker::PhantomData     } ,
};


/// A handle to this tokio Runtime with basic scheduler that can easily be cloned and that implements
/// Spawn and LocalSpawn traits.
///
/// Note that you have to call spawn on this from within a call to [TokioCt::block_on]
/// for futures to be polled. There is no `run` method like in [futures::executor::LocalPool].
/// This also means that when passing it to a library, the library shouldn't try to call a
/// `block_on` function itself, as nested `block_on`s will cause trouble. TODO: verify last statement.
///
/// ## Unwind Safety.
///
/// You must only spawn futures to this API that are unwind safe. Tokio will wrap it in
/// [std::panic::AssertUnwindSafe] and wrap the poll invocation with [std::panic::catch_unwind].
///
/// They reason that this is fine because they require `Send + 'static` on the future. As far
/// as I can tell this is wrong. Unwind safety can be circumvented in several ways even with
/// `Send + 'static`.
///
/// __As added footgun in the `LocalSpawn` impl for TokioLocalHandle we artificially add a Send
/// impl to your future so it can be spawned by tokio, which requires `Send` even for the
/// basic scheduler. This opens more ways to observe broken invariants, like `RefCell`, `TLS`, etc.__
///
/// You should make sure that if your future panics, no code that lives on after the spawned task has
/// unwound, nor any destructors called during the unwind can observe data in an inconsistent state.
///
/// See the relevant [catch_unwind RFC](https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md)
/// and it's discussion threads for more info as well as the documentation in stdlib.
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioLocalHandle
{
	pub(crate) spawner : TokioRtHandle,

	// This handle must not be Send. We want to be able to impl LocalSpawn for it, but tokio does not
	// provide us with the API to do so as their handle is Send and requires Send on the futures.
	//
	_no_send: PhantomData<*mut fn()> ,
}


impl TokioLocalHandle
{
	// Safety: Do not make this pub. The only way to obtain one should be from TokioCt in this
	// crate. Otherwise people could move a tokio handle to another thread and then turn it into
	// TokioLocalHandle.
	//
	pub(crate) fn new( spawner: TokioRtHandle ) -> Self
	{
		Self { spawner, _no_send: PhantomData::default() }
	}
}




impl LocalSpawn for TokioLocalHandle
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We transform the LocalFutureObj into a FutureObj. Just magic!
		//
		// This is safe because TokioHandle and TokioCt are not Send, so it can never venture to another thread than the
		// current_thread executor it's created from.
		//
		// This is necessary because tokio does not provide a handle that can spawn !Send futures.
		//
		let fut = unsafe { future.into_future_obj() };

		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.spawner.spawn( fut );

		Ok(())
	}
}


impl Spawn for TokioLocalHandle
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.spawner.spawn( future );

		Ok(())
	}
}


#[ cfg(test) ]
//
mod tests
{
	use super::*;

	static_assertions::assert_not_impl_any!( TokioLocalHandle: Send, Sync );
}

