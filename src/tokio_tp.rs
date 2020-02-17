//! Provides TokioTp executor specific functionality.
//
use
{
	crate       ::{ import::* } ,
	parking_lot :: Mutex,
	std         ::{ sync::Arc } ,
};


/// An executor that uses [tokio_executor::thread_pool::ThreadPool]
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
/// You should make sure that if your future panics, no code that lives on after the spawned task has
/// unwound, nor any destructors called during the unwind can observe data in an inconsistent state.
///
/// See the relevant [catch_unwind RFC](https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md)
/// and it's discussion threads for more info as well as the documentation in stdlib.
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioTp
{
	pub(crate) exec  : Arc< Mutex<Runtime> >,
	pub(crate) handle: TokioRtHandle        ,
}



impl TokioTp
{
	/// This is the entry point for this executor. You must call spawn on the handle from within a future that is run with block_on.
	//
	pub fn block_on< F: Future >( &mut self, f: F ) -> F::Output
	{
		self.exec.lock().block_on( f )
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
