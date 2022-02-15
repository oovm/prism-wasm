mod widgets;
use dioxus::{events::FormEvent, prelude::*};
use macrowind::tw;
use dioxus_prism::{use_prism_rust, UsePrism};
use widgets::*;

pub fn Editor(cx: Scope) -> Element {
    let (text, text_set) = use_state(&cx, || include_str!("mod.rs").to_string());
    let prism = use_prism_rust(&cx);
    let language = LanguageSelect(prism);
    let code = prism.render(text);
    let place_holder = prism.get_language();
    let issue = GithubReport("https://github.com/oovm/prism-wasm/issues");
    let style1 = tw!("flex flex-row").1;
    let style2 = tw!("flex-1 ml-2 mr-2").1;
    cx.render(rsx!(
        div {
            style: "{style1}",
            div {
                class: "form-control flex-1",
                textarea {
                    class: "textarea h-96 textarea-bordered textarea-primary",
                    id: "editor",
                    placeholder: "{place_holder}",
                    oninput: move |e| text_set(e.value.to_owned()),
                    value: "{text}",
                }
            }
            div {
                style: "{style2}",
                code
            }
        }
        div {
            class: "form-control",
            language
            issue
        }
    ))
}
