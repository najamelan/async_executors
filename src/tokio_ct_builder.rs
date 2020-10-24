//! Provides TokioCtBuilder which guarantees at type level that it is multi-threaded.
//
use
{
	crate        :: { TokioCt      } ,
	std          :: { sync::Arc } ,
	tokio        :: { task::LocalSet, runtime::Builder            } ,
};


/// Builder
//
#[ derive(Debug) ]
//
pub struct TokioCtBuilder
{
	builder: Builder,
}



impl TokioCtBuilder
{
	/// Constructor.
	//
	pub fn new() -> Self
	{
		Self
		{
			builder: Builder::new_current_thread(),
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
	pub fn build( &mut self ) -> Result<TokioCt, std::io::Error>
	{
		let exec = self.builder.build()?;

		Ok( TokioCt
		{
			exec : Arc::new( exec ) ,
			local: Arc::new( LocalSet::new()    ) ,
		})
	}
}

