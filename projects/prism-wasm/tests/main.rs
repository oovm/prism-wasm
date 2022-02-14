use wasm_bindgen_test::*;
use katex_wasmbind::prism_render;

#[test]
fn ready() {
    println!("it works!")
}

#[wasm_bindgen_test]
fn mode() {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
    assert_ne!(prism_render("\\frac12", "js"), prism_render("\\frac12", "js"));
}
