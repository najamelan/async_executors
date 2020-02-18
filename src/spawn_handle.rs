#[ allow(unused_imports) ]
//
use
{
	crate        :: { import::*, JoinHandle, remote_handle::remote_handle              } ,
	futures_util :: { task::{ SpawnExt, SpawnError }, future::{ FutureExt, abortable } } ,
	std          :: { future::Future, sync::{ Arc, atomic::AtomicBool }, rc::Rc        } ,
};


/// Let's you spawn and get a [JoinHandle] to await the output of a future.
///
/// This trait is not object safe, see [SpawnHandleOs](crate::SpawnHandleOs) for a best effort object safe one.
///
/// ## Performance
///
/// For [tokio] and [async-std](async_std_crate) this is generally faster than [SpawnExt::spawn], since
/// it's better aligned with the underlying API and doesn't require extra boxing.
//
#[ cfg_attr( feature = "docs", doc(cfg( feature = "spawn_handle" )) ) ]
//
pub trait SpawnHandle
{
	/// Spawn a future and return a [JoinHandle] that can be awaited for the output of the future.
	//
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send
	;
}


impl<T: SpawnHandle> SpawnHandle for Box<T>
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}


impl<T: SpawnHandle> SpawnHandle for Arc<T>
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}


impl<T: SpawnHandle> SpawnHandle for Rc<T>
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}


impl<T: SpawnHandle> SpawnHandle for &T
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}


impl<T: SpawnHandle> SpawnHandle for &mut T
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		(**self).spawn_handle( future )
	}
}


#[ cfg( feature = "tracing" ) ]
//
impl<T: SpawnHandle> SpawnHandle for Instrumented<T>
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let fut = future.instrument( self.span().clone() );

		self.inner().spawn_handle( fut )
	}
}


#[ cfg( feature = "tracing" ) ]
//
impl<T: SpawnHandle> SpawnHandle for WithDispatch<T>
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let fut = self.with_dispatch( future );

		self.inner().spawn_handle( fut )
	}
}




#[ cfg( feature = "async_std" ) ]
//
impl SpawnHandle for crate::async_std::AsyncStd
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, a_handle) = abortable( future );

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::AsyncStd
		{
			handle  : async_std_crate::task::spawn(fut) ,
			detached: AtomicBool::new(false)            ,
			a_handle                                    ,
		}})
	}
}


#[ cfg( feature = "tokio_tp" ) ]
//
impl SpawnHandle for crate::TokioTp
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, a_handle) = abortable( future );

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::Tokio
		{
			handle  : self.handle.spawn(fut) ,
			detached: AtomicBool::new(false)  ,
			a_handle                          ,
		}})
	}
}



#[ cfg(any( feature = "tokio_ct" )) ]
//
impl SpawnHandle for crate::TokioCt
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, a_handle) = abortable( future );

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::Tokio
		{
			handle  : self.handle.spawn(fut) ,
			detached: AtomicBool::new(false)  ,
			a_handle                          ,
		}})
	}
}



#[ cfg( feature = "bindgen" ) ]
//
impl SpawnHandle for crate::Bindgen
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, handle) = remote_handle( future );
		wasm_bindgen_futures::spawn_local(fut);

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



#[ cfg( feature = "localpool" ) ]
//
impl SpawnHandle for futures_executor::LocalSpawner
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, handle) = remote_handle( future );

		self.spawn( fut )?;

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}



#[ cfg( feature = "threadpool" ) ]
//
impl SpawnHandle for futures_executor::ThreadPool
{
	fn spawn_handle<Fut, Out>( &self, future: Fut ) -> Result<JoinHandle<Out>, SpawnError>

		where Fut: Future<Output = Out> + 'static + Send,
		      Out: 'static + Send

	{
		let (fut, handle) = remote_handle( future );

		self.spawn( fut )?;

		Ok( JoinHandle{ inner: crate::join_handle::InnerJh::RemoteHandle( Some(handle) ) } )
	}
}


