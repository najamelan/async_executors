#![ cfg_attr( nightly, feature( external_doc, doc_cfg    ) ) ]
#![ cfg_attr( nightly, doc    ( include = "../README.md" ) ) ]
#![ doc = "" ] // empty doc line to handle missing doc warning when the feature is missing.

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


#[ cfg( feature = "tokio_ct"     ) ] mod tokio_ct;
#[ cfg( feature = "tokio_ct"     ) ] pub use tokio_ct::*;
#[ cfg( feature = "tokio_ct"     ) ] mod tokio_ct_builder;
#[ cfg( feature = "tokio_ct"     ) ] pub use tokio_ct_builder::*;

#[ cfg( feature = "tokio_tp"     ) ] mod tokio_tp;
#[ cfg( feature = "tokio_tp"     ) ] mod tokio_tp_builder;
#[ cfg( feature = "tokio_tp"     ) ] pub use tokio_tp::*;
#[ cfg( feature = "tokio_tp"     ) ] pub use tokio_tp_builder::*;

#[ cfg( feature = "async_global" ) ] mod async_global;
#[ cfg( feature = "async_global" ) ] pub use async_global::*;

#[ cfg( feature = "async_std"    ) ] mod async_std;
#[ cfg( feature = "async_std"    ) ] pub use async_std::*;

#[ cfg( feature = "glommio"      ) ] mod glommio_ct;
#[ cfg( feature = "glommio"      ) ] pub use glommio_ct::*;

#[ cfg( feature = "bindgen"      ) ] mod bindgen;
#[ cfg( feature = "bindgen"      ) ] pub use bindgen::*;

#[ cfg( feature = "tracing"      ) ] mod tracing;

// Re-export for convenience.
//
#[ cfg( feature = "localpool"  ) ] pub use futures_executor::LocalPool;
#[ cfg( feature = "localpool"  ) ] pub use futures_executor::LocalSpawner;
#[ cfg( feature = "threadpool" ) ] pub use futures_executor::ThreadPool;

mod spawn_handle       ;
mod local_spawn_handle ;
mod join_handle        ;

pub use spawn_handle       ::*;
pub use local_spawn_handle ::*;
pub use join_handle        ::*;

