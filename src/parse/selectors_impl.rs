use proc_macro::token_stream::IntoIter as TokenTreeIter;

use crate::selectors::{ExplKindSelQuerry, SelQuerries};

use super::{error::selectors_impl::Error, Parse};

impl Parse for SelQuerries {
    type Error = Error;
    type Output = Result<Self, Self::Error>;
    fn parse(iter: &mut TokenTreeIter) -> Self::Output {
        let mut selectors = Self::new();
        loop {
            match ExplKindSelQuerry::parse(iter) {
                Ok(selector) => selectors.push(selector),
                Err(e) => match e.try_into() {
                    Err(_unrepr_err) => break,
                    Ok(err) => return Err(err),
                },
            }
        }
        if selectors.is_empty() {
            return Err(Error::Finished);
        }
        Ok(selectors)
    }
}
