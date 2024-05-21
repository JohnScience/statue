use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenTree};

use crate::{
    args::{Args, HtmlPath, Opts},
    sel_queries::SelQueryBraceGroupParser,
};

use super::{
    comma_seped_parser::CommaSepedParser, error::args_impl::Error, opts_parser::OptsParser, Parse,
};

impl Parse for Args {
    type Error = Error;
    type OkTy = Self;
    type Wrapper<T, E> = Result<T, E>;

    fn parse(iter: &mut TokenTreeIter) -> Result<Self::OkTy, Self::Error> {
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

        if !(matches!(elements_arg_name, TokenTree::Ident(selectors_arg_name) if selectors_arg_name.to_string() == "elements"))
        {
            return Err(Error::ElementsArgExpected);
        };

        let Some(colon) = iter.next() else {
        return Err(Error::ColonExpected);
    };

        if !(matches!(colon, TokenTree::Punct(colon) if colon.to_string() == ":")) {
            return Err(Error::ColonExpected);
        };

        let selectors = match SelQueryBraceGroupParser::parse(iter) {
            Ok(selectors) => selectors,
            Err(e) => return Err(e.into()),
        };
        let opts = CommaSepedParser::<OptsParser>::parse(iter)
            .unwrap_or(Ok(Opts::default()))
            // TODO: replace with returning an error variant
            .unwrap();
        Ok(Self {
            path: HtmlPath(html_path.to_owned()),
            sel_queries: selectors,
            opts,
        })
    }
}
