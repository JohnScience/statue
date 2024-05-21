# The html field

The `html` field in the invocation of the `statue::initialize_elements` macro like this one:

```rust
initialize_elements!(
    html: "../index.html",
    elements: {
        let header = Single("h1");
        let name_input = Single("#name-input");
    }
);
```

is always a [**string literal**](https://doc.rust-lang.org/reference/expressions/literal-expr.html#string-literal-expressions) like `"../index.html"`.

It cannot be a constant or a variable because procedural macros have access only to the tokens they receive as input and not to the compile-time or, moreover, runtime information.

The path in the `html` field is resolved **relative to the [`CARGO_MANIFEST_DIR`](https://doc.rust-lang.org/cargo/reference/environment-variables.html)**. This is the case because [`proc_macro::Span::source_file`](https://doc.rust-lang.org/proc_macro/struct.Span.html#method.source_file) is not available on stable Rust.

*TODO: Document the interaction of the workspaces and the resolution of the path.*
