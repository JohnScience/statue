use core::marker::PhantomData;

use tl::VDom;

use crate::elements::ElementKind;

use super::{ImplKindSelQuery, SingleElement};

pub(crate) struct SingleSelQueries(pub(super) Vec<ImplKindSelQuery>);

impl SingleSelQueries {
    pub(crate) fn into_elements<'a>(self, dom: &'a VDom) -> Vec<SingleElement<'a>> {
        let parser = dom.parser();
        self.0
            .into_iter()
            .map(|ImplKindSelQuery { name, ret_ty, syn }| {
                let handle = dom.query_selector(syn.0.as_str()).unwrap().next().unwrap();
                let node = handle.get(parser).unwrap();
                let elem_kind = ElementKind::new(node.as_tag().unwrap().name());
                SingleElement {
                    name,
                    kind: elem_kind,
                    syn,
                    ret_ty,
                    phantom: PhantomData,
                }
            })
            .collect()
    }
}
