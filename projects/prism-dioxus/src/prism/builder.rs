use super::*;

/// A builder for a [`UseKatex`] hook.
pub fn use_katex(cx: &ScopeState, katex: KaTeXOptions) -> &mut UsePrism {
    let katex = UsePrism { katex: Rc::new(RefCell::new(katex)), updater: cx.schedule_update() };
    cx.use_hook(|_| katex)
}
/// A builder for a [`UseKatex`] hook in display mode.
pub fn use_katex_display(cx: &ScopeState) -> &mut UsePrism {
    use_katex(cx, KaTeXOptions::display_mode())
}
/// A builder for a [`UseKatex`] hook in inline mode.
pub fn use_katex_inline(cx: &ScopeState) -> &mut UsePrism {
    use_katex(cx, KaTeXOptions::inline_mode())
}

impl Debug for UsePrism {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.katex.borrow(), f)
    }
}
