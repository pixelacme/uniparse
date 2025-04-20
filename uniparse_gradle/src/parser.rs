use crate::model::{DSLBlock, DSLValue};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    String(String),
    Bool(bool),
    Equals,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            // Skip whitespace
            c if c.is_whitespace() => {
                chars.next();
            }

            // Symbols
            '{' => {
                tokens.push(Token::OpenBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::CloseBrace);
                chars.next();
            }
            '(' => {
                tokens.push(Token::OpenParen);
                chars.next();
            }
            ')' => {
                tokens.push(Token::CloseParen);
                chars.next();
            }
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }

            // Strings
            '"' | '\'' => {
                let quote = chars.next().unwrap();
                let mut value = String::new();
                while let Some(&c) = chars.peek() {
                    if c == quote {
                        chars.next();
                        break;
                    }
                    value.push(c);
                    chars.next();
                }
                tokens.push(Token::String(value));
            }

            // Identifiers or booleans
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() || c == '_' || c == '.' || c == '-' {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                // Look ahead for ()
                if chars.peek() == Some(&'(') {
                    chars.next(); // consume (
                    if chars.peek() == Some(&')') {
                        chars.next(); // consume )
                        tokens.push(Token::Identifier(ident));
                        tokens.push(Token::OpenParen);
                        tokens.push(Token::CloseParen);
                        continue;
                    } else {
                        panic!("Unexpected char after '(': expected ')'");
                    }
                }

                match ident.as_str() {
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    _ => tokens.push(Token::Identifier(ident)),
                }
            }

            _ => {
                panic!("Unexpected character in input: {}", ch);
            }
        }
    }

    tokens
}

pub fn parse_tokens(tokens: &[Token], start: usize) -> Result<(DSLBlock, usize), String> {
    let mut entries = HashMap::new();
    let mut i = start;

    while i < tokens.len() {
        match &tokens[i] {
            Token::Identifier(key) => {
                let key = key.clone();
                i += 1;

                // Handle block call
                if i >= tokens.len() {
                    return Err(format!(
                        "Expected token after identifier '{}', but reached end",
                        key
                    ));
                }
                if let Token::OpenBrace = &tokens[i] {
                    let (nested_block, consumed) = parse_tokens(tokens, i + 1)?;
                    entries.insert(
                        key.clone(),
                        DSLValue::Block(DSLBlock {
                            name: key.clone(),
                            entries: nested_block.entries,
                        }),
                    );
                    i = consumed;
                    continue;
                }

                if matches!(tokens[i], Token::Equals) {
                    i += 1;
                    if let Token::String(s) = &tokens[i] {
                        entries.insert(key, DSLValue::Assignment(s.clone()));
                        i += 1;
                        continue;
                    }
                }

                if let Token::String(val1) = &tokens[i] {
                    if i + 2 < tokens.len() {
                        if let Token::Identifier(subkey) = &tokens[i + 1] {
                            if let Token::String(val2) = &tokens[i + 2] {
                                let mut args = HashMap::new();
                                args.insert("value".to_string(), DSLValue::String(val1.clone()));
                                args.insert(subkey.clone(), DSLValue::String(val2.clone()));
                                entries.insert(key, DSLValue::MultiArgs(args));
                                i += 3;
                                continue;
                            }
                        }
                    }
                }

                if matches!(
                    (&tokens[i], &tokens[i + 1]),
                    (Token::OpenParen, Token::CloseParen)
                ) {
                    entries.insert(key, DSLValue::FunctionCall(vec![]));
                    i += 2;
                    continue;
                }

                match &tokens[i] {
                    Token::String(s) => {
                        entries.insert(key, DSLValue::String(s.clone()));
                        i += 1;
                    }
                    Token::Bool(b) => {
                        entries.insert(key, DSLValue::Bool(*b));
                        i += 1;
                    }
                    Token::OpenBrace => {
                        let (nested, consumed) = parse_tokens(tokens, i + 1)?;
                        entries.insert(
                            key.clone(),
                            DSLValue::Block(DSLBlock {
                                name: key.clone(),
                                entries: nested.entries,
                            }),
                        );
                        i = consumed;
                    }
                    _ => panic!("Unexpected token after identifier: {:?}", tokens[i]),
                }
            }
            Token::CloseBrace => {
                return Ok((
                    DSLBlock {
                        name: "".to_string(),
                        entries,
                    },
                    i + 1,
                ));
            }
            _ => panic!("Unexpected token: {:?}", tokens[i]),
        }
    }

    Ok((
        DSLBlock {
            name: "".to_string(),
            entries,
        },
        i,
    ))
}

