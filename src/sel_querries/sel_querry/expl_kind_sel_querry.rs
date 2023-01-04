use super::super::{ImplKindSelQuerry, SelQuerryKind};

pub(crate) struct ExplKindSelQuerry {
    pub(crate) kind: SelQuerryKind,
    pub(crate) rest: ImplKindSelQuerry,
}
