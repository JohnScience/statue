pub(crate) enum SelectorKind {
    Single,
    Multi
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
}