use super::super::{SelQuerryKind, ImplKindSelQuerry};

pub(crate) struct ExplKindSelQuerry {
    pub(crate) kind: SelQuerryKind,
    pub(crate) rest: ImplKindSelQuerry,
}
