use tl::VDom;

use crate::elements::{Elements, SingleElement};

mod multi_sel_queries;
mod ret_ty_kind;
mod sel_query;
mod sel_query_kind;
mod sel_syntax;
mod single_sel_queries;

pub(crate) use multi_sel_queries::MultiSelQueries;
pub(crate) use ret_ty_kind::RetTyKind;
pub(crate) use sel_query::AnonSelQuery;
pub(crate) use sel_query::ExplKindSelQuery;
pub(crate) use sel_query::ImplKindSelQuery;
pub(crate) use sel_query_kind::SelQueryKind;
pub(crate) use sel_syntax::SelSyntax;
pub(crate) use single_sel_queries::SingleSelQueries;

pub(crate) struct SelQueries {
    single: SingleSelQueries,
    multi: MultiSelQueries,
}

pub(crate) struct SelQueryBraceGroupParser;

impl SelQueries {
    pub(crate) fn new() -> Self {
        Self {
            single: SingleSelQueries(Vec::new()),
            multi: MultiSelQueries(Vec::new()),
        }
    }

    pub(crate) fn push(&mut self, selector: ExplKindSelQuery) {
        let vec: &mut Vec<ImplKindSelQuery> = match selector.kind {
            SelQueryKind::Single => &mut self.single.0,
            SelQueryKind::Multi => &mut self.multi.0,
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
