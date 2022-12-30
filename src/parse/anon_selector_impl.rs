use proc_macro::{TokenTree, token_stream::IntoIter as TokenTreeIter, Delimiter};
use crate::selectors::{SelectorKind, SelectorSyntax, AnonSelector};
use super::{Parse, error::anon_selector_impl::Error};

impl Parse for AnonSelector {
    type Error = Error;
    type Output = Result<Self, Self::Error>;

    fn parse(iter: &mut TokenTreeIter) -> Self::Output {
        let Some(ident) = iter.next() else { return Err(Error::Finished) };
        let kind = match ident.to_string().as_str() {
            "Single" => SelectorKind::Single,
            "Multi" => SelectorKind::Multi,
            _ => return Err(Error::UnknownSelectorKind),
        };

        let Some(paren_group) = iter.next() else { return Err(Error::ParenExpected) };
        let TokenTree::Group(paren_group) = paren_group else { return Err(Error::ParenExpected) };
        if !(matches!(paren_group.delimiter(), Delimiter::Parenthesis)) {
            return Err(Error::ParenExpected);
        };
        let quoteless = paren_group.stream()
            .to_string()
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap()
            .to_owned();
        let Some(syn) = SelectorSyntax::new(quoteless) else {
            return Err(Error::InvalidSelectorSyntax);
        };

        Ok(Self { kind, syn})
    }
}