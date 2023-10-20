use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DataEnum, DeriveInput};

/// Merges the variants of two enums.
///
/// Adapted from Stargaze Launchpad:
/// https://github.com/public-awesome/launchpad/tree/main/packages/mint-hooks/derive
fn merge_variants(metadata: TokenStream, left: TokenStream, right: TokenStream) -> TokenStream {
    use syn::Data::Enum;

    // parse metadata
    let args = parse_macro_input!(metadata as AttributeArgs);
    if let Some(first_arg) = args.first() {
        return syn::Error::new_spanned(first_arg, "macro takes no arguments")
            .to_compile_error()
            .into();
    }

    // parse the left enum
    let mut left: DeriveInput = parse_macro_input!(left);
    let Enum(DataEnum {
        variants,
        ..
    }) = &mut left.data else {
        return syn::Error::new(left.ident.span(), "only enums can accept variants")
            .to_compile_error()
            .into();
    };

    // parse the right enum
    let right: DeriveInput = parse_macro_input!(right);
    let Enum(DataEnum {
        variants: to_add,
        ..
    }) = right.data else {
        return syn::Error::new(left.ident.span(), "only enums can provide variants")
            .to_compile_error()
            .into();
    };

    // insert variants from the right to the left
    variants.extend(to_add.into_iter());

    quote! { #left }.into()
}




/// Note: `#[valid_signature_query]` must be applied _before_ `#[cw_serde]`.
#[proc_macro_attribute]
pub fn valid_signature_query(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
            enum Right {
                #[returns(ValidSignatureResponse)]
                ValidSignature {
                    data: Binary,
                    signature: Binary,
                    payload: Option<Binary>
                },

                #[returns(ValidSignaturesResponse)]
                ValidSignatures {
                    data: Vec<Binary>,
                    signatures: Vec<Binary>,
                    payload: Option<Binary>
                }
            }
        }
        .into(),
    )
}