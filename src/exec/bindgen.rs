use
{
	crate                :: { SpawnHandle, LocalSpawnHandle, JoinHandle, join_handle::InnerJh } ,
	wasm_bindgen_futures :: { spawn_local                                                     } ,
	futures_task         :: { FutureObj, LocalFutureObj, Spawn, LocalSpawn, SpawnError        } ,
	futures_util         :: { FutureExt                                                       } ,
};


/// A type that implements [`Spawn`], [`LocalSpawn`], [`SpawnHandle`](crate::SpawnHandle) and [`LocalSpawnHandle`](crate::LocalSpawnHandle).
/// Spawns on the _wasm-bingen-futures_ executor. The executor is global, eg. not self contained
/// and zero sized.
//
#[ derive( Copy, Clone, Default ) ]
//
#[ cfg_attr( nightly, doc(cfg(all( feature = "bindgen", target_arch = "wasm32" ))) ) ]
//
pub struct Bindgen;


impl Bindgen
{
	/// Create a new Bindgen wrapper, forwards to `Default::default`.
	///
	pub fn new() -> Self
	{
		Self::default()
	}
}



impl Spawn for Bindgen
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		spawn_local( future );

		Ok(())
	}
}



impl LocalSpawn for Bindgen
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), SpawnError>
	{
		spawn_local( future );

		Ok(())
	}
}



impl<Out: 'static + Send> SpawnHandle<Out> for Bindgen
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, handle) = future.remote_handle();
		spawn_local(fut);

		Ok( JoinHandle{ inner: InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



impl<Out: 'static> LocalSpawnHandle<Out> for Bindgen
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let (fut, handle) = future.remote_handle();
		spawn_local(fut);

		Ok( JoinHandle{ inner: InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



impl std::fmt::Debug for Bindgen
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "WASM Bindgen executor" )
	}
}
