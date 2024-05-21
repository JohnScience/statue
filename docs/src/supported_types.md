# Supported types

At the time of writing, the `statue` library supports the following types of HTML elements:

* `HtmlElement` - the most generic type that represents an HTML element.
* `HtmlDialogElement` - represents a `<dialog>` element.
* `HtmlDivElement` - represents a `<div>` element.
* `HtmlImageElement` - represents an `<img>` element.
* `HtmlButtonElement` - represents a `<button>` element.
* `HtmlSpanElement` - represents a `<span>` element.
* `HtmlCanvasElement` - represents a `<canvas>` element.
* `HtmlInputElement` - represents an `<input>` element.

New types can be added in the future. Feel free to contribute to the project by opening a pull request with the desired type.

> One easy way to add support for more elements is to edit [`src/elements.rs`](https://github.com/JohnScience/statue/blob/main/statue/src/elements.rs).
> There you should
>
> * add the element to the `ElementKind` enum,
>
> ```rust
> #[derive(Debug, Clone, PartialEq)]
> pub(crate) enum ElementKind {
>     HtmlElement,
>     HtmlDialogElement,
>     HtmlDivElement,
>     HtmlImageElement,
>     HtmlButtonElement,
>     HtmlSpanElement,
>     HtmlCanvasElement,
>     HtmlInputElement,
> }
> ```
>
> * edit `ElementKind::new` method to return the element kind you added,
>
> ```rust
> impl ElementKind {
>     pub(crate) fn new(name: &Bytes) -> Self {
>         if name == "div" {
>             Self::HtmlDivElement
>         } else if name == "img" {
>             Self::HtmlImageElement
>         } else if name == "button" {
>             Self::HtmlButtonElement
>         } else if name == "canvas" {
>             Self::HtmlCanvasElement
>         } else if name == "span" {
>             Self::HtmlSpanElement
>         } else if name == "input" {
>             Self::HtmlInputElement
>         } else if name == "dialog" {
>             Self::HtmlDialogElement
>         } else {
>             Self::HtmlElement
>         }
>     }
> 
>     // ...
> }
> ```
>
> * edit `ElementKind::to_web_sys_name`
>
> ```rust
> impl ElementKind {
>     // ...
> 
>     pub(crate) fn to_web_sys_name(&self) -> &'static str {
>         match self {
>             Self::HtmlElement => "HtmlElement",
>             Self::HtmlDialogElement => "HtmlDialogElement",
>             Self::HtmlDivElement => "HtmlDivElement",
>             Self::HtmlImageElement => "HtmlImageElement",
>             Self::HtmlButtonElement => "HtmlButtonElement",
>             Self::HtmlCanvasElement => "HtmlCanvasElement",
>             Self::HtmlSpanElement => "HtmlSpanElement",
>             Self::HtmlInputElement => "HtmlInputElement",
>         }
>     }
> }
> ```
