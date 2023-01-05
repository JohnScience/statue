use super::super::{RetTyKind, SelSyntax};

pub(crate) struct ImplKindSelQuery {
    pub(crate) name: String,
    pub(crate) ret_ty: RetTyKind,
    pub(crate) syn: SelSyntax,
}
