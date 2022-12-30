use crate::selectors::Selectors;

pub(crate) struct HtmlPath(pub(crate) String);

pub(crate) struct Args {
    pub(crate) path: HtmlPath,
    pub(crate) selectors: Selectors
}
