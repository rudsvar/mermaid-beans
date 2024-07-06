use clap::Parser;
use reqwest::Url;

#[derive(clap::Parser)]
struct Args {
    /// URL or file path. If not provided, reads from stdin.
    uri: Option<String>,
    /// Choose beans to include by package.
    #[clap(short, long)]
    package_filter: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args: Args = Args::parse();
    let package_filter = args.package_filter.unwrap_or_default();
    let content = match args.uri {
        Some(uri) => match Url::parse(&uri) {
            Ok(url) => reqwest::blocking::get(url)?.text()?,
            Err(_) => std::fs::read_to_string(uri)?,
        },
        None => std::io::read_to_string(std::io::stdin())?,
    };
    let top_level: mermaid_beans::model::ContextWrapper = serde_json::from_str(&content)?;
    let mermaid = mermaid_beans::generator::generate_mermaid(top_level, &package_filter);
    println!("{}", mermaid);
    Ok(())
}
