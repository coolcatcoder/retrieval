use std::sync::atomic::{AtomicU32, Ordering};

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use rand::Rng;
use syn::{
    FnArg, GenericParam, Ident, ImplItem, ItemFn, ItemImpl, ItemTrait, LitInt, PatType, TraitItem,
    Type, TypeParam, TypeParamBound, parse::Parse, parse_macro_input, spanned::Spanned,
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

const CHECKPOINTS: u32 = 20;

#[proc_macro_attribute]
pub fn collect_experiment(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemTrait);
    collect_experiment_internal(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn collect_experiment_internal(
    input: TokenStream,
    mut item: ItemTrait,
) -> syn::Result<TokenStream> {
    if !input.is_empty() {
        return Err(syn::Error::new(
            input.span(),
            "This attribute accepts nothing but itself.",
        ));
    }

    let trait_ident = &item.ident;

    item.supertraits
        .push(TypeParamBound::Verbatim(quote! {Sized}));

    item.items.push(TraitItem::Verbatim(quote! {
        const REASON: retrieval::deref::Reason = retrieval::deref::Reason::Add;

        fn __get_self(self) -> Self {
            self
        }

        fn __next() -> impl #trait_ident;
    }));

    let ampersands = [syn::Token![&](Span::call_site()); 128];

    let checkpoints = (0..(CHECKPOINTS-1)).map(|index| {
        let index_plus_one = index+1;
        quote! {
            impl #trait_ident for retrieval::deref::DerefOnly<retrieval::deref::Checkpoint<#index>> {
                const REASON: retrieval::deref::Reason = retrieval::deref::Reason::Checkpoint;
                fn __next() -> impl #trait_ident {
                    retrieval::deref::DerefOnly::<#(#ampersands)*retrieval::deref::Checkpoint<#index_plus_one>>::new().__get_self()
                }
            }
        }
    });

    let end_checkpoint = CHECKPOINTS - 1;

    Ok(quote! {
        #item

        #(
            #checkpoints
        )*

        impl #trait_ident for retrieval::deref::DerefOnly<retrieval::deref::Checkpoint<#end_checkpoint>> {
            const REASON: retrieval::deref::Reason = retrieval::deref::Reason::End;

            fn __next() -> impl #trait_ident {
                retrieval::deref::DerefOnly::<retrieval::deref::Checkpoint<#end_checkpoint>>::new()
            }
        }
    })
}

#[proc_macro_attribute]
pub fn something(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemImpl);
    something_internal(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn something_internal(input: TokenStream, mut item: ItemImpl) -> syn::Result<TokenStream> {
    let mut rng = rand::rng();
    let checkpoint = rng.random_range(0..CHECKPOINTS);
    let ampersand_quantity = rng.random_range(1..=128);

    // TODO: Actually randomly generate the &s using a checkpoint system.
    let ampersands = (0..ampersand_quantity).map(|_| syn::Token![&](Span::call_site()));
    let ampersands_minus_one =
        (0..(ampersand_quantity - 1)).map(|_| syn::Token![&](Span::call_site()));

    let self_ty = std::mem::replace(
        &mut *item.self_ty,
        Type::Verbatim(
            quote! {retrieval::deref::DerefOnly<#(#ampersands)*retrieval::deref::Checkpoint<#checkpoint>>},
        ),
    );
    item.trait_ = Some((
        None,
        syn::parse2(self_ty.to_token_stream())?,
        Default::default(),
    ));

    let trait_ident = &item.trait_.as_ref().unwrap().1;
    item.items.push(ImplItem::Verbatim(quote! {
        fn __next() -> impl #trait_ident {
            retrieval::deref::DerefOnly::<#(#ampersands_minus_one)*retrieval::deref::Checkpoint<#checkpoint>>::new().__get_self()
        }
    }));

    Ok(item.to_token_stream())
}
