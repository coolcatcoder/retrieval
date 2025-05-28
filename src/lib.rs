#![doc = include_str!("../README.md")]
#![deny(clippy::unwrap_used)]
#![warn(clippy::pedantic)]

use proc_macro::TokenStream as StdTokenStream;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::env::VarError;
use syn::{
    FnArg, Generics, Ident, ImplItem, ItemFn, ItemImpl, ItemTrait, LitInt, Pat, Path, Stmt, Token,
    TraitItem, Type, Visibility, parse::Parse, parse_macro_input, spanned::Spanned,
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

fn internal_path(trait_path: &Path) -> Result<Path, syn::Error> {
    syn::parse2(quote! {#trait_path::__internal})
}

/// Place on a trait to turn it into a retrieval trait, which is capable of collecting implementations.\
/// Optionally allows the capacity to be specified. Defaults to 1000. The higher the capacity, the longer it will take to compile.
///
/// The trait becomes a module. It contains the QUANTITY of implementations as well as the actual Trait.
///
/// Due to how this works internally, there are a few restrictions:\
/// All associated items must have a default. (Allows associated types to have defaults, which would normally not be allowed.)
/// ```rust
/// # use retrieval::retrieve;
/// # fn main() {}
/// #[retrieve(capacity=10)]
/// trait Message {
///     const STR: &str = "";
/// }
/// const MESSAGE_QUANTITY: u16 = Message::QUANTITY;
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
    let trait_vis = std::mem::replace(&mut item.vis, Visibility::Public(<Token![pub]>::default()));

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
    let setup = replaceable_implementations::setup(capacity + 1);
    let path_to_setup: Path = syn::parse2(quote! {#trait_ident::__internal})?;
    let initial_implementation = replaceable_implementations::initial_implementation(
        &path_to_setup,
        syn::parse2(quote! {
            impl #trait_ident::__internal::Final for #trait_ident::__internal::Container<0> {}
        })?,
    )?;

    let output = quote! {
        #[allow(non_snake_case)]
        #trait_vis mod #trait_ident {
            // Make sure everything is in scope.
            use super::*;

            #item

            /// The amount of implementations of this trait.
            pub const QUANTITY: u16 = {
                const fn get_quantity<const INDEX: u16>() -> u16
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
                /// The final implementation.
                /// Only implemented once, at the end.
                pub trait Final {}

                /// Contains the retrieved implementations.
                /// Each implementation is stored under a different INDEX.
                pub struct Container<const INDEX: u16>;

                #setup
            }
        }

        // Can't be hidden in the internal module, as that may put the default types out of scope.
        impl #trait_ident::Trait for #trait_ident::__internal::Container<0> {
            #(#default_types)*
            type NEXT = Self;
            const END: bool = true;
        }
        #initial_implementation
    };

    Ok(output)
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
    send_internal(&input.into(), item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn send_internal(input: &TokenStream, mut item: ItemImpl) -> syn::Result<TokenStream> {
    if !input.is_empty() {
        return Err(syn::Error::new(
            input.span(),
            "This attribute accepts nothing but itself.",
        ));
    }

    let Type::Path(trait_path) = &*item.self_ty else {
        return Err(syn::Error::new(
            item.self_ty.span(),
            "The inherent impl's type is not a retrieval trait, which doesn't make sense.",
        ));
    };
    let trait_path = trait_path.path.clone();
    let internal_path = internal_path(&trait_path)?;
    let trait_ident_string = trait_path
        .segments
        .last()
        .ok_or(syn::Error::new(
            trait_path.span(),
            "Expected the retrieval trait path to contain at least one segment.",
        ))?
        .ident
        .to_string();

    let (previous_implementations, replace) =
        replaceable_implementations::replace_implementation(
            &internal_path,
            trait_ident_string,
            true,
        )
        .map_err(|err| match err {
            VarError::NotPresent => syn::Error::new(
                Span::call_site(),
                "The crate name was not present in the environment variables.",
            ),
            VarError::NotUnicode(crate_name) => syn::Error::new(
                Span::call_site(),
                format!(
                    "The crate name was not unicode. Crate name: {}",
                    crate_name.display(),
                ),
            ),
        })?;
    let implementation = replace(syn::parse2(quote! {
        impl #internal_path::Final for #internal_path::Container<#previous_implementations> {}
    })?);

    let current = LitInt::new(&(previous_implementations).to_string(), Span::call_site());
    let next = LitInt::new(
        &(previous_implementations - 1).to_string(),
        Span::call_site(),
    );

    *item.self_ty = Type::Verbatim(quote! {#trait_path::__internal::Container<#current>});

    item.trait_ = Some((
        None,
        syn::parse2(quote! {#trait_path::Trait})?,
        <Token![for]>::default(),
    ));

    item.items.push(ImplItem::Verbatim(quote! {
        type NEXT = #trait_path::__internal::Container<#next>;
    }));

    let output = quote! {
        #item
        #implementation
    };

    Ok(output)
}

/// Place on a generic function to convert it into a non-generic function that iterates through all implementations of a [retrieval trait](macro@retrieve).\
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
/// const MESSAGES: [&str; Message::QUANTITY as usize] = {
///     let mut messages = [""; Message::QUANTITY as usize];
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
        let type_param_span = internal.sig.generics.span();
        let generic = internal
            .sig
            .generics
            .type_params_mut()
            .next()
            .ok_or(syn::Error::new(
                type_param_span,
                "Expected a single type param.",
            ))?;
        let syn::TypeParamBound::Trait(trait_bound) = generic
            .bounds
            .first_mut()
            .ok_or(syn::Error::new(type_param_span, "Expected a bound."))?
        else {
            return Err(syn::Error::new(
                generic.bounds.span(),
                "The singular generic should only have one trait bound.\nPlease see https://github.com/coolcatcoder/retrieval/issues/7.",
            ));
        };
        let module_path = trait_bound.path.clone();
        trait_bound.path.segments.push(syn::parse2(quote! {Trait})?);
        module_path
    };
    let generic_ident = &internal
        .sig
        .generics
        .type_params()
        .next()
        .expect("TO DO: Come up with a good reason why I expect this to never fail.")
        .ident;

    // Get the module from the last segment of the trait bound.
    //let trait_ident = &trait_bound.path.segments.last().unwrap().ident;

    // Create the external function's signature from the internal's but without the generics.
    let mut external_sig = internal.sig.clone();
    let external_vis = internal.vis.clone();
    external_sig.generics = Generics::default();
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
    let internal_start_ident =
        Ident::new(&format!("__internal_0_{external_ident}"), Span::call_site());

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
            &format!("__internal_{next_index}_{external_ident}"),
            Span::call_site(),
        );
        internal.block.stmts.push(syn::parse2(
            quote! {#internal_next_ident::<#generic_ident::NEXT>(#(#inputs),*);},
        )?);

        output.extend(quote! {#internal});
    }

    Ok(output)
}
