use proc_macro::TokenTree;

use crate::sel_queries::RetTyKind;

use super::{error::ret_ty_kind_impl::RetTyKindExpected, Parse};

impl Parse for RetTyKind {
    type Error = RetTyKindExpected;
    type OkTy = Self;
    type Wrapper<O, E> = Result<O, E>;

    fn parse(
        iter: &mut proc_macro::token_stream::IntoIter,
    ) -> Self::Wrapper<Self::OkTy, Self::Error> {
        let Some(ident) = iter.next() else { return Err(RetTyKindExpected) };
        let TokenTree::Ident(ident) = ident else { return Err(RetTyKindExpected) };
        match ident.to_string().as_str() {
            "T" => Ok(Self::T),
            "RcT" => Ok(Self::RcT),
            _ => Err(RetTyKindExpected),
        }
    }
}
