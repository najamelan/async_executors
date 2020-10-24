//! Provides TokioTpBuilder which guarantees at type level that it is multi-threaded.
//
use
{
	crate          :: { TokioTp        } ,
	std            :: { sync::Arc } ,
	tokio::runtime :: { Builder                           } ,
};


/// Builder
//
#[ derive(Debug) ]
//
pub struct TokioTpBuilder
{
	builder: Builder,
}


impl TokioTpBuilder
{
	/// Constructor.
	//
	pub fn new() -> Self
	{
		Self
		{
			builder: Builder::new_multi_thread(),
		}
	}

	/// Returns the builder from tokio so you can configure it, see: [Builder].
	//
	pub fn tokio_builder( &mut self ) -> &mut Builder
	{
		&mut self.builder
	}


	/// Create the actual executor.
	//
	pub fn build( &mut self ) -> Result<TokioTp, std::io::Error>
	{
		let exec = self.builder.build()?;

		Ok( TokioTp
		{
			exec  : Arc::new( exec ),
		})
	}
}

