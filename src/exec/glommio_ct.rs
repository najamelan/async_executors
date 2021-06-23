use
{
	crate         :: { LocalSpawnHandle, SpawnHandle, JoinHandle, YieldNow     } ,
	std           :: { future::Future, rc::Rc                                  } ,
	futures_task  :: { FutureObj, LocalSpawn,  Spawn, SpawnError               } ,
	futures_util  :: { FutureExt, task::LocalSpawnExt, future::LocalFutureObj  } ,
	glommio_crate :: { LocalExecutor, LocalExecutorBuilder, GlommioError, Task } ,
};
use crate::{SpawnBlocking, BlockingHandle};
use std::sync::Arc;
use nix::sched::CpuSet;


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
/// Calling spawn from outside [`block_on`](GlommioCt::block_on) will panic.
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
	/// Create an executor. Note: in order to spawn you need to run [`block_on`](Self::block_on).
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
	fn sleep( &self, dur: std::time::Duration ) -> futures_core::future::BoxFuture<'static, ()>
	{
		futures_timer::Delay::new( dur ).boxed()
	}
}


impl YieldNow for GlommioCt
{
	/// Await this future in order to yield to the executor.
	//
	fn yield_now( &self ) -> crate::YieldNowFut
	{
		// only yield if any other tasks are waiting.
		//
		if glommio_crate::Local::need_preempt()
		{
			crate::YieldNowFut{ done: false }
		}

		else
		{
			// This will return Ready immediately.
			//
			crate::YieldNowFut{ done: true }
		}
	}
}

impl SpawnBlocking for GlommioCt {
	fn spawn_blocking<F, R>(&self, f: F) -> BlockingHandle<R>
        where F: FnOnce() -> R + Send + 'static,
                                        R: Send + 'static,
    {
		let alive_arc = Arc::new(());
		let alive = Arc::downgrade(&alive_arc);
		let handle = std::thread::spawn(move || {
			bind_to_cpu_set(to_cpu_set(None.into_iter())).unwrap();
			let r = f();
			drop(alive_arc);
			r
		});
        BlockingHandle::std_thread(handle, alive)
	}
}

macro_rules! to_io_error {
    ($error:expr) => {{
        match $error {
            Ok(x) => Ok(x),
            Err(nix::Error::Sys(_)) => Err(std::io::Error::last_os_error()),
            Err(nix::Error::InvalidUtf8) => {
                Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
            }
            Err(nix::Error::InvalidPath) => {
                Err(std::io::Error::from(std::io::ErrorKind::InvalidInput))
            }
            Err(nix::Error::UnsupportedOperation) => {
                Err(std::io::Error::from(std::io::ErrorKind::Other))
            }
        }
    }};
}
fn bind_to_cpu_set(cpuset: CpuSet) -> std::io::Result<()> {
	let pid = nix::unistd::Pid::this();
	to_io_error!(nix::sched::sched_setaffinity(pid, &cpuset))
}
fn to_cpu_set(cores: impl Iterator<Item = i32>) -> CpuSet {
	let mut set = CpuSet::new();
	let mut is_set = false;
	for i in cores {
		set.set(i as _).unwrap();
		is_set = true;
	}
	if !is_set {
		for i in 0..CpuSet::count() {
			set.set(i as _).unwrap();
		}
	}
	set
}

#[ cfg(test) ]
//
mod tests
{
	use super::*;

	// It's important that this is not Send, as we allow spawning !Send futures on it.
	//
	static_assertions::assert_not_impl_any!( GlommioCt: Send, Sync );
}
