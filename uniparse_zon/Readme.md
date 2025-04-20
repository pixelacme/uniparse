# 🌀 uniparse_zon

[![CI](https://github.com/pixelacme/uniparse/actions/workflows/ci.yml/badge.svg)](https://github.com/pixelacme/uniparse/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/uniparse_zon.svg)](https://crates.io/crates/uniparse_zon)
[![Docs.rs](https://docs.rs/uniparse_zon/badge.svg)](https://docs.rs/uniparse_zon)
[![Coverage](https://codecov.io/gh/pixelacme/uniparse/branch/main/graph/badge.svg)](https://codecov.io/gh/pixelacme/uniparse)

A minimal, fast parser for `.zon` files (Zig package metadata) with full support for:

- ✅ Parsing `.zon` files into a typed AST
- 🧠 Structured access (`get`, `set`, `remove`)
- 🧩 Path-based traversal and mutation
- 🔁 Deserialize to native Rust structs (`serde`)
- 💾 Display + pretty formatting to Zig-style `.zon` format

---

## 🛠 Installation

```toml
[dependencies]
uniparser_zon = "1.0.0"
```

---

## 📦 Features

- No dependencies other than `serde` and `uniparse_core`
- Handles nested `.zon` objects and lists
- Gracefully parses booleans, strings, and structured lists
- Supports programmatic editing and saving

---

## 🔧 Usage

### Parse and Read

```rust
use uniparse_zon::model::ZonFile;

let input = std::fs::read_to_string("build.zig.zon").unwrap();
let zon = ZonFile::parse_str(&input).unwrap();

// Get a nested field
if let Some(url) = zon.get(&["dependencies", "zigimg", "url"]).and_then(|v| v.as_str()) {
    println!("zigimg URL = {}", url);
}
```

### Deserialize to Struct

```rust
#[derive(Debug, serde::Deserialize)]
struct RootZon {
    name: String,
    version: String,
    paths: Vec<String>,
    dependencies: std::collections::HashMap<String, Dependency>,
}

#[derive(Debug, serde::Deserialize)]
struct Dependency {
    url: String,
    hash: String,
    lazy: Option<bool>,
}

let root: RootZon = zon.as_struct().unwrap();
println!("Project: {} v{}", root.name, root.version);
```

### Mutate or insert values

```rust
use uniparse_zon::model::ZonValue;

zon.set(&["dependencies", "my_crate", "url"], ZonValue::String("https://example.com".into())).unwrap();
zon.set(&["dependencies", "my_crate", "lazy"], ZonValue::Bool(true)).unwrap();
```

### Remove a field

```rust
zon.remove(&["dependencies", "zigimg", "hash"]).unwrap();
```

### Serialize back to .zon format

```rust
println!("{}", zon.to_string_pretty());
```

### Full Example

```rust
fn main() {
    let input = std::fs::read_to_string("build.zig.zon").unwrap();
    let zon = ZonFile::parse_str(&input).unwrap();
    let data = zon.data.to_json();
    println!("{}", serde_json::to_string_pretty(&data).unwrap());
    let root: RootZon = zon.as_struct().expect("failed to map to root");

    // ✅ Access all dependencies
    println!("Dependencies:");
    for (name, dep) in &root.dependencies {
        println!("  {}", name);
        println!("    url:  {}", dep.url);
        println!("    hash: {}", dep.hash);
        println!("    lazy: {}", dep.lazy.unwrap_or(false));
    }

    let new_dep = ZonValue::Object(HashMap::from([
        (
            "url".to_string(),
            ZonValue::String("https://example.com".into()),
        ),
        ("hash".to_string(), ZonValue::String("abc123".into())),
        ("lazy".to_string(), ZonValue::Bool(true)),
    ]));

    zon.set(&["dependencies", "package_name"], new_dep).unwrap();

    zon.set(&["version"], ZonValue::String("9.9.9".into()))
        .unwrap();

    // Print updated
    println!("{}", zon.to_string_pretty());

    let url = zon.data.get_path(&["dependencies", "package_name", "url"]);

    if let Some(ZonValue::String(url)) = url {
        println!("package_url, {}", url)
    }

    let new_url = ZonValue::String("https://new.example.com".into());
    
    zon.data
        .set_path(&["dependencies", "package_name", "url"], new_url)
        .unwrap();

    let newUrl = zon
        .data
        .get_path(&["dependencies", "package_name", "url"])
        .and_then(|v| v.as_str())
        .unwrap_or("no Url");

    println!("new: {}", newUrl);
}
```

---

### Project Structure

```
src/
├── lib.rs        // Exports model + parser
├── model.rs      // AST definitions, getters/setters
├── parser.rs     // Tokenizer + recursive descent parser
```

---

## 📄 License

MIT OR Apache-2.0

---

## 🙌 Contributions

PRs welcome! This project is intended to be zero-dependency and lightweight.

---

## 🔮 Future Ideas

- Fomat-preserving write support
- Cli formatting, linting and editing
- Full comment rentention (AST annotation)


---

Let me know if you want a `Cargo.toml`, badges, or docs.rs setup too — happy to write that as well.
