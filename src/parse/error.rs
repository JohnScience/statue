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
    #[repr(u8)]
    #[derive(Debug)]
    pub(crate) enum Error {
        Finished = super::anon_selector_impl::Error::Finished as _,
        UnknownSelectorKind = super::anon_selector_impl::Error::UnknownSelectorKind as _,
        ParenExpected = super::anon_selector_impl::Error::ParenExpected as _,
        InvalidSelectorSyntax = super::anon_selector_impl::Error::InvalidSelectorSyntax as _,
        LetExpected = super::anon_selector_impl::Error::HI + 1,
        VariableNameExpected = Self::LetExpected as u8 + 1,
        EqExpected = Self::VariableNameExpected as u8 + 1,
        SemiExpected = Self::EqExpected as u8 + 1,
    }

    impl Error {
        pub(crate) const HI: u8 = Self::SemiExpected as _;
    }

    impl From<super::anon_selector_impl::Error> for Error {
        fn from(err: super::anon_selector_impl::Error) -> Self {
            unsafe { core::mem::transmute(err) }
        }
    }
}

pub(crate) mod selectors_impl {
    pub(crate) struct ExplKindSelectorFinishedErrIsUnrepresentable;
    type Result<T> = core::result::Result<T, ExplKindSelectorFinishedErrIsUnrepresentable>;

    #[repr(u8)]
    #[derive(Debug)]
    pub(crate) enum Error {
        UnknownSelectorKind = super::expl_kind_selector_impl::Error::UnknownSelectorKind as _,
        ParenExpected = super::expl_kind_selector_impl::Error::ParenExpected as _,
        InvalidSelectorSyntax = super::expl_kind_selector_impl::Error::InvalidSelectorSyntax as _,
        LetExpected = super::expl_kind_selector_impl::Error::LetExpected as _,
        VariableNameExpected = super::expl_kind_selector_impl::Error::VariableNameExpected as _,
        EqExpected = super::expl_kind_selector_impl::Error::EqExpected as _,
        SemiExpected = super::expl_kind_selector_impl::Error::SemiExpected as _,
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
    #[repr(u8)]
    #[derive(Debug)]
    pub(crate) enum Error {
        UnknownSelectorKind = super::selectors_impl::Error::UnknownSelectorKind as _,
        ParenExpected = super::selectors_impl::Error::ParenExpected as _,
        InvalidSelectorSyntax = super::selectors_impl::Error::InvalidSelectorSyntax as _,
        LetExpected = super::selectors_impl::Error::LetExpected as _,
        VariableNameExpected = super::selectors_impl::Error::VariableNameExpected as _,
        EqExpected = super::selectors_impl::Error::EqExpected as _,
        SemiExpected = super::selectors_impl::Error::SemiExpected as _,
        Finished = super::selectors_impl::Error::Finished as _,
        BraceGroupExpected = super::selectors_impl::Error::HI + 1,
    }

    impl From<super::selectors_impl::Error> for Error {
        fn from(err: super::selectors_impl::Error) -> Self {
            unsafe { core::mem::transmute(err) }
        }
    }
}