#![forbid(missing_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![doc = include_str!("../Readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/31191489")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/31191489")]

use wasm_bindgen::prelude::*;
#[wasm_bindgen(module = "/src/prism.min.js")]
extern "C" {
    #[wasm_bindgen]
    pub fn prism_render(input: &str, lang: &str) -> String;
}
