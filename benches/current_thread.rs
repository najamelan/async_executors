use
{
	criterion         :: { Criterion, criterion_group, criterion_main } ,
	futures           :: { future::{ ready }, executor::{ block_on }, task::{ LocalSpawn, LocalSpawnExt, SpawnExt } } ,
	futures           :: { StreamExt, SinkExt, channel::mpsc::{ unbounded }, channel::oneshot::{ self, Sender } } ,
	async_executors   :: { * } ,
};





fn spawn_one( c: &mut Criterion )
{
	let mut group = c.benchmark_group( "Spawn one future and await a channel" );


	group.bench_function( "LocalPool", |b|
	{
		b.iter( ||
		{
			let (mut tx, mut rx) = unbounded();
			let mut pool = LocalPool::new();

			pool.spawn( async move
			{
				assert!( tx.send( 3u8 ).await.is_ok() );

			}).expect( "spawn" );

			pool.run();

			assert_eq!( 3u8, block_on( rx.next() ).expect( "receive" ) );

		});
	});


	group.bench_function( "TokioCt", |b|
	{
		b.iter( ||
		{
			let (mut tx, mut rx) = unbounded();
			let mut pool = TokioCt::new();

			pool.spawn( async move
			{
				assert!( tx.send( 3u8 ).await.is_ok() );

			}).expect( "spawn" );

			pool.run().expect( "run tokio_ct" );

			assert_eq!( 3u8, block_on( rx.next() ).expect( "receive" ) );

		});
	});
}





fn spawn_many( c: &mut Criterion )
{
	let mut group = c.benchmark_group( "Spawn many futures that wait on a channel" );

	for tasks in [1024, 2048, 4096].iter()
	{
		group.bench_with_input( format!( "LocalPool spawn {}", &tasks ), tasks, |b, i|
		{
			b.iter( ||
			{
				let (mut tx, rx) = unbounded();
				let mut pool = LocalPool::new();

				for _ in 0..*i
				{
					let mut tx = tx.clone();

					pool.spawn( async move
					{
						assert!( tx.send( 3usize ).await.is_ok() );

					}).expect( "spawn" );
				}

				pool.run();

				block_on( tx.close() ).expect( "close channel" );

				let sum = block_on( rx.fold( 0, |acc, x| ready( acc + x ) ) );
				assert_eq!( (3*i) as usize, sum );

			});
		});


		group.bench_with_input( format!( "TokioCt spawn {}", &tasks ), tasks, |b, i|
		{
			b.iter( ||
			{
				let (mut tx, rx) = unbounded();
				let mut pool = TokioCt::new();

				for _ in 0..*i
				{
					let mut tx = tx.clone();

					pool.spawn( async move
					{
						assert!( tx.send( 3usize ).await.is_ok() );

					}).expect( "spawn" );
				}

				pool.run().expect( "run tokio_ct" );

				block_on( tx.close() ).expect( "close channel" );

				let sum = block_on( rx.fold( 0, |acc, x| ready( acc + x ) ) );
				assert_eq!( (3*i) as usize, sum );

			});
		});
	}
}




fn spawn_nested( c: &mut Criterion )
{
	let mut group = c.benchmark_group( "Spawn nested futures" );

	for depth in [100, 1000, 5000].iter()
	{
		group.bench_with_input( format!( "LocalPool nested spawn {}", &depth ), depth, |b, d|
		{
			b.iter( ||
			{
				let mut pool = LocalPool::new();
				let (tx, rx) = oneshot::channel();

				let task = nest( 0, *d, tx, pool.handle() );

				pool.spawn_local( task ).expect( "spawn" );
				pool.run();

				assert_eq!( *d, block_on( rx ).expect( "receive on channel" ) );

			});
		});


		group.bench_with_input( format!( "TokioCt spawn {}", &depth ), depth, |b, d|
		{
			b.iter( ||
			{
				let mut pool = TokioCt::new();
				let (tx, rx) = oneshot::channel();

				let task = nest( 0, *d, tx, pool.handle() );

				pool.spawn_local( task ).expect( "spawn"        );
				pool.run        (      ).expect( "run tokio_ct" );

				assert_eq!( *d, block_on( rx ).expect( "receive on channel" ) );

			});
		});
	}
}


/// Will keep spawning nested tasks until depth and then send the current depth back over a channel
/// to be verified.
//
async fn nest( current: usize, depth: usize, tx: Sender<usize>, mut exec: impl LocalSpawn + Clone + 'static )
{
	if current < depth
	{
		let task = nest( current+1, depth, tx, exec.clone() );

		exec.spawn_local( task ).expect( "spawn in nest" );
	}

	else
	{
		tx.send( current ).expect( "send in nest" );
	}
}



criterion_group!( benches,

	spawn_one,
	spawn_many,
	spawn_nested
);

criterion_main! (benches);
