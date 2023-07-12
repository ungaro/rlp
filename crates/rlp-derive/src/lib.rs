#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/alloy.jpg",
    html_favicon_url = "https://raw.githubusercontent.com/alloy-rs/core/main/assets/favicon.ico"
)]
#![warn(missing_docs, unreachable_pub, rustdoc::all)]
#![deny(unused_must_use, unused_crate_dependencies)]
#![cfg_attr(docsrs, feature(doc_cfg, doc_auto_cfg))]

extern crate proc_macro;

mod de;
mod en;
mod utils;

use de::*;
use en::*;
use proc_macro::TokenStream;

/// Derives `Encodable` for the type which encodes the all fields as list:
/// `<rlp-header, fields...>`
#[proc_macro_derive(RlpEncodable, attributes(rlp))]
pub fn encodable(input: TokenStream) -> TokenStream {
    syn::parse(input)
        .and_then(|ast| impl_encodable(&ast))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives `Encodable` for the type which encodes the fields as-is, without a
/// header: `<fields...>`
#[proc_macro_derive(RlpEncodableWrapper, attributes(rlp))]
pub fn encodable_wrapper(input: TokenStream) -> TokenStream {
    syn::parse(input)
        .and_then(|ast| impl_encodable_wrapper(&ast))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives `MaxEncodedLen` for types of constant size.
#[proc_macro_derive(RlpMaxEncodedLen, attributes(rlp))]
pub fn max_encoded_len(input: TokenStream) -> TokenStream {
    syn::parse(input)
        .and_then(|ast| impl_max_encoded_len(&ast))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives `Decodable` for the type whose implementation expects an rlp-list
/// input: `<rlp-header, fields...>`
///
/// This is the inverse of `RlpEncodable`.
#[proc_macro_derive(RlpDecodable, attributes(rlp))]
pub fn decodable(input: TokenStream) -> TokenStream {
    syn::parse(input)
        .and_then(|ast| impl_decodable(&ast))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Derives `Decodable` for the type whose implementation expects only the
/// individual fields encoded: `<fields...>`
///
/// This is the inverse of `RlpEncodableWrapper`.
#[proc_macro_derive(RlpDecodableWrapper, attributes(rlp))]
pub fn decodable_wrapper(input: TokenStream) -> TokenStream {
    syn::parse(input)
        .and_then(|ast| impl_decodable_wrapper(&ast))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}
