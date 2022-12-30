use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenTree, Delimiter};

use crate::{selectors::SelectorBraceGroupParser, args::{Args, HtmlPath}};

use super::{Parse, error::args_impl::Error};

impl Parse for Args {
    type Error = Error;
    type Output = Result<Self, Self::Error>;
    fn parse(iter: &mut TokenTreeIter) -> Self::Output {
    let Some(html) = iter.next() else {
        return Err(Error::ArgsExpected);
    };
    if !(matches!(html, TokenTree::Ident(html) if html.to_string() == "html")) {
        return Err(Error::HtmlArgExpected);
    };

    let Some(colon) = iter.next() else {
        return Err(Error::ColonExpected);
    };
    if !(matches!(colon, TokenTree::Punct(colon) if colon.to_string() == ":")) {
        return Err(Error::ColonExpected);
    };

    let Some(html_path) = iter.next() else {
        return Err(Error::StrLitHtmlPathExpected);
    };

    let TokenTree::Literal(html_path) = html_path else {
        return Err(Error::StrLitHtmlPathExpected);
    };

    let path = html_path.to_string();
    let html_path = path.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap();

    let Some(comma) = iter.next() else {
        return Err(Error::CommaExpected);
    };

    if !(matches!(comma, TokenTree::Punct(comma) if comma.to_string() == ",")) {
        return Err(Error::CommaExpected);
    };

    let Some(elements_arg_name) = iter.next() else {
        return Err(Error::ElementsArgExpected);
    };

    if !(matches!(elements_arg_name, TokenTree::Ident(selectors_arg_name) if selectors_arg_name.to_string() == "elements")) {
        return Err(Error::ElementsArgExpected);
    };

    let Some(colon) = iter.next() else {
        return Err(Error::ColonExpected);
    };

    if !(matches!(colon, TokenTree::Punct(colon) if colon.to_string() == ":")) {
        return Err(Error::ColonExpected);
    };

    let selectors = SelectorBraceGroupParser::parse(iter).unwrap();
        Ok(Self { path: HtmlPath(html_path.to_owned()), selectors })
    }
}