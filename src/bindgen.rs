use
{
	crate                :: { import::*   } ,
	wasm_bindgen_futures :: { spawn_local } ,
};


/// We currently only support a global Bindgen threadpool. In principle this is the only supported
/// executor that allows full control. We could expose an interface that allows users to control
/// the lifetime and scope of a Bindgen threadpool.
//
#[ derive( Clone, Default ) ]
//
pub struct Bindgen {}


impl Spawn for Bindgen
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		spawn_local( future );

		Ok(())
	}
}



impl LocalSpawn for Bindgen
{
	fn spawn_local_obj( &self, future: LocalFutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		spawn_local( future );

		Ok(())
	}
}


impl std::fmt::Debug for Bindgen
{
	fn fmt( &self, f: &mut std::fmt::Formatter<'_> ) -> std::fmt::Result
	{
		write!( f, "WASM Bindgen executor" )
	}
}
