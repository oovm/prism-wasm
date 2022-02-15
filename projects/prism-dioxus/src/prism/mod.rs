use std::{
    cell::{Ref, RefCell, RefMut},
    fmt::{Debug, Formatter},
    rc::Rc,
};

use dioxus::prelude::*;
use dioxus_elements::{div, GlobalAttributes};

use crate::{PrismOptions};

pub mod builder;


/// A hook which keeping the context of KaTeX formula.
pub struct UsePrism {
    prism: Rc<RefCell<PrismOptions>>,
    updater: Rc<dyn Fn() + 'static>,
}

impl UsePrism {
    /// Get all config of KaTeX formula.
    pub fn get_config(&self) -> Ref<'_, PrismOptions> {
        self.prism.borrow()
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
        LazyNodes::new_some(move |cx: NodeFactory| -> VNode {
            cx.element(
                div,
                cx.bump().alloc([]),
                cx.bump().alloc([div.dangerous_inner_html(cx, format_args!("{out}", out = out))]),
                cx.bump().alloc([]),
                None,
            )
        })
    }
}
