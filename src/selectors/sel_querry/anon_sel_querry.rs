use super::super::{SelSyntax, SelQuerryKind};

pub(crate) struct AnonSelQuerry {
    pub(crate) kind: SelQuerryKind,
    pub(crate) syn: SelSyntax,
}
