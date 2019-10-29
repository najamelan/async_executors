use
{
	criterion         :: { Criterion, criterion_group, criterion_main } ,
	futures           :: { future::{ ready }, executor::{ block_on }, task::{ Spawn, SpawnExt } } ,
	futures           :: { StreamExt, SinkExt, channel::mpsc::{ unbounded }, channel::oneshot::{ self, Sender } } ,
	async_executors   :: { * } ,
};





fn spawn_one( c: &mut Criterion )
{
	let mut group = c.benchmark_group( "Spawn one future and await a channel" );


	group.bench_function( "ThreadPool", |b|
	{
		b.iter( ||
		{
			let (mut tx, mut rx) = unbounded();
			let mut pool = ThreadPool::new().expect( "create threadpool" );

			pool.spawn( async move
			{
				assert!( tx.send( 3u8 ).await.is_ok() );

			}).expect( "spawn" );

			assert_eq!( 3u8, block_on( rx.next() ).expect( "receive" ) );

		});
	});
}





fn spawn_many( c: &mut Criterion )
{
	let mut group = c.benchmark_group( "Spawn many futures that wait on a channel" );

	for tasks in [1024, 2048, 4096].iter()
	{
		group.bench_with_input( format!( "ThreadPool spawn {}", &tasks ), tasks, |b, i|
		{
			b.iter( ||
			{
				let (mut tx, rx) = unbounded();
				let mut pool = ThreadPool::new().expect( "create threadpool" );

				for _ in 0..*i
				{
					let mut tx = tx.clone();

					pool.spawn( async move
					{
						assert!( tx.send( 3usize ).await.is_ok() );

					}).expect( "spawn" );
				}

				block_on( tx.close() ).expect( "close channel" );

				let sum = block_on( rx.fold( 0, |acc, x| ready( acc + x ) ) );
				assert_eq!( (3*i) as usize, sum );

			});
		});
	}
}




// fn spawn_nested( c: &mut Criterion )
// {
// 	let mut group = c.benchmark_group( "Spawn nested futures" );

// 	for depth in [100, 1000, 5000].iter()
// 	{
// 		group.bench_with_input( format!( "ThreadPool nested spawn {}", &depth ), depth, |b, d|
// 		{
// 			b.iter( ||
// 			{
// 				let mut pool = ThreadPool::new().expect( "create threadpool" );
// 				let (tx, rx) = oneshot::channel();

// 				let task = nest( 0, *d, tx, pool.handle() );

// 				pool.spawn( task ).expect( "spawn" );

// 				assert_eq!( *d, block_on( rx ).expect( "receive on channel" ) );

// 			});
// 		});
// 	}
// }


// /// Will keep spawning nested tasks until depth and then send the current depth back over a channel
// /// to be verified.
// //
// async fn nest( current: usize, depth: usize, tx: Sender<usize>, mut exec: impl Spawn + Clone + Send + 'static )
// {
// 	if current < depth
// 	{
// 		let task = nest( current+1, depth, tx, exec.clone() );

// 		exec.spawn( task ).expect( "spawn in nest" );
// 	}

// 	else
// 	{
// 		tx.send( current ).expect( "send in nest" );
// 	}
// }



criterion_group!( benches,

	spawn_one,
	spawn_many,
	// spawn_nested
);

criterion_main! (benches);
