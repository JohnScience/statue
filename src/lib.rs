use proc_macro::TokenStream;

pub(crate) mod selectors;
pub(crate) mod args;
mod parse;

use parse::Parse;
use args::Args;

#[proc_macro]
pub fn apply_selectors(ts: TokenStream) -> TokenStream {
    let mut iter = ts.into_iter();
    let Args { path, selectors } = Args::parse(&mut iter).unwrap();
    panic!("{:#?}", selectors)
    //TokenStream::new()
}
