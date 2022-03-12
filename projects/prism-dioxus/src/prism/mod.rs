use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Debug, Formatter},
    rc::Rc,
    sync::Arc,
};

use dioxus::prelude::*;
use prism_wasmbind::PrismLanguage;

use crate::PrismOptions;

pub mod builder;

/// A hook which keeping the context of KaTeX formula.
pub struct UsePrism {
    prism: Rc<RefCell<PrismOptions>>,
    updater: Arc<dyn Fn() + 'static>,
}

impl UsePrism {
    /// Get all config of KaTeX formula.
    pub fn get_config(&self) -> Ref<'_, PrismOptions> {
        self.prism.borrow()
    }
    /// Get all config of KaTeX formula.
    pub fn get_language(&self) -> String {
        self.get_config().language.to_string()
    }
    /// Get all config of KaTeX formula.
    pub fn set_language(&self, language: &str) {
        self.get_config_mut().language = match language {
            "javascript" => PrismLanguage::JavaScript,
            "rust" => PrismLanguage::Rust,
            "html" | "markup" => PrismLanguage::HTML,
            "css" => PrismLanguage::Css,
            _ => return,
        };
        self.needs_update()
    }
    /// Get the mutable reference of all config.
    pub fn get_config_mut(&self) -> RefMut<'_, PrismOptions> {
        self.prism.borrow_mut()
    }
    /// Notify the scheduler to re-render the component.
    pub fn needs_update(&self) {
        (self.updater)();
    }
}

impl UsePrism {
    /// Compile the formula to HTML.
    ///
    /// Never fails even if the formula is invalid.
    pub fn render(&self, input: &str) -> LazyNodes {
        let config = self.prism.borrow_mut();
        let out = config.render(input);
        rsx! {
            pre {
                dangerous_inner_html: "{out}"
            }
        }
    }
}
