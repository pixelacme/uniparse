use serde::{Deserialize, Serialize};
use std::{fs, path::Path, usize};

/// Represents a parsed `go.mod` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoMod {
    /// The module path declared in `go.mod`, e.g., `example.com/m`.
    pub module: String,
    /// The Go version specified, e.g., `1.20`.
    pub go_version: String,
    /// List of dependencies declared via `require` in `go.mod`.
    pub requires: Vec<GoDependency>,
    // pub replaces: Vec<GoReplace>,
    // pub excludes: Vec<GoExclude>,
}

/// Represents a single `require` dependency entry in a `go.mod` file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GoDependency {
    /// Name of the module, e.g., `github.com/foo/bar`.
    pub name: String,
    /// Required version, e.g., `v1.2.3`.
    pub version: String,
}

impl GoMod {
    /// Get a string field from the `GoMod` by path.
    ///
    /// Supported paths:
    /// - `["module"]`
    /// - `["go_version"]`
    /// - `["requires", "<index>", "name" | "version"]`
    pub fn get(&self, path: &[&str]) -> Option<&str> {
        match path {
            ["module"] => Some(&self.module),
            ["go_version"] => Some(&self.go_version),
            ["requires", idx_str, field] => {
                let idx = idx_str.parse::<usize>().ok()?;
                let dep = self.requires.get(idx)?;

                match *field {
                    "name" => Some(&dep.name),
                    "version" => Some(&dep.version),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    /// Set a string field in the `GoMod` by path.
    ///
    /// Supported paths:
    /// - `["module"]`
    /// - `["go_version"]`
    /// - `["requires", "<index>", "name" | "version"]`
    ///
    /// # Errors
    /// Returns `Err` if the path is unsupported or index is invalid.
    pub fn set(&mut self, path: &[&str], value: &str) -> Result<(), String> {
        match path {
            ["module"] => {
                self.module = value.to_string();
                Ok(())
            }
            ["go_version"] => {
                self.go_version = value.to_string();
                Ok(())
            }
            ["requires", idx_str, field] => {
                let idx = idx_str.parse::<usize>().map_err(|_| "Invalid index")?;
                let dep = self.requires.get_mut(idx).ok_or("Index out of bounds")?;

                match *field {
                    "name" => {
                        dep.name = value.to_string();
                        Ok(())
                    }
                    "version" => {
                        dep.version = value.to_string();
                        Ok(())
                    }
                    _ => Err("Unknown field".into()),
                }
            }
            _ => Err("Unsupported path".into()),
        }
    }

    /// Remove a dependency by index from the `requires` list.
    ///
    /// Only supports paths in the format `["requires", "<index>"]`.
    ///
    /// # Errors
    /// Returns `Err` if the path is invalid or index is out of bounds.
    pub fn remove(&mut self, path: &[&str]) -> Result<(), String> {
        match path {
            ["requires", idx_str] => {
                let idx = idx_str.parse::<usize>().map_err(|_| "Invalid index")?;
                if idx >= self.requires.len() {
                    return Err("Index out of bounds".into());
                }
                self.requires.remove(idx);
                Ok(())
            }
            _ => Err("Remove only supports ['requires', idx]".into()),
        }
    }

    /// Parses a `go.mod` file from the given path.
    ///
    /// # Errors
    /// Returns a [`ParseError`] if the file can't be read or parsed.
    pub fn parse_file(path: impl AsRef<Path>) -> Result<GoMod, ParseError> {
        let content = fs::read_to_string(path)?;
        Self::parse_str(&content)
    }

    /// Parses the contents of a `go.mod` file from a string.
    ///
    /// # Errors
    /// Returns a [`ParseError`] if required fields are missing or the syntax is invalid.
    pub fn parse_str(content: &str) -> Result<GoMod, ParseError> {
        let mut module = None;
        let mut go_version = None;
        let mut requires = Vec::new();
        let mut in_require_block = false;

        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();

            if trimmed.is_empty() || trimmed.starts_with("//") {
                continue;
            }

            match trimmed {
                l if l.starts_with("module ") => {
                    module = Some(l["module ".len()..].trim().to_string())
                }
                l if l.starts_with("go ") => go_version = Some(l["go ".len()..].trim().to_string()),
                "require (" => in_require_block = true,
                ")" if in_require_block => in_require_block = false,
                l if in_require_block || l.starts_with("require ") => {
                    let cleaned = l.strip_prefix("require").unwrap_or(l).trim();
                    let parts: Vec<&str> = cleaned.split_whitespace().collect();
                    if parts.len() >= 2 {
                        requires.push(GoDependency {
                            name: parts[0].to_string(),
                            version: parts[1].to_string(),
                        });
                    } else {
                        return Err(ParseError::Syntax {
                            line: i + 1,
                            msg: format!("Invalid require entry: `{}`", line),
                        });
                    }
                }
                _ => {}
            }
        }

        let module = module.ok_or(ParseError::MissingField("module"))?;
        let go_version = go_version.ok_or(ParseError::MissingField("go version"))?;

        Ok(GoMod {
            module,
            go_version,
            requires,
            // replaces,
            // excludes,
        })
    }
}

/// Errors returned by `go.mod` parsing routines.
#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    /// IO error when reading a file.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Unexpected or malformed syntax in the file.
    #[error("Unexpected token on line {line}: {msg}")]
    Syntax {
        /// Line number (starting at 1).
        line: usize,
        /// Details of the error.
        msg: String,
    },

    /// A required field (e.g. `module`, `go`) was not found.
    #[error("Missing required field: {0}")]
    MissingField(&'static str),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn fixture_go_mod() -> &'static str {
        r#"
        module example.com/test
        go 1.20

        require (
            github.com/one/lib v1.0.0
            github.com/two/lib v2.3.4
        )

        require github.com/three/lib v0.9.1
    "#
    }

    #[test]
    fn test_parse_str_single_and_block() {
        let parsed = GoMod::parse_str(fixture_go_mod()).unwrap();

        assert_eq!(parsed.module, "example.com/test");
        assert_eq!(parsed.go_version, "1.20");
        assert_eq!(parsed.requires.len(), 3);
        assert_eq!(parsed.requires[0].name, "github.com/one/lib");
        assert_eq!(parsed.requires[1].version, "v2.3.4");
        assert_eq!(parsed.requires[2].name, "github.com/three/lib");
    }

    #[test]
    fn test_get_paths() {
        let parsed = GoMod::parse_str(fixture_go_mod()).unwrap();

        assert_eq!(parsed.get(&["module"]), Some("example.com/test"));
        assert_eq!(parsed.get(&["go_version"]), Some("1.20"));
        assert_eq!(parsed.get(&["requires", "1", "version"]), Some("v2.3.4"));
        assert_eq!(parsed.get(&["requires", "99", "name"]), None);
        assert_eq!(parsed.get(&["invalid"]), None);
    }

    #[test]
    fn test_set_paths() {
        let mut parsed = GoMod::parse_str(fixture_go_mod()).unwrap();

        parsed.set(&["module"], "example.com/changed").unwrap();
        parsed.set(&["requires", "1", "version"], "v9.9.9").unwrap();

        assert_eq!(parsed.module, "example.com/changed");
        assert_eq!(parsed.requires[1].version, "v9.9.9");

        let bad = parsed.set(&["requires", "999", "name"], "nope");
        assert!(bad.is_err());
    }

    #[test]
    fn test_remove_path() {
        let mut parsed = GoMod::parse_str(fixture_go_mod()).unwrap();
        assert_eq!(parsed.requires.len(), 3);

        parsed.remove(&["requires", "1"]).unwrap();
        assert_eq!(parsed.requires.len(), 2);
        assert_eq!(parsed.requires[1].name, "github.com/three/lib");

        let result = parsed.remove(&["requires", "99"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_fields() {
        let no_module = "go 1.18";
        let result = GoMod::parse_str(no_module);
        assert!(matches!(result, Err(ParseError::MissingField("module"))));

        let no_go = "module my.com/thing";
        let result = GoMod::parse_str(no_go);
        assert!(matches!(
            result,
            Err(ParseError::MissingField("go version"))
        ));
    }

    #[test]
    fn test_invalid_require_entry() {
        let content = r#"
        module a.com/b
        go 1.20
        require github.com/foo/bar
    "#;

        let result = GoMod::parse_str(content);
        assert!(matches!(result, Err(ParseError::Syntax { .. })));
    }

    #[test]
    fn test_parse_file_ok() {
        let content = r#"
        module local.test/foo
        go 1.19
        require github.com/test/lib v1.2.3
    "#;

        let path = PathBuf::from("tmp_test.mod");
        fs::write(&path, content).unwrap();

        let parsed = GoMod::parse_file(&path).unwrap();
        assert_eq!(parsed.module, "local.test/foo");
        assert_eq!(parsed.requires[0].version, "v1.2.3");

        fs::remove_file(&path).unwrap();
    }

    #[test]
    fn test_parse_file_io_error() {
        let result = GoMod::parse_file("nonexistent_path.go.mod");
        assert!(matches!(result, Err(ParseError::Io(_))));
    }
}
