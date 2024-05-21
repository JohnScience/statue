# Return type kinds

The "return type kind" is a concept that allows you to specify the type of the variable in Rust that will hold the result of the invocation of the `query_selector` or `query_selector_all` method.

Sometimes, you want to reference the elements in multiple places in your code - for example, when you want to reference the same element in various event listeners. In such cases, you can use the `RcT` return type kind.

*TODO: add an example of using the `RcT` return type kind*

Currently, there are two return type kinds available:

* `RcT` - a reference-counted pointer to the element. Wraps the element in an `Rc<T>` where `T` is the type of the element.
* `T` - the element itself.
