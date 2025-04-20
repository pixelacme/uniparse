# uniparse_gradle

[![CI](https://github.com/pixelacme/uniparse/actions/workflows/ci.yml/badge.svg)](https://github.com/pixelacme/uniparse/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/uniparse_gradle.svg)](https://crates.io/crates/uniparse_gradle)
[![Docs.rs](https://docs.rs/uniparse_gradle/badge.svg)](https://docs.rs/uniparse_gradle)
[![Coverage](https://codecov.io/gh/pixelacme/uniparse/branch/main/graph/badge.svg)](https://codecov.io/gh/pixelacme/uniparse)

`uniparse-gradle` provides a fast, ergonomic, and fully-typed way to parse gradle files.

---

## ✨ Features

- 🔍 Parse `gradle` files into structured Rust types
- 🧾 Helpful error handling with line numbers
- 📦 Designed for use in tools, analysis, or converters
- 🧪 Fully tested and ready for production use

---

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
uniparse_gradle = "0.1"
```

---

## Parse a string
```
use uniparse_gradle::parse_str;

let mut parsed = DSLBlock::parse_str(input).unwrap();
println!("Original:\n{}", parsed.to_string_pretty());
```

### Parse a file
```
use uniparse_gradle::parse_file;

let gradle = parse_file("go.mod")?;
println!("Module: {}", gradle.module);
```

### Full Example

```
pub mod model;
pub mod parser;
use std::collections::HashMap;

use model::{DSLBlock, DSLValue};
use uniparse_core::ParsedFile;

fn main() {
    let input = r#"
    plugins {
        id 'groovy'
        id 'application'
        id 'com.github.johnrengelman.shadow' version '5.2.0'
    }

    repositories {
        jcenter()
        flatDir {
            dirs 'lib'
        }
    }

    archivesBaseName = 'gradle-example'

    dependencies {
        implementation 'org.codehaus.groovy:groovy:3.+'

        // Groovy module dependency
        implementation 'org.codehaus.groovy:groovy-json:3.+'

        // Strictly exact version of Maven dependency
        implementation 'com.codevineyard:hello-world:1.0.1!!'

        // Dependency from local jar
        implementation ':simple-jar'
    }

    application {
        mainClassName = 'com.adjectivecolournoun.gradle.Greetz'
    }
    "#;

    let mut parsed = DSLBlock::parse_str(input).unwrap();
    println!("Original:\n{}", parsed.to_string_pretty());

    // set
    parsed
        .set(
            &["application", "newDep"],
            model::DSLValue::String("com.example.gradle".into()),
        )
        .unwrap();

    // get
    if let Some(DSLValue::String(val)) = parsed.get(&["application", "newDep"]) {
        println!("Got it: {}", val);
    } else {
        println!("Not found");
    }

    // remove
    parsed.remove(&["application", "mainClassName"]).unwrap();

    println!("\n🧾 After mutation:\n{}", parsed.to_string_pretty());
}
```

### 📦 Structs
```
pub enum DSLValue {
    String(String),
    Bool(bool),
    Block(DSLBlock),
    Assignment(String),
    FunctionCall(Vec<DSLValue>),
    MultiArgs(HashMap<String, DSLValue>),
}

pub struct DSLBlock {
    pub name: String,
    pub entries: HashMap<String, DSLValue>,
}

```

---

## Not yet supported

- Rewritting or formatiing `gradle` files

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