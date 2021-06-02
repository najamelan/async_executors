//! Provides TokioTp executor specific functionality.
//
use
{
	crate          :: { SpawnHandle, JoinHandle, join_handle::InnerJh     } ,
	std            :: { sync::{ Arc, atomic::AtomicBool }, future::Future } ,
	futures_task   :: { FutureObj, Spawn, SpawnError                      } ,
	tokio::runtime :: { Runtime                                           } ,
};


/// An executor that uses [tokio::runtime::Runtime].
///
/// ## Example
///
/// The following example shows how to pass an executor to a library function.
///
/// ```rust
/// use
/// {
///    futures          :: { task::{ Spawn, SpawnExt } } ,
///    async_executors  :: { TokioTpBuilder            } ,
///    tokio::runtime   :: { Builder                   } ,
///    std::convert     :: { TryFrom                   } ,
///    futures::channel :: { oneshot, oneshot::Sender  } ,
/// };
///
///
/// fn lib_function( exec: impl Spawn, tx: Sender<&'static str> )
/// {
///    exec.spawn( async
///    {
///       tx.send( "I can spawn from a library" ).expect( "send string" );
///
///    }).expect( "spawn task" );
/// }
///
///
/// fn main()
/// {
///    // You must use the builder. This guarantees that TokioTp is always backed up by a threadpool.
///    // You can set other configurations by calling `tokio_builder()` on TokioTpBuilder, so you get
///    // access to the `tokio::runtime::Builder`.
///    //
///    let exec = TokioTpBuilder::new().build().expect( "create tokio threadpool" );
///
///    let program = async
///    {
///       let (tx, rx) = oneshot::channel();
///
///       lib_function( &exec, tx );
///       assert_eq!( "I can spawn from a library", rx.await.expect( "receive on channel" ) );
///    };
///
///    exec.block_on( program );
/// }
/// ```
///
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
/// If a future is run with `block_on` as opposed to `spawn`, the panic will not be caught and the
/// thread calling `block_on` will be unwound.
///
/// Note that unwind safety is related to logic errors, not related to the memory safety issues that cannot happen
/// in safe rust (memory safety, undefined behavior, unsoundness, data races, ...). See the relevant
/// [catch_unwind RFC](https://github.com/rust-lang/rfcs/blob/master/text/1236-stabilize-catch-panic.md)
/// and it's discussion threads for more info as well as the documentation of [std::panic::UnwindSafe].
//
#[ derive( Debug, Clone ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "tokio_tp" )) ) ]
//
pub struct TokioTp
{
	pub(crate) exec: Option< Arc<Runtime> >,
}



impl TokioTp
{
	/// Forwards to [Runtime::block_on].
	//
	pub fn block_on< F: Future >( &self, f: F ) -> F::Output
	{
		self.exec.as_ref().unwrap().block_on( f )
	}


	/// See: [tokio::runtime::Runtime::shutdown_timeout]
	///
	///  This tries to unwrap the Arc<Runtime> we hold, so that works only if no other clones are around. If this is not the
	///  only reference, self will be returned to you as an error. It means you cannot shutdown the runtime because there are
	///  other clones of the executor still alive.
	//
	pub fn shutdown_timeout( mut self, duration: std::time::Duration ) -> Result<(), Self>
	{
		let arc = self.exec.take().unwrap();

		let rt  = match Arc::try_unwrap( arc )
		{
			Ok(rt) => rt,
			Err(arc) =>
			{
				self.exec = Some(arc);
				return Err(self);
			}
		};

		rt.shutdown_timeout( duration );

		Ok(())
	}


	/// See: [tokio::runtime::Runtime::shutdown_background]
	///
	///  This tries to unwrap the Arc<Runtime> we hold, so that works only if no other clones are around. If this is not the
	///  only reference, self will be returned to you as an error. It means you cannot shutdown the runtime because there are
	///  other clones of the executor still alive.
	//
	pub fn shutdown_background( mut self ) -> Result<(), Self>
	{
		let arc = self.exec.take().unwrap();

		let rt  = match Arc::try_unwrap( arc )
		{
			Ok(rt) => rt,
			Err(arc) =>
			{
				self.exec = Some(arc);
				return Err(self);
			}
		};

		rt.shutdown_background();

		Ok(())
	}
}


#[ cfg( feature = "tokio_io" ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "tokio_io" )) ) ]
//
impl crate::TokioIo for TokioTp {}


impl Spawn for TokioTp
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		let _ = self.exec.as_ref().unwrap().spawn( future );

		Ok(())
	}
}



impl<Out: 'static + Send> SpawnHandle<Out> for TokioTp
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		Ok( JoinHandle{ inner: InnerJh::Tokio
		{
			handle  : self.exec.as_ref().unwrap().spawn( future ) ,
			detached: AtomicBool::new( false  ) ,
		}})
	}
}



#[ cfg(all( feature = "timer", not(feature="tokio_timer" )) ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "timer", feature = "tokio_tp" )) ) ]
//
impl crate::Timer for TokioTp
{
	type SleepFuture = futures_timer::Delay;

	fn sleep( &self, dur: std::time::Duration ) -> Self::SleepFuture
	{
		futures_timer::Delay::new( dur )
	}
}



#[ cfg( feature = "tokio_timer" ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "tokio_timer", feature = "tokio_tp" ))) ) ]
//
impl crate::Timer for TokioTp
{
	type SleepFuture = tokio::time::Sleep;

	fn sleep( &self, dur: std::time::Duration ) -> Self::SleepFuture
	{
		tokio::time::sleep( dur )
	}
}
