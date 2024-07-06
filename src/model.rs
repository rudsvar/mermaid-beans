//! Spring Boot Actuator context model.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// Top level structure for a Spring context.
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextWrapper {
    /// Spring contexts.
    pub contexts: BTreeMap<String, Context>,
}

/// A Spring context.
#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    /// Spring beans.
    pub beans: BTreeMap<String, Bean>,
}

/// A Spring bean.
#[derive(Debug, Serialize, Deserialize)]
pub struct Bean {
    /// Bean aliases.
    pub aliases: Vec<String>,
    /// Bean scope.
    pub scope: String,
    /// Bean type.
    pub r#type: String,
    /// Bean resource.
    pub resource: Option<String>,
    /// Bean dependencies.
    pub dependencies: Vec<String>,
}
