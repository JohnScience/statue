use proc_macro::TokenStream;

pub(crate) mod args;
pub(crate) mod elements;
pub(crate) mod extend_token_stream;
mod parse;
pub(crate) mod sel_queries;
pub(crate) mod util;

use args::Args;
use parse::Parse;

#[proc_macro]
pub fn initialize_elements(ts: TokenStream) -> TokenStream {
    let mut iter = ts.into_iter();
    let args = Args::parse(&mut iter).unwrap();
    args.handle()
}
