use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::elements::ElementKind;

pub(crate) enum RetTyKind {
    T,
    RcT,
}

impl RetTyKind {
    fn fmt_as_ty(ts: &mut TokenStream, ret_ty: &RetTyKind, ident: &str) {
        match ret_ty {
            RetTyKind::T => ts.extend([TokenTree::Ident(Ident::new(ident, Span::call_site()))]),
            RetTyKind::RcT => {
                ts.extend([
                    TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                    TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                    TokenTree::Ident(Ident::new("std", Span::call_site())),
                    TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                    TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                    TokenTree::Ident(Ident::new("rc", Span::call_site())),
                    TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                    TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                    TokenTree::Ident(Ident::new("Rc", Span::call_site())),
                    TokenTree::Punct(proc_macro::Punct::new('<', proc_macro::Spacing::Alone)),
                    TokenTree::Ident(Ident::new(ident, Span::call_site())),
                    TokenTree::Punct(proc_macro::Punct::new('>', proc_macro::Spacing::Alone)),
                ]);
            }
        }
    }

    pub(crate) fn fmt_as_concrete_ty(
        ts: &mut TokenStream,
        ret_ty: &RetTyKind,
        elem_kind: &ElementKind,
    ) {
        RetTyKind::fmt_as_ty(ts, ret_ty, elem_kind.to_web_sys_name())
    }

    pub(crate) fn fmt_as_inferred_ty(ts: &mut TokenStream, ret_ty: &RetTyKind) {
        match ret_ty {
            RetTyKind::T => ts.extend([TokenTree::Ident(Ident::new("_", Span::call_site()))]),
            RetTyKind::RcT => ts.extend([
                TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                TokenTree::Ident(Ident::new("std", Span::call_site())),
                TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                TokenTree::Ident(Ident::new("rc", Span::call_site())),
                TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                TokenTree::Punct(proc_macro::Punct::new(':', proc_macro::Spacing::Joint)),
                TokenTree::Ident(Ident::new("Rc", Span::call_site())),
                TokenTree::Punct(proc_macro::Punct::new('<', proc_macro::Spacing::Alone)),
                TokenTree::Ident(Ident::new("_", Span::call_site())),
                TokenTree::Punct(proc_macro::Punct::new('>', proc_macro::Spacing::Alone)),
            ]),
        }
    }

    pub(crate) fn extend_with_optional_into(ts: &mut TokenStream, ret_ty: &RetTyKind) {
        match ret_ty {
            RetTyKind::T => {}
            RetTyKind::RcT => ts.extend([
                TokenTree::Punct(Punct::new('.', Spacing::Alone)),
                TokenTree::Ident(Ident::new("into", Span::call_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            ]),
        }
    }
}
