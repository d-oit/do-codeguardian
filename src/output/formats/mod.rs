//! # Output Format Implementations
//!
//! This module contains implementations of the OutputFormatter trait
//! for various output formats supported by CodeGuardian.

pub mod html;
pub mod json;
pub mod markdown;
pub mod sarif;
pub mod yaml;

pub use html::HtmlFormatter;
pub use json::JsonFormatter;
pub use markdown::MarkdownFormatter;
pub use sarif::SarifFormatter;
pub use yaml::YamlFormatter;
