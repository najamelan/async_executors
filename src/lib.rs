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


#[ cfg( feature = "localpool"  ) ] mod localpool;
#[ cfg( feature = "localpool"  ) ] pub use localpool::*;

#[ cfg( feature = "threadpool" ) ] mod threadpool;
#[ cfg( feature = "threadpool" ) ] pub use threadpool::*;

#[ cfg( feature = "juliex"     ) ] mod juliex;
#[ cfg( feature = "juliex"     ) ] pub use juliex::*;

#[ cfg( feature = "tokio_ct"   ) ] mod tokio_ct;
#[ cfg( feature = "tokio_ct"   ) ] pub use tokio_ct::*;

#[ cfg( feature = "tokio_tp"   ) ] mod tokio_tp;
#[ cfg( feature = "tokio_tp"   ) ] pub use tokio_tp::*;

#[ cfg( feature = "async_std"  ) ] mod async_std;
#[ cfg( feature = "async_std"  ) ] pub use async_std::*;

#[ cfg( feature = "bindgen"    ) ] mod bindgen;
#[ cfg( feature = "bindgen"    ) ] pub use bindgen::*;



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


	#[ cfg(any( feature = "bindgen", feature = "threadpool", feature = "localpool", feature = "juliex", feature = "tokio_ct", feature = "tokio_tp", feature = "async_std" )) ]
	//
	pub(crate) use
	{
		futures :: { future::{ FutureObj }           } ,
		futures :: { task::SpawnError as FutSpawnErr } ,

	};


	#[ cfg(any( feature = "threadpool", feature = "localpool" )) ]
	//
	pub(crate) use
	{
		futures :: { future::FutureExt, task::SpawnExt } ,
		std     :: { future::Future    } ,
	};


	#[ cfg(any( feature = "localpool", feature = "tokio_ct", feature = "bindgen" )) ]
	//
	pub(crate) use
	{
		futures :: { future::LocalFutureObj } ,
	};
}


