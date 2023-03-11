//! This crate contains an attribute macro `#[bool_like]`. By default, this macro just implements
//! [core::ops::Not] for a simple two-variant enum. `!` applied to one variant will produce the
//! other.
//!
//! Optionally, the sub-attribute `#[into_false]` may be applied to one of the variants to indicate
//! that variant is equivalent to `false`, and the other variant is equivalent to `true`.
//! [core::convert::From] will be implemented for both the enum and `bool`.
//!
//! A chess piece color is a good use case for a two-variant enum. The [core::ops::Not]
//! implementation can be treated as if it were a bool, because you know you will never have more
//! than two players in Chess (unless you frequent /r/anarchychess)
//! ```
//! use bool_like::bool_like;
//!
//! /// A Chess piece color.
//! #[derive(Copy, Clone, Debug, PartialEq, Eq)]
//! #[bool_like]
//! enum Player {
//!     Black,
//!     White,
//! }
//!
//! // core::ops::Not is implemented for Player
//! assert_eq!(! Player::Black, Player::White);
//! assert_eq!(! Player::White, Player::Black);
//! ```
//!
//! For some types, it makes sense for each variant to map to `true` or `false`
//! ```
//! use bool_like::bool_like;
//!
//! /// An answer to a question.
//! #[derive(Copy, Clone, Debug, PartialEq, Eq)]
//! #[bool_like]
//! enum Answer {
//!     #[into_false]
//!     No,
//!     Yes,
//! }
//!
//! // The same thing we did before
//! assert_eq!(! Answer::No, Answer::Yes);
//! assert_eq!(! Answer::Yes, Answer::No);
//!
//! // `core::convert::From<bool>` is implemented for Answer
//! assert_eq!(Answer::No, Answer::from(false));
//! assert_eq!(Answer::Yes, Answer::from(true));
//!
//! // `core::convert::From<Answer>` is implemented for bool
//! assert_eq!(bool::from(Answer::No), false);
//! assert_eq!(bool::from(Answer::Yes), true);
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

const INTO_FALSE: &str = "into_false";

/// Implement `core::ops::Not` for a two-variant enum and, optionally, if the `[into_false]` macro
/// is applied, `core::convert::From<bool>` and `core::convert::Into<bool>`.
#[proc_macro_attribute]
pub fn bool_like(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(input as DeriveInput);

    // Ensure that the input is an enum with exactly two variants
    let data_enum = match ast.data {
        syn::Data::Enum(ref mut data_enum) => data_enum,
        _ => panic!("The `bool_like` attribute can only be used for enums"),
    };
    if data_enum.variants.len() != 2 {
        panic!("The `bool_like` attribute can only be derived for enums with exactly two variants");
    }

    // Check if one of the variants has the `#[into_false]` attribute
    let mut variant_false = None;
    for variant in data_enum.variants.iter_mut() {
        for attr in &variant.attrs {
            if attr.path.is_ident(INTO_FALSE) {
                variant_false = Some(variant.ident.clone());
                break;
            }
        }
        if variant_false.is_some() {
            variant.attrs = variant
                .attrs
                .iter()
                .cloned()
                .filter(|attr| !attr.path.is_ident(INTO_FALSE))
                .collect();
            break;
        }
    }

    // The name of the enum
    let ident = ast.ident.clone();

    // The names of the two variants
    let variant1 = data_enum.variants[0].ident.clone();
    let variant2 = data_enum.variants[1].ident.clone();

    // Generate the implementation of `Not` for the enum
    let not_impl = quote! {
        impl ::core::ops::Not for #ident {
            type Output = #ident;
            fn not(self) -> Self::Output {
                match self {
                    #ident::#variant1 => #ident::#variant2,
                    #ident::#variant2 => #ident::#variant1,
                }
            }
        }
    };

    // Generate the core::convert implementations (if applicable)
    let into_bool_impl = match variant_false {
        Some(variant) => quote! {
            impl ::core::convert::From<#ident> for bool {
                fn from(other: #ident) -> Self {
                    if let #ident::#variant = other {
                        false
                    } else {
                        true
                    }
                }
            }
            impl ::core::convert::From<bool> for #ident {
                fn from(other: bool) -> Self {
                    if other == false {
                        #ident::#variant
                    } else {
                        ! #ident::#variant
                    }
                }
            }
        },
        None => quote! {},
    };

    let gen = quote! {
        #ast
        #not_impl
        #into_bool_impl
    };

    gen.into()
}
