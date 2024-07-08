use mermaid_beans::model::ContextWrapper;
use regex::Regex;

#[test]
fn parse_and_write() {
    // Read, parse, and generate mermaid.
    let src = std::fs::read_to_string("tests/beans.json");
    let top_level: ContextWrapper = serde_json::from_str(&src.unwrap()).unwrap();
    let mermaid = mermaid_beans::generator::generate_mermaid(
        top_level,
        &Regex::new("com.example.demo").unwrap(),
    );

    // Update README.md with the generated mermaid.
    let re = regex::Regex::new(r"(?s)```mermaid[^(```)]*```").unwrap();
    let readme = std::fs::read_to_string("README.md").unwrap();
    let readme = re.replace(&readme, |_: &regex::Captures| {
        format!("```mermaid\n{}```", mermaid)
    });
    std::fs::write("README.md", readme.as_bytes()).unwrap();
}
