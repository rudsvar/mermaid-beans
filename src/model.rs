use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextWrapper {
    pub contexts: BTreeMap<String, Context>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub beans: BTreeMap<String, Bean>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bean {
    pub aliases: Vec<String>,
    pub scope: String,
    pub r#type: String,
    pub resource: Option<String>,
    pub dependencies: Vec<String>,
}
