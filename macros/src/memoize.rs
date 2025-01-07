use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::*, punctuated::*, spanned::*, *};

mod kw {
    syn::custom_keyword!(ignore);
    syn::custom_keyword!(key_function);
}

#[derive(Default)]
struct CacheOptions {
    ignore: Vec<Ident>,
    key_function: Option<(Ident, Ident)>,
}

enum CacheOption {
    Ignore(Vec<Ident>),
    KeyFunction(Ident, Ident),
}

impl Parse for CacheOption {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let la = input.lookahead1();

        if la.peek(kw::ignore) {
            input.parse::<kw::ignore>().unwrap();
            input.parse::<Token![=]>().unwrap();
            let bracketed_content;
            bracketed!(bracketed_content in input);

            let result = Punctuated::<LitStr, Token![,]>::parse_terminated(&bracketed_content)
                .unwrap()
                .into_iter()
                .map(|lit_str| lit_str.parse::<Ident>().unwrap())
                .collect::<Vec<_>>();

            return Ok(CacheOption::Ignore(result));
        }

        if la.peek(kw::key_function) {
            input.parse::<kw::key_function>().unwrap();
            input.parse::<Token![=]>().unwrap();
            let input = input.parse::<LitStr>().unwrap();
            let input_value = input.value();

            let (key_function_name_str, key_function_return_str) =
                input_value.split_once(" -> ").unwrap();
            let key_function_name = Ident::new(key_function_name_str, input.span());
            let key_function_return = Ident::new(key_function_return_str, input.span());

            return Ok(CacheOption::KeyFunction(
                key_function_name,
                key_function_return,
            ));
        }

        Err(la.error())
    }
}

impl Parse for CacheOptions {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut opts = Self::default();

        let attrs = Punctuated::<CacheOption, syn::Token![,]>::parse_terminated(input)?;

        for opt in attrs {
            match opt {
                CacheOption::Ignore(ident) => opts.ignore.extend(ident),
                CacheOption::KeyFunction(name, return_type) => {
                    opts.key_function = Some((name, return_type));
                }
            }
        }

        Ok(opts)
    }
}

pub fn memoize_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let options: CacheOptions = syn::parse(args).unwrap();

    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = parse_macro_input!(item as ItemFn);

    let fn_input_names = sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, .. }) => Some(*pat.clone()),
            FnArg::Receiver(_) => None,
        })
        .collect::<Vec<_>>();

    let cache_input_names = sig
        .inputs
        .iter()
        .filter_map(|arg| match arg {
            FnArg::Typed(PatType { pat, .. }) => Some(*pat.clone()),
            FnArg::Receiver(_) => None,
        })
        .filter(|pat| match pat {
            Pat::Ident(PatIdent { ident, .. }) => {
                !options.ignore.iter().any(|ignore| ignore == ident)
            }
            _ => true,
        })
        .collect::<Vec<_>>();

    let fn_return_type = match &sig.output {
        ReturnType::Default => quote! { () },
        ReturnType::Type(_, ty) => ty.to_token_stream(),
    };

    let cache_key_name = match &options.key_function {
        Some((name, _)) => quote! { #name(#(#fn_input_names),*) },
        None => quote! { (#(#cache_input_names.clone()),*) },
    };

    let cache_key_return_type = match &options.key_function {
        Some((_, return_type)) => quote! { #return_type },
        None => fn_return_type.clone(),
    };

    let internal_fn_name = format!("__{}_internal", sig.ident);
    let cache_static_var_name = format!("__CACHE_{}", sig.ident.to_string().to_uppercase());

    let internal_fn_ident = Ident::new(&internal_fn_name, sig.span());
    let cache_static_var_ident = Ident::new(&cache_static_var_name, sig.span());

    let internal_sig = Signature {
        ident: internal_fn_ident.clone(),
        ..sig.clone()
    };

    quote!(
        thread_local! {
            static #cache_static_var_ident: std::cell::RefCell<advent_of_code::maneatingape::hash::FastMap<#cache_key_return_type, #fn_return_type>> = std::cell::RefCell::new(advent_of_code::maneatingape::hash::FastMapBuilder::new());
        }

        #(#attrs)*
        #vis #internal_sig #block

        #(#attrs)*
        #vis #sig {
            let cache_key = #cache_key_name;

            let cached_result_option = #cache_static_var_ident.with(|cache| {
                cache.borrow().get(&cache_key).cloned()
            });

            if let Some(cached_result) = cached_result_option {
                return cached_result;
            }

            let result = #internal_fn_ident (#(#fn_input_names),*);

            #cache_static_var_ident.with(|cache| {
                cache.borrow_mut().insert(cache_key, result);
            });

            result
        }
    )
    .into()
}
