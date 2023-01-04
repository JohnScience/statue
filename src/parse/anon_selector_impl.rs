use super::{error::anon_selector_impl::Error, Parse};
use crate::selectors::{AnonSelQuerry, SelQuerryKind, SelSyntax};
use proc_macro::{token_stream::IntoIter as TokenTreeIter, Delimiter, TokenTree};

impl Parse for AnonSelQuerry {
    type Error = Error;
    type Output = Result<Self, Self::Error>;

    fn parse(iter: &mut TokenTreeIter) -> Self::Output {
        let Some(ident) = iter.next() else { return Err(Error::Finished) };
        let kind = match ident.to_string().as_str() {
            "Single" => SelQuerryKind::Single,
            "Multi" => SelQuerryKind::Multi,
            _ => return Err(Error::UnknownSelectorKind),
        };

        let Some(paren_group) = iter.next() else { return Err(Error::ParenExpected) };
        let TokenTree::Group(paren_group) = paren_group else { return Err(Error::ParenExpected) };
        if !(matches!(paren_group.delimiter(), Delimiter::Parenthesis)) {
            return Err(Error::ParenExpected);
        };
        let quoteless = paren_group
            .stream()
            .to_string()
            .strip_prefix("\"")
            .unwrap()
            .strip_suffix("\"")
            .unwrap()
            .to_owned();
        let Some(syn) = SelSyntax::new(quoteless) else {
            return Err(Error::InvalidSelectorSyntax);
        };

        Ok(Self { kind, syn })
    }
}
