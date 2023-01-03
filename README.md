# Statue

> Easier way to querry selectors for static HTML pages.

[![Crates.io](https://img.shields.io/crates/v/statue)](https://crates.io/crates/statue)
[![Docs.rs](https://docs.rs/statue/badge.svg)](https://docs.rs/statue)
[![License](https://img.shields.io/crates/l/statue)](https://crates.io/crates/statue)

Spare yourself from writing

```rust
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

let save_files_btn = document
    .query_selector("#save-files")
    .unwrap()
    .unwrap()
    .dyn_into::<HtmlButtonElement>()
    .unwrap();
```

and write

```rust
initialize_elements!(
    html: "index.html", elements: {
        let work_area = Single("#work-area");
        let layer_list_div = Single("#layer-list");   
        let save_files_btn = Single("#save-files");
    }
);
```

instead.

## Note on optimization

Eventually, it can be possible to just traverse the tree of HTML nodes to get
the elements chosen by selectors. However, this is not implemented yet.

## Note on (in-)completeness of implementation

The implementation is **incomplete**. For example, many elements of [`web_sys`]
are not yet supported. One easy way to add support for more elements is to edit
`src/elements.rs` and add the element to the `ElementKind` enum. Then, edit `new`
method of `ElementKind` to return the element kind you added. Finally, edit `to_web_sys_name`.

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
