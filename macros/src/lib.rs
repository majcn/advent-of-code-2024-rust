use proc_macro::TokenStream;

mod memoize;
use crate::memoize::memoize_impl;

#[proc_macro_attribute]
pub fn memoize(attr: TokenStream, item: TokenStream) -> TokenStream {
    memoize_impl(attr, item)
}
