use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use std::sync::{
    OnceLock,
    atomic::{AtomicU32, Ordering},
};
use syn::{
    FnArg, Ident, ImplItem, ItemFn, ItemImpl, ItemTrait, LitInt, Token, TraitItem, Type,
    parse::Parse, parse_macro_input, spanned::Spanned,
};

struct RetrieveAttribute {
    capacity: u32,
}

impl Parse for RetrieveAttribute {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let capacity = match input.parse::<Option<Ident>>()? {
            Some(_) => {
                input.parse::<Token![=]>()?;
                input.parse::<LitInt>()?.base10_parse()?
            }
            None => match input.parse::<Option<LitInt>>()? {
                Some(capacity) => capacity.base10_parse()?,
                None => 1000,
            },
        };

        Ok(RetrieveAttribute { capacity })
    }
}

/// Place on a trait to turn it into a retrieval trait.
/// This has to be done in the crate root currently. Everything else can be used anywhere.
/// TODO: Document this better.
#[proc_macro_attribute]
pub fn retrieve(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemTrait);
    retrieve_internal(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn retrieve_internal(input: TokenStream, mut item: ItemTrait) -> syn::Result<TokenStream> {
    let capacity = syn::parse2::<RetrieveAttribute>(input)?.capacity;

    let trait_ident = &item.ident;
    let mod_ident = Ident::new(&trait_ident.to_string().to_lowercase(), Span::call_site());

    item.items.push(TraitItem::Verbatim(quote! {
        /// The next type in the chain.
        type NEXT: #trait_ident;

        /// Is this the end of the chain?
        const END: bool = false;
    }));

    // One extra for the one required impl.
    let switches = generate_switches(capacity+1);

    let output = quote! {
        #item

        mod #mod_ident {
            /// The final implementation.
            /// Only implemented once, at the end.
            pub trait Final {}

            pub const LENGTH: usize = {
                const fn get_length<const INDEX: usize>() -> usize
                where
                    retrieval::Container<INDEX>: Final,
                {
                    INDEX
                }
                get_length()
            };

            #switches
        }

        impl #trait_ident for retrieval::Container<0> {
            type NEXT = Self;
            const END: bool = true;
        }
        impl #mod_ident::Final for retrieval::Container<0>
        where
            for<'a> #mod_ident::Switch0: core::marker::Unpin,
        {}
    };

    Ok(output)
}

fn generate_switches(amount: u32) -> TokenStream {
    let mut output = TokenStream::new();
    (0..amount).for_each(|index| {
        let ident = Ident::new(&format!("Switch{index}"), Span::call_site());
        output.extend(quote! {
            pub struct #ident;
        });
    });
    output
}

