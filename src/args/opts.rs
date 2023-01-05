use proc_macro::{Delimiter, Group, Ident, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::sel_querries::RetTyKind;

pub(crate) struct Opts {
    pub(crate) window_ret_ty: Option<RetTyKind>,
    pub(crate) document_ret_ty: Option<RetTyKind>,
}

impl Opts {
    fn declare_window(&self, ts: &mut TokenStream) {
        let window_ret_ty = self.window_ret_ty.unwrap_or(RetTyKind::T);
        ts.extend([
            TokenTree::Ident(Ident::new("let", Span::call_site())),
            TokenTree::Ident(Ident::new("window", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ]);
        RetTyKind::fmt_as_inferred_ty(ts, &window_ret_ty);
        ts.extend([
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Ident(Ident::new("web_sys", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Ident(Ident::new("window", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("unwrap", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        ]);
        RetTyKind::extend_with_optional_into(ts, &window_ret_ty);
        ts.extend([TokenTree::Punct(Punct::new(';', Spacing::Alone))]);
    }

    fn declare_document(&self, ts: &mut TokenStream) {
        let document_ret_ty = self.document_ret_ty.unwrap_or(RetTyKind::T);
        ts.extend([
            TokenTree::Ident(Ident::new("let", Span::call_site())),
            TokenTree::Ident(Ident::new("document", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ]);
        RetTyKind::fmt_as_inferred_ty(ts, &document_ret_ty);
        ts.extend([
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Ident(Ident::new("window", Span::call_site())),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("document", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("unwrap", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        ]);
        RetTyKind::extend_with_optional_into(ts, &document_ret_ty);
        ts.extend([TokenTree::Punct(Punct::new(';', Spacing::Alone))]);
    }

    pub(super) fn declare_window_and_document(&self, ts: &mut TokenStream) {
        self.declare_window(ts);
        self.declare_document(ts);
    }

    pub(super) fn hide_window_and_document_if_needed(&self, ts: &mut TokenStream) {
        if self.window_ret_ty.is_none() {
            ts.extend([
                TokenTree::Ident(Ident::new("drop", Span::call_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                    let mut nested_ts = TokenStream::new();
                    nested_ts.extend([TokenTree::Ident(Ident::new("window", Span::call_site()))]);
                    nested_ts
                })),
                TokenTree::Punct(Punct::new(';', Spacing::Alone)),
            ]);
        }
        if self.document_ret_ty.is_none() {
            ts.extend([
                TokenTree::Ident(Ident::new("drop", Span::call_site())),
                TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                    let mut nested_ts = TokenStream::new();
                    nested_ts.extend([TokenTree::Ident(Ident::new("document", Span::call_site()))]);
                    nested_ts
                })),
                TokenTree::Punct(Punct::new(';', Spacing::Alone)),
            ]);
        }
    }
}

impl Default for Opts {
    fn default() -> Self {
        Self {
            window_ret_ty: Some(RetTyKind::T),
            document_ret_ty: Some(RetTyKind::T),
        }
    }
}
