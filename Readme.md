# 🧩 Uniparse Workspace

![Rust Version](https://img.shields.io/badge/rust-stable-blue.svg)
![CI](https://github.com/pixelacme/uniparse/actions/workflows/ci.yml/badge.svg)
[![codecov](https://codecov.io/gh/pixelacme/uniparse/branch/main/graph/badge.svg)](https://codecov.io/gh/pixelacme/uniparse)

**Uniparse** is a modular Rust workspace providing minimal, fast, and structured parsers for domain-specific configuration file formats such as:

- [`uniparse_zon`](./uniparse_zon): parses `.zon` files used in the Zig ecosystem
- [`uniparse_gradle`](./uniparse_gradle): parses simplified Gradle-style DSL
- [`uniparse_go`](./uniparse_god): parses Go `go.mod` and `go.work` files

This repository is structured as a Rust [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) to manage related crates together, share CI/CD pipelines, and simplify development.

---

## 🗂 Workspace Layout

Each crate is fully independent, documented, tested, and published individually to [crates.io](https://crates.io).

---

## 📦 Getting Started

Add any individual parser to your `Cargo.toml`:

```toml
[dependencies]
uniparse-zon = "1.0"
uniparse-gradle = "1.0"
uniparse-gomod = "1.0"
```

## 🧪 Testing + Coverage

CI is powered by GitHub Actions, and includes:
- cargo test for all crates
- cargo clippy + cargo fmt --check
- cargo llvm-cov for coverage (80% minimum enforced)
- Uploads to Codecov

### Badges

## 🚀 Publishing

Each crate is versioned and released independently using cargo publish, automated through GitHub Actions.

Versioning & Releases
Patch/minor/major bumps are triggered via release workflows

Tagged versions trigger CI and publish steps per crate

Only changed crates are released

```
cargo release patch

cargo release minor

cargo release major
```

## 🔄 Dependency Management
This workspace uses Dependabot to keep both Cargo and GitHub Action dependencies up to date. Weekly PRs are created and labeled automatically for safe upgrades.

## 🧠 Philosophy
- ✨ Minimal: minimal or no unnecessary dependencies
- 🧩 Composable: small focused crates
- 🔍 Parse + Access + Modify: all supported out of the box
- 💬 Human-readable and programmatically editable

## 🙌 Contributing
PRs and issues are welcome! See CONTRIBUTING.md if you're unsure where to start.

## 🪪 License

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

This project is licensed under the MIT License. You're free to use it in personal or commercial projects.