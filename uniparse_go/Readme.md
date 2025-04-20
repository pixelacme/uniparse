# uniparse_go

[![CI](https://github.com/pixelacme/uniparse/actions/workflows/ci.yml/badge.svg)](https://github.com/pixelacme/uniparse/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/uniparse_go.svg)](https://crates.io/crates/uniparse_go)
[![Docs.rs](https://docs.rs/uniparse_go/badge.svg)](https://docs.rs/uniparse_go)
[![Coverage](https://codecov.io/gh/pixelacme/uniparse/branch/main/graph/badge.svg)](https://codecov.io/gh/pixelacme/uniparse)

> A lightweight, type-safe parser for Go `go.mod` files in Rust.

`uniparse-go` provides a fast, ergonomic, and fully-typed way to parse Go module manifests (`go.mod`). It supports both single-line and multi-line `require` blocks and validates the presence of critical fields like `module` and `go` version.

---

## ✨ Features

- 🔍 Parse `go.mod` files into structured Rust types
- ✅ Supports both single and multi-line `require` blocks
- 🧾 Helpful error handling with line numbers
- 📦 Designed for use in tools, analysis, or converters
- 🧪 Fully tested and ready for production use

---

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
uniparse_go = "0.1"
```

---

## Parse a string
```
use uniparse_go::parse_str;

let go = parse_str(r#"
    module example.com/project
    go 1.21
    require github.com/gin-gonic/gin v1.7.7
"#)?;

assert_eq!(go.module, "example.com/project");
assert_eq!(go.requires[0].name, "github.com/gin-gonic/gin");
```

### Parse a file
```
use uniparse_go::parse_file;

let go = parse_file("go.mod")?;
println!("Module: {}", go.module);
```

### 📦 Structs
```
pub struct go {
    pub module: String,
    pub go_version: String,
    pub requires: Vec<GoDependency>,
}

pub struct GoDependency {
    pub name: String,
    pub version: String,
}

```

---

## Not yet supported

- `replace` and `exclude` blocks
- Comments attached to dependencies
- Rewritting or formatiing `.mod` files

---

## 📚 Documentation

- API Reference (docs.rs)
- Crates.io

---

## 🔒 License

Licensed under:

MIT License (LICENSE-MIT)

---

## 🙌 Contributions

Feel free to open an issue or pull request! This crate is part of the uniparse project.