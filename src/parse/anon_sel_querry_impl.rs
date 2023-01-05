use super::{comma_seped_parser::CommaSepedParser, error::anon_selector_impl::Error, Parse};
use crate::sel_querries::{AnonSelQuerry, RetTyKind, SelQuerryKind, SelSyntax};
use proc_macro::{token_stream::IntoIter as TokenTreeIter, Delimiter, TokenTree};

impl Parse for AnonSelQuerry {
    type Error = Error;
    type OkTy = Self;
    type Wrapper<T, E> = Result<T, E>;

    fn parse(iter: &mut TokenTreeIter) -> Self::Wrapper<Self::OkTy, Self::Error> {
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
        let mut paren_group_contents_iter = paren_group.stream().into_iter();
        let Some(str_lit) = paren_group_contents_iter.next() else {
            // TODO: replace with returning an error variant
            panic!("Expected a string literal");
        };
        let TokenTree::Literal(str_lit) = str_lit else {
            panic!("Expected a string literal");
        };

        let quoteless = str_lit
            .to_string()
            .strip_prefix("\"")
            .expect("Expected a string literal")
            .strip_suffix("\"")
            .expect("Expected a string literal")
            .to_owned();

        let Some(syn) = SelSyntax::new(quoteless) else {
            return Err(Error::InvalidSelectorSyntax);
        };

        let ret_ty = CommaSepedParser::<RetTyKind>::parse(&mut paren_group_contents_iter)
            .unwrap_or(Ok(RetTyKind::T))
            // TODO: replace with returning an error variant
            .expect("Unknown return type kind");

        Ok(Self { kind, syn, ret_ty })
    }
}