pub fn strip_comments(input: &str) -> String {
    input
        .lines()
        .map(|line| {
            match line.find("//") {
                Some(idx) => &line[..idx], // Cut at comment start
                None => line,
            }
        })
        .map(str::trim_end) // Remove trailing spaces
        .filter(|line| !line.is_empty()) // Skip blank lines
        .collect::<Vec<_>>()
        .join("\n")
}

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use uniparse_core::ParsedFile;

impl FromStr for DSLBlock {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean = strip_comments(s);
        let tokens = tokenize(&clean);
        println!("TOKENS: {:#?}", tokens);
        let (parsed, _) = parse_tokens(&tokens, 0)?;
        Ok(parsed)
    }
}

impl Display for DSLBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        fn write_block(f: &mut Formatter<'_>, block: &DSLBlock, indent: usize) -> std::fmt::Result {
            let pad = "    ".repeat(indent);
            for (key, val) in &block.entries {
                match val {
                    DSLValue::String(s) => writeln!(f, "{}{} \"{}\"", pad, key, s)?,
                    DSLValue::Bool(b) => writeln!(f, "{}{} {}", pad, key, b)?,
                    DSLValue::Block(b) => {
                        writeln!(f, "{}{} {{", pad, key)?;
                        write_block(f, b, indent + 1)?;
                        writeln!(f, "{}}}", pad)?;
                    }
                    DSLValue::Assignment(val) => writeln!(f, "{}{} = \"{}\"", pad, key, val)?,
                    DSLValue::FunctionCall(args) => {
                        if args.is_empty() {
                            writeln!(f, "{}{}()", pad, key)?;
                        } else {
                            let arg_str = args
                                .iter()
                                .map(|v| match v {
                                    DSLValue::String(s) => format!("\"{}\"", s),
                                    DSLValue::Bool(b) => b.to_string(),
                                    _ => "?".into(),
                                })
                                .collect::<Vec<_>>()
                                .join(", ");
                            write!(f, "{}{}({})", pad, key, arg_str)?;
                        }
                    }
                    DSLValue::MultiArgs(map) => {
                        for (subkey, subval) in map {
                            if let DSLValue::String(s) = subval {
                                writeln!(f, "{}{} {} \"{}\"", pad, key, subkey, s)?;
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        write_block(f, self, 0)
    }
}

impl ParsedFile for DSLBlock {
    fn parse_str(source: &str) -> Result<Self, String> {
        DSLBlock::from_str(source)
    }

    fn to_string_pretty(&self) -> String {
        self.to_string()
    }
}

impl DSLBlock {
    pub fn get(&self, path: &[&str]) -> Option<&DSLValue> {
        let mut current = self.entries.get(path[0])?;

        for key in &path[1..] {
            current = match current {
                DSLValue::Block(block) => block.entries.get(*key)?,
                _ => return None,
            }
        }

        Some(current)
    }

    pub fn set(&mut self, path: &[&str], value: DSLValue) -> Result<(), String> {
        if path.is_empty() {
            return Err("Path cannot be empty".into());
        }

        let mut current = &mut self.entries;

        for (i, key) in path.iter().enumerate() {
            let key_string = key.to_string();

            if i == path.len() - 1 {
                current.insert(key_string, value);
                return Ok(());
            }

            let is_new = !current.contains_key(&key_string);
            if is_new {
                current.insert(
                    key_string.clone(),
                    DSLValue::Block(DSLBlock {
                        name: key_string.clone(),
                        entries: HashMap::new(),
                    }),
                );
            }

            match current.get_mut(&key_string) {
                Some(DSLValue::Block(b)) => {
                    current = &mut b.entries;
                }
                _ => return Err(format!("Path '{}' is not a block", key)),
            }
        }

        Err("Unexpected error while traversing path".into())
    }

    pub fn remove(&mut self, path: &[&str]) -> Result<(), String> {
        if path.is_empty() {
            return Err("path cannot be empty".into());
        }

        let mut current = &mut self.entries;

        for key in &path[..path.len() - 1] {
            current = match current.get_mut(*key) {
                Some(DSLValue::Block(block)) => &mut block.entries,
                _ => return Err(format!("Path segment '{}' is not a block", key)),
            }
        }

        current.remove(*path.last().unwrap());
        Ok(())
    }
}

impl DSLValue {
    pub fn as_str(&self) -> Option<&str> {
        match self {
            DSLValue::String(s) | DSLValue::Assignment(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DSLValue::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_block(&self) -> Option<&DSLBlock> {
        match self {
            DSLValue::Block(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_block_mut(&mut self) -> Option<&mut DSLBlock> {
        match self {
            DSLValue::Block(b) => Some(b),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_input() -> &'static str {
        r#"
        plugins {
            id "application"
            id "java"
        }

        dependencies {
            implementation "org.example:lib:1.2.3"
            testImplementation "junit:junit:4.13"
        }

        application {
            mainClassName = "com.example.Main"
            debug true
        }

        buildDir = "build/output"
        clean()
        "#
    }

    #[test]
    fn test_tokenize_basic() {
        let tokens = tokenize(sample_input());
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::Identifier(s) if s == "plugins"))
        );
        assert!(
            tokens
                .iter()
                .any(|t| matches!(t, Token::String(s) if s == "application"))
        );
        assert!(tokens.iter().any(|t| matches!(t, Token::Bool(true))));
    }

    #[test]
    fn test_parse_tokens_structure() {
        let tokens = tokenize(sample_input());
        let (block, _) = parse_tokens(&tokens, 0).unwrap();

        assert!(block.entries.contains_key("plugins"));
        assert!(block.entries.contains_key("dependencies"));
        assert!(block.entries.contains_key("application"));
        assert!(block.entries.contains_key("buildDir"));
    }

    #[test]
    fn test_get_nested_value() {
        let block = DSLBlock::from_str(sample_input()).unwrap();
        let val = block.get(&["application", "mainClassName"]);
        assert_eq!(val.and_then(DSLValue::as_str), Some("com.example.Main"));
    }

    #[test]
    fn test_set_and_get_path() {
        let mut block = DSLBlock::from_str(sample_input()).unwrap();
        block
            .set(&["application", "debug"], DSLValue::Bool(false))
            .unwrap();

        let updated = block.get(&["application", "debug"]);
        assert_eq!(updated.and_then(DSLValue::as_bool), Some(false));
    }

    #[test]
    fn test_remove_path() {
        let mut block = DSLBlock::from_str(sample_input()).unwrap();
        block.remove(&["application", "mainClassName"]).unwrap();

        assert!(block.get(&["application", "mainClassName"]).is_none());
    }

    #[test]
    fn test_parse_assignment_and_function_call() {
        let block = DSLBlock::from_str(sample_input()).unwrap();

        let val = block.get(&["buildDir"]);
        assert_eq!(val.and_then(DSLValue::as_str), Some("build/output"));

        let clean = block.get(&["clean"]);
        assert!(matches!(clean, Some(DSLValue::FunctionCall(_))));
    }

    #[test]
    fn test_multi_args_parsing() {
        let input = r#"
        options "opt1" level "debug"
    "#;

        let tokens = tokenize(input);
        let (block, _) = parse_tokens(&tokens, 0).unwrap();

        if let DSLValue::MultiArgs(args) = block.entries.get("options").unwrap() {
            assert_eq!(args.get("value").and_then(DSLValue::as_str), Some("opt1"));
            assert_eq!(args.get("level").and_then(DSLValue::as_str), Some("debug"));
        } else {
            panic!("Expected MultiArgs DSLValue");
        }
    }

    #[test]
    fn test_empty_function_call() {
        let input = r#"deploy()"#;
        let tokens = tokenize(input);
        let (block, _) = parse_tokens(&tokens, 0).unwrap();

        match block.entries.get("deploy").unwrap() {
            DSLValue::FunctionCall(args) => assert!(args.is_empty()),
            _ => panic!("Expected empty function call"),
        }
    }

    #[test]
    fn test_strip_comments() {
        let input = r#"
        plugins {
            id "java" // apply Java plugin
        } // end of plugins
        "#;

        let stripped = strip_comments(input);
        assert!(!stripped.contains("//"));
        assert!(stripped.contains("id \"java\""));
    }

    #[test]
    #[should_panic(expected = "Unexpected character in input: $")]
    fn test_unexpected_char_panics() {
        let _ = tokenize("invalid$char");
    }

    #[test]
    fn test_display_output() {
        let block = DSLBlock::from_str(sample_input()).unwrap();
        let output = format!("{}", block);
        assert!(output.contains("mainClassName"));
        assert!(output.contains("buildDir"));
    }
}
