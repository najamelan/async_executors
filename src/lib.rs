// These are only available on nightly
//
#![cfg_attr( feature = "docs", feature( doc_cfg, external_doc )) ]

// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( feature = "docs", doc(include = "../README.md") )]
//!

#![ doc  ( html_root_url = "https://docs.rs/async_executors" ) ]
#![ deny ( missing_docs                                      ) ]
#![ allow( clippy::suspicious_else_formatting                ) ]

#![ warn
(
	anonymous_parameters          ,
	missing_copy_implementations  ,
	missing_debug_implementations ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	single_use_lifetimes          ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unreachable_pub               ,
	unused_extern_crates          ,
	unused_qualifications         ,
	variant_size_differences      ,
)]



#[ cfg( feature = "tokio_ct" ) ] mod tokio_ct;
#[ cfg( feature = "tokio_ct" ) ] pub use tokio_ct::*;

#[ cfg( feature = "tokio_tp" ) ] mod tokio_tp;
#[ cfg( feature = "tokio_tp" ) ] pub use tokio_tp::*;

#[ cfg( feature = "async_std") ] mod async_std;
#[ cfg( feature = "async_std") ] pub use async_std::*;

#[ cfg( feature = "bindgen"  ) ] mod bindgen;
#[ cfg( feature = "bindgen"  ) ] pub use bindgen::*;

#[ cfg( feature = "spawn_handle" ) ] mod spawn_handle          ;
#[ cfg( feature = "spawn_handle" ) ] mod spawn_handle_os       ;
#[ cfg( feature = "spawn_handle" ) ] mod local_spawn_handle    ;
#[ cfg( feature = "spawn_handle" ) ] mod local_spawn_handle_os ;
#[ cfg( feature = "spawn_handle" ) ] mod join_handle           ;
#[ cfg( feature = "spawn_handle" ) ] mod remote_handle         ;

#[ cfg( feature = "spawn_handle" ) ] pub use spawn_handle          ::*;
#[ cfg( feature = "spawn_handle" ) ] pub use spawn_handle_os       ::*;
#[ cfg( feature = "spawn_handle" ) ] pub use local_spawn_handle    ::*;
#[ cfg( feature = "spawn_handle" ) ] pub use local_spawn_handle_os ::*;
#[ cfg( feature = "spawn_handle" ) ] pub use join_handle           ::*;



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
	#[ cfg(any( feature = "bindgen", feature = "tokio_ct", feature = "tokio_tp", feature = "async_std" )) ]
	//
	pub(crate) use
	{
		futures_task :: { FutureObj, Spawn, SpawnError as FutSpawnErr } ,
	};


	#[ cfg(any( feature = "tokio_ct", feature = "bindgen" )) ]
	//
	pub(crate) use
	{
		futures_task :: { LocalFutureObj, LocalSpawn } ,
	};


	#[ cfg(any( feature = "tokio_ct", feature = "tokio_tp" )) ]
	//
	pub(crate) use
	{
		std :: { convert::TryFrom, future::Future } ,
		tokio::{ runtime::{ Builder, Runtime, Handle as TokioRtHandle } },

	};


	#[ cfg( feature = "spawn_handle" ) ]
	//
	pub(crate) use
	{
		std :: { task::{ Poll, Context }, pin::Pin } ,
	};


	#[ cfg( feature = "tracing" ) ]
	//
	pub(crate) use
	{
		tracing_futures :: { Instrument, WithDispatch, Instrumented } ,
	};
}


