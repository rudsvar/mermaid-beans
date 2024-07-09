#![doc = include_str!("../README.md")]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub
)]
#![deny(unused_must_use)]

use filter::Filters;
use generator::Direction;
use model::ContextWrapper;

pub mod filter;
pub mod generator;
pub mod model;

/// Generates a Mermaid diagram.
pub fn mermaid_beans(context: ContextWrapper, filters: &Filters, direction: Direction) -> String {
    let context = filter::filter_context(context, filters);
    generator::generate_mermaid(context, direction)
}

/// Generates a Mermaid diagram wrapped in a markdown code block.
pub fn mermaid_beans_markdown(
    context: ContextWrapper,
    filters: &Filters,
    direction: Direction,
) -> String {
    let context = filter::filter_context(context, filters);
    generator::generate_mermaid_markdown(context, direction)
}
