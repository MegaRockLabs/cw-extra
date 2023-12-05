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



#[proc_macro_attribute]
pub fn smart_account_query(metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        metadata,
        input,
        quote! {
            enum Right {

                /// cw1
                #[returns(::cw82::CanExecuteResponse)]
                CanExecute { 
                    sender: String, 
                    msg: ::cosmwasm_std::CosmosMsg<T> 
                },

                /// cw81
                #[returns(::cw82::ValidSignatureResponse)]
                ValidSignature {
                    data: ::cosmwasm_std::Binary,
                    signature: ::cosmwasm_std::Binary,
                    payload: Option<::cosmwasm_std::Binary>
                },

                #[returns(::cw82::ValidSignaturesResponse)]
                ValidSignatures {
                    data: Vec<::cosmwasm_std::Binary>,
                    signatures: Vec<::cosmwasm_std::Binary>,
                    payload: Option<::cosmwasm_std::Binary>
                }
            }
        }
        .into(),
    )
}