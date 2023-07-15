//! Provides TokioTp executor specific functionality.
//
use
{
	crate          :: { SpawnHandle, JoinHandle, BlockingHandle          } ,
	std            :: { fmt, sync::Arc, future::Future, convert::TryFrom } ,
	futures_task   :: { FutureObj, Spawn, SpawnError                     } ,
	tokio::runtime :: { Runtime, RuntimeFlavor, Handle, Builder          } ,
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
///    async_executors  :: { TokioTp                   } ,
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
///    // This creates the runtime with defaults. It enables io and timers based on
///    // the features enabled on _async_executors_. You can also create `TokioTp` from
///    // a tokio `Runtime` or a `Handle`.
///    //
///    let exec = TokioTp::new().expect( "create tokio threadpool" );
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
	spawner: Spawner,
}


#[derive(Debug, Clone)]
enum Spawner
{
	Runtime( Arc<Runtime> ) ,
	Handle ( Handle       ) ,
}


/// Allows to create a [`TokioTp`] from a [`Runtime`].
///
/// # Errors
///
/// Will fail if you pass a multithreaded runtime. In that case it will return your [`Runtime`].
//
impl TryFrom<Runtime> for TokioTp
{
	type Error = Runtime;

	fn try_from( rt: Runtime ) -> Result<Self, Runtime>
	{
		match rt.handle().runtime_flavor()
		{
			RuntimeFlavor::MultiThread => Ok( Self
			{
				spawner: Spawner::Runtime( Arc::new(rt) ) ,
			}),

			_ => Err( rt ),
		}
	}
}


/// Allows to create a [`TokioTp`] from a [`Handle`].
///
/// # Errors
///
/// Will fail if you pass a multithreaded runtime. In that case it will return your [`Handle`].
//
impl TryFrom<Handle> for TokioTp
{
	type Error = Handle;

	fn try_from( handle: Handle ) -> Result<Self, Handle>
	{
		match handle.runtime_flavor()
		{
			RuntimeFlavor::MultiThread => Ok( Self
			{
				spawner: Spawner::Handle( handle ) ,
			}),

			_ => Err( handle ),
		}
	}
}



impl TokioTp
{
	/// Create a new [`TokioTp`]. Uses a default multithreaded [`Runtime`] setting timers and io depending
	/// on the features enabled on _async_executors_.
	//
	pub fn new() -> Result<Self, TokioTpErr>
	{
		let mut builder = Builder::new_multi_thread();


		#[ cfg( feature = "tokio_io" ) ]
		//
		builder.enable_io();

		#[ cfg( feature = "tokio_timer" ) ]
		//
		builder.enable_time();


		let rt = builder.build().map_err( |e| TokioTpErr::Builder(e.kind()) )?;

		Ok(Self
		{
			spawner: Spawner::Runtime(Arc::new( rt )),
		})
	}



	/// Try to construct a [TokioTp] from the currently entered [Runtime]. You can do this
	/// if you want to construct your runtime with the tokio macros eg:
	///
	/// ```
	/// #[tokio::main]
	/// async fn main()
	/// {
	///    // ...
	/// }
	/// ```
	///
	/// # Warning
	///
	/// `TokioTp::new()` is preferred over this. It's brief, doesn't require macros and is
	/// the intended behavior for this type. The whole library aims at a paradigm without
	/// global executors.
	///
	/// The main footgun here is that you are now already in async context, so you must not
	/// call [`TokioTp::block_on`]. `block_on` will panic when run from within an existing
	/// async context.
	///
	/// # Errors
	///
	/// Will fail if trying to construct from a current thread runtime or if no runtime
	/// is running.
	//
	pub fn try_current() -> Result< Self, TokioTpErr >
	{
		let handle = Handle::try_current()
			.map_err(|_| TokioTpErr::NoRuntime )?;

		Self::try_from( handle )
			.map_err(|_| TokioTpErr::WrongFlavour )
	}


	/// Forwards to [Runtime::block_on] or [Handle::block_on].
	///
	/// # Panics
	///
	/// If called when a runtime is already entered (eg. in async context), like when you created this
	/// executor with [`TokioTp::try_current`], this will panic.
	//
	pub fn block_on< F: Future >( &self, f: F ) -> F::Output
	{
		match &self.spawner
		{
			Spawner::Runtime( rt     ) => rt    .block_on( f ) ,
			Spawner::Handle ( handle ) => handle.block_on( f ) ,
		}
	}


