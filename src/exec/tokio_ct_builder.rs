//! Provides TokioCtBuilder which guarantees at type level that it is single-threaded.
//
use
{
	crate :: { TokioCt                          } ,
	std   :: { rc::Rc                           } ,
	tokio :: { task::LocalSet, runtime::Builder } ,
};


/// Builder to create a [`TokioCt`] executor. This guarantees that `TokioCt` always has a runtime that is single-threaded,
/// as tokio does not make this information available on it's `Runtime` type.
///
/// Further allows you access to the tokio builder so you can set the other configuration options on it as you see fit.
//
#[ derive(Debug) ]
//
#[ cfg_attr( nightly, doc(cfg( feature = "tokio_ct" )) ) ]
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
	/// If you `mem::swap` it, your warranty is void.
	//
	pub fn tokio_builder( &mut self ) -> &mut Builder
	{
		&mut self.builder
	}


	/// Create the actual executor.
	///
	/// The error comes from tokio. From their docs, no idea why it is there or what could go wrong.
	//
	pub fn build( &mut self ) -> Result<TokioCt, std::io::Error>
	{
		#[ cfg( feature = "tokio_io" ) ]
		//
		self.builder.enable_io();

		#[ cfg( feature = "tokio_timer" ) ]
		//
		self.builder.enable_time();

		let exec = self.builder.build()?;

		Ok( TokioCt
		{
			exec : Rc::new( exec            ) ,
			local: Rc::new( LocalSet::new() ) ,
		})
	}
}


impl Default for TokioCtBuilder
{
	fn default() -> Self
	{
		Self::new()
	}
}

