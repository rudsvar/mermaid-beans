use crate::model::ContextWrapper;

pub fn to_mermaid(context: ContextWrapper, package_filter: &str) -> String {
    let mut mermaid = String::new();
    mermaid.push_str("graph TD;\n");
    for (context_name, context) in context.contexts {
        mermaid.push_str(&format!("subgraph {}\n", context_name));
        for (bean_name, bean) in context.beans {
            if !bean.r#type.starts_with(package_filter) {
                continue;
            }
            mermaid.push_str(&format!("    {}[{}]\n", bean_name, bean.r#type));
            for dependency in bean.dependencies {
                mermaid.push_str(&format!("    {} --> {}\n", bean_name, dependency));
            }
        }
        mermaid.push_str("end\n");
    }
    mermaid
}
