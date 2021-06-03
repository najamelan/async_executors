use
{
	futures_util    :: { future::{ FutureExt }                                               } ,
	futures_task    :: { SpawnError, LocalFutureObj, FutureObj                               } ,
	crate           :: { JoinHandle, SpawnHandle, LocalSpawnHandle, Timer, TokioIo, YieldNow } ,
	tracing_futures :: { Instrument, Instrumented, WithDispatch                              } ,
};



impl<T, Out> SpawnHandle<Out> for Instrumented<T> where T: SpawnHandle<Out>, Out: 'static + Send
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let fut = future.instrument( self.span().clone() );

		self.inner().spawn_handle_obj( FutureObj::new(fut.boxed()) )
	}
}



impl<T, Out> SpawnHandle<Out> for WithDispatch<T> where T: SpawnHandle<Out>, Out: 'static + Send
{
	fn spawn_handle_obj( &self, future: FutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let fut = self.with_dispatch( future );

		self.inner().spawn_handle_obj( FutureObj::new(fut.boxed()) )
	}
}



impl<T, Out> LocalSpawnHandle<Out> for Instrumented<T> where T: LocalSpawnHandle<Out>, Out: 'static
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let fut = future.instrument( self.span().clone() );

		self.inner().spawn_handle_local_obj( LocalFutureObj::new(fut.boxed_local()) )
	}
}



impl<T, Out> LocalSpawnHandle<Out> for WithDispatch<T> where T: LocalSpawnHandle<Out>, Out: 'static
{
	fn spawn_handle_local_obj( &self, future: LocalFutureObj<'static, Out> ) -> Result<JoinHandle<Out>, SpawnError>
	{
		let fut = self.with_dispatch(future);

		self.inner().spawn_handle_local_obj( LocalFutureObj::new(fut.boxed_local()) )
	}
}



impl<T> Timer for Instrumented<T> where T: Timer
{
	type SleepFuture = Instrumented<T::SleepFuture>;

	fn sleep( &self, dur: std::time::Duration ) -> Self::SleepFuture
	{
		self.inner().sleep( dur ).instrument( self.span().clone() )
	}
}



impl<T> Timer for WithDispatch<T> where T: Timer
{
	type SleepFuture = WithDispatch<T::SleepFuture>;

	fn sleep( &self, dur: std::time::Duration ) -> Self::SleepFuture
	{
		self.with_dispatch( self.inner().sleep( dur ) )
	}
}


impl<T> TokioIo for Instrumented<T> where T: TokioIo {}
impl<T> TokioIo for WithDispatch<T> where T: TokioIo {}

impl<T> YieldNow for Instrumented<T> where T: YieldNow {}
impl<T> YieldNow for WithDispatch<T> where T: YieldNow {}
