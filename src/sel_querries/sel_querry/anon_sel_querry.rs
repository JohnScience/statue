use super::super::{RetTyKind, SelQuerryKind, SelSyntax};

pub(crate) struct AnonSelQuerry {
    pub(crate) kind: SelQuerryKind,
    pub(crate) ret_ty: RetTyKind,
    pub(crate) syn: SelSyntax,
}
