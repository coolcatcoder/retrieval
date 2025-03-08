use std::sync::atomic::{AtomicU32, Ordering};

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, parse_macro_input, spanned::Spanned, Ident, ItemTrait, LitInt, TraitItem, TypeParamBound};

struct MacroCounter {
    macro_ident: Ident,
    max: u64,
}

impl Parse for MacroCounter {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(MacroCounter {
            macro_ident: input.parse()?,
            max: input.parse::<LitInt>()?.base10_parse()?,
        })
    }
}

/// Runs a macro an amount of times, giving it the current iteration.
#[proc_macro]
pub fn macro_counter(input: StdTokenStream) -> StdTokenStream {
    let MacroCounter { macro_ident, max } = parse_macro_input!(input as MacroCounter);

    let mut output = quote! {};

    (0..max).for_each(|i| {
        let i = LitInt::new(i.to_string().as_str(), Span::call_site());

        output.extend(quote! {
            #macro_ident!(#i);
        });
    });

    output.into()
}

#[proc_macro]
pub fn unique(_: StdTokenStream) -> StdTokenStream {
    static COUNTER: AtomicU32 = AtomicU32::new(0);

    let current = COUNTER.fetch_add(1, Ordering::Relaxed);

    let current = LitInt::new(current.to_string().as_str(), Span::call_site());

    quote! {#current}.into()
}

#[proc_macro_attribute]
pub fn retrieve(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemTrait);
    retrieve_internal(input.into(), item).unwrap_or_else(syn::Error::into_compile_error).into()
}

fn retrieve_internal(input: TokenStream, mut item: ItemTrait) -> syn::Result<TokenStream> {
    if !input.is_empty() {
        return Err(syn::Error::new(input.span(), "This attribute accepts nothing but itself."));
    }

    item.supertraits.push(TypeParamBound::Verbatim(quote!{Sized}));

    item.items.push(TraitItem::Verbatim(quote! {
        fn internal_do_not_use_directly_get_self(self) -> Self {
            self
        }
    }));

    Ok(quote! {
        #item
    })
}
