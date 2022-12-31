use proc_macro::TokenStream;

pub(crate) mod args;
pub(crate) mod elements;
mod parse;
pub(crate) mod extend_token_stream;
pub(crate) mod selectors;

use args::Args;
use parse::Parse;

#[proc_macro]
pub fn initialize_elements(ts: TokenStream) -> TokenStream {
    let mut iter = ts.into_iter();
    let args = Args::parse(&mut iter).unwrap();
    args.handle()
}
