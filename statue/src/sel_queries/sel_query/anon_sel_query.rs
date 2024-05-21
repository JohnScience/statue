use super::super::{RetTyKind, SelQueryKind, SelSyntax};

pub(crate) struct AnonSelQuery {
    pub(crate) kind: SelQueryKind,
    pub(crate) ret_ty: RetTyKind,
    pub(crate) syn: SelSyntax,
}
