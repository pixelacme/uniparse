use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DSLValue {
    String(String),
    Bool(bool),
    Block(DSLBlock),
    Assignment(String),                   // ✅ for key = "value"
    FunctionCall(Vec<DSLValue>),          // ✅ for key(), key("arg")
    MultiArgs(HashMap<String, DSLValue>), // ✅ for id "a" version "b"
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DSLBlock {
    pub name: String,
    pub entries: HashMap<String, DSLValue>,
}
