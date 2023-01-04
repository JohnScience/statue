#[derive(Debug)]
pub(crate) struct SelectorSyntax(pub(crate) String);

impl SelectorSyntax {
    pub(crate) fn new(cand: String) -> Option<Self> {
        Some(Self(cand))
    }
}