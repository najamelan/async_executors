//! Provides TokioTpBuilder which guarantees at type level that it is multi-threaded.
//
use
{
	crate          :: { TokioTp   } ,
	std            :: { sync::Arc } ,
	tokio::runtime :: { Builder   } ,
};


/// Builder to create a [`TokioTp`] executor. This guarantees that `TokioTp` always has a runtime that is multi-threaded,
/// as tokio does not make this information available on it's `Runtime` type.
///
/// Further allows you access to the tokio builder so you can set the other configuration options on it as you see fit.
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
	/// If you `mem::swap` it, your warranty is void.
	//
	pub fn tokio_builder( &mut self ) -> &mut Builder
	{
		&mut self.builder
	}


	/// Create the actual executor.
	///
	/// The error comes from tokio. From their docs, no idea why it is there or what could go wrong.
	/// Suppose spawning threads could fail...
	//
	pub fn build( &mut self ) -> Result<TokioTp, std::io::Error>
	{
		let exec = self.builder.build()?;

		Ok( TokioTp
		{
			exec: Some( Arc::new(exec) ),
		})
	}
}


impl Default for TokioTpBuilder
{
	fn default() -> Self
	{
		Self::new()
	}
}

