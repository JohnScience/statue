use super::{SelectorSyntax, SelectorKind};

pub(crate) struct AnonSelector {
    pub(crate) kind: SelectorKind,
    pub(crate) syn: SelectorSyntax,
}
