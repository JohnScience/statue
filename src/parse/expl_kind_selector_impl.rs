use crate::selectors::AnonSelector;
use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenTree};

use crate::selectors::{ExplKindSelector, ImplKindSelector};

use super::{error::expl_kind_selector_impl::Error, Parse};

impl Parse for ExplKindSelector {
    type Error = Error;
    type Output = Result<Self, Self::Error>;
    /// Each selector is expected to take the form
    /// `Single("<selector>")` or `Multi("<selector>")`.
    fn parse(iter: &mut TokenTreeIter) -> Self::Output {
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

        let AnonSelector { kind, syn } = match AnonSelector::parse(iter) {
            Ok(selector) => selector,
            Err(e) => return Err(e.into()),
        };

        let Some(semi) = iter.next() else { return Err(Error::SemiExpected) };
        if !(matches!(semi, TokenTree::Punct(semi) if semi.as_char() == ';')) {
            return Err(Error::SemiExpected);
        };

        Ok(Self {
            kind,
            rest: ImplKindSelector { name, syn },
        })
    }
}
