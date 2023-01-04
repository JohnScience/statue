use super::super::{RetTyKind, SelSyntax};

pub(crate) struct ImplKindSelQuerry {
    pub(crate) name: String,
    pub(crate) ret_ty: RetTyKind,
    pub(crate) syn: SelSyntax,
}
