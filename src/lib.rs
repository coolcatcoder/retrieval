#![doc = include_str!("../README.md")]

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::sync::{
    OnceLock,
    atomic::{AtomicU32, Ordering},
};
use syn::{
    FnArg, Ident, ImplItem, ItemFn, ItemImpl, ItemTrait, LitInt, Pat, Stmt, Token, TraitItem, Type,
    Visibility, parse::Parse, parse_macro_input, spanned::Spanned,
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

/// Place on a trait to turn it into a retrieval trait, which is capable of collecting implementations.  
/// Optionally allows the capacity to be specified. Defaults to 1000. The higher the capacity, the longer it will take to compile.
///
/// The trait becomes a module. It contains the QUANTITY of implementations as well as the actual Trait.
///
/// Due to how this works internally, there are a few restrictions:  
/// All associated items must have a default. (Allows associated types to have defaults, which would normally not be allowed.)  
/// You can only have a max of 5 retrieval traits in 1 crate. (If you need more than 5, then please raise an issue on github!)
/// ```rust
/// # use retrieval::retrieve;
/// # fn main() {}
/// #[retrieve(capacity=10)]
/// trait Message {
///     const STR: &str = "";
/// }
/// const MESSAGE_QUANTITY: usize = Message::QUANTITY;
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

    let trait_ident = std::mem::replace(&mut item.ident, Ident::new("Trait", Span::call_site()));
    let trait_vis = std::mem::replace(&mut item.vis, Visibility::Public(Default::default()));

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
            let generics = &item.generics;
            let default_type = default.1;

            Some(quote! {type #ident #generics = #default_type;})
        })
        .collect();

    item.items.push(TraitItem::Verbatim(quote! {
        /// The next type in the chain.
        type NEXT: Trait;

        /// Is this the end of the chain?
        const END: bool = false;
    }));

    // One extra for the one required impl.
    let switches = generate_switches(capacity + 1);

    let output = quote! {
        #[allow(non_snake_case)]
        #trait_vis mod #trait_ident {
            // Make sure everything is in scope.
            use super::*;

            #item

            /// The amount of implementations of this trait.
            pub const QUANTITY: usize = {
                const fn get_quantity<const INDEX: usize>() -> usize
                where
                    __internal::Container<INDEX>: __internal::Final,
                {
                    INDEX
                }
                get_quantity()
            };

            #[doc(hidden)]
            /// !!! INTERNAL !!!
            /// If you are not a maintainer of the retrieval crate, then you are lost.
            ///
            /// Contains internal implementation details.
            pub mod __internal {
                /// Self is the same type as T.
                /// Used to bypass trivial bounds.
                pub trait Is<T> {}
                impl<T> Is<T> for T {}

                /// The final implementation.
                /// Only implemented once, at the end.
                pub trait Final {}

                /// Contains the retrieved implementations.
                /// Each implementation is stored under a different INDEX.
                pub struct Container<const INDEX: usize>;

                #switches
            }
        }

        // Can't be hidden in the internal module, as that may put the default types out of scope.
        impl #trait_ident::Trait for #trait_ident::__internal::Container<0> {
            #(#default_types)*
            type NEXT = Self;
            const END: bool = true;
        }
        impl<T: #trait_ident::__internal::Is<#trait_ident::__internal::Container<0>>> #trait_ident::__internal::Final for T
        where #trait_ident::__internal::Switch0<T, true>: core::marker::Unpin,
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
            pub struct #ident<T, const BOOL: bool>(core::marker::PhantomData<T>);
        });
    });
    output
}

/// A single counter of trait implementations.
/// Tells traits apart using the crate name and the trait name.
struct TraitCounter {
    crate_name: String,
    trait_name: String,

    counter: AtomicU32,
}

/// No deadlock way of counting trait implementations.
struct TraitCounters([OnceLock<TraitCounter>; 5], OnceLock<Box<TraitCounters>>);

impl TraitCounters {
    const fn new() -> Self {
        Self([const { OnceLock::new() }; 5], OnceLock::new())
    }

    fn get(&self, trait_name: String) -> Result<u32, std::env::VarError> {
        let crate_name: String = std::env::var("CARGO_CRATE_NAME")?;

        Ok(self.get_internal(crate_name, trait_name))
    }

