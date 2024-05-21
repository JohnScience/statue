# Statue

> Easier way to query selectors for static HTML pages.

[![Crates.io](https://img.shields.io/crates/v/statue)](https://crates.io/crates/statue)
[![Docs.rs](https://docs.rs/statue/badge.svg)](https://docs.rs/statue)
[![License](https://img.shields.io/crates/l/statue)](https://crates.io/crates/statue)

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

## Note on optimization

Eventually, it can be possible to just traverse the tree of HTML nodes to get
the elements chosen by selectors. However, this is not implemented yet.

## Note on (in-)completeness of implementation

The implementation is **incomplete**. For example, many elements of [`web_sys`]
are not yet supported. One easy way to add support for more elements is to edit
`src/elements.rs` and add the element to the `ElementKind` enum. Then, edit `ElementKind::new`
method to return the element kind you added. Finally, edit `ElementKind::to_web_sys_name`.

## Note on HTML spec compliance

Internally, the crate uses [`tl`] to parse [`HTML`] and [`CSS`] selectors. This gives
us extra speed and flexibility. However, the crate is not fully compliant with
the HTML spec.

## Note on perceived performance

On its own, `initialize_elements!` macro leads to a minor increase in initial
load time because WASM script will first execute selector queries first and only
after that all the truly useful code will execute.

Eventually, it can be possible to befriend `initialize_elements!` with some
attribute macro, that would rearrange the code so that selector queries would
get executed exactly when they are needed.

## Note on quality of error messages

Do not expect the error messages to be very helpful. Expect
strict order of arguments and little tolerance to random or missing
commas/semis.

## File structure

- `src/`: Source code.
- `target/`: Compiled code (generated).
- `.gitingore`: [Git ignore file].
- `.markdownlint.json`: Configuration file for Markdown linting, used by
[`DavidAnson.vscode-markdownlint`] Visual Studio Code extension.
- `Cargo.toml` and `Cargo.lock` : Manifest and machine-generated list of
dependencies. [Learn more](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html).
- `LICENSE-APACHE`: Apache License, Version 2.0.
- `LICENSE-MIT`: MIT License.
- `README.md`: This file.

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

[`web_sys`]: https://docs.rs/web-sys/latest/web_sys/index.html
[Git ignore file]: https://git-scm.com/docs/gitignore
[`DavidAnson.vscode-markdownlint`]: https://marketplace.visualstudio.com/items?itemName=DavidAnson.vscode-markdownlint
[`HTML`]: https://html.spec.whatwg.org/multipage/
[`CSS`]: https://www.w3.org/TR/css-syntax-3/
[`tl`]: https://crates.io/crates/tl
[`Document::query_selector_all`]: https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html#method.query_selector_all
