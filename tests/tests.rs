use mermaid_beans::model::ContextWrapper;

#[test]
fn parse_and_write() {
    let src = std::fs::read_to_string("tests/beans.json");
    let top_level: ContextWrapper = serde_json::from_str(&src.unwrap()).unwrap();
    let mermaid = mermaid_beans::generator::generate_mermaid(top_level, "com.example.demo");
    let markdown = format!("```mermaid\n{}\n```", mermaid);
    std::fs::write("tests/mermaid.md", markdown).unwrap();
}
