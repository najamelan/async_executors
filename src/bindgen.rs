use
{
	crate                :: { import::*   } ,
	wasm_bindgen_futures :: { spawn_local } ,
};


/// A type that implements `Spawn` and `LocalSpawn` and spawns on the wasm-bingen-futures executor.
/// Can spawn `!Send` futures.
//
#[ derive( Copy, Clone, Default ) ]
//
pub struct Bindgen;


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
