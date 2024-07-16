use std::str::FromStr;

use clap::Parser;
use mermaid_beans::{filter::Filters, generator::Direction};
use regex::Regex;

/// A Mermaid diagram generator for Spring Boot Actuator beans.
#[derive(clap::Parser)]
struct Args {
    /// URL or file path. If not provided, reads from stdin.
    uri: Option<String>,
    /// Include beans by type (package).
    #[clap(short, long)]
    r#type: Vec<Regex>,
    /// Include beans by name.
    #[clap(short, long)]
    name: Vec<Regex>,
    /// Direction of the graph.
    #[clap(short, long, default_value = "left-to-right")]
    direction: Direction,
    /// Wrap the output in a code block.
    #[clap(short, long)]
    wrap: bool,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let content = match args.uri {
        Some(uri) => match url::Url::from_str(&uri) {
            Ok(_) => ureq::get(&uri).call()?.into_string()?,
            Err(_) => std::fs::read_to_string(uri)?,
        },
        None => std::io::read_to_string(std::io::stdin())?,
    };
    let top_level: mermaid_beans::model::ContextWrapper = serde_json::from_str(&content)?;
    let filters = Filters {
        r#type: args.r#type,
        name: args.name,
    };
    let mermaid = if args.wrap {
        mermaid_beans::mermaid_beans_markdown(top_level, &filters, args.direction)
    } else {
        mermaid_beans::mermaid_beans(top_level, &filters, args.direction)
    };
    println!("{}", mermaid);
    Ok(())
}
