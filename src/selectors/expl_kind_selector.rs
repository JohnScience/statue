use super::{SelectorKind, ImplKindSelector};

pub(crate) struct ExplKindSelector {
    pub(crate) kind: SelectorKind,
    pub(crate) rest: ImplKindSelector,
}
