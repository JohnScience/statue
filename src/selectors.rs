use core::marker::PhantomData;

use tl::VDom;

use crate::elements::{ElementKind, Elements, MultipleElements, SingleElement};

pub(crate) enum SelectorKind {
    Single,
    Multi,
}

#[derive(Debug)]
pub(crate) struct SelectorSyntax(String);

#[derive(Debug)]
pub(crate) struct ImplKindSelector {
    pub(crate) name: String,
    pub(crate) syn: SelectorSyntax,
}

pub(crate) struct AnonSelector {
    pub(crate) kind: SelectorKind,
    pub(crate) syn: SelectorSyntax,
}

pub(crate) struct ExplKindSelector {
    pub(crate) kind: SelectorKind,
    pub(crate) rest: ImplKindSelector,
}

#[derive(Debug)]
pub(crate) struct SingleSelectors(Vec<ImplKindSelector>);
#[derive(Debug)]
pub(crate) struct MultiSelectors(Vec<ImplKindSelector>);

#[derive(Debug)]
pub(crate) struct Selectors {
    single: SingleSelectors,
    multi: MultiSelectors,
}

pub(crate) struct SelectorBraceGroupParser;

impl SelectorSyntax {
    pub(crate) fn new(cand: String) -> Option<Self> {
        Some(Self(cand))
    }
}

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
                    phantom: PhantomData,
                }
            })
            .collect()
    }
}

impl MultiSelectors {
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
                    common_kind,
                    phantom: PhantomData,
                }
            })
            .collect()
    }
}

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
