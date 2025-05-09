//! A minimal parser for Go `go.mod` files, written in Rust.
//!
//! This crate provides tools to parse `go.mod` files into structured Rust data types.
//! It supports reading from strings and files, and validates essential fields such as
//! `module`, `go` version, and `require` entries.
//!
//! # Example
//!
//! ```rust
//! use uniparse_go::{GoDependency, GoMod, ParseError};
//!
//! let gomod = GoMod::parse_str(r#"
//!     module example.com/m
//!     go 1.20
//!     require github.com/foo/bar v1.2.3
//! "#).unwrap();
//!
//! assert_eq!(gomod.module, "example.com/m");
//! ```

mod model;

pub use model::{GoDependency, GoMod, ParseError};