	/// See: [tokio::runtime::Runtime::shutdown_timeout]
	///
	///  This tries to unwrap the Arc<Runtime> we hold, so that works only if no other clones are around. If this is not the
	///  only reference, self will be returned to you as an error. It means you cannot shutdown the runtime because there are
	///  other clones of the executor still alive.
	///
	///  # Errors
	///  - [`TokioTpErr::Cloned`]: if the the [`TokioTp`] has been cloned. You can only shut down the last one.
	///  - [`TokioTpErr::Handle`]: if the the [`TokioTp`] has been created from a handle. That is we don't own the [`Runtime`].
	//
	pub fn shutdown_timeout( self, duration: std::time::Duration ) -> Result<(), TokioTpErr>
	{
		let Self{ spawner } = self;

		let arc = match spawner
		{
			Spawner::Handle ( handle ) => return Err( TokioTpErr::Handle(Self{ spawner: Spawner::Handle(handle) }) ) ,
			Spawner::Runtime( arc    ) => arc,
		};


		let rt  = match Arc::try_unwrap(arc)
		{
			Ok(rt) => rt,
			Err(arc) =>
			{
				let this = Self{ spawner: Spawner::Runtime(arc) };
				return Err( TokioTpErr::Cloned(this) );
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
	///
	///  # Errors
	///  - [`TokioTpErr::Cloned`]: if the the [`TokioTp`] has been cloned. You can only shut down the last one.
	///  - [`TokioTpErr::Handle`]: if the the [`TokioTp`] has been created from a handle. That is we don't own the [`Runtime`].
	//
	pub fn shutdown_background( self ) -> Result<(), TokioTpErr>
	{
		let Self{ spawner } = self;

		let arc = match spawner
		{
			Spawner::Handle ( handle ) => return Err( TokioTpErr::Handle(Self{ spawner: Spawner::Handle(handle) }) ) ,
			Spawner::Runtime( arc    ) => arc,
		};


		let rt  = match Arc::try_unwrap(arc)
		{
			Ok(rt) => rt,
			Err(arc) =>
			{
				let this = Self{ spawner: Spawner::Runtime(arc) };
				return Err( TokioTpErr::Cloned(this) );
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
	/// Never fails.
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the JoinHandle, so the task becomes detached.
		//
		match &self.spawner
		{
			Spawner::Runtime( rt     ) => drop( rt    .spawn(future) ) ,
			Spawner::Handle ( handle ) => drop( handle.spawn(future) ) ,
		}

		Ok(())
	}
}



impl<Out: 'static + Send> SpawnHandle<Out> for TokioTp
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let handle = match &self.spawner
		{
			Spawner::Runtime( rt     ) => rt    .spawn(future) ,
			Spawner::Handle ( handle ) => handle.spawn(future) ,
		};

		Ok( JoinHandle::tokio(handle) )
	}
}



impl crate::YieldNow for TokioTp {}



impl<R: Send + 'static> crate::SpawnBlocking<R> for TokioTp
{
	fn spawn_blocking<F>( &self, f: F ) -> BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	{
		let handle = match &self.spawner
		{
			Spawner::Runtime( rt     ) => rt    .spawn_blocking( f ) ,
			Spawner::Handle ( handle ) => handle.spawn_blocking( f ) ,
		};

		BlockingHandle::tokio( handle )
	}


	fn spawn_blocking_dyn( &self, f: Box< dyn FnOnce()->R + Send > ) -> BlockingHandle<R>
	{
		self.spawn_blocking( f )
	}
}




#[ cfg(all( feature = "timer", not(feature="tokio_timer" )) ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "timer", feature = "tokio_tp" ))) ) ]
//
impl crate::Timer for TokioTp
{
	fn sleep( &self, dur: std::time::Duration ) -> futures_core::future::BoxFuture<'static, ()>
	{
		Box::pin( futures_timer::Delay::new(dur) )
	}
}



#[ cfg( feature = "tokio_timer" ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "tokio_timer", feature = "tokio_tp" ))) ) ]
//
impl crate::Timer for TokioTp
{
	fn sleep( &self, dur: std::time::Duration ) -> futures_core::future::BoxFuture<'static, ()>
	{
		Box::pin( tokio::time::sleep(dur) )
	}
}


#[cfg( feature = "tokio_tp" )]
/// A few errors that can happen while using _tokio_ executors.
#[derive(Debug, Clone)]
pub enum TokioTpErr
{
	/// The [tokio::runtime::builder] returned an error when construting the [Runtime].
	Builder( std::io::ErrorKind ),

	/// There are other clones of the [Runtime], so we cannot shut it down.
	Cloned ( TokioTp ),

	/// This executor was constructed from the a [Handle], so cannot be shut down.
	Handle ( TokioTp ),

	/// Can't create from current runtime because no runtime currently entered.
	NoRuntime,

	/// Can't construct from a current thread runtime.
	WrongFlavour,
}


impl fmt::Display for TokioTpErr
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		use TokioTpErr::*;

		match self
		{
			Builder(source) =>
				write!( f, "tokio::runtime::Builder returned an error: {source}" ),
			Cloned(_) => write!( f, "The TokioTp executor was cloned. Only the last copy can shut it down." ),
			Handle(_) => write!( f, "The TokioTp was created from tokio::runtime::Handle. Only an owned executor (created from `Runtime`) can be shut down." ),
			NoRuntime => write!( f, "Call to tokio::Handle::try_current failed, generally because no entered runtime is active." ),
			WrongFlavour => write!( f, "Can't create TokioTp from a current thread `Runtime`." ),
		}
	}
}


impl std::error::Error for TokioTpErr {}
