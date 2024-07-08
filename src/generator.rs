//! Mermaid diagram generator.

use crate::model::ContextWrapper;

/// Generates a Mermaid diagram from a `ContextWrapper` and a package filter.
pub fn generate_mermaid(context: ContextWrapper, package_filter: &str) -> String {
    let mut mermaid = String::new();
    mermaid.push_str("graph TD;\n");
    mermaid.push_str("classDef transparent fill:#0000\n");
    for (context_name, context) in context.contexts {
        mermaid.push_str(&format!("subgraph {}\n", context_name));
        mermaid.push_str("direction LR");
        for (bean_name, bean) in context.beans {
            if !bean.r#type.contains(package_filter) {
                continue;
            }
            let bean_name: String = bean_name.chars().filter(|c| c.is_alphanumeric()).collect();
            mermaid.push_str(&format!("    {}[{}]\n", bean_name, bean_name));
            for dependency in bean.dependencies {
                let dependency: String =
                    dependency.chars().filter(|c| c.is_alphanumeric()).collect();
                mermaid.push_str(&format!("    {} --> {}\n", bean_name, dependency));
            }
        }
        mermaid.push_str("end\n");
        mermaid.push_str(&format!("class {} transparent\n", context_name));
    }
    mermaid
}
