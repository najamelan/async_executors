use
{
	crate        :: { SpawnHandle, LocalSpawnHandle, JoinHandle, BlockingHandle            } ,
	std          :: { fmt, rc::Rc, future::Future, convert::TryFrom                        } ,
	tokio        :: { task::LocalSet, runtime::{ Builder, Runtime, Handle, RuntimeFlavor } } ,
	futures_task :: { FutureObj, LocalFutureObj, Spawn, LocalSpawn, SpawnError             } ,
};


#[derive(Debug, Clone)]
enum Spawner
{
	Runtime( Rc<Runtime> ) ,
	Handle ( Handle      ) ,
}


/// An executor that uses a [`tokio::runtime::Runtime`] with the [current thread](tokio::runtime::Builder::new_current_thread)
/// and a [`tokio::task::LocalSet`]. Can spawn `!Send` futures.
///
/// ## Creation of the runtime
///
/// ```
/// // Make sure to set the `tokio_ct` feature on async_executors.
/// //
/// use
/// {
///    async_executors :: { TokioCt, LocalSpawnHandleExt } ,
///    tokio           :: { runtime::Builder             } ,
///    std             :: { rc::Rc                       } ,
/// };
///
/// // If you need to configure tokio, you can use `tokio::runtimer::Builder`
/// // to create your [`Runtime`] and then create the `TokioCt` from it.
///
/// let exec = TokioCt::new().expect( "create tokio runtime" );
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
/// When a future spawned on this wrapper panics, the panic will be caught by tokio in the poll function.
///
/// You must only spawn futures to this API that are unwind safe. Tokio will wrap spawned tasks in
/// [`std::panic::AssertUnwindSafe`] and wrap the poll invocation with [`std::panic::catch_unwind`].
///
/// They reason that this is fine because they require `Send + 'static` on the task. As far
/// as I can tell this is wrong. Unwind safety can be circumvented in several ways even with
/// `Send + 'static` (eg. `parking_lot::Mutex` is `Send + 'static` but `!UnwindSafe`).
///
/// You should make sure that if your future panics, no code that lives on after the panic,
/// nor any destructors called during the unwind can observe data in an inconsistent state.
///
/// Note: the future running from within `block_on` as opposed to `spawn` does not exhibit this behavior and will panic
/// the current thread.
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
	spawner: Spawner,
	local: Rc< LocalSet > ,
}


/// Create a `TokioCt` from a `Runtime`.
///
/// # Errors
///
/// Will fail if you pass a multithreaded runtime. In that case it will return your [`Runtime`].
//
impl TryFrom<Runtime> for TokioCt
{
	type Error = Runtime;

	fn try_from( rt: Runtime ) -> Result<Self, Runtime>
	{
		match rt.handle().runtime_flavor()
		{
			RuntimeFlavor::CurrentThread => Ok( Self
			{
				spawner: Spawner::Runtime( Rc::new(rt) ) ,
				local: Rc::new( LocalSet::new() ) ,
			}),

			_ => Err( rt ),
		}
	}
}


/// Create a [`TokioCt`] from a [`Handle`].
///
/// # Errors
/// Will fail if you pass a handle to a multithreaded runtime. Will return your [`Handle`].
//
impl TryFrom<Handle> for TokioCt
{
	type Error = Handle;

	fn try_from( handle: Handle ) -> Result<Self, Handle>
	{
		match handle.runtime_flavor()
		{
			RuntimeFlavor::CurrentThread => Ok( Self
			{
				spawner: Spawner::Handle( handle ) ,
				local: Rc::new( LocalSet::new() ) ,
			}),

			_ => Err( handle ),
		}
	}
}



impl TokioCt
{
	/// Create a new `TokioCt`. Uses a default current thread [`Runtime`] setting timers and io depending
	/// on the features enabled on _async_executors_.
	//
	pub fn new() -> Result<Self, TokioCtErr>
	{
		let mut builder = Builder::new_current_thread();


		#[ cfg( feature = "tokio_io" ) ]
		//
		builder.enable_io();

		#[ cfg( feature = "tokio_timer" ) ]
		//
		builder.enable_time();


		let rt = builder.build().map_err( |e| TokioCtErr::Builder(e.kind()) )?;

		Ok(Self
		{
			spawner: Spawner::Runtime(Rc::new( rt )),
			local  : Rc::new( LocalSet::new() ) ,
		})
	}



	/// Try to construct a [TokioCt] from the currently entered [Runtime]. You can do this
	/// if you want to construct your runtime with the tokio macros eg:
	///
	/// ```
	/// #[tokio::main(flavor = "current_thread")]
	/// async fn main()
	/// {
	///    // ...
	/// }
	/// ```
	///
	/// # Warning
	///
	/// `TokioCt::new()` is preferred over this. It's brief, doesn't require macros and is
	/// the intended behavior for this type. The whole library aims at a paradigm without
	/// global executors.
	///
	/// The main footgun here is that you are now already in async context, so you must call
	/// [`TokioCt::run_until`] instead of [`TokioCt::block_on`]. `block_on` will panic when run from
	/// within an existing async context. This can be surprising for your upstream libraries to
	/// which you pass a [TokioCt] executor.
	///
	/// # Errors
	///
	/// Will fail if trying to construct from a multithreaded runtime or if no runtime
	/// is running.
	///
	///
	pub fn try_current() -> Result< Self, TokioCtErr >
	{
		let handle = Handle::try_current()
			.map_err(|_| TokioCtErr::NoRuntime )?;

		Self::try_from( handle )
			.map_err(|_| TokioCtErr::WrongFlavour )
	}



