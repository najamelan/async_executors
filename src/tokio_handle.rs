use
{
	crate          :: { import::*               } ,
	tokio::runtime :: { Handle as TokioRtHandle } ,
};

/// A handle to a [TokioCt](crate::TokioCt) or [TokioTp](crate::TokioTp) executor. It implements `Spawn`, `SpawnHandle` and
/// `SpawnHandleOs` traits.
///
/// For [TokioTp](crate::TokioTp) this can be used to avoid a drop order problem for the tokio Runtime. See the
/// documentation for [TokioTp](crate::TokioTp) for an explanation.
///
/// For [TokioCt](crate::TokioCt) this can be used to send a future from another thread to run on the [TokioCt](crate::TokioCt).
///
/// The handle is only operational as long as the parent executor is alive. There is no compiler
/// assisted lifetime tracking for this as generally spawned futures you would like to give the
/// handle to need to be `'static`, so usability would be rather hampered, however you must make
/// sure you manage the lifetimes manually.
///
/// If the parent executor is already dropped when [spawn](futures_util::task::SpawnExt::spawn) is called, the future just
/// get's dropped silently without ever being polled.
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
/// See the relevant [catch_unwind RFC](https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md)
/// and it's discussion threads for more info as well as the documentation in stdlib.
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioHandle
{
	pub(crate) spawner: TokioRtHandle,
}


impl TokioHandle
{
	pub(crate) fn new( spawner: TokioRtHandle ) -> Self
	{
		Self { spawner }
	}
}


impl Spawn for TokioHandle
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.spawner.spawn( future );

		Ok(())
	}
}
