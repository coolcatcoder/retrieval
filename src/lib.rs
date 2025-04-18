use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use std::sync::{
    OnceLock,
    atomic::{AtomicU32, Ordering},
};
use syn::{
    FnArg, Ident, ImplItem, ItemFn, ItemImpl, ItemTrait, LitInt, Pat, Stmt, Token, TraitItem, Type,
    parse::Parse, parse_macro_input, spanned::Spanned,
};

/// Allows a number in the attribute, or goes with a default.
/// Optionally allows ident= before the number, where ident can be any ident.
struct NumberAttribute<const DEFAULT: u32>(u32);

impl<const DEFAULT: u32> Parse for NumberAttribute<DEFAULT> {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(NumberAttribute(match input.parse::<Option<Ident>>()? {
            Some(_) => {
                input.parse::<Token![=]>()?;
                input.parse::<LitInt>()?.base10_parse()?
            }
            None => match input.parse::<Option<LitInt>>()? {
                Some(capacity) => capacity.base10_parse()?,
                None => DEFAULT,
            },
        }))
    }
}

/// Gets the public module ident from the trait ident.
fn module_public_from_trait(trait_ident: &impl ToString) -> Ident {
    Ident::new(&trait_ident.to_string().to_lowercase(), Span::call_site())
}

/// Gets the internal module ident from the trait ident.
fn module_internal_from_trait(trait_ident: &impl ToString) -> Ident {
    Ident::new(
        &format!("__internal_{}", trait_ident.to_string().to_lowercase()),
        Span::call_site(),
    )
}

/// Place on a trait to turn it into a retrieval trait, which is capable of collecting implementations.  
/// Optionally allows the capacity to be specified. Defaults to 1000. The higher the capacity, the longer it will take to compile.
///
/// Generates a module that has the name of the trait but in in lowercase. It contains the QUANTITY of implementations along with a few implementation details.
///
/// Due to how this works internally, there are a few restrictions:  
/// This has to be done in the crate root currently.  
/// All associated items must have a default. (Allows associated types to have defaults, which would normally not be allowed.)  
/// You can only have a max of 5 retrieval traits in 1 crate. (If you need more than 5, then please raise an issue on github!)
/// ```rust
/// # use retrieval::retrieve;
/// # fn main() {}
/// #[retrieve(capacity=10)]
/// trait Message {
///     const STR: &str = "";
/// }
///
/// #[retrieve]
/// trait Something {
///     type Blah = ();
///     fn bubble() {}
/// }
///
/// #[retrieve(1000)]
/// trait Empty {}
/// ```
#[proc_macro_attribute]
pub fn retrieve(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemTrait);
    retrieve_internal(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn retrieve_internal(input: TokenStream, mut item: ItemTrait) -> syn::Result<TokenStream> {
    let capacity = syn::parse2::<NumberAttribute<1000>>(input)?.0;

    let trait_ident = &item.ident;
    let module_public_ident = module_public_from_trait(trait_ident);
    let module_internal_ident = module_internal_from_trait(trait_ident);

    // Associated types aren't allowed defaults, so we remove them, and pass the default to our initial implementation.
    let default_types: Vec<TokenStream> = item
        .items
        .iter_mut()
        .filter_map(|item| {
            let TraitItem::Type(item) = item else {
                return None;
            };

            let default = item.default.take()?;
            let ident = &item.ident;
            let default_type = default.1;

            Some(quote! {type #ident = #default_type;})
        })
        .collect();

    item.items.push(TraitItem::Verbatim(quote! {
        /// The next type in the chain.
        type NEXT: #trait_ident;

        /// Is this the end of the chain?
        const END: bool = false;
    }));

    // One extra for the one required impl.
    let switches = generate_switches(capacity + 1);

    let output = quote! {
        #item

        /// Contains information related to the retrieval trait of the same name.
        mod #module_public_ident {
            /// The amount of implementations of this trait.
            pub const QUANTITY: usize = {
                const fn get_quantity<const INDEX: usize>() -> usize
                where
                    crate::#module_internal_ident::Container<INDEX>: super::#module_internal_ident::Final,
                {
                    INDEX
                }
                get_quantity()
            };
        }

        #[doc(hidden)]
        mod #module_internal_ident {
            /// The final implementation.
            /// Only implemented once, at the end.
            #[doc(hidden)]
            pub trait Final {}

            /// Contains the retrieved implementations.
            /// Each implementation is stored under a different INDEX.
            #[doc(hidden)]
            pub struct Container<const INDEX: usize>;

            #switches
        }

        impl #trait_ident for #module_internal_ident::Container<0> {
            #(#default_types)*
            type NEXT = Self;
            const END: bool = true;
        }
        impl #module_internal_ident::Final for #module_internal_ident::Container<0>
        where
            for<'a> #module_internal_ident::Switch0: core::marker::Unpin,
        {}
    };

    Ok(output)
}

fn generate_switches(amount: u32) -> TokenStream {
    let mut output = TokenStream::new();
    (0..amount).for_each(|index| {
        let ident = Ident::new(&format!("Switch{index}"), Span::call_site());
        output.extend(quote! {
            #[doc(hidden)]
            pub struct #ident;
        });
    });
    output
}

