use std::str::FromStr;

use clap::Parser;
use regex::Regex;

#[derive(clap::Parser)]
struct Args {
    /// URL or file path. If not provided, reads from stdin.
    uri: Option<String>,
    /// Include beans by type (or package).
    #[clap(short, long)]
    r#type: Option<Regex>,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let r#type = args.r#type.unwrap_or(Regex::new("")?);
    let content = match args.uri {
        Some(uri) => match url::Url::from_str(&uri) {
            Ok(_) => ureq::get(&uri).call()?.into_string()?,
            Err(_) => std::fs::read_to_string(uri)?,
        },
        None => std::io::read_to_string(std::io::stdin())?,
    };
    let top_level: mermaid_beans::model::ContextWrapper = serde_json::from_str(&content)?;
    let mermaid = mermaid_beans::generator::generate_mermaid(top_level, &r#type);
    println!("{}", mermaid);
    Ok(())
}
