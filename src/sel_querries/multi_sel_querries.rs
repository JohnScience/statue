use core::marker::PhantomData;

use tl::VDom;

use crate::elements::{MultipleElements, ElementKind};

use super::ImplKindSelQuerry;

pub(crate) struct MultiSelQuerries(pub(super) Vec<ImplKindSelQuerry>);

impl MultiSelQuerries {
    pub(crate) fn into_elements<'a>(self, dom: &'a VDom) -> Vec<MultipleElements<'a>> {
        let parser = dom.parser();
        self.0
            .into_iter()
            .map(|selector| {
                let mut query_res_iter = dom.query_selector(selector.syn.0.as_str()).expect(
                    format!(
                        "Failed to create an iterator over results of query for \"{:?}\" selector.",
                        selector.syn.0.as_bytes()
                    )
                    .as_str(),
                );
                let handle = query_res_iter
                    .next()
                    .expect("At least one element is expected for Multi selector.");
                let mut count = 1;
                let node = handle.get(parser).expect("Failed to get node from handle.");
                let mut common_kind = ElementKind::new(node.as_tag().expect("Tag expected").name());
                loop {
                    match query_res_iter.next() {
                        Some(handle) => {
                            let node = handle.get(parser).unwrap();
                            let elem_kind = ElementKind::new(node.as_tag().unwrap().name());
                            common_kind = ElementKind::common(&common_kind, &elem_kind);
                            count += 1;
                        }
                        None => break,
                    }
                }
                MultipleElements {
                    count,
                    name: selector.name,
                    syn: selector.syn,
                    common_kind,
                    phantom: PhantomData,
                }
            })
            .collect()
    }
}
