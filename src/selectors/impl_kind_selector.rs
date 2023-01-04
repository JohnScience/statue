use super::SelectorSyntax;

#[derive(Debug)]
pub(crate) struct ImplKindSelector {
    pub(crate) name: String,
    pub(crate) syn: SelectorSyntax,
}
