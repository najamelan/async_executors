mod ring_impl;

use
{
	ring_impl         :: Ring ,
	criterion         :: { Criterion, criterion_group, criterion_main } ,
	async_executors   :: { * } ,
	futures::task     :: { SpawnExt } ,
};



fn ring( c: &mut Criterion )
{
	let mut group = c.benchmark_group( "Ring benchmark" );

	for nodes in [3, 10, 100].iter()
	{
		group.bench_function( format!( "LocalPool spawn {}", &nodes ), |b|
		{
			b.iter( ||
			{
				let mut pool = LocalPool::new();
				let mut ring = Ring::new( *nodes );

				for node in ring.start_parallel()
				{
					pool.spawn( node ).expect( "spawn" );
				}

				pool.run();
			});
		});


		group.bench_function( format!( "TokioCt spawn {}", &nodes ), |b|
		{
			b.iter( ||
			{
				let mut pool = TokioCt::new();
				let mut ring = Ring::new( *nodes );

				for node in ring.start_parallel()
				{
					pool.spawn( node ).expect( "spawn" );
				}

				pool.run().expect( "run tokio_ct" );
			});
		});
	}
}

criterion_group!(benches, ring);
criterion_main! (benches);
