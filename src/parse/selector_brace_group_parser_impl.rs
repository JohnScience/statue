use proc_macro::{token_stream::IntoIter as TokenTreeIter, Delimiter, TokenTree};

use crate::selectors::{SelQuerryBraceGroupParser, SelQuerries};

use super::{error::selector_brace_group_parser_impl::Error, Parse};

impl Parse for SelQuerryBraceGroupParser {
    type Error = Error;
    type Output = Result<SelQuerries, Self::Error>;
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
        SelQuerries::parse(&mut brace_group_contents_iter).map_err(|e| e.into())
    }
}
