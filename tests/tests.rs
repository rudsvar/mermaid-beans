use mermaid_beans::{filter::Filters, generator::Direction, model::ContextWrapper};
use regex::Regex;

#[test]
fn parse_and_write() {
    // Read, parse, and generate mermaid.
    let src = std::fs::read_to_string("tests/beans.json");
    let top_level: ContextWrapper = serde_json::from_str(&src.unwrap()).unwrap();
    let mermaid = mermaid_beans::mermaid_beans_markdown(
        top_level,
        &Filters {
            //r#type: Regex::new("com.example.demo").unwrap(),
            name: Regex::new("(Controller|Service|Repository|entity)").unwrap(),
            ..Default::default()
        },
        Direction::TopDown,
    );

    // Update README.md with the generated mermaid.
    let re = regex::Regex::new(r"(?s)```mermaid[^`]*```").unwrap();
    let readme = std::fs::read_to_string("README.md").unwrap();
    let readme = re.replace(&readme, |_: &regex::Captures| mermaid.clone());
    std::fs::write("README.md", readme.as_bytes()).unwrap();
}
