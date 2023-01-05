use core::marker::PhantomData;

use tl::Bytes;

use crate::sel_queries::{RetTyKind, SelSyntax};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ElementKind {
    HtmlElement,
    HtmlDivElement,
    HtmlImageElement,
    HtmlButtonElement,
    HtmlCanvasElement,
}

pub(crate) struct SingleElement<'a> {
    pub(crate) name: String,
    pub(crate) kind: ElementKind,
    pub(crate) ret_ty: RetTyKind,
    pub(crate) syn: SelSyntax,
    pub(crate) phantom: PhantomData<&'a ()>,
}

pub(crate) struct MultipleElements<'a> {
    pub(crate) name: String,
    pub(crate) syn: SelSyntax,
    pub(crate) ret_ty: RetTyKind,
    pub(crate) count: usize,
    pub(crate) common_kind: ElementKind,
    pub(crate) phantom: PhantomData<&'a ()>,
}

pub(crate) struct Elements<'a> {
    pub(crate) single: Vec<SingleElement<'a>>,
    pub(crate) multiple: Vec<MultipleElements<'a>>,
}

impl ElementKind {
    pub(crate) fn new(name: &Bytes) -> Self {
        if name == "div" {
            Self::HtmlDivElement
        } else if name == "img" {
            Self::HtmlImageElement
        } else if name == "button" {
            Self::HtmlButtonElement
        } else if name == "canvas" {
            Self::HtmlCanvasElement
        } else {
            Self::HtmlElement
        }
    }

    pub(crate) fn common(first: &Self, second: &Self) -> Self {
        if first == second {
            first.clone()
        } else {
            Self::HtmlElement
        }
    }

    pub(crate) fn to_web_sys_name(&self) -> &'static str {
        match self {
            Self::HtmlElement => "HtmlElement",
            Self::HtmlDivElement => "HtmlDivElement",
            Self::HtmlImageElement => "HtmlImageElement",
            Self::HtmlButtonElement => "HtmlButtonElement",
            Self::HtmlCanvasElement => "HtmlCanvasElement",
        }
    }
}
