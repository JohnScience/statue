use core::marker::PhantomData;

use tl::VDom;

use crate::elements::{ElementKind, MultipleElements};

use super::ImplKindSelQuery;

pub(crate) struct MultiSelQueries(pub(super) Vec<ImplKindSelQuery>);

impl MultiSelQueries {
    pub(crate) fn into_elements<'a>(self, dom: &'a VDom) -> Vec<MultipleElements<'a>> {
        let parser = dom.parser();
        self.0
            .into_iter()
            .map(|ImplKindSelQuery { name, ret_ty, syn }| {
                let mut query_res_iter = dom.query_selector(syn.0.as_str()).expect(
                    format!(
                        "Failed to create an iterator over results of query for \"{:?}\" selector.",
                        syn.0.as_bytes()
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
                    name,
                    ret_ty,
                    syn,
                    common_kind,
                    phantom: PhantomData,
                }
            })
            .collect()
    }
}
