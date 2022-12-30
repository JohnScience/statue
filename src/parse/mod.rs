use proc_macro::token_stream::IntoIter as TokenTreeIter;

mod error;
mod anon_selector_impl;
mod expl_kind_selector_impl;
mod selectors_impl;
mod selector_brace_group_parser_impl;

pub(crate) trait Parse: Sized {
    type Error;
    type Output;
    fn parse(iter: &mut TokenTreeIter) -> Self::Output;
}

