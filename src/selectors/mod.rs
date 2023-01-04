use tl::VDom;

use crate::elements::{Elements, SingleElement};

mod selector_querry_kind;
mod selector_syntax;
mod sel_querry;
mod single_sel_querries;
mod multi_sel_querries;

pub(crate) use selector_querry_kind::SelQuerryKind;
pub(crate) use selector_syntax::SelSyntax;
pub(crate) use sel_querry::ImplKindSelQuerry;
pub(crate) use sel_querry::AnonSelQuerry;
pub(crate) use sel_querry::ExplKindSelQuerry;
pub(crate) use single_sel_querries::SingleSelQuerries;
pub(crate) use multi_sel_querries::MultiSelQuerries;

pub(crate) struct SelQuerries {
    single: SingleSelQuerries,
    multi: MultiSelQuerries,
}

pub(crate) struct SelQuerryBraceGroupParser;

impl SelQuerries {
    pub(crate) fn new() -> Self {
        Self {
            single: SingleSelQuerries(Vec::new()),
            multi: MultiSelQuerries(Vec::new()),
        }
    }

    pub(crate) fn push(&mut self, selector: ExplKindSelQuerry) {
        let vec: &mut Vec<ImplKindSelQuerry> = match selector.kind {
            SelQuerryKind::Single => &mut self.single.0,
            SelQuerryKind::Multi => &mut self.multi.0,
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
