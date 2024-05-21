pub(crate) mod ret_ty_kind_impl {
    #[derive(Debug)]
    pub(crate) struct RetTyKindExpected;
}

pub(crate) mod anon_selector_impl {
    #[repr(u8)]
    #[derive(Debug)]
    pub(crate) enum Error {
        Finished = 0,
        UnknownSelectorKind = 1,
        ParenExpected = 2,
        InvalidSelectorSyntax = 3,
    }

    impl Error {
        pub(crate) const HI: u8 = Self::InvalidSelectorSyntax as _;
    }
}

pub(crate) mod expl_kind_selector_impl {
    use super::anon_selector_impl::Error as Suberror;

    #[repr(u8)]
    #[derive(Debug)]
    pub(crate) enum Error {
        Finished = Suberror::Finished as _,
        UnknownSelectorKind = Suberror::UnknownSelectorKind as _,
        ParenExpected = Suberror::ParenExpected as _,
        InvalidSelectorSyntax = Suberror::InvalidSelectorSyntax as _,
        LetExpected = Suberror::HI + 1,
        VariableNameExpected = Self::LetExpected as u8 + 1,
        EqExpected = Self::VariableNameExpected as u8 + 1,
        SemiExpected = Self::EqExpected as u8 + 1,
    }

    impl Error {
        pub(crate) const HI: u8 = Self::SemiExpected as _;
    }

    impl From<Suberror> for Error {
        fn from(err: Suberror) -> Self {
            unsafe { core::mem::transmute(err) }
        }
    }
}

pub(crate) mod selectors_impl {
    use super::expl_kind_selector_impl::Error as Suberror;
    pub(crate) struct ExplKindSelectorFinishedErrIsUnrepresentable;
    type Result<T> = core::result::Result<T, ExplKindSelectorFinishedErrIsUnrepresentable>;

    #[repr(u8)]
    #[derive(Debug)]
    pub(crate) enum Error {
        UnknownSelectorKind = Suberror::UnknownSelectorKind as _,
        ParenExpected = Suberror::ParenExpected as _,
        InvalidSelectorSyntax = Suberror::InvalidSelectorSyntax as _,
        LetExpected = Suberror::LetExpected as _,
        VariableNameExpected = Suberror::VariableNameExpected as _,
        EqExpected = Suberror::EqExpected as _,
        SemiExpected = Suberror::SemiExpected as _,
        // Note that Finished error is redefined here
        Finished = super::expl_kind_selector_impl::Error::HI + 1,
    }

    impl Error {
        pub(crate) const HI: u8 = Self::Finished as _;
    }

    impl TryFrom<super::expl_kind_selector_impl::Error> for Error {
        type Error = ExplKindSelectorFinishedErrIsUnrepresentable;

        fn try_from(err: super::expl_kind_selector_impl::Error) -> Result<Self> {
            if matches!(err, super::expl_kind_selector_impl::Error::Finished) {
                return Err(ExplKindSelectorFinishedErrIsUnrepresentable);
            }
            unsafe { core::mem::transmute(err) }
        }
    }
}

pub(crate) mod selector_brace_group_parser_impl {
    use super::selectors_impl::Error as Suberror;
    #[repr(u8)]
    #[derive(Debug)]
    pub(crate) enum Error {
        UnknownSelectorKind = Suberror::UnknownSelectorKind as _,
        ParenExpected = Suberror::ParenExpected as _,
        InvalidSelectorSyntax = Suberror::InvalidSelectorSyntax as _,
        LetExpected = Suberror::LetExpected as _,
        VariableNameExpected = Suberror::VariableNameExpected as _,
        EqExpected = Suberror::EqExpected as _,
        SemiExpected = Suberror::SemiExpected as _,
        Finished = Suberror::Finished as _,
        BraceGroupExpected = Suberror::HI + 1,
    }

    impl Error {
        pub(crate) const HI: u8 = Self::BraceGroupExpected as _;
    }

    impl From<Suberror> for Error {
        fn from(err: Suberror) -> Self {
            unsafe { core::mem::transmute(err) }
        }
    }
}

pub(crate) mod args_impl {
    use super::selector_brace_group_parser_impl::Error as Suberror;

    #[repr(u8)]
    #[derive(Debug)]
    // TODO: remove dead_code eventually
    #[allow(dead_code)]
    pub(crate) enum Error {
        UnknownSelectorKind = Suberror::UnknownSelectorKind as _,
        ParenExpected = Suberror::ParenExpected as _,
        InvalidSelectorSyntax = Suberror::InvalidSelectorSyntax as _,
        LetExpected = Suberror::LetExpected as _,
        VariableNameExpected = Suberror::VariableNameExpected as _,
        EqExpected = Suberror::EqExpected as _,
        SemiExpected = Suberror::SemiExpected as _,
        Finished = Suberror::Finished as _,
        BraceGroupExpected = Suberror::BraceGroupExpected as _,
        ArgsExpected = Suberror::HI + 1,
        HtmlArgExpected = Self::ArgsExpected as u8 + 1,
        ColonExpected = Self::HtmlArgExpected as u8 + 1,
        StrLitHtmlPathExpected = Self::ColonExpected as u8 + 1,
        CommaExpected = Self::StrLitHtmlPathExpected as u8 + 1,
        ElementsArgExpected = Self::CommaExpected as u8 + 1,
    }

    impl From<Suberror> for Error {
        fn from(err: Suberror) -> Self {
            unsafe { core::mem::transmute(err) }
        }
    }
}
