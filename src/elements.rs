use core::marker::PhantomData;

use tl::Bytes;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ElementKind {
    GenericHTMLElement,
    Div,
    Img,
}

#[derive(Debug)]
pub(crate) struct SingleElement<'a> {
    pub(crate) name: String,
    pub(crate) kind: ElementKind,
    pub(crate) phantom: PhantomData<&'a ()>,
}

#[derive(Debug)]
pub(crate) struct MultipleElements<'a> {
    pub(crate) name: String,
    pub(crate) count: usize,
    pub(crate) common_kind: ElementKind,
    pub(crate) phantom: PhantomData<&'a ()>,
}

#[derive(Debug)]
pub(crate) struct Elements<'a> {
    pub(crate) single: Vec<SingleElement<'a>>,
    pub(crate) multiple: Vec<MultipleElements<'a>>,
}

impl ElementKind {
    pub(crate) fn new(name: &Bytes) -> Self {
        if name == "div" {
            Self::Div
        } else if name == "img" {
            Self::Img
        } else {
            Self::GenericHTMLElement
        }
    }

    pub(crate) fn common(first: &Self, second: &Self) -> Self {
        if first == second {
            first.clone()
        } else {
            Self::GenericHTMLElement
        }
    }
}