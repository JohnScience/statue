# Hello world without statue

First of all, we will create a simple webpage that displays `"Hello, <name>"` with `wasm_bindgen` **but without `statue`** (and bundler). This will help you understand the basics of Rust and WebAssembly for browsers.

Go to an empty directory:

```console
mkdir wb-hello-world
cd wb-hello-world
```

## wb-hello-world/index.html

In this empty directory, create the `index.html` with the following contents:

```html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type"/>
  </head>
  <body>
    <script type="module">
      import init from './wb-hello-world/pkg/wb_hello_world.js';
      init();
    </script>
    <h1>Hello, friend!</h1>
    <input type="text" placeholder="Enter your name" id="name-input">
  </body>
</html>
```

This is a simple HTML file that has

* An `ES6` module that imports the `hello_world` package and calls the `init` function,
* An `h1` element that in the absence of JS or WASM displays `"Hello, friend!"`,
* An `input` element that allows the user to enter their name.

If we were to run a `tree` command in the `wb-hello-world` directory, we would see the following:

```text
index.html
```

## wb-hello-world/wb-hello-world

Ensure that you have [`wasm-pack`](https://rustwasm.github.io/wasm-pack/) installed:

```console
wasm-pack --version
```

With `wasm-pack` installed, create a new Rust WASM project:

```console
wasm-pack new wb-hello-world
```

If you were to run a `tree` command in the `wb-hello-world` directory again, you would see the following:

```text
index.html
wb-hello-world
    ├───src
    │   ├───lib.rs
    │   └───utils.rs
    ├───Cargo.toml
    .................
    └───tests
```

What matters to us is that the nested `wb-hello-world` directory is a Rust library that we can build into a WASM package.

## wb-hello-world/wb-hello-world/Cargo.toml

Find the `[dependencies]` section in the `Cargo.toml` file of the Rust library:

```toml
[dependencies]
wasm-bindgen = "0.2.84"

# The `console_error_panic_hook` crate provides better debugging of panics by
# logging them with `console.error`. This is great for development, but requires
# all the `std::fmt` and `std::panicking` infrastructure, so isn't great for
# code size when deploying.
console_error_panic_hook = { version = "0.1.7", optional = true }
```

and add more dependencies:

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

Each of them will play a role in our project:

* `wasm-bindgen` will allow us to expose Rust functions to JavaScript.
* `console_error_panic_hook` will provide better debugging of panics, if any occur, by logging them with `console.error`.
* `web-sys` will provide bindings to the Web APIs.

## wb-hello-world/wb-hello-world/src/lib.rs

Replace the contents of the `lib.rs` file with the following:

```rust
mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    set_panic_hook();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let header = document.query_selector("h1").unwrap().unwrap();
    let name_input = document.get_element_by_id("name-input").unwrap()
        .dyn_into::<web_sys::HtmlInputElement>().unwrap();
    
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::InputEvent| {
            let input_element = event.target().unwrap().dyn_into::<web_sys::HtmlInputElement>().unwrap();
            let name = input_element.value();
            header.set_inner_html(&format!("Hello, {name}!")); 
        });
        name_input.add_event_listener_with_callback("input", closure.as_ref().unchecked_ref()).unwrap();
        closure.forget();
    }
}
```

This code does the following:

* Adds `wb-hello-world/wb-hello-world/src/utils.rs` as a module called `utils`.
* Imports the `set_panic_hook` function from the `utils` module.
* Imports the [`wasm_bindgen` prelude](https://rustwasm.github.io/wasm-bindgen/api/wasm_bindgen/prelude/index.html).
* Defines a `start` function that will be called when the WASM module is loaded.

The `start` function does the following:

* Calls the `set_panic_hook` function to set up better debugging of panics.
* Gets the `window` and `document` objects from the Web APIs.
* Queries the `h1` element and the `name-input` element. Since `web_sys` doesn't have a dedicated type for `h1` elements, we keep the type of the `header` variable as [`web_sys::Element`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.Element.html).
* Creates a closure that is then added as a listener to the `input` event on the `name_input` element. The closure updates the `h1` element with the text `"Hello, <name>!"` where `<name>` is the value of the `name_input` element. This pattern is described in the [official `wasm-bindgen` guide](https://rustwasm.github.io/docs/wasm-bindgen/examples/closures.html).

## Build the project

Once you have the `Cargo.toml` and `lib.rs` files set up, build the project:

```console
cd wb-hello-world
wasm-pack build --target web
cd ..
```

This will create a `pkg` directory in the `wb-hello-world` directory, and the `pkg` directory will contain the `wb_hello_world.js` file, among others.

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