/// TODO: Document this better.
#[proc_macro_attribute]
pub fn send(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemImpl);
    send_internal(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn send_internal(input: TokenStream, mut item: ItemImpl) -> syn::Result<TokenStream> {
    if !input.is_empty() {
        return Err(syn::Error::new(
            input.span(),
            "This attribute accepts nothing but itself.",
        ));
    }

    // Sadly the only way I know of counting...
    // TODO: Replace with dashmap or something like that to handle multiple traits.
    //static TRAIT_COUNTERS: LazyLock<DashMap<String, AtomicU32, foldhash::fast::FixedState>> = LazyLock::new(|| {DashMap::with_hasher(foldhash::fast::FixedState::with_seed(13578576515223716000))});
    //static TRAIT_COUNTERS: DashMap<String, u32, foldhash::fast::FixedState> = DashMap::with_hasher(foldhash::fast::FixedState::with_seed(13578576515223716000));
    //static INDEX: AtomicU32 = AtomicU32::new(0);
    //let index = INDEX.fetch_add(1, Ordering::Relaxed);
    //let index = *TRAIT_COUNTERS.ins
    // TODO: A max of 5 traits? That sucks.
    static TRAIT_COUNTERS: [(AtomicU32, OnceLock<String>); 5] =
        [const { (AtomicU32::new(0), OnceLock::new()) }; 5];

    let trait_ident = item.self_ty.to_token_stream().to_string();
    let mut index = None;
    for (maybe_index, ident) in TRAIT_COUNTERS.iter() {
        let ident = ident.get_or_init(|| trait_ident.clone());
        if *ident == trait_ident {
            index = Some(maybe_index.fetch_add(1, Ordering::Relaxed));
            break;
        }
    }

    let Some(index) = index else {
        return Err(syn::Error::new(
            item.self_ty.span(),
            "Something went wrong with the proc macro atomics.\nI'm sorry.\nDid you try use more than 5 traits? We don't support that.",
        ));
    };

    let index_previous = LitInt::new(&(index).to_string(), Span::call_site());
    let index_current = LitInt::new(&(index + 1).to_string(), Span::call_site());

    let self_ty = std::mem::replace(
        &mut *item.self_ty,
        Type::Verbatim(quote! {retrieval::Container<#index_current>}),
    );

    item.trait_ = Some((
        None,
        syn::parse2(self_ty.to_token_stream())?,
        Default::default(),
    ));
    let mod_ident = Ident::new(
        &self_ty.to_token_stream().to_string().to_lowercase(),
        Span::call_site(),
    );

    item.items.push(ImplItem::Verbatim(quote! {
        type NEXT = retrieval::Container<#index_previous>;
    }));

    let switch_previous = Ident::new(&format!("Switch{index}"), Span::call_site());
    let switch_current = Ident::new(&format!("Switch{}", index + 1), Span::call_site());

    let output = quote! {
        #item

        impl core::marker::Unpin for crate::#mod_ident::#switch_previous where for<'a> [()]: Sized {}
        impl crate::#mod_ident::Final for retrieval::Container<#index_current>
        where
            for<'a> crate::#mod_ident::#switch_current: core::marker::Unpin,
        {}
    };

    Ok(output)
}

/// Converts a function into one that iterates over every implementation of a trait.
/// TODO: Document this better.
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

    if internal.sig.generics.params.len() != 1 {
        return Err(syn::Error::new(
            internal.sig.generics.span(),
            "Only one generic is supported at this time.\nPlease see TODO: Insert issue number here.",
        ));
    }

    let generic = internal.sig.generics.type_params().next().unwrap();
    let syn::TypeParamBound::Trait(generic_trait) = generic.bounds.first().unwrap() else {
        return Err(syn::Error::new(
            generic.bounds.span(),
            "The singular generic should only have a trait bound.\nPlease see TODO: Insert issue number here.",
        ));
    };
    let mod_ident = Ident::new(
        &generic_trait
            .path
            .segments
            .last()
            .unwrap()
            .ident
            .to_string()
            .to_lowercase(),
        Span::call_site(),
    );
    let generic_ident = &generic.ident;
    let mut external_sig = internal.sig.clone();
    external_sig.generics = Default::default();

    let external_ident = internal.sig.ident;
    internal.sig.ident = Ident::new(
        &format!("__internal_{}", &external_ident),
        Span::call_site(),
    );
    let internal_ident = &internal.sig.ident;

    internal.block.stmts.insert(
        0,
        syn::parse2(quote! {
            if #generic_ident::END {
                return;
            }
        })?,
    );
    let inputs = internal.sig.inputs.iter().map(|input| {
        let FnArg::Typed(input) = input else {
            panic!("Cannot have self input. That doesn't make sense.");
        };

        &input.pat
    });
    let inputs_again = internal.sig.inputs.iter().map(|input| {
        let FnArg::Typed(input) = input else {
            panic!("Cannot have self input. That doesn't make sense.");
        };

        &input.pat
    });
    internal.block.stmts.push(syn::parse2(
        quote! {#internal_ident::<T::NEXT>(#(#inputs),*);},
    )?);

    let output = quote! {
        #internal

        #external_sig {
            #internal_ident::<retrieval::Container<{crate::#mod_ident::LENGTH}>>(#(#inputs_again),*);
        }
    };

    Ok(output)
}
