use
{
	futures::task    :: { LocalSpawn, LocalSpawnExt } ,
	async_executors  :: { TokioCt                   } ,
	tokio::runtime   :: { Builder                   } ,
	std              :: { convert::TryFrom, rc::Rc  } ,
	futures::channel :: { oneshot, oneshot::Sender  } ,
};


fn lib_function( exec: impl LocalSpawn, tx: Sender<String> )
{
	exec.spawn_local( async
	{
		let not_send = Rc::new(5);

		tx.send( format!( "I can spawn a !Send future from a library with tokio runtime: {}", &not_send ) ).expect( "send string" );

	}).expect( "spawn task" );
}


fn main()
{
	// You provide the builder, and async_executors will set the right scheduler.
	// Of course you can set other configuration on the builder before.
	//
	let mut exec = TokioCt::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );
	let spawner = exec.local_handle();

	let program = async
	{
		let (tx, rx) = oneshot::channel();

		lib_function( &spawner, tx );
		println!( "{}", rx.await.expect( "receive on channel" ) );
	};

	exec.block_on( program );
}
