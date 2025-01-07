use proc_macro::TokenStream;

mod memoize;
use crate::memoize::memoize_impl;

#[proc_macro_attribute]
pub fn memoize(args: proc_macro::TokenStream, item: proc_macro::TokenStream) -> TokenStream {
    memoize_impl(args, item)
}