	/// This is the entry point for this executor. Once this call returns, no remaining tasks shall be polled anymore.
	/// However the tasks stay in the executor, so if you make a second call to `block_on` with a new task, the older
	/// tasks will start making progress again.
	///
	/// For simplicity, it's advised to just create top level task that you run through `block_on` and make sure your
	/// program is done when it returns.
	///
	/// See: [tokio::runtime::Runtime::block_on]
	///
	/// ## Panics
	///
	/// This function will panic if it is called from an async context, including but not limited to making a nested
	/// call. It will also panic if the provided future panics.
	///
	/// When you created this executor with [`TokioCt::try_current`], you should call `run_until` instead.
	//
	pub fn block_on<F: Future>( &self, f: F ) -> F::Output
	{
		match &self.spawner
		{
			Spawner::Runtime( rt     ) => rt    .block_on( self.local.run_until( f ) ) ,
			Spawner::Handle ( handle ) => handle.block_on( self.local.run_until( f ) ) ,
		}
	}



	/// Run the given future to completion. This is the entrypoint for execution of all the code spawned on this
	/// executor when you are already in an async context.
	/// Eg. when you have created this executor from an already running runtime with [`TokioCt::try_current`].
	/// This will run the [`tokio::task::LocalSet`] which makes spawning possible.
	///
	/// Similarly to [`TokioCt::block_on`], spawned tasks will no longer be polled once the given future
	/// has ended, but will stay in the executor and you can call this function again to have every task
	/// continue to make progress.
	//
	pub async fn run_until<F: Future>( &self, f: F ) -> F::Output
	{
		self.local.run_until( f ).await
	}
}



impl Spawn for TokioCt
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the tokio JoinHandle, so the task becomes detached.
		//
		drop( self.local.spawn_local(future) );

		Ok(())
	}
}



impl LocalSpawn for TokioCt
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		// We drop the tokio JoinHandle, so the task becomes detached.
		//
		drop( self.local.spawn_local(future) );

		Ok(())
	}
}



impl<Out: 'static + Send> SpawnHandle<Out> for TokioCt
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let handle = match &self.spawner
		{
			Spawner::Runtime( rt     ) => rt    .spawn( future ) ,
			Spawner::Handle ( handle ) => handle.spawn( future ) ,
		};

		Ok( JoinHandle::tokio(handle) )
	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for TokioCt
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let handle = self.local.spawn_local( future );

		Ok( JoinHandle::tokio(handle) )

	}
}



#[ cfg(all( feature = "timer", not(feature="tokio_timer" )) ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "timer", feature = "tokio_ct" ))) ) ]
//
impl crate::Timer for TokioCt
{
	fn sleep( &self, dur: std::time::Duration ) -> futures_core::future::BoxFuture<'static, ()>
	{
		Box::pin( futures_timer::Delay::new(dur) )
	}
}



#[ cfg( feature = "tokio_timer" ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "tokio_timer", feature = "tokio_ct" ))) ) ]
//
impl crate::Timer for TokioCt
{
	fn sleep( &self, dur: std::time::Duration ) -> futures_core::future::BoxFuture<'static, ()>
	{
		Box::pin( tokio::time::sleep(dur) )
	}
}



#[ cfg( feature = "tokio_io" ) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "tokio_io" )) ) ]
//
impl crate::TokioIo for TokioCt {}


impl crate::YieldNow for TokioCt {}



impl<R: Send + 'static> crate::SpawnBlocking<R> for TokioCt
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



#[cfg( feature = "tokio_ct" )]
/// A few errors that can happen while using _tokio_ executors.
#[derive(Debug, Clone)]
pub enum TokioCtErr
{
	/// The [tokio::runtime::builder] returned an error when construting the [Runtime].
	Builder( std::io::ErrorKind ),

	/// There are other clones of the [Runtime], so we cannot shut it down.
	Cloned( TokioCt ),

	/// This executor was constructed from the a [Handle], so cannot be shut down.
	Handle( TokioCt ),

	/// Can't create from current runtime because no runtime currently entered.
	NoRuntime,

	/// Can't construct from a multithreaded runtime.
	WrongFlavour,
}


impl fmt::Display for TokioCtErr
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		use TokioCtErr::*;

		match self
		{
			Builder(source) =>
				write!( f, "tokio::runtime::Builder returned an error: {source}" ),
			Cloned(_) => write!( f, "The TokioCt executor was cloned. Only the last copy can shut it down." ),
			Handle(_) => write!( f, "The TokioCt was created from tokio::runtime::Handle. Only an owned executor (created from `Runtime`) can be shut down." ),
			NoRuntime => write!( f, "Call to tokio::Handle::try_current failed, generally because no entered runtime is active." ),
			WrongFlavour => write!( f, "Can't create TokioCt from a multithreaded `Runtime`." ),
		}
	}
}


impl std::error::Error for TokioCtErr {}



#[ cfg(test) ]
//
mod tests
{
	use super::*;

	// It's important that this is not Send, as we allow spawning !Send futures on it.
	//
	static_assertions::assert_not_impl_any!( TokioCt: Send, Sync );
}
