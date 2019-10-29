use
{
	crate::import::*,
	std::{ fmt, future::Future, task::{ Context, Poll }, pin::Pin } ,
};

/// This is a modified FutureObj from the futures-rs lib. We just use it here to
/// be able to add a Send bound. See TokioCtHandle as to why.
//
pub(super) struct CastFutureObj<T>(LocalFutureObj<'static, T>);

       impl<T> Unpin for CastFutureObj<T> {}
unsafe impl<T> Send  for CastFutureObj<T> {}

impl<T> CastFutureObj<T>
{
	/// Create a `FutureObj` from a custom trait object representation.
	//
	pub(super) unsafe fn new_no_send_bound(f: LocalFutureObj<'static, T> ) -> CastFutureObj<T>
	{
		Self(f)
	}
}

impl<T> fmt::Debug for CastFutureObj<T>
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		f.debug_struct( "CastFutureObj" ).finish()
	}
}

impl<T> Future for CastFutureObj<T>
{
	type Output = T;

	fn poll( mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T>
	{
		LocalFutureObj::poll( Pin::new( &mut self.0 ), cx )
	}
}
