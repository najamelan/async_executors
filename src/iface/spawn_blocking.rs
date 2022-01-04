#[ allow(unused_imports) ]
//
use
{
	futures_util :: { future::{ FutureExt, abortable }, task::SpawnExt                    } ,
	futures_task :: { SpawnError, FutureObj                                               } ,
	crate        :: { BlockingHandle                                                      } ,
	std          :: { pin::Pin, future::Future, sync::{ Arc, atomic::AtomicBool }, rc::Rc } ,
	blanket      :: { blanket                                                             } ,
};


/// Indicate the executor can provide a threadpool for blocking operations.
/// This cannot be object safe because it works on a closure, which is un-nameble.
/// that means the method has to be generic. If you must store an executor with
/// this trait, you will have to make your struct generic.
///
/// There is an object safe method on the trait ([SpawnBlocking::spawn_blocking_void])
/// that only accepts a boxed closure without return value.
//
// Doesn't work with blanket.
// #[ blanket(derive( Mut, Box, Arc, Rc )) ]
//
pub trait SpawnBlocking
{
	/// Runs the provided closure on a thread where blocking is acceptable.
	//
	fn spawn_blocking<F, R>( &self, f: F ) -> BlockingHandle<R>

		where F   : FnOnce() -> R + Send + 'static ,
	         R   : Send + 'static                 ,
	         Self: Sized                          ,
	;

	/// Runs the provided closure on a thread where blocking is acceptable. This part of the trait is
	/// object safe but your closure must be boxed and you cannot have a return value.
	//
	fn spawn_blocking_void( &self, f: Box< dyn FnOnce() + Send > ) -> BlockingHandle<()>;
}


impl<T: SpawnBlocking> SpawnBlocking for &T
{
	fn spawn_blocking<F, R>( &self, f: F ) -> BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	         R: Send + 'static                 ,
	         T: Sized                          ,
	{
		(**self).spawn_blocking( f )
	}


	fn spawn_blocking_void( &self, f: Box< dyn FnOnce() + Send > ) -> BlockingHandle<()>
	{
		(**self).spawn_blocking( f )
	}
}


impl<T: SpawnBlocking> SpawnBlocking for &mut T
{
	fn spawn_blocking<F, R>( &self, f: F ) -> BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	         R: Send + 'static                 ,
	         T: Sized                          ,
	{
		(**self).spawn_blocking( f )
	}


	fn spawn_blocking_void( &self, f: Box< dyn FnOnce() + Send > ) -> BlockingHandle<()>
	{
		(**self).spawn_blocking( f )
	}
}


impl<T: SpawnBlocking> SpawnBlocking for Box<T>
{
	fn spawn_blocking<F, R>( &self, f: F ) -> BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	         R: Send + 'static                 ,
	         T: Sized                          ,
	{
		(**self).spawn_blocking( f )
	}


	fn spawn_blocking_void( &self, f: Box< dyn FnOnce() + Send > ) -> BlockingHandle<()>
	{
		(**self).spawn_blocking( f )
	}
}


impl<T: SpawnBlocking> SpawnBlocking for Arc<T>
{
	fn spawn_blocking<F, R>( &self, f: F ) -> BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	         R: Send + 'static                 ,
	         T: Sized                          ,
	{
		(**self).spawn_blocking( f )
	}


	fn spawn_blocking_void( &self, f: Box< dyn FnOnce() + Send > ) -> BlockingHandle<()>
	{
		(**self).spawn_blocking( f )
	}
}


impl<T: SpawnBlocking> SpawnBlocking for Rc<T>
{
	fn spawn_blocking<F, R>( &self, f: F ) -> BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	         R: Send + 'static                 ,
	         T: Sized                          ,
	{
		(**self).spawn_blocking( f )
	}


	fn spawn_blocking_void( &self, f: Box< dyn FnOnce() + Send > ) -> BlockingHandle<()>
	{
		(**self).spawn_blocking( f )
	}
}
