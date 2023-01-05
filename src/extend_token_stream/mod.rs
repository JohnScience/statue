use proc_macro::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};

use crate::{
    elements::{Elements, MultipleElements, SingleElement},
    sel_querries::RetTyKind,
};

pub(crate) trait ExtendTokenStream {
    fn extend_token_stream(self, ts: &mut TokenStream);
}

impl<'a> ExtendTokenStream for SingleElement<'a> {
    fn extend_token_stream(self, ts: &mut TokenStream) {
        let Self {
            name,
            kind,
            syn,
            ret_ty,
            phantom: _phantom,
        } = self;
        ts.extend([
            TokenTree::Ident(Ident::new("let", Span::call_site())),
            TokenTree::Ident(Ident::new(name.as_str(), Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ]);
        RetTyKind::fmt_as_concrete_ty(ts, &ret_ty, &kind);
        ts.extend([
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Ident(Ident::new("document", Span::call_site())),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("query_selector", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                let mut ts = TokenStream::new();
                ts.extend([TokenTree::Literal(Literal::string(&syn.0))]);
                ts
            })),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("unwrap", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("unwrap", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("dyn_into", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new('<', Spacing::Alone)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Ident(Ident::new("web_sys", Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Punct(Punct::new(':', Spacing::Joint)),
            TokenTree::Ident(Ident::new(kind.to_web_sys_name(), Span::call_site())),
            TokenTree::Punct(Punct::new('>', Spacing::Alone)),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("unwrap", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        ]);
        RetTyKind::extend_with_optional_into(ts, &ret_ty);
        ts.extend([TokenTree::Punct(Punct::new(';', Spacing::Alone))]);
    }
}

impl<'a> ExtendTokenStream for MultipleElements<'a> {
    fn extend_token_stream(self, ts: &mut TokenStream) {
        let Self {
            name,
            count: _count,
            ret_ty,
            common_kind: _common_kind,
            syn,
            phantom: _phantom,
        } = self;
        ts.extend([
            TokenTree::Ident(Ident::new("let", Span::call_site())),
            TokenTree::Ident(Ident::new(name.as_str(), Span::call_site())),
            TokenTree::Punct(Punct::new(':', Spacing::Alone)),
        ]);
        RetTyKind::fmt_as_inferred_ty(ts, &ret_ty);
        ts.extend([
            TokenTree::Punct(Punct::new('=', Spacing::Alone)),
            TokenTree::Ident(Ident::new("document", Span::call_site())),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("query_selector_all", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, {
                let mut ts = TokenStream::new();
                ts.extend([TokenTree::Literal(Literal::string(&syn.0))]);
                ts
            })),
            TokenTree::Punct(Punct::new('.', Spacing::Alone)),
            TokenTree::Ident(Ident::new("unwrap", Span::call_site())),
            TokenTree::Group(Group::new(Delimiter::Parenthesis, TokenStream::new())),
        ]);
        RetTyKind::extend_with_optional_into(ts, &ret_ty);
        ts.extend([TokenTree::Punct(Punct::new(';', Spacing::Alone))]);
    }
}

impl<'a> ExtendTokenStream for Vec<SingleElement<'a>> {
    fn extend_token_stream(self, ts: &mut TokenStream) {
        for single_element in self {
            single_element.extend_token_stream(ts);
        }
    }
}

impl<'a> ExtendTokenStream for Vec<MultipleElements<'a>> {
    fn extend_token_stream(self, ts: &mut TokenStream) {
        for multiple_elements in self {
            multiple_elements.extend_token_stream(ts);
        }
    }
}

impl<'a> ExtendTokenStream for Elements<'a> {
    fn extend_token_stream(self, ts: &mut TokenStream) {
        let Self { single, multiple } = self;
        single.extend_token_stream(ts);
        multiple.extend_token_stream(ts);
    }
}
