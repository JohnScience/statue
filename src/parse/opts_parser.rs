use crate::{args::Opts, sel_querries::RetTyKind};
use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenTree};

use super::{option_combinator::OptionCombinator, Parse};

#[derive(Debug)]
pub(crate) enum Error {
    OptsArgExpected,
    ColonExpected,
    BraceGroupExpected,
    WindowRetTyArgExpected,
    CommaExpected,
    OptionRetTyExpected,
    DocumentRetTyArgExpected,
}

pub(crate) struct OptsParser;

impl Parse for OptsParser {
    type Error = Error;
    type OkTy = Opts;
    type Wrapper<T, E> = Result<T, E>;

    fn parse(iter: &mut TokenTreeIter) -> Self::Wrapper<Self::OkTy, Self::Error> {
        let Some(ident) = iter.next() else { return Err(Error::OptsArgExpected) };
        let TokenTree::Ident(ident) = ident else { return Err(Error::OptsArgExpected) };
        if ident.to_string() != "opts" {
            return Err(Error::OptsArgExpected);
        };
        let Some(colon) = iter.next() else { return Err(Error::ColonExpected) };
        let TokenTree::Punct(colon) = colon else { return Err(Error::ColonExpected) };
        if !(colon.as_char() == ':') {
            return Err(Error::ColonExpected);
        };
        let Some(brace_group) = iter.next() else { return Err(Error::BraceGroupExpected) };
        let TokenTree::Group(brace_group) = brace_group else { return Err(Error::BraceGroupExpected) };
        if brace_group.delimiter() != proc_macro::Delimiter::Brace {
            return Err(Error::BraceGroupExpected);
        };
        let mut brace_group_contents_iter = brace_group.stream().into_iter();
        let Some(window_ret_ty_arg) = brace_group_contents_iter.next() else { return Err(Error::WindowRetTyArgExpected) };
        let TokenTree::Ident(window_ret_ty_arg) = window_ret_ty_arg else { return Err(Error::WindowRetTyArgExpected) };
        if window_ret_ty_arg.to_string() != "window_ret_ty" {
            return Err(Error::WindowRetTyArgExpected);
        };
        let Some(colon) = brace_group_contents_iter.next() else { return Err(Error::ColonExpected) };
        let TokenTree::Punct(colon) = colon else { return Err(Error::ColonExpected) };
        if !(colon.as_char() == ':') {
            return Err(Error::ColonExpected);
        };
        let window_ret_ty = OptionCombinator::<RetTyKind>::parse(&mut brace_group_contents_iter)
            .map_err(|_| Error::OptionRetTyExpected)?;
        let Some(comma) = brace_group_contents_iter.next() else { return Err(Error::CommaExpected) };
        let TokenTree::Punct(comma) = comma else { return Err(Error::CommaExpected) };
        if !(comma.as_char() == ',') {
            return Err(Error::CommaExpected);
        };
        let Some(document_ret_ty_arg) = brace_group_contents_iter.next() else { return Err(Error::DocumentRetTyArgExpected) };
        let TokenTree::Ident(document_ret_ty_arg) = document_ret_ty_arg else { return Err(Error::DocumentRetTyArgExpected) };
        if document_ret_ty_arg.to_string() != "document_ret_ty" {
            return Err(Error::DocumentRetTyArgExpected);
        };
        let Some(colon) = brace_group_contents_iter.next() else { return Err(Error::ColonExpected) };
        let TokenTree::Punct(colon) = colon else { return Err(Error::ColonExpected) };
        if !(colon.as_char() == ':') {
            return Err(Error::ColonExpected);
        };
        let document_ret_ty = OptionCombinator::<RetTyKind>::parse(&mut brace_group_contents_iter)
            .map_err(|_| Error::OptionRetTyExpected)?;
        Ok(Opts {
            window_ret_ty,
            document_ret_ty,
        })
    }
}
