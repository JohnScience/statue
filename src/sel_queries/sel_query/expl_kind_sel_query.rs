use super::super::{ImplKindSelQuery, SelQueryKind};

pub(crate) struct ExplKindSelQuery {
    pub(crate) kind: SelQueryKind,
    pub(crate) rest: ImplKindSelQuery,
}
