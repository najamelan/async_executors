use
{
	crate         :: { LocalSpawnHandle, SpawnHandle, JoinHandle, YieldNow     } ,
	std           :: { future::Future, rc::Rc                                  } ,
	futures_task  :: { FutureObj, LocalSpawn,  Spawn, SpawnError               } ,
	futures_util  :: { FutureExt, task::LocalSpawnExt, future::LocalFutureObj  } ,
	glommio_crate :: { LocalExecutor, LocalExecutorBuilder, GlommioError, Task } ,
};


/// Single threaded [glommio](https://docs.rs/glommio) executor. This executor works
/// on Linux 5.8+ only.
///
/// You will probably need to augment your rlimit_memlock. See glommio documentation
/// for details.
///
/// This always has io_uring enabled and will pull in quite some dependencies including
/// liburing from C.
///
/// # Panics
///
/// Calling spawn from outside [block_on] will panic.
//
#[ derive(Debug, Clone) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "glommio" )) ) ]
//
pub struct GlommioCt
{
	exec: Rc<LocalExecutor>,
}


impl GlommioCt
{
	/// Create an executor. Note: in order to spawn you need to run [block_on].
	//
	pub fn new( builder: LocalExecutorBuilder ) -> Result< Self, GlommioError<()> >
	{
		let exec = Rc::new( builder.make()? );

		Ok( Self{ exec } )
	}



	/// Runs the executor until the given future completes.
	/// This is the entry point of the executor. Calls to spawn will only work from the
	/// context of the future provided here.
	//
	pub fn block_on<F: Future>( &self, future: F ) -> F::Output
	{
		self.exec.run( future )
	}
}



impl LocalSpawn for GlommioCt
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		Task::local( future ).detach();
		Ok(())
	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for GlommioCt
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> )

		-> Result<JoinHandle<Out>, SpawnError>
	{
		let (remote, handle) = future.remote_handle();

		Task::local( remote ).detach();

		Ok( JoinHandle::remote_handle(handle) )
	}
}



impl Spawn for GlommioCt
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		self.spawn_local( future )
	}
}



impl<Out: Send + 'static> SpawnHandle<Out> for GlommioCt
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (remote, handle) = future.remote_handle();

		Task::local( remote ).detach();

		Ok( JoinHandle::remote_handle(handle) )
	}
}



#[ cfg( feature = "timer" ) ]
//
impl crate::Timer for GlommioCt
{
	type SleepFuture = futures_timer::Delay;

	fn sleep( &self, dur: std::time::Duration ) -> Self::SleepFuture
	{
		futures_timer::Delay::new( dur )
	}
}


impl YieldNow for GlommioCt {}



#[ cfg(test) ]
//
mod tests
{
	use super::*;

	// It's important that this is not Send, as we allow spawning !Send futures on it.
	//
	static_assertions::assert_not_impl_any!( GlommioCt: Send, Sync );
}
