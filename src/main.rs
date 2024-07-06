use clap::Parser;

#[derive(clap::Parser)]
struct Args {
    path: String,
    #[clap(short, long)]
    package_filter: Option<String>,
}

fn main() {
    let args: Args = Args::parse();
    let path = args.path;
    let package_filter = args.package_filter.unwrap_or_default();
    let content = std::fs::read_to_string(path).unwrap();
    let top_level: mermaid_beans::model::ContextWrapper = serde_json::from_str(&content).unwrap();
    let mermaid = mermaid_beans::generator::generate_mermaid(top_level, &package_filter);
    println!("{}", mermaid);
}
