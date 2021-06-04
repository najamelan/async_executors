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
//
#[ blanket(derive( Ref, Mut, Box, Arc, Rc )) ]
//
pub trait SpawnBlocking
{
	/// Runs the provided closure on a thread where blocking is acceptable.
	//
	fn spawn_blocking<F, R>( &self, f: F ) -> BlockingHandle<R>

		where F: FnOnce() -> R + Send + 'static ,
	         R: Send + 'static                 ,
	;
}

