use log::warn;
use super::*;


pub fn LanguageSelect(ctx: &UsePrism) -> LazyNodes {
    let c = ctx.get_language();
    let set_mode = move |e:FormEvent| {
        warn!("set_mode: {:?}", e.value);
    };
    rsx!(
        label {
            class: "cursor-pointer label",
            span {
                class: "label-text",
                "Compile Mode"
            }
            select {
                class: "select select-primary w-full max-w-xs",
                value: "{c}",
                onchange: set_mode,
                option {
                    value: "m",
                    "Normal"
                }
                option {
                    value: "i",
                    "Inline"
                }
                option {
                    value: "s",
                    "Scoped"
                }
                option {
                    value: "k",
                    "DataKey"
                }
                option {
                    value: "v",
                    "DataValue"
                }
            }
        }
    )
}

pub fn GithubReport(link: &str) -> LazyNodes {
    rsx! {
        a {
            href: "{link}",
            target: "_blank",
            button {
                class: "py-2 px-4 mr-2 mb-2 text-sm font-medium text-gray-900 bg-white rounded-lg border border-gray-200 hover:bg-gray-100 hover:text-blue-700 focus:z-10 focus:ring-2 focus:ring-blue-700 focus:text-blue-700 dark:bg-gray-800 dark:text-gray-400 dark:border-gray-600 dark:hover:text-white dark:hover:bg-gray-700",
                r#type: "button",
                "Report bug on github"
            }
        }
    }
}
