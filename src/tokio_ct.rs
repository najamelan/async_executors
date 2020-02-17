//! Provides TokioCt executor specific functionality.
//
use
{
	crate          :: { import::*             } ,
	std            :: { rc::Rc, cell::RefCell } ,
};



/// An executor that uses a [tokio::runtime::Runtime] with the [basic scheduler](tokio::runtime::Builder::basic_scheduler).
///
/// ## Unwind Safety.
///
/// You must only spawn futures to this API that are unwind safe. Tokio will wrap it in
/// [std::panic::AssertUnwindSafe] and wrap the poll invocation with [std::panic::catch_unwind].
///
/// They reason that this is fine because they require `Send + 'static` on the future. As far
/// as I can tell this is wrong. Unwind safety can be circumvented in several ways even with
/// `Send + 'static` (eg. parking_lot::Mutex is Send + 'static but !UnwindSafe).
///
/// __As added footgun in the `LocalSpawn` impl for TokioCt we artificially add a Send
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
pub struct TokioCt
{
	pub(crate) exec  : Rc<RefCell< Runtime >> ,
	pub(crate) handle: TokioRtHandle          ,
}



impl TokioCt
{
	/// This is the entry point for this executor. You must call spawn on the handle from within a future that is run with block_on.
	//
	pub fn block_on< F: Future >( &mut self, f: F ) -> F::Output
	{
		self.exec.borrow_mut().block_on( f )
	}
}




impl TryFrom<&mut Builder> for TokioCt
{
	type Error = std::io::Error;

	fn try_from( builder: &mut Builder ) -> Result<Self, Self::Error>
	{
		let exec = builder.basic_scheduler().build()?;

		Ok( Self
		{
			 handle  : exec.handle().clone()         ,
			 exec    : Rc::new( RefCell::new(exec) ) ,
		})
	}
}


impl Spawn for TokioCt
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.handle.spawn( future );

		Ok(())
	}
}



impl LocalSpawn for TokioCt
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
		let _ = self.handle.spawn( fut );

		Ok(())
	}
}



#[ cfg(test) ]
//
mod tests
{
	use super::*;

	static_assertions::assert_not_impl_any!( TokioCt: Send, Sync );
}
