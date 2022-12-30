use proc_macro::{TokenStream, token_stream::IntoIter as TokenTreeIter, TokenTree};

pub(crate) mod selector;
mod parse;

use parse::Parse;

use crate::selector::SelectorBraceGroupParser;

const ERROR_MSG_ARGS: &str = r#"
Expected arguments: `html: "<path>"` and `elements: {
    let sel1 = Single("\#my-id");
    let sel2 = Multi("div > p");
    ...
}` in precisely this order"#;

const ERROR_MSG_PATH: &str = "Path is expected to be a string literal";

#[proc_macro]
pub fn apply_selectors(ts: TokenStream) -> TokenStream {
    let mut iter: TokenTreeIter = ts.into_iter();
    let Some(html) = iter.next() else {
        panic!("{ERROR_MSG_ARGS}");
    };
    if !(matches!(html, TokenTree::Ident(html) if html.to_string() == "html")) {
        panic!("Named argument `html` is expected. {ERROR_MSG_ARGS}");
    };

    let Some(colon) = iter.next() else {
        panic!("Colon is expected. {ERROR_MSG_ARGS}");
    };
    if !(matches!(colon, TokenTree::Punct(colon) if colon.to_string() == ":")) {
        panic!("Colon is expected. {ERROR_MSG_ARGS}");
    };

    let Some(html_path) = iter.next() else {
        panic!("String literal HTML path is expected. {ERROR_MSG_ARGS}");
    };

    let TokenTree::Literal(html_path) = html_path else {
        panic!("String literal HTML path is expected. {ERROR_MSG_PATH}");
    };

    let path = html_path.to_string();
    let html_path = path.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap();

    let Some(comma) = iter.next() else {
        panic!("Comma is expected. {ERROR_MSG_ARGS}");
    };

    if !(matches!(comma, TokenTree::Punct(comma) if comma.to_string() == ",")) {
        panic!("Comma is expected. {ERROR_MSG_ARGS}");
    };

    let Some(elements_arg_name) = iter.next() else {
        panic!("Named argument `elements` is expected. {ERROR_MSG_ARGS}");
    };

    if !(matches!(elements_arg_name, TokenTree::Ident(selectors_arg_name) if selectors_arg_name.to_string() == "elements")) {
        panic!("Named argument `elements` is expected. {ERROR_MSG_ARGS}");
    };

    let Some(colon) = iter.next() else {
        panic!("{ERROR_MSG_ARGS}");
    };

    if !(matches!(colon, TokenTree::Punct(colon) if colon.to_string() == ":")) {
        panic!("{ERROR_MSG_ARGS}");
    };

    let selectors = SelectorBraceGroupParser::parse(&mut iter).unwrap();
    TokenStream::new()
}
