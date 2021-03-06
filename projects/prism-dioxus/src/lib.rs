#![forbid(missing_docs)]
#![forbid(missing_debug_implementations)]
#![forbid(rustdoc::missing_crate_level_docs)]
#![doc = include_str ! ("../readme.md")]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/31191489")]
#![doc(html_favicon_url = "https://avatars.githubusercontent.com/u/31191489")]

pub use self::prism::{
    builder::{use_prism, use_prism_rust},
    UsePrism,
};
pub use prism_wasmbind::*;

mod prism;
