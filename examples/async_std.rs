use
{
	futures::task   :: { Spawn, SpawnExt } ,
	async_executors :: { AsyncStd        } ,
};


fn lib_function( exec: impl Spawn )
{
	exec.spawn( async
	{
		println!( "I can spawn from a library" );

	}).expect( "spawn task" );
}


fn main()
{
	let exec = AsyncStd::default();

	lib_function( exec );
}
