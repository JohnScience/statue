use core::marker::PhantomData;
use proc_macro::{token_stream::IntoIter as TokenTreeIter, TokenTree};

use crate::util::ResultLike;

use super::Parse;

pub(crate) enum Error<T: Parse> {
    SomeOrNoneExpected,
    ParenGroupExpected,
    InnerError(T::Error),
}

pub(crate) struct OptionCombinator<T>(PhantomData<*const T>);

impl<T: Parse> Parse for OptionCombinator<T>
where
    T::Wrapper<T::OkTy, T::Error>: ResultLike<T::OkTy, T::Error>,
{
    type Error = Error<T>;
    type OkTy = Option<T::OkTy>;
    type Wrapper<O, E> = Result<O, E>;

    fn parse(iter: &mut TokenTreeIter) -> Self::Wrapper<Self::OkTy, Self::Error> {
        let Some(ident) = iter.next() else { return Err(Error::SomeOrNoneExpected) };
        let TokenTree::Ident(ident) = ident else { return Err(Error::SomeOrNoneExpected) };
        match ident.to_string().as_str() {
            "None" => Ok(None),
            "Some" => {
                let Some(paren_group) = iter.next() else { return Err(Error::ParenGroupExpected) };
                let TokenTree::Group(paren_group) = paren_group else { return Err(Error::ParenGroupExpected) };
                if paren_group.delimiter() != proc_macro::Delimiter::Parenthesis {
                    return Err(Error::ParenGroupExpected);
                };
                let mut paren_group_contents_iter = paren_group.stream().into_iter();
                T::parse(&mut paren_group_contents_iter)
                    .map(Some)
                    .map_err(Error::InnerError)
            }
            _ => Err(Error::SomeOrNoneExpected),
        }
    }
}
