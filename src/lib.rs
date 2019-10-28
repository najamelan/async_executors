// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( feature = "external_doc", feature(external_doc) )]
#![cfg_attr( feature = "external_doc", doc(include = "../README.md"))]
//!


#![ doc    ( html_root_url = "https://docs.rs/async_executors" ) ]
#![ deny   ( missing_docs                                     ) ]
#![ forbid ( unsafe_code                                      ) ]
#![ allow  ( clippy::suspicious_else_formatting               ) ]

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



// External dependencies
//
mod import
{
	pub(crate) use
	{
		std :: {  } ,
	};


	#[ cfg( test ) ]
	//
	pub(crate) use
	{
		pretty_assertions :: { assert_eq } ,
	};
}


