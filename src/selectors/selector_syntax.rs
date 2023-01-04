#[derive(Debug)]
pub(crate) struct SelSyntax(pub(crate) String);

impl SelSyntax {
    pub(crate) fn new(cand: String) -> Option<Self> {
        Some(Self(cand))
    }
}