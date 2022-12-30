use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenTree, Delimiter};

use crate::selectors::{Selectors, SelectorBraceGroupParser};

use super::{Parse, error::selector_brace_group_parser_impl::Error};

impl Parse for SelectorBraceGroupParser {
    type Error = Error;
    type Output = Result<Selectors, Self::Error>;
    fn parse(iter: &mut TokenTreeIter) -> Self::Output {
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
        Selectors::parse(&mut brace_group_contents_iter).map_err(|e| e.into())
    }
}