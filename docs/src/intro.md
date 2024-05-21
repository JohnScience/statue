# Introduction

This book is about [`statue`](https://crates.io/crates/statue), a Rust library that intends to provide a convenient way to query selectors for static HTML pages. The `statue` library is a part of the [Rust and WebAssembly ecosystem](https://rustwasm.github.io/) and is designed to be used in the browsers.

* To learn more about CSS selectors, see [this](https://www.w3schools.com/css/css_selectors.asp).
* To learn more about querying selectors in JavaScript, see the docs for [`document.querySelector`](https://www.w3schools.com/jsref/met_document_queryselector.asp) and [`document.querySelectorAll`](https://www.w3schools.com/jsref/met_document_queryselectorall.asp).

## Place in the Rust frontend ecosystem

The `statue` library is meant to be used together with...

* [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen) for interoperability between Rust and JavaScript.
* [`js-sys`](https://crates.io/crates/js-sys) for using JavaScript APIs and types, such as [`js_sys::Reflect::get`](https://rustwasm.github.io/wasm-bindgen/api/js_sys/Reflect/fn.get.html) (in JavaScript, `target[propertyKey]`), from Rust.
* [`web-sys`](https://crates.io/crates/web-sys) for using browser APIs and types, such as `HtmlElement` ( [docs.rs](https://docs.rs/web-sys/latest/web_sys/struct.HtmlElement.html) | [MDN](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement) ), from Rust.
* [`wasm-pack`](https://rustwasm.github.io/docs/wasm-pack/) for building and working with rust-generated WebAssembly packages.

If you're unfamiliar with `wasm-bindgen`, take a look at their [official guide](https://rustwasm.github.io/docs/wasm-bindgen/).

Unlike [`yew`](https://yew.rs/) or [`leptos`](https://leptos.dev/), `statue` does not aim to provide a full-fledged frontend framework. Instead, it focuses on a specific task: querying selectors in static HTML pages.

This makes `statue` a good fit for Vanilla-style, as opposed to framework-style, web development. See ["Vanilla JavaScript Vs. JavaScript Frameworks: Ten Top Differences"](https://blog.openreplay.com/vanilla-javascript-vs-javascript-frameworks/) article by Nwakor Chidinma Favour, if you want to learn more.

## Features

* **Compile-time checks**. The `statue` library harnesses the power of Rust's procedural macros to provide compile-time checks for selectors. You can ensure that your selectors are expressed using valid syntax and point to valid element(s) before running your code.
* **HTML element type inference**. The `statue` library infers the types of the HTML elements that are pointed to by selectors based on the statically-known HTML. If the element type is currently not supported, it is inferred as [`web_sys::HtmlElement`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.HtmlElement.html), which you may then cast to a more specific type yourself.
* **Quick compile-time**. Even though procedural macros are generally slower to compile than regular Rust code, the `statue` library is designed to compile quickly. It intentionally does not depend on `proc_macro2`, `syn`, and `quote` traid of crates, which are known to slow down compilation times. Additionally, `statue` uses [`tl`](https://crates.io/crates/tl), which is fast and whose tradoffs between speed and applicability are a perfect fit for `statue`.
