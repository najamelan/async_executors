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

	// Don't do this at home, but in a very basic example like here, the process might exit before
	// the future runs. Use join handles, or channels to synchronize in real code if you need to wait
	// for some task to finish.
	//
 	std::thread::sleep( std::time::Duration::from_millis(10) );
}
