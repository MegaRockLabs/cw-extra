use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, AttributeArgs, DataEnum, DeriveInput};


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
pub fn basic_smart_account_query(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
            enum Right {

                /// cw1
                #[returns(CanExecuteResponse)]
                CanExecute { sender: String, msg: CosmosMsg },


                /// cw81
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



/// Note: `#[valid_signature_query]` must be applied _before_ `#[cw_serde]`.
#[proc_macro_attribute]
pub fn smart_account_query(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
            enum Right {

                /// cw1
                #[returns(CanExecuteResponse)]
                CanExecute { sender: String, msg: CosmosMsg<T> },


                /// cw81
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




#[proc_macro_attribute]
pub fn extended_smart_account_query(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
            enum Right {

                /// cw1
                #[returns(CanExecuteResponse)]
                CanExecute { sender: String, msg: CosmosMsg<T> },

                /// cw2 for non-raw
                /// not necessary since be queried by raw
                /// useful if raw query not available and
                /// as syntactic sugar for consistency 
                #[returns(ContractVersion)]
                ContractVersion {},

                /// cw22 is not confirmed and still in PR
                /// this is a custom extended version
                #[returns(bool)]
                SupportedInterface {
                    name: String,
                    version: Option<String>,
                },

                #[returns(Vec<bool>)]
                SupportedInterfaces {
                    interfaces: Vec<(String, Option<String>)>
                },

                /// cw81
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