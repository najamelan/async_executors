//! Provides TokioTp executor specific functionality.
//
use
{
	crate          :: { import::*                                 } ,
	tokio::runtime :: { Builder, Runtime, Handle as TokioRtHandle } ,
	std            :: { sync::Arc                                 } ,
};


/// An executor that uses [tokio_executor::thread_pool::ThreadPool]
//
#[ derive( Debug, Clone ) ]
//
pub struct TokioTp
{
	exec   : Arc<Runtime>  ,
	spawner: TokioRtHandle ,
}



impl TryFrom<&mut Builder> for TokioTp
{
	type Error = std::io::Error;

	fn try_from( builder: &mut Builder ) -> Result<Self, Self::Error>
	{
		let exec    = builder.threaded_scheduler().build()?;
		let spawner = exec.handle().clone();

		Ok( Self { exec: Arc::new( exec ), spawner } )
	}
}



impl Spawn for TokioTp
{
	fn spawn_obj( &self, future: FutureObj<'static, ()> ) -> Result<(), FutSpawnErr>
	{
		// Impl in tokio is actually infallible, so no point in converting the error type.
		//
		self.spawner.spawn( future );

		Ok(())
	}
}





