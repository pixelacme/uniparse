use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::parser::parse_zon;

#[derive(Debug, Deserialize)]
pub struct RootZon {
    pub name: String,
    pub version: String,
    pub paths: Vec<String>,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ZonFile {
    pub data: ZonValue,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum ZonValue {
    String(String),
    Bool(bool),
    List(Vec<ZonValue>),
    Object(HashMap<String, ZonValue>),
}

#[derive(Debug, Deserialize)]
pub struct Dependency {
    pub url: String,
    pub hash: String,
    pub lazy: Option<bool>,
}

impl std::str::FromStr for ZonFile {
    type Err = String;

    fn from_str(src: &str) -> Result<Self, Self::Err> {
        let parsed = parse_zon(src)?;
        Ok(ZonFile { data: parsed })
    }
}

impl Display for ZonFile {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.data)
    }
}

impl ZonFile {
    pub fn parse_str(src: &str) -> Result<Self, String>
    where
        Self: Sized,
    {
        Self::from_str(src)
    }

    pub fn to_string_pretty(&self) -> String {
        // format!("{:#?}", self.data)
        self.data.to_string()
    }

    pub fn set(&mut self, path: &[&str], value: ZonValue) -> Result<(), String> {
        let mut current = &mut self.data;

        for (i, key) in path.iter().enumerate() {
            match current {
                ZonValue::Object(map) => {
                    if i == path.len() - 1 {
                        map.insert(key.to_string(), value);
                        return Ok(());
                    } else {
                        current = map
                            .entry(key.to_string())
                            .or_insert_with(|| ZonValue::Object(HashMap::new()));
                    }
                }
                _ => return Err(format!("Path {:?} is not an object", &path[..=i])),
            }
        }

        Err("invalid empty path".into())
    }

    pub fn get(&self, path: &[&str]) -> Option<&ZonValue> {
        let mut current = &self.data;

        for key in path {
            current = match current {
                ZonValue::Object(map) => map.get(*key)?,
                _ => return None,
            }
        }
        Some(current)
    }

    pub fn remove(&mut self, path: &[&str]) -> Result<(), String> {
        if path.is_empty() {
            return Err("Path cannot be empty".into());
        }

        let mut current = &mut self.data;

        for i in 0..path.len() - 1 {
            current = match current {
                ZonValue::Object(map) => map.get_mut(path[i]).ok_or("Path not found")?,
                _ => return Err("Intermediate value is not an object".into()),
            }
        }

        match current {
            ZonValue::Object(map) => {
                map.remove(&path.last().unwrap().to_string());
                Ok(())
            }
            _ => Err("Target is not an object".into()),
        }
    }

    pub fn as_struct<T: for<'de> Deserialize<'de>>(&self) -> Result<T, String> {
        let json: serde_json::Value = self.data.to_json();
        serde_json::from_value(json).map_err(|e| format!("Deserialization error: {e}"))
    }
}

impl std::fmt::Display for ZonValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ZonValue::String(s) => write!(f, "\"{}\"", s),
            ZonValue::Bool(b) => write!(f, "{}", b),
            ZonValue::List(list) => {
                writeln!(f, ".{{")?;
                for val in list {
                    writeln!(f, "    {},", val)?;
                }
                write!(f, "}}")
            }
            ZonValue::Object(map) => {
                writeln!(f, ".{{")?;
                for (k, v) in map {
                    writeln!(f, "    .{} = {},", k, v)?;
                }
                write!(f, "}}")
            }
        }
    }
}

impl ZonValue {
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            ZonValue::String(s) => serde_json::Value::String(s.clone()),
            ZonValue::Bool(b) => serde_json::Value::Bool(*b),
            ZonValue::List(items) => {
                serde_json::Value::Array(items.iter().map(|v| v.to_json()).collect())
            }
            ZonValue::Object(map) => serde_json::Value::Object(
                map.iter().map(|(k, v)| (k.clone(), v.to_json())).collect(),
            ),
        }
    }

    pub fn get_path(&self, path: &[&str]) -> Option<&ZonValue> {
        let mut current = self;

        for key in path {
            current = match current {
                ZonValue::Object(map) => map.get(*key)?,
                _ => return None,
            };
        }

        Some(current)
    }

    pub fn set_path(&mut self, path: &[&str], value: ZonValue) -> Result<(), String> {
        if path.is_empty() {
            return Err("Path cannot be empty".into());
        }

        let mut current = self;
        for (i, key) in path.iter().enumerate() {
            match current {
                ZonValue::Object(map) => {
                    if i == path.len() - 1 {
                        map.insert(key.to_string(), value);
                        return Ok(());
                    } else {
                        current = map
                            .entry(key.to_string())
                            .or_insert_with(|| ZonValue::Object(Default::default()));
                    }
                }
                _ => return Err(format!("Path element '{}' is not an object", key)),
            }
        }

        Err("Unexpected error setting value".into())
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            ZonValue::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            ZonValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn as_list(&self) -> Option<&Vec<ZonValue>> {
        match self {
            ZonValue::List(list) => Some(list),
            _ => None,
        }
    }

    #[allow(dead_code)]
    fn as_object(&self) -> Option<&HashMap<String, ZonValue>> {
        match self {
            ZonValue::Object(obj) => Some(obj),
            _ => None,
        }
    }
}

