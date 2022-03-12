use prism_wasmbind::prism_render;
use wasm_bindgen_test::*;

#[test]
fn ready() {
    println!("it works!")
}

#[wasm_bindgen_test]
fn mode() {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    assert_eq!(
        prism_render("let mut a = 1", "rust"),
        "<span class=\"token keyword\">let</span> <span class=\"token keyword\">mut</span> a <span class=\"token operator\">=</span> <span class=\"token number\">1</span>"
    );
}
