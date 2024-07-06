use mermaid_beans::model::ContextWrapper;

#[test]
fn parse_and_write() {
    let src = std::fs::read_to_string("tests/beans.json");
    let top_level: ContextWrapper = serde_json::from_str(&src.unwrap()).unwrap();
    std::fs::write(
        "tests/mermaid.txt",
        mermaid_beans::mermaid::to_mermaid(top_level, "com.example.demo"),
    )
    .unwrap();
}
