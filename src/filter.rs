//! Bean context filtering.

use std::collections::BTreeMap;

use regex::Regex;

use crate::model::{Bean, Context, ContextWrapper};

/// Filters for the package and bean names.
#[derive(Clone, Debug)]
pub struct Filters {
    /// Regular expression for the package name.
    pub r#type: Regex,
    /// Regular expression for the bean name.
    pub name: Regex,
}

impl Default for Filters {
    fn default() -> Self {
        Self {
            r#type: Regex::new("").unwrap(),
            name: Regex::new("").unwrap(),
        }
    }
}

/// Filters beans by package and bean name, and removes any dependencies that are not in the filtered beans.
pub fn filter_context(context: ContextWrapper, filters: &Filters) -> ContextWrapper {
    let mut new_contexts: BTreeMap<String, Context> = BTreeMap::new();
    for (context_name, context) in context.contexts {
        // Get all beans that match the filters.
        let new_beans: BTreeMap<String, Bean> = context
            .beans
            .into_iter()
            .filter(|(name, bean)| {
                filters.r#type.is_match(&bean.r#type) && filters.name.is_match(name)
            })
            .collect();

        // Remove dependencies that are not in the filtered beans.
        let new_beans: BTreeMap<String, Bean> = new_beans
            .clone()
            .into_iter()
            .map(|(name, mut bean)| {
                bean.dependencies.retain(|dep| new_beans.contains_key(dep));
                (name, bean)
            })
            .collect();

        if !new_beans.is_empty() {
            new_contexts.insert(context_name, Context { beans: new_beans });
        }
    }
    ContextWrapper {
        contexts: new_contexts,
    }
}
