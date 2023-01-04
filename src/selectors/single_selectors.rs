use core::marker::PhantomData;

use tl::VDom;

use crate::elements::ElementKind;

use super::{ImplKindSelector, SingleElement};

#[derive(Debug)]
pub(crate) struct SingleSelectors(pub(super) Vec<ImplKindSelector>);

impl SingleSelectors {
    pub(crate) fn into_elements<'a>(self, dom: &'a VDom) -> Vec<SingleElement<'a>> {
        let parser = dom.parser();
        self.0
            .into_iter()
            .map(|selector| {
                let handle = dom
                    .query_selector(selector.syn.0.as_str())
                    .unwrap()
                    .next()
                    .unwrap();
                let node = handle.get(parser).unwrap();
                let elem_kind = ElementKind::new(node.as_tag().unwrap().name());
                SingleElement {
                    name: selector.name,
                    kind: elem_kind,
                    syn: selector.syn,
                    phantom: PhantomData,
                }
            })
            .collect()
    }
}
