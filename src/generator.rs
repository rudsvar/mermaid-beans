//! Mermaid diagram generator.

use regex::Regex;

use crate::model::ContextWrapper;

static ALLOWED_CHARACTERS: &str = "#._";

/// Generates a Mermaid diagram from a `ContextWrapper` and a package filter.
pub fn generate_mermaid(context: ContextWrapper, r#type: &Regex) -> String {
    let mut mermaid = String::new();
    mermaid.push_str("graph TD;\n");
    mermaid.push_str("classDef transparent fill:#0000\n");
    for (context_name, context) in context.contexts {
        mermaid.push_str(&format!("subgraph {}\n", context_name));
        mermaid.push_str("direction LR");
        for (bean_name, bean) in context.beans {
            if !r#type.is_match(&bean.r#type) {
                continue;
            }
            let legal_character = |c: &char| c.is_alphanumeric() || ALLOWED_CHARACTERS.contains(*c);
            let bean_name: String = bean_name.chars().filter(legal_character).collect();
            let bean_type: String = bean.r#type.chars().filter(legal_character).collect();
            mermaid.push_str(&format!(
                "    {bean_name}(<b>{bean_name}</b><div style=\"color: gray\">{bean_type}</div>)\n",
            ));
            for dependency in bean.dependencies {
                let dependency: String = dependency.chars().filter(legal_character).collect();
                mermaid.push_str(&format!("    {bean_name} --> {dependency}\n",));
            }
        }
        mermaid.push_str("end\n");
        mermaid.push_str(&format!("class {} transparent\n", context_name));
    }
    mermaid
}
