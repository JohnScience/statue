use proc_macro::token_stream::IntoIter as TokenTreeIter;

mod anon_selector_impl;
mod args_impl;
mod error;
mod expl_kind_selector_impl;
mod selector_brace_group_parser_impl;
mod selectors_impl;

pub(crate) trait Parse: Sized {
    type Error;
    type Output;
    fn parse(iter: &mut TokenTreeIter) -> Self::Output;
}
