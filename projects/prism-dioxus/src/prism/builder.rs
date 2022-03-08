use super::*;

/// A builder for a [`UsePrism`] hook.
pub fn use_prism(cx: &ScopeState, prism: PrismOptions) -> &mut UsePrism {
    let katex = UsePrism { prism: Rc::new(RefCell::new(prism)), updater: cx.schedule_update() };
    cx.use_hook(|_| katex)
}
/// A builder for a [`UsePrism`] hook in display mode.
pub fn use_prism_rust(cx: &ScopeState) -> &mut UsePrism {
    use_prism(cx, PrismOptions::rust())
}

impl Debug for UsePrism {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.prism.borrow(), f)
    }
}
