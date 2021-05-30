use
{
	async_executors  :: { GlommioCt                } ,
	futures::channel :: { oneshot, oneshot::Sender } ,
	futures::task    :: { Spawn, SpawnExt          } ,
	glommio_crate    :: { LocalExecutorBuilder     } ,
};



fn lib_function( exec: impl Spawn, tx: Sender<&'static str> )
{
	exec.spawn( async
	{
		tx.send( "I can spawn from a library" ).expect( "send string" );

	}).expect( "spawn task" );
}



fn main()
{
	// You provide the builder, and async_executors will set the right scheduler.
	// Of course you can set other configuration on the builder before.
	//
	let builder = LocalExecutorBuilder::new();
	let exec    = GlommioCt::new( builder ).expect( "create exec" );

	let program = async
	{
		let (tx, rx) = oneshot::channel();

		lib_function( &exec, tx );

		println!( "{}", rx.await.expect( "receive on channel" ) );
	};

	exec.block_on( program );
}
