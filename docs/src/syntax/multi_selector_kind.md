# Multi selector kind

The `Multi` selector kind is used to define a group of elements in the HTML file. Since [`web_sys::Document::query_selector_all`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html#method.query_selector_all) returns a [`web_sys::NodeList`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.NodeList.html) wrapped in a `Result`, it's impossible to return a canonical type for the group of elements. Therefore, the type of the variable in Rust will be `web_sys::NodeList`.

## Arguments

The `Multi` selector kind can also be seen a function where

* the first argument (**mandatory**) is the selector itself and
* the second argument (**optional**) is the "return type kind".

For example, the macro invocation below:

```rust
initialize_elements!(
    html: "index.html",
    elements: {
        let layers = Multi(".layer");
    }
);
```

will expand to the following code:

```rust
let window = web_sys::window().unwrap();
let document = window.document().unwrap();

let layers: web_sys::NodeList = document
    .query_selector_all(".layer")
    .unwrap();
```

You can learn more about return type kinds in the dedicated section.
