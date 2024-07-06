use mermaid_beans::model::ContextWrapper;

#[test]
fn parse() {
    let src = std::fs::read_to_string("tests/beans.json");
    let top_level: ContextWrapper = serde_json::from_str(&src.unwrap()).unwrap();
    println!("{:#?}", top_level);
}