/// Place on an inherent impl of a [retrieval trait](macro@retrieve) in order to send it for retrieval.
/// ```
/// # use retrieval::*;
/// # fn main() {}
/// #[retrieve]
/// trait Message {
///     const STR: &str = "";
/// }
///
/// #[send]
/// impl Message {
///     const STR: &str = "Hello world!";
/// }
/// ```
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
            format!("Something went wrong with the proc macro atomics.\nI'm sorry.\nDid you try use more than 5 traits? We don't support that.\n{:?}", TRAIT_COUNTERS),
        ));
    };

    let index_previous = LitInt::new(&(index).to_string(), Span::call_site());
    let index_current = LitInt::new(&(index + 1).to_string(), Span::call_site());

    let module_internal_ident = module_internal_from_trait(&item.self_ty.to_token_stream());
    let self_ty = std::mem::replace(
        &mut *item.self_ty,
        Type::Verbatim(quote! {crate::#module_internal_ident::Container<#index_current>}),
    );

    item.trait_ = Some((
        None,
        syn::parse2(self_ty.to_token_stream())?,
        Default::default(),
    ));

    item.items.push(ImplItem::Verbatim(quote! {
        type NEXT = crate::#module_internal_ident::Container<#index_previous>;
    }));

    let switch_previous = Ident::new(&format!("Switch{index}"), Span::call_site());
    let switch_current = Ident::new(&format!("Switch{}", index + 1), Span::call_site());

    let output = quote! {
        #item

        impl core::marker::Unpin for crate::#module_internal_ident::#switch_previous where for<'a> [()]: Sized {}
        impl crate::#module_internal_ident::Final for crate::#module_internal_ident::Container<#index_current>
        where
            for<'a> crate::#module_internal_ident::#switch_current: core::marker::Unpin,
        {}
    };

    Ok(output)
}

/// Place on a generic function to convert it into a non-generic function that iterates through all implementations of a [retrieval trait](macro@retrieve).  
/// Optionally allows the capacity to be specified. Defaults to 1000. The higher the capacity, the longer it will take to compile.
///
/// Due to [an issue](https://github.com/coolcatcoder/retrieval/issues/7) the function must have only one generic with only one trait bound.
/// ```rust
/// # use retrieval::*;
/// # fn main() {}
/// #[retrieve]
/// trait Message {
///     const STR: &str = "";
/// }
///
/// #[iterate]
/// const fn collect_messages<T: Message>(messages: &mut [&str], index: &mut usize) {
///     messages[*index] = T::STR;
///     *index += 1;
/// }
///
/// const MESSAGES: [&str; message::QUANTITY] = {
///     let mut messages = [""; message::QUANTITY];
///     let mut index = 0;
///
///     collect_messages(&mut messages, &mut index);
///     messages
/// };
/// ```
#[proc_macro_attribute]
pub fn iterate(input: StdTokenStream, item: StdTokenStream) -> StdTokenStream {
    let item = parse_macro_input!(item as ItemFn);
    iterate_internal(input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn iterate_internal(input: TokenStream, internal: ItemFn) -> syn::Result<TokenStream> {
    // Work out how many functions we will need to reach the target recursion limit.
    let recursion_limit = syn::parse2::<NumberAttribute<1000>>(input)?.0;
    let functions_needed = recursion_limit.div_ceil(128);

    if internal.sig.generics.params.len() != 1 {
        return Err(syn::Error::new(
            internal.sig.generics.span(),
            "Only one generic is supported at this time.\nPlease see https://github.com/coolcatcoder/retrieval/issues/7.",
        ));
    }

    let generic = internal.sig.generics.type_params().next().unwrap();
    let generic_ident = &generic.ident;
    let syn::TypeParamBound::Trait(trait_bound) = generic.bounds.first().unwrap() else {
        return Err(syn::Error::new(
            generic.bounds.span(),
            "The singular generic should only have one trait bound.\nPlease see https://github.com/coolcatcoder/retrieval/issues/7.",
        ));
    };

    // Get the module from the last segment of the trait bound.
    let trait_ident = &trait_bound.path.segments.last().unwrap().ident;
    let module_public_ident = module_public_from_trait(trait_ident);
    let module_internal_ident = module_internal_from_trait(trait_ident);

    // Create the external function's signature from the internal's but without the generics.
    let mut external_sig = internal.sig.clone();
    external_sig.generics = Default::default();
    let external_ident = &external_sig.ident;
    let inputs: Vec<&Pat> = external_sig
        .inputs
        .iter()
        .map(|input| {
            let FnArg::Typed(input) = input else {
                panic!("Cannot have self input. That doesn't make sense.");
            };

            &*input.pat
        })
        .collect();

    // The first internal function's ident.
    let internal_start_ident = Ident::new(
        &format!("__internal_0_{}", external_ident),
        Span::call_site(),
    );

    let mut output = quote! {
        #external_sig {
            #internal_start_ident::<crate::#module_internal_ident::Container<{crate::#module_public_ident::QUANTITY}>>(#(#inputs),*);
        }
    };

    let if_end: Stmt = syn::parse2(quote! {
        if #generic_ident::END {
            return;
        }
    })?;

    for index in 0..functions_needed {
        let next_index = if index == functions_needed - 1 {
            0
        } else {
            index + 1
        };
        let mut internal = internal.clone();

        internal.sig.ident = Ident::new(
            &format!("__internal_{}_{}", index, &external_ident),
            Span::call_site(),
        );

        internal.block.stmts.insert(0, if_end.clone());

        let internal_next_ident = Ident::new(
            &format!("__internal_{}_{}", next_index, external_ident),
            Span::call_site(),
        );
        internal.block.stmts.push(syn::parse2(
            quote! {#internal_next_ident::<#generic_ident::NEXT>(#(#inputs),*);},
        )?);

        output.extend(quote! {#internal});
    }

    Ok(output)
}
