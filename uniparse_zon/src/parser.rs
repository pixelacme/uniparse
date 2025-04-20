use std::collections::HashMap;

use crate::model::ZonValue;

#[derive(Debug, Clone, PartialEq)]
enum ZonToken {
    DotKey(String),
    Equals,
    OpenBrace,
    CloseBrace,
    String(String),
    Bool(bool),
    Comma,
}

pub fn parse_zon(input: &str) -> Result<ZonValue, String> {
    let tokens = tokenize(input)?;
    // println!("TOKENS: {:#?}", tokens); // 👈 print token stream
    let (val, _) = parse_value(&tokens, 0)?;
    Ok(val)
}

fn tokenize(input: &str) -> Result<Vec<ZonToken>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '.' => {
                chars.next(); // consume '.'
                // NEW: check for `. {` as root-level object
                if chars.peek() == Some(&'{') {
                    chars.next(); // consume '{'
                    tokens.push(ZonToken::OpenBrace);
                    continue;
                }

                let mut key = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '-' {
                        key.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if key == "true" {
                    tokens.push(ZonToken::Bool(true));
                } else if key == "false" {
                    tokens.push(ZonToken::Bool(false));
                } else {
                    tokens.push(ZonToken::DotKey(key));
                }
            }
            '=' => {
                chars.next();
                tokens.push(ZonToken::Equals);
            }
            '{' => {
                chars.next();
                tokens.push(ZonToken::OpenBrace);
            }
            '}' => {
                chars.next();
                tokens.push(ZonToken::CloseBrace);
            }
            ',' => {
                chars.next();
                tokens.push(ZonToken::Comma);
            }
            '"' => {
                chars.next(); // consume quote
                let mut val = String::new();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        break;
                    }
                    val.push(c);
                }
                tokens.push(ZonToken::String(val));
            }
            c if c.is_whitespace() => {
                chars.next(); // skip
            }
            c if c.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match ident.as_str() {
                    "true" => tokens.push(ZonToken::Bool(true)),
                    "false" => tokens.push(ZonToken::Bool(false)),
                    _ => return Err(format!("Unknown identifier: {}", ident)),
                }
            }
            _ => {
                return Err(format!("Unexpected character: {}", ch));
            }
        }
    }

    Ok(tokens)
}

fn parse_value(tokens: &[ZonToken], mut i: usize) -> Result<(ZonValue, usize), String> {
    match tokens.get(i) {
        Some(ZonToken::OpenBrace) => {
            i += 1;

            // 🔍 Peek ahead to see if it's a list or object
            let is_list = matches!(tokens.get(i), Some(ZonToken::String(_)));

            if is_list {
                let mut list = Vec::new();

                while i < tokens.len() && !matches!(tokens[i], ZonToken::CloseBrace) {
                    if let Some(ZonToken::String(s)) = tokens.get(i) {
                        list.push(ZonValue::String(s.clone()));
                        i += 1;

                        if tokens.get(i) == Some(&ZonToken::Comma) {
                            i += 1;
                        }
                    } else {
                        return Err(format!("Expected string in list, got {:?}", tokens.get(i)));
                    }
                }

                if tokens.get(i) != Some(&ZonToken::CloseBrace) {
                    return Err(format!(
                        "Expected closing '}}' for list, got {:?}",
                        tokens.get(i)
                    ));
                }

                return Ok((ZonValue::List(list), i + 1));
            }

            // ✅ Parse object as before
            let mut object = HashMap::new();

            while i < tokens.len() && !matches!(tokens[i], ZonToken::CloseBrace) {
                match &tokens[i] {
                    ZonToken::DotKey(key) => {
                        i += 1;
                        if tokens.get(i) != Some(&ZonToken::Equals) {
                            return Err(format!("Expected '=' after key '{}'", key));
                        }
                        i += 1;
                        let (val, next) = parse_value(tokens, i)?;
                        object.insert(key.clone(), val);
                        i = next;

                        if tokens.get(i) == Some(&ZonToken::Comma) {
                            i += 1;
                        }
                    }
                    _ => return Err(format!("Expected .key, got {:?}", tokens.get(i))),
                }
            }

            if tokens.get(i) != Some(&ZonToken::CloseBrace) {
                return Err(format!(
                    "Expected closing '}}' for object, got {:?}",
                    tokens.get(i)
                ));
            }

            Ok((ZonValue::Object(object), i + 1))
        }

        Some(ZonToken::String(_s)) => {
            // let mut values = Vec::new();
            // while let Some(ZonToken::String(s)) = tokens.get(i) {
            //     values.push(ZonValue::String(s.clone()));
            //     i += 1;
            //     if tokens.get(i) == Some(&ZonToken::Comma) {
            //         i += 1;
            //     } else {
            //         break;
            //     }
            // }
            // Ok((ZonValue::List(values), i))
            let mut values = Vec::new();
            while let Some(ZonToken::String(s)) = tokens.get(i) {
                values.push(ZonValue::String(s.clone()));
                i += 1;
                if tokens.get(i) == Some(&ZonToken::Comma) {
                    i += 1;
                } else {
                    break;
                }
            }
            if values.len() == 1 {
                Ok((values.into_iter().next().unwrap(), i))
            } else {
                Ok((ZonValue::List(values), i))
            }
            // Ok((ZonValue::String(s.clone()), i + 1))
        }

        Some(ZonToken::Bool(b)) => Ok((ZonValue::Bool(*b), i + 1)),

        Some(ZonToken::DotKey(k)) if k == "true" || k == "false" => {
            let val = k == "true";
            Ok((ZonValue::Bool(val), i + 1))
        }

        _ => Err(format!("Unexpected token at {}", i)),
    }
}

