use proc_macro::TokenStream;

use crate::{selectors::Selectors, extend_token_stream::ExtendTokenStream};
use std::path::{Path, PathBuf};
use tl;

pub(crate) struct HtmlPath(pub(crate) String);

impl HtmlPath {
    fn to_absolute(self) -> PathBuf {
        let Self(path) = self;
        let mut path = Path::new(&path).to_owned();
        if !path.is_absolute() {
            let invocation_crate_root = std::env::current_dir().unwrap();
            path = invocation_crate_root.join(path).canonicalize().unwrap();
        };
        path
    }

    fn read(self) -> String {
        let abs_html_path = self.to_absolute();
        std::fs::read_to_string(&abs_html_path).unwrap()
    }
}

pub(crate) struct Args {
    pub(crate) path: HtmlPath,
    pub(crate) selectors: Selectors,
}

impl Args {
    pub(crate) fn handle(self) -> TokenStream {
        let Self { path, selectors } = self;
        let html = path.read();
        let dom = tl::parse(html.as_str(), tl::ParserOptions::default()).unwrap();
        let elems = selectors.into_elements(&dom);
        let mut ts = TokenStream::new();
        elems.extend_token_stream(&mut ts);
        ts
    }
}
