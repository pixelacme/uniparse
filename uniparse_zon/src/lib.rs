//! ```rust
//! use uniparse_zon::{ZonFile, RootZon};
//!
//! let input = r#".{ .name = "example", .version = "1.0.0", .paths = .{ "src" }, .dependencies = .{} }"#;
//! let zon = ZonFile::parse_str(input).unwrap();
//! let structured: RootZon = zon.as_struct().unwrap();
//! assert_eq!(structured.name, "example");
//! ```

mod model;
mod parser;

pub use model::{Dependency, RootZon, ZonFile, ZonValue};
pub use parser::parse_zon;