//===================================//
// T E S T S                         //
//===================================//

#[test]
fn test_parse_simple_string() {
    let input = r#".{ .name = "zig", }"#;
    let result = parse_zon(input).unwrap();

    match result {
        ZonValue::Object(map) => {
            assert_eq!(map.get("name"), Some(&ZonValue::String("zig".into())));
        }
        _ => panic!("Expected object"),
    }
}

#[test]
fn test_parse_booleans() {
    let input = r#".{ .enabled = true, .disabled = false, }"#;
    let result = parse_zon(input).unwrap();

    if let ZonValue::Object(map) = result {
        assert_eq!(map.get("enabled"), Some(&ZonValue::Bool(true)));
        assert_eq!(map.get("disabled"), Some(&ZonValue::Bool(false)));
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_parse_list_of_strings() {
    let input = r#".{ .paths = .{ "a", "b", "c", }, }"#;
    let result = parse_zon(input).unwrap();

    if let ZonValue::Object(map) = result {
        if let ZonValue::List(list) = map.get("paths").unwrap() {
            let strings: Vec<_> = list.iter().map(|v| v.as_str().unwrap()).collect();
            assert_eq!(strings, vec!["a", "b", "c"]);
        } else {
            panic!("Expected list");
        }
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_nested_object() {
    let input = r#".{ .outer = .{ .inner = "value", }, }"#;
    let result = parse_zon(input).unwrap();

    if let ZonValue::Object(map) = result {
        if let ZonValue::Object(nested) = map.get("outer").unwrap() {
            assert_eq!(
                nested.get("inner").unwrap(),
                &ZonValue::String("value".into())
            );
        } else {
            panic!("Expected inner object");
        }
    } else {
        panic!("Expected outer object");
    }
}

#[test]
fn test_parse_single_string_value() {
    let input = r#".{ .only = "one", }"#;
    let result = parse_zon(input).unwrap();

    if let ZonValue::Object(map) = result {
        assert_eq!(map.get("only").unwrap().as_str().unwrap(), "one");
    } else {
        panic!("Expected object");
    }
}

#[test]
fn test_error_invalid_char() {
    let input = r#".{ .bad = @nope, }"#;
    let result = parse_zon(input);
    assert!(result.is_err());
}

#[test]
fn test_error_missing_closing() {
    let input = r#".{ .key = "value" "#; // missing closing brace
    let result = parse_zon(input);
    assert!(result.is_err());
}
