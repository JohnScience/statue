use crate::sel_queries::AnonSelQuery;
use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenTree};

use crate::sel_queries::{ExplKindSelQuery, ImplKindSelQuery};

use super::{error::expl_kind_selector_impl::Error, Parse};

impl Parse for ExplKindSelQuery {
    type Error = Error;
    type OkTy = Self;
    type Wrapper<O, E> = Result<O, E>;

    /// Each selector is expected to take the form
    /// `Single("<selector>")` or `Multi("<selector>")`.
    fn parse(iter: &mut TokenTreeIter) -> Self::Wrapper<Self::OkTy, Self::Error> {
        let Some(let_) = iter.next() else { return Err(Error::Finished) };
        if !(matches!(let_, TokenTree::Ident(let_) if let_.to_string() == "let")) {
            return Err(Error::LetExpected);
        }

        let Some(name) = iter.next() else { return Err(Error::VariableNameExpected) };
        let TokenTree::Ident(ident) = name else { return Err(Error::VariableNameExpected) };
        let name = ident.to_string();

        let Some(eq) = iter.next() else { return Err(Error::EqExpected) };
        if !(matches!(eq, TokenTree::Punct(eq) if eq.as_char() == '=')) {
            return Err(Error::EqExpected);
        };

        let AnonSelQuery { kind, syn, ret_ty } = match AnonSelQuery::parse(iter) {
            Ok(selector) => selector,
            Err(e) => return Err(e.into()),
        };

        let Some(semi) = iter.next() else { return Err(Error::SemiExpected) };
        if !(matches!(semi, TokenTree::Punct(semi) if semi.as_char() == ';')) {
            return Err(Error::SemiExpected);
        };

        Ok(Self {
            kind,
            rest: ImplKindSelQuery { name, syn, ret_ty },
        })
    }
}
