use crate::{prism_render};
use std::fmt::{Display, Formatter};

/// A Rust struct that represents the options for the Prism Wasm library.
#[derive(Debug, Copy, Clone)]
pub enum PrismLanguage {
    /// The Rust language.
    HTML,
    /// The JavaScript language.
    JavaScript,
    /// The TypeScript language.
    Rust,
    /// The Python language.
    Css,
}

impl Display for PrismLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PrismLanguage::HTML => write!(f, "markup"),
            PrismLanguage::JavaScript => write!(f, "javascript"),
            PrismLanguage::Rust => write!(f, "rust"),
            PrismLanguage::Css => write!(f, "css"),
        }
    }
}

/// Read <https://katex.org/docs/options.html> for more information.
#[derive(Debug, Clone)]
pub struct PrismOptions {
    /// Whether to render the math in the display mode.
    pub language: PrismLanguage,

}

impl Default for PrismOptions {
    fn default() -> Self {
        Self {
            language: PrismLanguage::Rust,
        }
    }
}

impl PrismOptions {
    /// Render code as rust
    pub fn css() -> PrismOptions {
        PrismOptions { language: PrismLanguage::Css, ..PrismOptions::default() }
    }
    /// Render code as rust
    pub fn rust() -> PrismOptions {
        PrismOptions { language: PrismLanguage::Rust, ..PrismOptions::default() }
    }
    /// Render formulas to html string.
    pub fn render(&self, input: &str) -> String {
        prism_render(input, &self.language.to_string())
    }
}

impl PrismOptions {}
