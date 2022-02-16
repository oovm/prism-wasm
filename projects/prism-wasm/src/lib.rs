#![forbid(missing_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/31191489")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/31191489")]

use wasm_bindgen::prelude::*;
mod options;
pub use options::{PrismLanguage, PrismOptions};

#[wasm_bindgen(module = "/src/prism.min.js")]
extern "C" {
    /// FFI of prism render function
    #[wasm_bindgen]
    pub fn prism_render(input: &str, lang: &str) -> String;
}
/// The needed css which should packing to project
pub const PRISM_CSS: &str = include_str!("prism.min.css");
