use crate::{ import::*, JoinHandle };

/// Indicates that this executor can spawn futures returning a JoinHandle
/// that allows recovering the output of the task.
///
/// This allows spawning tasks that do not have `()` as output as well.
/// If you drop the JoinHandle the task will be cancelled.
//
pub trait SpawnHandle: Spawn
{
	/// Spawn a task, getting a join handle that can be awaited to recover the output of the task.
	//
	fn spawn_handle<T: 'static + Send>( &mut self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< JoinHandle<T>, FutSpawnErr >;
}
