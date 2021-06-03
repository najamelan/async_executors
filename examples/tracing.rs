use
{
	async_executors :: { AsyncStd, SpawnHandle, SpawnHandleExt } ,
	tracing_futures :: { Instrument                            } ,
	tracing_crate   as tracing                                   ,
};


async fn lib_function( exec: impl SpawnHandle<()> )
{
	exec.spawn_handle( async
	{
		tracing::info!( "I can spawn from a library" );

	}).expect( "spawn task" ).await;
}



fn main()
{
	tracing_subscriber::fmt::Subscriber::builder()

	   .with_max_level( tracing::Level::INFO )
	   .init()
	;

	let exec = AsyncStd.instrument( tracing::info_span!( "tracing-example" ) );

	AsyncStd::block_on( lib_function(exec) );

	tracing::info!( "end of main, not instrumented." );
}
