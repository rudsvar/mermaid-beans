//! Mermaid diagram generator.

use std::fmt::Display;

use crate::model::ContextWrapper;

static ALLOWED_CHARACTERS: &str = "#._";

/// Direction of the graph.
#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum Direction {
    /// Top-down.
    TopToBottom,
    /// Left-right.
    LeftToRight,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::TopToBottom => write!(f, "TB"),
            Direction::LeftToRight => write!(f, "LR"),
        }
    }
}

/// Generates a Mermaid diagram.
pub fn generate_mermaid(context: ContextWrapper, direction: Direction) -> String {
    let mut mermaid = String::new();
    mermaid.push_str(&format!("graph {};\n", direction));
    mermaid.push_str("classDef transparent fill:#0000;\n");
    for (context_name, context) in context.contexts {
        mermaid.push_str(&format!("subgraph \"{}\";\n", context_name));
        mermaid.push_str(&format!("    direction {};\n", direction));
        for (bean_name, bean) in context.beans {
            let legal_character = |c: &char| c.is_alphanumeric() || ALLOWED_CHARACTERS.contains(*c);
            let bean_name: String = bean_name.chars().filter(legal_character).collect();
            let bean_type: String = bean.r#type.chars().filter(legal_character).collect();
            mermaid.push_str(&format!(
                "    {bean_name}(
        <b>{bean_name}</b>
        <div style=\"color: gray\">{bean_type}<br></div>
    )\n",
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

/// Generates a Mermaid diagram string and wraps it in a code block.
pub fn generate_mermaid_markdown(context: ContextWrapper, direction: Direction) -> String {
    let mut mermaid = String::new();
    mermaid.push_str("```mermaid\n");
    mermaid.push_str(&generate_mermaid(context, direction));
    mermaid.push_str("```");
    mermaid
}
