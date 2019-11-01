use crate::{ import::*, JoinHandle };

/// Indicates that this executor can spawn futures returning a JoinHandle
/// that allows recovering the output of the task.
///
/// Works for futures that are `!Send`.
///
/// This allows spawning tasks that do not have `()` as output as well.
/// If you drop the JoinHandle the task will be cancelled.
//
pub trait LocalSpawnHandle: LocalSpawn
{
	/// Spawn a task, getting a join handle that can be awaited to recover the output of the task.
	//
	fn spawn_handle_local<T: 'static>( &mut self, fut: impl Future< Output=T > + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >;
}
