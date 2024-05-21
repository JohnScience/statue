# The elements field

The `elements` field in the invocation of the `statue::initialize_elements` macro like this one:

```rust
initialize_elements!(
    html: "../index.html",
    elements: {
        let header = Single("h1");
        let name_input = Single("#name-input");
    }
);
```

is a sequence of tokens resembling a [block expression](https://doc.rust-lang.org/reference/expressions/block-expr.html) like this one:

```rust
{
    let header = Single("h1");
    let name_input = Single("#name-input");
}
```

This block expression is used to define the mapping between the variables in Rust and the individual elements and their groups in the HTML file.

Each line in the block expression must be a variable binding like this one:

```rust
let header = Single("h1");
```

where

* `header` is the name of the variable in Rust,
* `Single` is the kind of the selector, and
* `"h1"` is the [selector](https://www.w3schools.com/cssref/css_selectors.php) itself.

*Note: currently, it's impossible to specify the type for the variable in Rust because it would require parsing a type. And doing so is complicated without `syn`. If you need this feature, the contributions are welcome*.

## Selector kinds

* `Single` - a single element. Specifies that the element is unique and its type will be known at compile time.
* `Multi` - a group of elements. Since [`web_sys::Document::query_selector_all`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Document.html#method.query_selector_all) returns a [`web_sys::NodeList`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.NodeList.html) wrapped in a `Result`, it's impossible to return a canonical type for the group of elements. Therefore, the type of the variable in Rust will be `web_sys::NodeList`.

*Note: internally the macro already knows the common denominator type for the group of elements, yet it's currently impossible to utilize this information in the generated code. Once the ecosystem for rustic wrappers around the Web APIs matures, this feature will be implemented*.