    fn get_internal(&self, crate_name: String, trait_name: String) -> u32 {
        for trait_counter in &self.0 {
            let trait_counter = trait_counter.get_or_init(|| TraitCounter {
                crate_name: crate_name.clone(),
                trait_name: trait_name.clone(),

                counter: AtomicU32::new(0),
            });

            if trait_counter.crate_name == crate_name && trait_counter.trait_name == trait_name {
                return trait_counter.counter.fetch_add(1, Ordering::Relaxed);
            }
        }

        let next = self.1.get_or_init(|| Box::new(TraitCounters::new()));
        next.get_internal(crate_name, trait_name)
    }
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
    // TODO: A max of 5 traits? That sucks.
    static TRAIT_COUNTERS: TraitCounters = TraitCounters::new();

    let Type::Path(trait_path) = &*item.self_ty else {
        return Err(syn::Error::new(
            item.self_ty.span(),
            "The inherent impl's type is not a retrieval trait, which doesn't make sense.",
        ));
    };
    let trait_path = trait_path.path.clone();
    let trait_ident_string = trait_path.segments.last().unwrap().ident.to_string();
    let Ok(index) = TRAIT_COUNTERS.get(trait_ident_string) else {
        return Err(syn::Error::new(input.span(), "Could not get crate name."));
    };

    let index_previous = LitInt::new(&(index).to_string(), Span::call_site());
    let index_current = LitInt::new(&(index + 1).to_string(), Span::call_site());

    *item.self_ty = Type::Verbatim(quote! {#trait_path::__internal::Container<#index_current>});

    item.trait_ = Some((
        None,
        syn::parse2(quote! {#trait_path::Trait})?,
        Default::default(),
    ));

    item.items.push(ImplItem::Verbatim(quote! {
        type NEXT = #trait_path::__internal::Container<#index_previous>;
    }));

    let switch_previous = Ident::new(&format!("Switch{index}"), Span::call_site());
    let switch_current = Ident::new(&format!("Switch{}", index + 1), Span::call_site());

    let output = quote! {
        #item

        impl<T> core::marker::Unpin for #trait_path::__internal::#switch_previous<T, false> {}
        impl<T: #trait_path::__internal::Is<#trait_path::__internal::Container<#index_current>>> #trait_path::__internal::Final for T
        where #trait_path::__internal::#switch_current<T, true>: core::marker::Unpin,
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
/// const MESSAGES: [&str; Message::QUANTITY] = {
///     let mut messages = [""; Message::QUANTITY];
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

fn iterate_internal(input: TokenStream, mut internal: ItemFn) -> syn::Result<TokenStream> {
    // Work out how many functions we will need to reach the target recursion limit.
    let recursion_limit = syn::parse2::<NumberAttribute<1000>>(input)?.0;
    let functions_needed = recursion_limit.div_ceil(128);

    if internal.sig.generics.params.len() != 1 {
        return Err(syn::Error::new(
            internal.sig.generics.span(),
            "Only one generic is supported at this time.\nPlease see https://github.com/coolcatcoder/retrieval/issues/7.",
        ));
    }

    let module_path: syn::Path = {
        let generic = internal.sig.generics.type_params_mut().next().unwrap();
        let syn::TypeParamBound::Trait(trait_bound) = generic.bounds.first_mut().unwrap() else {
            return Err(syn::Error::new(
                generic.bounds.span(),
                "The singular generic should only have one trait bound.\nPlease see https://github.com/coolcatcoder/retrieval/issues/7.",
            ));
        };
        let module_path = trait_bound.path.clone();
        trait_bound.path.segments.push(syn::parse2(quote! {Trait})?);
        module_path
    };
    let generic_ident = &internal.sig.generics.type_params().next().unwrap().ident;

    // Get the module from the last segment of the trait bound.
    //let trait_ident = &trait_bound.path.segments.last().unwrap().ident;

    // Create the external function's signature from the internal's but without the generics.
    let mut external_sig = internal.sig.clone();
    let external_vis = internal.vis.clone();
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
        #external_vis #external_sig {
            #internal_start_ident::<#module_path::__internal::Container<{#module_path::QUANTITY}>>(#(#inputs),*);
        }
    };

    let if_end: Stmt = syn::parse2(quote! {
        if #generic_ident::END {
            return;
        }
    })?;
    internal.vis = Visibility::Inherited;

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
