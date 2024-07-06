use crate::model::ContextWrapper;

/// Generate a Mermaid diagram from a `ContextWrapper` and a package filter.
pub fn generate_mermaid(context: ContextWrapper, package_filter: &str) -> String {
    let mut mermaid = String::new();
    mermaid.push_str("graph TD;\n");
    mermaid.push_str("classDef transparent fill:#0000\n");
    for (context_name, context) in context.contexts {
        mermaid.push_str(&format!("subgraph {}\n", context_name));
        for (bean_name, bean) in context.beans {
            if !bean.r#type.starts_with(package_filter) {
                continue;
            }
            let bean_name = bean_name.replace('$', "");
            let bean_type = bean.r#type.replace('$', "");
            mermaid.push_str(&format!("    {}[\"{}\"]\n", bean_name, bean_type));
            for dependency in bean.dependencies {
                let dependency = dependency.replace('$', "");
                mermaid.push_str(&format!("    {} --> {}\n", bean_name, dependency));
            }
        }
        mermaid.push_str("end\n");
        mermaid.push_str(&format!("class {} transparent\n", context_name));
    }
    mermaid
}
