use core::marker::PhantomData;
use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenTree};

use super::Parse;

pub(super) struct CommaSepedParser<T: Parse>(pub(super) PhantomData<*const T>);

impl<T: Parse> Parse for CommaSepedParser<T> {
    type Error = T::Error;
    type OkTy = T::OkTy;
    type Wrapper<O, E> = Option<T::Wrapper<O, E>>;

    fn parse(iter: &mut TokenTreeIter) -> Option<T::Wrapper<T::OkTy, T::Error>> {
        let Some(comma) = iter.next() else { return None };
        let TokenTree::Punct(comma) = comma else { return None };
        if !(comma.as_char() == ',') {
            return None;
        };
        Some(T::parse(iter))
    }
}
