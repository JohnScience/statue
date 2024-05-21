# "Hello world" with statue

Using ["Hello world" without statue](./hello_world_without_statue.md) as a starting point, we can update it to use `statue` for querying selectors.

## wb-hello-world/wb-hello-world/Cargo.toml

Find the dependencies in the `Cargo.toml` file:

```toml
[dependencies]
wasm-bindgen = "0.2.84"
console_error_panic_hook = { version = "0.1.7", optional = true }

[dependencies.web-sys]
version = "0.3.4"
features = [
  'Document',
  'Element',
  'HtmlElement',
  'HtmlInputElement',
  'Window',
  'EventTarget',
  'InputEvent'
]
```

And add the `statue` dependency:

```toml
[dependencies]
wasm-bindgen = "0.2.84"
console_error_panic_hook = { version = "0.1.7", optional = true }
statue = "0.3"

[dependencies.web-sys]
version = "0.3.4"
features = [
  'Document',
  'Element',
  'HtmlElement',
  'HtmlInputElement',
  'Window',
  'EventTarget',
  'InputEvent'
]
```

## wb-hello-world/wb-hello-world/src/lib.rs

Replace the initialization of the elements via the manual querying of selectors and casting to the desired types:

```rust
let window = web_sys::window().unwrap();
let document = window.document().unwrap();

let header = document.query_selector("h1").unwrap().unwrap();
let name_input = document.get_element_by_id("name-input").unwrap()
    .dyn_into::<web_sys::HtmlInputElement>().unwrap();
```

to the invocation of the `statue::initialize_elements` macro:

```rust
initialize_elements!{
    html: "../index.html",
    elements: {
        let header = Single("h1");
        let name_input = Single("#name-input");
    }
};
```

*Note: the various aspects of the syntax of the macro will be discussed in the [`Syntax`](../syntax/index.md) section of the guide*.

Also, update the `use` statements from this:

```rust
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
```

to this:

```rust
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;
use statue::initialize_elements;
```

## Build the project

Once you have the `Cargo.toml` and `lib.rs` files set up, build the project:

```console
cd wb-hello-world
wasm-pack build --target web
cd ..
```

This will create a `pkg` directory in the nested `wb-hello-world` directory, and the `pkg` directory will contain the `wb_hello_world.js` file, among others.

## Run the project

> **Note**: You cannot directly open `index.html` in your web browser due to [CORS][cors]
> limitations. Instead, you can set up a quick development environment using
> Python's built-in HTTP server:
>
> ```console
> wasm-pack build --target web
> python3 -m http.server 8080
> ```
>
> If you don't have Python installed, you can also use [miniserve][miniserve] which
> is installable via Cargo:
>
> ```console
> cargo install miniserve
> miniserve . --index "index.html" -p 8080
> ```

[cors]: https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS
[miniserve]: https://crates.io/crates/miniserve

## About the advantages of using `statue`

Even in this simple example, using `statue` has some benefits:

* You can enjoy the benefits of using contextual suggestions and type inference in your IDE, since the `statue` macro invocation expands to strongly-typed expressions with inferred types.
* If you make a typo in the selector, the `statue` macro invocation will fail to compile, which will help you catch the errors at compile time.
* If there are breaking changes in `index.html`, the `statue` macro invocation will fail to compile, which will help you catch the errors early.
* You can use the familiar CSS selector syntax to query elements instead of picking the right method from the `web_sys` API.
