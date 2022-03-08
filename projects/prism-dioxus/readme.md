PrismJS for Dioxus
==================

Render math with PrismJS in Dioxus!

## How to use

- First you need to import css cdn:

```html
<link rel="stylesheet" href="https://raw.githubusercontent.com/oovm/prism-wasm/dev/projects/prism-wasm/src/prism.min.css">
```

- Call `use_prism_rust` hook to prepare context.
- Call `render` to get code highlight node.

```rust
use dioxus_prism::use_prism_rust;

let rust = use_prism_rust(&cx);
let code = rust.render(text);
```
