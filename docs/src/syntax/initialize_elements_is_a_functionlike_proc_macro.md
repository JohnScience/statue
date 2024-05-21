# The syntax of initialize_elements macro

First of all, `statue::initialize_elements` is a [function-like procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros).

## Delimiters

Since `statue::initialize_elements` is a [function-like procedural macro](https://doc.rust-lang.org/reference/procedural-macros.html#function-like-procedural-macros), it can technically be used in any of the following ways:

* With parentheses (❌ not recommended):

```rust
initialize_elements!(/*tokens*/);
```

* With square brackets (❌ not recommended):

```rust
initialize_elements![/*tokens*/];
```

* With curly braces (✔️ recommended):

```rust
initialize_elements!{/*tokens*/};
```

However, since the tokens in its invocations resemble an object, **it's recommended to use curly braces**.

## Tokens

Any function-like proc macro is a function that takes a stream of tokens ([`proc_macro::TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html)) as input and produces a stream of tokens ([`proc_macro::TokenStream`](https://doc.rust-lang.org/proc_macro/struct.TokenStream.html)) as output.

```rust
#[proc_macro]
fn initialize_elements(tokens: TokenStream) -> TokenStream {
    // ...
}
```

The proc macro cannot get access to the type information of the variables declared in the invocation of the macro. Therefore, the macro must be able to infer the information from the tokens it receives, and - possibly - from the context in which it is invoked. However, the proc macros currently have limited access to the information about the context in which they are invoked.

The structure of the accepted tokens will be discussed [in the next subsection](./structure_of_the_accepted_tokens.md).
