use proc_macro::{token_stream::IntoIter as TokenTreeIter, Delimiter, TokenTree};

use crate::sel_querries::{SelQuerries, SelQuerryBraceGroupParser};

use super::{error::selector_brace_group_parser_impl::Error, Parse};

impl Parse for SelQuerryBraceGroupParser {
    type Error = Error;
    type OkTy = SelQuerries;
    type Wrapper<T, E> = Result<T, E>;

    fn parse(iter: &mut TokenTreeIter) -> Self::Wrapper<Self::OkTy, Self::Error> {
        let Some(brace_group) = iter.next() else {
            return Err(Error::BraceGroupExpected);
        };
        let TokenTree::Group(brace_group) = brace_group else {
            return Err(Error::BraceGroupExpected);
        };
        if brace_group.delimiter() != Delimiter::Brace {
            return Err(Error::BraceGroupExpected);
        };
        let mut brace_group_contents_iter = brace_group.stream().into_iter();
        SelQuerries::parse(&mut brace_group_contents_iter).map_err(|e| e.into())
    }
}
