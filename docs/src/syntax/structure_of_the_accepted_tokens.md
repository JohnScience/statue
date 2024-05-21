# Structure of the accepted tokens

The tokens accepted by the `statue::initialize_elements` macro can be seen as an object with the following fields:

* `html` (**mandatory**) - a string literal representing the path to the HTML file, **relative to the [`CARGO_MANIFEST_DIR`](https://doc.rust-lang.org/cargo/reference/environment-variables.html)**.
* `elements` (**mandatory**) - a sequence of tokens resembling a [block expression](https://doc.rust-lang.org/reference/expressions/block-expr.html).
* `opts` (**optional**) - a sequence of tokens resembling an object.

For example:

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

For ease of parsing,

* **all of the fields must appear in the same order as shown above**. The `html` field must be the first one, the `elements` field must be the second one, and the `opts` field, if present, must be the last one.
* **trailing commas are not allowed**.
