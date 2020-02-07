#![ cfg( feature = "tokio_tp" ) ]

// Tested:
//
// ✔ pass a &mut TokioTp to a function that takes exec: `&mut impl Spawn`
// ✔ pass a      TokioTp to a function that takes exec: `impl Spawn + Clone`
//
mod common;

use
{
	common          :: * ,
	async_executors :: * ,
	futures         :: { channel::{ mpsc, oneshot }, executor::block_on, StreamExt } ,
	tokio::runtime  :: { Builder                                                   } ,
	std             :: { convert::TryFrom                                          } ,
};


// pass a &mut TokioTp to a function that takes exec: `&mut impl Spawn`
//
#[ test ]
//
fn test_spawn()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	increment( 4, &exec, tx );

	let result = block_on( rx.next() ).expect( "Some" );

		assert_eq!( 5u8, result );
}


// pass a &mut TokioTp to a function that takes exec: `impl Spawn + Clone`
//
#[ test ]
//
fn test_spawn_with_clone()
{
	let (tx, mut rx) = mpsc::channel( 1 );
	let exec         = TokioTp::try_from( &mut Builder::new() ).expect( "create tokio threadpool" );

	increment_by_value( 4, &exec, tx );

	let result = block_on( rx.next() );

		assert_eq!( Some( 5u8 ), result );
}


// pass a builder with some config set.
//
#[ test ]
//
fn test_build_name_thread()
{
	let (tx, rx) = oneshot::channel();

	let exec = TokioTp::try_from( Builder::new().thread_name( "test_thread" ) ).expect( "create tokio threadpool" );

	let task = async move
	{
		let name = std::thread::current().name().expect( "some name" ).to_string();
		tx.send( name ).expect( "send on oneshot" );
	};

	exec.spawn( task ).expect( "spawn" );

	block_on( async
	{
		assert_eq!( rx.await.expect( "read channel" ), "test_thread" );

	});
}

