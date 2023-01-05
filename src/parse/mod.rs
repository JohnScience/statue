use proc_macro::token_stream::IntoIter as TokenTreeIter;

mod anon_sel_querry_impl;
mod args_impl;
mod comma_seped_parser;
mod error;
mod expl_kind_sel_querry_impl;
mod option_combinator;
mod opts_parser;
mod ret_ty_kind_impl;
mod sel_querries_impl;
mod sel_querry_brace_group_parser_impl;

pub(crate) trait Parse: Sized {
    type Error;
    type OkTy;
    type Wrapper<T, E>;

    fn parse(iter: &mut TokenTreeIter) -> Self::Wrapper<Self::OkTy, Self::Error>;
}
