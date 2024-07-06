fn main() {
    let path = std::env::args().nth(1).unwrap();
    let package_filter = std::env::args().nth(2).unwrap_or("".to_string());
    let content = std::fs::read_to_string(path).unwrap();
    let top_level: mermaid_beans::model::ContextWrapper = serde_json::from_str(&content).unwrap();
    let mermaid = mermaid_beans::mermaid::to_mermaid(top_level, &package_filter);
    println!("{}", mermaid);
}