//===================================//
// T E S T S                         //
//===================================//

#[cfg(test)]
mod tests {
    use crate::model::{RootZon, ZonFile, ZonValue};
    use std::collections::HashMap;

    fn sample_zon() -> ZonFile {
        let input = r#"
        .{
            .name = "test",
            .version = "0.1.0",
            .paths = .{
                "src",
                "README.md"
            },
            .dependencies = .{
                .zigimg = .{
                    .url = "https://example.com/zigimg.tar.gz",
                    .hash = "abc123",
                    .lazy = true
                }
            }
        }
        "#;

        ZonFile::parse_str(input).expect("Failed to parse sample zon")
    }

    #[test]
    fn test_basic_parse_and_get() {
        let zon = sample_zon();
        assert_eq!(zon.get(&["name"]).and_then(|v| v.as_str()), Some("test"));
        assert_eq!(
            zon.get(&["version"]).and_then(|v| v.as_str()),
            Some("0.1.0")
        );
    }

    #[test]
    fn test_nested_access() {
        let zon = sample_zon();
        let url = zon
            .get(&["dependencies", "zigimg", "url"])
            .and_then(|v| v.as_str());
        assert_eq!(url, Some("https://example.com/zigimg.tar.gz"));
    }

    #[test]
    fn test_set_new_value() {
        let mut zon = sample_zon();
        zon.set(
            &["dependencies", "my_dep", "url"],
            ZonValue::String("http://a.com".into()),
        )
        .unwrap();

        assert_eq!(
            zon.get(&["dependencies", "my_dep", "url"])
                .and_then(|v| v.as_str()),
            Some("http://a.com")
        );
    }

    #[test]
    fn test_remove_key() {
        let mut zon = sample_zon();
        zon.remove(&["dependencies", "zigimg", "hash"]).unwrap();

        assert_eq!(zon.get(&["dependencies", "zigimg", "hash"]), None);
    }

    #[test]
    fn test_to_struct_conversion() {
        let zon = sample_zon();
        let root: RootZon = zon.as_struct().unwrap();

        assert_eq!(root.name, "test");
        assert_eq!(root.version, "0.1.0");
        assert!(root.paths.contains(&"src".to_string()));
        assert!(root.dependencies.contains_key("zigimg"));
    }

    #[test]
    fn test_to_json_conversion() {
        let zon = sample_zon();
        let json = zon.data.to_json();

        assert_eq!(
            json["dependencies"]["zigimg"]["lazy"],
            serde_json::json!(true)
        );
    }

    #[test]
    fn test_display_formatting_works() {
        let zon = sample_zon();
        let s = zon.to_string_pretty();
        println!("Formatted Zon:\n{}", s); // <-- helpful debug

        println!("{}", s);

        assert!(
            s.contains(".name"),
            "Expected formatted output to contain .name"
        );
        assert!(
            s.contains(".version"),
            "Expected formatted output to contain .version"
        );
        assert!(
            s.contains(".zigimg"),
            "Expected formatted output to contain .zigimg"
        );
    }

    #[test]
    fn test_zonvalue_accessors() {
        let val = ZonValue::String("hello".into());
        assert_eq!(val.as_str(), Some("hello"));
        assert_eq!(val.as_bool(), None);

        let val = ZonValue::Bool(true);
        assert_eq!(val.as_bool(), Some(true));
        assert_eq!(val.as_str(), None);
    }

    #[test]
    fn test_set_path_on_zonvalue() {
        let mut val = ZonValue::Object(HashMap::new());

        val.set_path(&["foo", "bar"], ZonValue::Bool(true)).unwrap();
        let b = val.get_path(&["foo", "bar"]).and_then(|v| v.as_bool());
        assert_eq!(b, Some(true));
    }

    #[test]
    fn test_invalid_remove() {
        let mut zon = sample_zon();
        let result = zon.remove(&[]);
        assert!(result.is_err());

        let result = zon.remove(&["paths", "nonexistent"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_set_on_non_object() {
        let mut zon = ZonFile {
            data: ZonValue::String("not an object".into()),
        };

        let result = zon.set(&["foo"], ZonValue::Bool(true));
        assert!(result.is_err());
    }
}
