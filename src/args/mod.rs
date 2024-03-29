use proc_macro::TokenStream;

use crate::{extend_token_stream::ExtendTokenStream, sel_queries::SelQueries};
use std::path::{Path, PathBuf};
use tl;

mod opts;

pub(crate) use opts::Opts;

pub(crate) struct HtmlPath(pub(crate) String);

impl HtmlPath {
    fn to_absolute(self) -> PathBuf {
        let Self(path) = self;
        let mut path = Path::new(&path).to_owned();
        if !path.is_absolute() {
            // In case of workpace, the invocation crate root is still the root of the crate
            // and not the root of the workspace.
            let invocation_crate_root = std::env::var("CARGO_MANIFEST_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|e| panic!("Failed to get manifest dir: {e:?}"));
            path = invocation_crate_root.join(&path);
            path = path.canonicalize()
                .unwrap_or_else(|e| panic!("Failed to canonicalize {path:?}: {e:?}"));
        };
        path
    }

    fn read(self) -> String {
        let abs_html_path = self.to_absolute();
        std::fs::read_to_string(&abs_html_path)
            .unwrap_or_else(|e| panic!("Failed to read file at {abs_html_path:?}: {e:?}"))
    }
}

pub(crate) struct Args {
    pub(crate) path: HtmlPath,
    pub(crate) sel_queries: SelQueries,
    pub(crate) opts: Opts,
}

impl Args {
    pub(crate) fn handle(self) -> TokenStream {
        let Self {
            path,
            sel_queries,
            opts,
        } = self;
        let html = path.read();
        let dom = tl::parse(html.as_str(), tl::ParserOptions::default()).unwrap();
        let elems = sel_queries.into_elements(&dom);
        let mut ts = TokenStream::new();
        opts.declare_window_and_document(&mut ts);
        elems.extend_token_stream(&mut ts);
        opts.hide_window_and_document_if_needed(&mut ts);
        ts
    }
}
