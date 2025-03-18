use std::sync::atomic::{AtomicU32, Ordering};

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    FnArg, GenericParam, Ident, ItemFn, ItemTrait, LitInt, PatType, TraitItem, TypeParam,
    TypeParamBound, parse::Parse, parse_macro_input, spanned::Spanned,
};

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
pub fn collect(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemTrait);
    collect_internal(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn collect_internal(input: TokenStream, mut item: ItemTrait) -> syn::Result<TokenStream> {
    if !input.is_empty() {
        return Err(syn::Error::new(
            input.span(),
            "This attribute accepts nothing but itself.",
        ));
    }

    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {Sized}));

    item.items.push(TraitItem::Verbatim(quote! {
        fn __get_self(self) -> Self {
            self
        }
    }));

    let trait_ident = item.ident.clone();

    Ok(quote! {
        #item

        impl #trait_ident for retrieval::core::DefaultElement {}
    })
}

#[proc_macro_attribute]
pub fn iterate(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemFn);
    iterate_internal(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn iterate_internal(input: TokenStream, mut internal: ItemFn) -> syn::Result<TokenStream> {
    if !input.is_empty() {
        return Err(syn::Error::new(
            input.span(),
            "This attribute accepts nothing but itself.",
        ));
    }

    let mut external = internal.clone();

    let internal_ident = Ident::new(
        format!("__internal_{}", internal.sig.ident).as_str(),
        internal.sig.ident.span(),
    );

    let parameters = internal.sig.inputs.iter().map(|input| {
        let FnArg::Typed(input) = input else {
            panic!("Cannot have self input. That doesn't make sense.");
        };

        &input.pat
    });

    let mut witnesses: Vec<FnArg> = vec![];
    let generics = external
        .sig
        .generics
        .params
        .into_iter()
        .filter(|generic_param| {
            if let GenericParam::Type(generic_param) = generic_param {
                let ident = &generic_param.ident;
                witnesses.push(syn::parse2(quote! {_: &impl FnOnce() -> #ident}).unwrap());
                false
            } else {
                true
            }
        });
    external.sig.generics.params = generics.collect();

    // Very confusing. Iterating all combinations is tough.
    let elements: Vec<TokenStream> = Vec::with_capacity(100_usize.pow(witnesses.len() as u32));

    for i in 0..100_u32.pow(witnesses.len() as u32) {
        let inner = ();
    }

    external.block = syn::parse2(quote! {{
        macro_rules! repeat_function {
            ($i:expr) => {
                #internal_ident(#(#parameters),*, &||{retrieval::core::Element::<$i>.__get_self()});
            };
        }

        retrieval::core::macro_counter!(repeat_function 100);
    }})?;

    internal.sig.inputs.extend(witnesses);
    internal.sig.ident = internal_ident;

    Ok(quote! {
        #internal
        #external
    })
}

// #[proc_macro_attribute]
// pub fn experiment(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
//     let item = parse_macro_input!(item as ItemFn);
//     experiment_internal(input.into(), item)
//         .unwrap_or_else(syn::Error::into_compile_error)
//         .into()
// }

// fn experiment_internal(input: TokenStream, mut internal: ItemFn) -> syn::Result<TokenStream> {
//     if !input.is_empty() {
//         return Err(syn::Error::new(
//             input.span(),
//             "This attribute accepts nothing but itself.",
//         ));
//     }

//     internal.sig.generics.params.push(GenericParam::Type(TypeParam::try_from(quote!{}).unwrap()));

//     //external.sig.generics.params.into_iter().fil

//     Ok(quote! {
//         #internal
//         #external
//     })
// }

#[proc_macro]
pub fn macro_counter_ident(input: StdTokenStream) -> StdTokenStream {
    let MacroCounter { macro_ident, max } = parse_macro_input!(input as MacroCounter);

    let mut output = quote! {};

    (0..max).for_each(|i| {
        let i_plus_one = Ident::new(format!("Number{}", i + 1).as_str(), Span::call_site());
        let i = Ident::new(format!("Number{i}").as_str(), Span::call_site());

        output.extend(quote! {
            #macro_ident!(#i, #i_plus_one);
        });
    });

    output.into()
}
