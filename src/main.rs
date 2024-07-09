use std::str::FromStr;

use clap::Parser;
use mermaid_beans::{filter::Filters, generator::Direction};
use regex::Regex;

#[derive(clap::Parser)]
struct Args {
    /// URL or file path. If not provided, reads from stdin.
    uri: Option<String>,
    /// Include beans by type (package).
    #[clap(short, long)]
    r#type: Option<Regex>,
    /// Include beans by name.
    #[clap(short, long)]
    name: Option<Regex>,
    /// Direction of the graph.
    #[clap(short, long, default_value = "LR")]
    direction: Direction,
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
        r#type: args.r#type.unwrap_or(Regex::new("")?),
        name: args.name.unwrap_or(Regex::new("")?),
    };
    let mermaid = mermaid_beans::mermaid_beans_markdown(top_level, &filters, args.direction);
    println!("{}", mermaid);
    Ok(())
}
