// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( feature = "external_doc", feature(external_doc) )]
#![cfg_attr( feature = "external_doc", doc(include = "../README.md"))]
//!


#![ doc    ( html_root_url = "https://docs.rs/async_executors" ) ]
#![ deny   ( missing_docs                                      ) ]
#![ allow  ( clippy::suspicious_else_formatting                ) ]

#![ warn
(
	missing_debug_implementations ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unused_extern_crates          ,
	unused_qualifications         ,
	single_use_lifetimes          ,
	unreachable_pub               ,
	variant_size_differences      ,
)]


#[ cfg( feature = "localpool" ) ] mod localpool;
#[ cfg( feature = "localpool" ) ] pub use localpool::*;

#[ cfg( feature = "tokio_ct"  ) ] mod tokio_ct;
#[ cfg( feature = "tokio_ct"  ) ] pub use tokio_ct::*;



// External dependencies
//
mod import
{
	// #[ cfg( test ) ]
	// //
	// pub(crate) use
	// {
	// 	pretty_assertions :: { assert_eq } ,
	// };


	#[ cfg(any( feature = "bindgen", feature = "localpool", feature = "juliex", feature = "tokio_ct" )) ]
	//
	pub(crate) use
	{
		futures :: { future::{ FutureObj } } ,
		futures :: { task::SpawnError as FutSpawnErr  } ,

	};


	#[ cfg(any( feature = "localpool", feature = "tokio_ct" )) ]
	//
	pub(crate) use
	{
		futures :: { future::LocalFutureObj } ,
	};


	#[ cfg( feature = "localpool" ) ]
	//
	pub(crate) use
	{
		std     :: { future::Future } ,

		futures :: { future::FutureExt, task::{ LocalSpawnExt, SpawnExt }  } ,
		futures :: { executor::{ LocalPool as FutLocalPool, LocalSpawner } } ,
	};


	#[ cfg( feature = "tokio_ct" ) ]
	//
	pub(crate) use
	{
		tokio_executor::
		{
			SpawnError    as TokioSpawnError ,

			current_thread::
			{
				CurrentThread as TokioCtExec     ,
				Handle        as TokioCtSpawner  ,
				RunError      as TokioRunError   ,
			},
		},


		std::marker::PhantomData,
	};
}


