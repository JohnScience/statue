use proc_macro::TokenStream;

use crate::selectors::Selectors;
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
    pub(crate) selectors: Selectors
}

impl Args {
    pub(crate) fn handle(self) -> TokenStream {
        let Self { path, selectors } = self;
        let html = path.read();
        let dom = tl::parse(html.as_str(), tl::ParserOptions::default()).unwrap();
        let parser = dom.parser();
        let save_files_handle = dom.query_selector("#save-files").unwrap().next().unwrap();
        let save_files_node = save_files_handle.get(parser).unwrap();
        let save_files_html = save_files_node.outer_html(parser);
        panic!("{:#?}", save_files_html);
    }
}