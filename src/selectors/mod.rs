use tl::VDom;

use crate::elements::{Elements, SingleElement};

mod selector_kind;
mod selector_syntax;
mod impl_kind_selector;
mod anon_selector;
mod expl_kind_selector;
mod single_selectors;
mod multi_selectors;

pub(crate) use selector_kind::SelectorKind;
pub(crate) use selector_syntax::SelectorSyntax;
pub(crate) use impl_kind_selector::ImplKindSelector;
pub(crate) use anon_selector::AnonSelector;
pub(crate) use expl_kind_selector::ExplKindSelector;
pub(crate) use single_selectors::SingleSelectors;
pub(crate) use multi_selectors::MultiSelectors;

#[derive(Debug)]
pub(crate) struct Selectors {
    single: SingleSelectors,
    multi: MultiSelectors,
}

pub(crate) struct SelectorBraceGroupParser;

impl Selectors {
    pub(crate) fn new() -> Self {
        Self {
            single: SingleSelectors(Vec::new()),
            multi: MultiSelectors(Vec::new()),
        }
    }

    pub(crate) fn push(&mut self, selector: ExplKindSelector) {
        let vec: &mut Vec<ImplKindSelector> = match selector.kind {
            SelectorKind::Single => &mut self.single.0,
            SelectorKind::Multi => &mut self.multi.0,
        };
        vec.push(selector.rest);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.single.0.is_empty() && self.multi.0.is_empty()
    }

    pub(crate) fn into_elements<'a>(self, dom: &'a VDom) -> Elements<'a> {
        let single = self.single.into_elements(&dom);
        let multiple = self.multi.into_elements(&dom);
        Elements { single, multiple }
    }
}
