//! An example showing how to take an executor in an API that can use SpawnHandle
//! with multiple output types.
//!
//! You can also make your object generic over the executor. This shows how you can avoid that.
//
use
{
	std             :: { sync::Arc                             } ,
	async_executors :: { AsyncStd, SpawnHandle, SpawnHandleExt } ,
};


// We create a custom trait and tell the compiler that it can only ever be implemented
// when the receiver implements all of the SpawnHandle variants we need.
//
// See the trait_set example for a more ergonomic way of doing this.
//
pub trait CustomSpawnHandle : SpawnHandle<String> + SpawnHandle<u8> + Send + Sync {}

// A blanket impl for all types that fit our requirements. The compiler actually
// recognises this as a proof that any dyn CustomSpawnHandle will also implement
// SpawnHandle<String> and SpawnHandle<u8>, so we can call methods even from
// SpawnHandleExt on it.
//
impl<T> CustomSpawnHandle for T

	where T: SpawnHandle<String> + SpawnHandle<u8> + Send + Sync

{}


struct Connection
{
	// Since it's hard to get a clone on a trait object, we just wrap it
	// in an Arc, that way we can pass it to child tasks. Spawning does
	// not require mutable access.
	//
	exec: Arc< dyn CustomSpawnHandle > ,
}


impl Connection
{
	pub fn new( exec: Arc< dyn CustomSpawnHandle > ) -> Self
	{
		Self { exec }
	}

	// Use the API we wanted, hooray.
	//
	pub async fn run( &self )
	{
		let process_request = async { String::from( "Processing request" ) };
		let other_request   = async { 5                                    };

		let request_handle = self.exec.spawn_handle( process_request ).expect( "spawn process_request" );
		let other_handle   = self.exec.spawn_handle( other_request   ).expect( "spawn other_request"   );

		println!( "A string from process_request: {}", request_handle.await );
		println!( "A u8 from other_request: {}"      , other_handle  .await );
	}
}


fn main()
{
	let exec = Arc::new( AsyncStd::default() );
	let ex2  = exec.clone();

	// We can't just spawn conn.run(), because it takes &self, but a spawned
	// task needs to be 'static. Since conn now lives inside the async task,
	// we don't borrow a local variable from our main function. The compiler
	// can assert that conn lives at least as long as the task itself.
	//
	let task = async move
	{
		let conn = Connection::new( ex2 );

		conn.run().await;
	};


	let join_handle = exec.spawn_handle( task ).expect( "spawn task" );

	AsyncStd::block_on( join_handle );
}
