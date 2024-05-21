# Opportunities for improvement

* It would be awesome to have a standardized typed wrapper around the [`web_sys::NodeList`](https://rustwasm.github.io/wasm-bindgen/api/web_sys/struct.NodeList.html) type. This would allow the user to iterate over the elements in the list without having to call the `get` method on the list. This would also allow the user to use the `for` loop syntax to iterate over the elements in the list.
* It would be awesome to have a macro that would track the usage of the elements in the HTML file and generate the code in a way that would interleave the querying the elements with their usage. This would improve the interactivity of the pages and the perceived performance of the applications using `statue`.
* It would be amazing to use the structure of the HTML file to query the elements optimally (e.g. perform selector optimization). This would allow the user to write the selectors in an easy way and have the macro generate the optimized code.
* It would be nice to have a way to support a parser with full HTML spec compliance. Even though `tl` is fast and works like a charm, there still can be some edge cases where it might not work as expected.
