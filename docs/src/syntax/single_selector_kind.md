# Single selector kind

The `Single` selector kind is used to specify that the element is unique and its type will be known at compile time.

For example, the macro invocation below:

```rust
initialize_elements!(
    html: "../index.html",
    elements: {
        let header = Single("h1");
        let name_input = Single("#name-input");
    }
);
```

Will expand to the following code:

```rust
let window = web_sys::window().unwrap();
let document = window.document().unwrap();

let header: web_sys::HtmlElement = document
    .query_selector("h1")
    .unwrap()
    .unwrap()
    .dyn_into::<::web_sys::HtmlElement>()
    .unwrap();
let name_input: web_sys::HtmlInputElement = document
    .query_selector("#name-input")
    .unwrap()
    .unwrap()
    .dyn_into::<::web_sys::HtmlInputElement>()
    .unwrap();
```

assuming that the element with id `name-input` is an `input` element. The type of the header variable is `web_sys::HtmlElement` because there's no dedicated type for `h1` elements in `web_sys`.

## Arguments

The `Single` selector kind can also be seen a function where

* the first argument (**mandatory**) is the selector itself and
* the second argument (**optional**) is the "return type kind".

For example, the macro invocation below:

```rust
initialize_elements!(
    html: "index.html", elements: {
        let save_files_btn = Single("#save-files", RcT);
    }
);
```

will expand to the following code:

```rust
let window = web_sys::window().unwrap();
let document = window.document().unwrap();

let save_files_btn: std::rc::Rc<web_sys::HtmlButtonElement> = document
    .query_selector("#save-files")
    .unwrap()
    .unwrap()
    .dyn_into::<web_sys::HtmlButtonElement>()
    .unwrap()
    .into();
```

assuming that the element with id `save-files` is a `button` element.

You can learn more about return type kinds in the dedicated section.
