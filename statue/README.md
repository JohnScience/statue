# Statue

> Easier way to query selectors for static HTML pages.

[![Crates.io](https://img.shields.io/crates/v/statue)](https://crates.io/crates/statue)
[![Docs.rs](https://docs.rs/statue/badge.svg)](https://docs.rs/statue)
[![License](https://img.shields.io/crates/l/statue)](https://crates.io/crates/statue)

**[User guide](https://johnscience.github.io/statue/)**

Spare yourself from writing

```rust
let window = web_sys::window().unwrap();
let document = window.document().unwrap();

let work_area = document
    .query_selector("#work-area")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlDivElement>()
    .unwrap();

let layer_list_div = document
    .query_selector("#layer-list")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlDivElement>()
    .unwrap();

let save_files_btn: Rc<HtmlButtonElement> = document
    .query_selector("#save-files")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlButtonElement>()
    .unwrap()
    .into();
```

and write

```rust
initialize_elements!(
    html: "index.html", elements: {
        let work_area = Single("#work-area");
        let layer_list_div = Single("#layer-list");   
        let save_files_btn = Single("#save-files", RcT);
    }
);
```

instead.

If you want to have `Rc<Window>` or `Rc<Documemt>`, or maybe hide them afterwards,
you can do so by supplying optional `opts` argument:

```rust
initialize_elements!(
    html: "index.html",
    elements: {
        let work_area = Single("#work-area");
        let layer_list_div = Single("#layer-list");   
        let save_files_btn = Single("#save-files", RcT);
    },
    opts: {
        window_ret_ty: Some(RcT),
        document_ret_ty: None
    }
);
```

If you want to invoke [`Document::query_selector_all`], you can use `Multi`
selector query, though this functionality hasn't been tested thoroughly.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>


[`Document::query_selector_all`]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html#method.query_selector_all
