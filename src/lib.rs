#![ cfg_attr( nightly, feature(doc_cfg) ) ]
#![ doc = include_str!("../README.md") ]

#![ doc   ( html_root_url = "https://docs.rs/async_executors" ) ]
#![ deny  ( missing_docs                                      ) ]
#![ forbid( unsafe_code                                       ) ]
#![ allow ( clippy::suspicious_else_formatting                ) ]

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


/// The executor implementations.
//
pub mod exec;

/// The traits exposed by this crate.
//
pub mod iface;

#[ cfg( any
(
	feature = "tokio_ct",
	feature = "tokio_tp",
	feature = "async_global",
	feature = "async_std",
	feature = "glommio",
	feature = "bindgen"
)) ]
pub use exec::*;
pub use iface::*;

// Re-export for convenience.
//
#[ cfg( feature = "localpool"  ) ] pub use futures_executor::LocalPool;
#[ cfg( feature = "localpool"  ) ] pub use futures_executor::LocalSpawner;
#[ cfg( feature = "threadpool" ) ] pub use futures_executor::ThreadPool;



