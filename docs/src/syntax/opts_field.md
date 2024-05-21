# The opts field

The `opts` field in the invocation of the `statue::initialize_elements` macro like this one:

```rust
initialize_elements!(
    html: "../index.html",
    elements: {
        let header = Single("h1");
        let name_input = Single("#name-input");
    },
    opts: {
        window_ret_ty: Some(RcT),
        document_ret_ty: None
    }
);
```

is always a sequence of tokens resembling an object. It is optional and can be omitted. If present, it must follow after the `elements` field.

If defined, the `opts` field's object must contain both of the following fields:

* `window_ret_ty` - an optional field that can be either (1) `None` or (2) `Some` with the "return type kind" for the `window` variable.
* `document_ret_ty` - an optional field that can be either (1) `None` or (2) `Some` with the "return type kind" for the `document` variable.

You can learn more about return type kinds in the dedicated section.
