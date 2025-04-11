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

// NEW UNIMPL

#[proc_macro_attribute]
pub fn collect_unimpl(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemTrait);
    collect_internal_unimpl(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn collect_internal_unimpl(
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

    item.items.push(TraitItem::Verbatim(quote! {
        /// The next type in the chain.
        type NEXT: #trait_ident;

        /// Is this the end of the chain?
        const END: bool = false;
    }));

    let mut output = quote! {
        #item

        struct Container<const INDEX: usize>;

        /// The final implementation.
        /// Only implemented once, at the end.
        trait Final {}

        impl #trait_ident for Container<0> {
            type NEXT = Self;
            const END: bool = true;
        }
        impl Final for Container<0>
        where
            for<'a> Switch0: Unpin,
        {}
    };
    generate_switches(&mut output);

    Ok(output)
}

#[proc_macro]
pub fn testing_drop(input: StdTokenStream) -> StdTokenStream {
    let mut output = quote! {
        struct Assert<const BOOL: bool>;
        trait False {}
        impl False for Assert<false> {}

        struct Container<const INDEX: usize>;

        /// The final implementation.
        /// Only implemented once, at the end.
        trait Final {}

        /// We store traits on a type, that then goes to another type, and another and another.
        trait TraitChain {
            type NEXT: TraitChain;
            /// Is this the end of the chain?
            const END: bool = false;

            const STR: &str;
        }

        // impl Caller for Container<0>
        // where
        //     for<'a> Assert<{ std::mem::needs_drop::<Switch0>() }>: False,
        // {
        //     const LENGTH: usize = 0;
        // }
        impl TraitChain for Container<0> {
            type NEXT = Self;
            const END: bool = true;
            const STR: &str = "";
        }
        impl Final for Container<0>
        where
            for<'a> Switch0: Unpin,
        {}
    };

    generate_switches(&mut output);

    output.into()
}

fn generate_switches(output: &mut TokenStream) {
    (0..2000).for_each(|index| {
        let ident = Ident::new(&format!("Switch{index}"), Span::call_site());
        output.extend(quote! {
            struct #ident;
        });
    });
}

#[proc_macro]
pub fn drop_send(input: StdTokenStream) -> StdTokenStream {
    // Sadly the only way I know of counting...
    static INDEX: AtomicU32 = AtomicU32::new(0);
    let index = INDEX.fetch_add(1, Ordering::Relaxed);

    let index_previous = LitInt::new(&(index).to_string(), Span::call_site());
    let index_current = LitInt::new(&(index+1).to_string(), Span::call_site());
    let switch_previous = Ident::new(&format!("Switch{index}"), Span::call_site());
    let switch_current = Ident::new(&format!("Switch{}", index+1), Span::call_site());

    let output = quote! {
        // impl Drop for #switch_previous {
        //     fn drop(&mut self) {}
        // }
        // impl Caller for Container<#index_current>
        // where
        //     for<'a> Assert<{ std::mem::needs_drop::<#switch_current>() }>: False,
        // {
        //     const LENGTH: usize = #index_current;
        // }
        impl TraitChain for Container<#index_current> {
            type NEXT = Container<#index_previous>;
            const STR: &str = "a";
        }
        impl Unpin for #switch_previous where for<'a> [()]: Sized {}
        impl Final for Container<#index_current>
        where
            for<'a> #switch_current: Unpin,
        {}
    };

    output.into()
}
