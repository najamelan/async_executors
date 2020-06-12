use
{
	async_executors :: { AsyncStd, SpawnHandle, SpawnHandleExt } ,
	tracing_futures :: { Instrument            } ,
	tracing_crate as tracing,
	async_std_crate as async_std,
};


async fn lib_function( exec: impl SpawnHandle<()> )
{
	exec.spawn_handle( async
	{
		tracing::info!( "I can spawn from a library" );

	}).expect( "spawn task" ).await;
}


#[ async_std::main ]
//
async fn main()
{
	tracing_subscriber::fmt::Subscriber::builder()

	   .with_max_level( tracing::Level::INFO )
	   .init()
	;

	let exec = AsyncStd.instrument( tracing::info_span!( "tracing-example" ) );

	lib_function( exec ).await;

	tracing::info!( "end of main, not instrumented." );
}
