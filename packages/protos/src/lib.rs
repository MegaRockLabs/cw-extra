mod utils;
use quote::{quote, ToTokens};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, NestedMeta};
use utils::{comp_err, has_generic_t, merge_variants};





/// Procedural macro to extend an enum with standardized signature validation variants.
///
/// This macro inserts additional variants into a `QueryMsg` enum for signature validation in smart
/// contracts, as defined in the CW81 specification. It supports an optional custom payload type to
/// override the default `Binary` type for the `payload` field, which remains wrapped in `Option`.
///
/// # Arguments
///
/// The macro accepts **zero or one type argument** via the attribute:
/// - **Payload type** (optional): The type for the `payload` field in the `ValidSignature` and
///   `ValidSignatures` variants (e.g., `CustomPayload`), wrapped in `Option`. Defaults to
///   `Option<cosmwasm_std::Binary>` if not provided.
///
/// # Generated Variants
///
/// The macro inserts the following query variants:
/// - `ValidSignature`: Verifies a single signature against provided data and an optional payload.
/// - `ValidSignatures`: Verifies multiple signatures against a list of data and an optional payload
///   (included due to the `multi` feature).
///
/// # Notes
/// - This is the `multi` feature version, which includes the `ValidSignatures` variant for batch
///   verification.
/// - The `#[valid_signature_multi]` attribute must be applied **before** `#[cw_serde]` or other
///   derive macros.
/// - The enum must derive `#[derive(QueryResponses)]` from `cosmwasm_schema` to support the
///   `#[returns(...)]` attributes used in the generated variants.
///
/// # Examples
///
/// ## Example 1: Basic usage with default payload type
///
/// ```rust,ignore
/// use cw81::valid_signature_multi;
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::Binary;
///
/// #[valid_signature_multi]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw81::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<::cosmwasm_std::Binary>,
/// //     },
/// //
/// //     #[returns(::cw81::ValidSignaturesResponse)]
/// //     ValidSignatures {
/// //         data: Vec<::cosmwasm_std::Binary>,
/// //         signatures: Vec<::cosmwasm_std::Binary>,
/// //         payload: Option<::cosmwasm_std::Binary>,
/// //     },
/// // }
/// ```
///
/// ## Example 2: Usage with a custom payload type
///
/// ```rust,ignore
/// #[cw_serde]
/// pub struct CustomPayload {
///     pub metadata: String,
/// }
///
/// #[valid_signature_multi(CustomPayload)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw81::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<CustomPayload>,
/// //     },
/// //
/// //     #[returns(::cw81::ValidSignaturesResponse)]
/// //     ValidSignatures {
/// //         data: Vec<::cosmwasm_std::Binary>,
/// //         signatures: Vec<::cosmwasm_std::Binary>,
/// //         payload: Option<CustomPayload>,
/// //     },
/// // }
/// ```
///
/// # Errors
///
/// - Fails with a compile-time error if the attribute argument is invalid (e.g., not a type path or
///   more than one argument).
/// - Fails if the input is not a valid enum or if the merge with generated variants cannot be
///   performed.
///
/// This macro is part of the CW81 specification for signature validation.
#[proc_macro_attribute]
pub fn valid_signature_multi(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);

    let payload_type = match args.len() {
        0 => quote!(Option<::cosmwasm_std::Binary>),
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => quote!(Option<#path>),
            other => return comp_err!(other, "Expected a type for the payload, like `CustomPayload`"),
        },
        _ => return comp_err!(&args[1], "Expected at most 1 argument"),
    };

    merge_variants(
        input,
        quote! {
            enum Right {
                #[returns(::cw81::ValidSignatureResponse)]
                ValidSignature {
                    data: ::cosmwasm_std::Binary,
                    signature: ::cosmwasm_std::Binary,
                    payload: #payload_type
                },
                #[returns(::cw81::ValidSignaturesResponse)]
                ValidSignatures {
                    data: Vec<::cosmwasm_std::Binary>,
                    signatures: Vec<::cosmwasm_std::Binary>,
                    payload: #payload_type
                }
            }
        }
        .into(),
    )
}


/// Procedural macro to extend an enum with a standardized signature validation variant.
///
/// This macro inserts a single variant into a `QueryMsg` enum for signature validation in smart
/// contracts, as defined in the CW81 specification. It supports an optional custom payload type to
/// override the default `Binary` type for the `payload` field, which remains wrapped in `Option`.
///
/// # Arguments
///
/// The macro accepts **zero or one type argument** via the attribute:
/// - **Payload type** (optional): The type for the `payload` field in the `ValidSignature` variant
///   (e.g., `CustomPayload`), wrapped in `Option`. Defaults to `Option<cosmwasm_std::Binary>` if not
///   provided.
///
/// # Generated Variants
///
/// The macro inserts the following query variant:
/// - `ValidSignature`: Verifies a single signature against provided data and an optional payload.
///
/// # Notes
/// - The `#[valid_signature_one]` attribute must be applied **before** `#[cw_serde]` or other derive
///   macros.
/// - The enum must derive `#[derive(QueryResponses)]` from `cosmwasm_schema` to support the
///   `#[returns(...)]` attributes used in the generated variant.
/// - This macro is designed for single-signature validation, excluding the `ValidSignatures` variant
///   included in the `multi` version.
///
/// # Examples
///
/// ## Example 1: Basic usage with default payload type
///
/// ```rust,ignore
/// use cw81::valid_signature_one;
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::Binary;
///
/// #[valid_signature_one]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw81::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<::cosmwasm_std::Binary>,
/// //     },
/// // }
/// ```
///
/// ## Example 2: Usage with a custom payload type
///
/// ```rust,ignore
/// #[cw_serde]
/// pub struct CustomPayload {
///     pub metadata: String,
/// }
///
/// #[valid_signature_one(CustomPayload)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw81::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<CustomPayload>,
/// //     },
/// // }
/// ```
///
/// # Errors
///
/// - Fails with a compile-time error if the attribute argument is invalid (e.g., not a type path or
///   more than one argument).
/// - Fails if the input is not a valid enum or if the merge with generated variants cannot be
///   performed.
///
/// This macro is part of the CW81 specification for signature validation.
#[proc_macro_attribute]
pub fn valid_signature_one(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);

    let payload_type = match args.len() {
        0 => quote!(Option<::cosmwasm_std::Binary>),
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => quote!(Option<#path>),
            other => return comp_err!(other, "Expected a type for the payload, like `CustomPayload`"),
        },
        _ => return comp_err!(&args[1], "Expected at most 1 argument"),
    };

    merge_variants(
        input,
        quote! {
            enum Right {
                #[returns(::cw81::ValidSignatureResponse)]
                ValidSignature {
                    data: ::cosmwasm_std::Binary,
                    signature: ::cosmwasm_std::Binary,
                    payload: #payload_type
                },
            }
        }
        .into(),
    )
}



/// Procedural macro to extend an enum with standardized smart account query variants.
///
/// This macro checks whether the input enum is generic over `T` and inserts variants accordingly. If
/// the enum is generic over `T`, the `CanExecute` variant uses `CosmosMsg<T>`; otherwise, it uses the
/// non-generic `CosmosMsg`. It supports an optional custom payload type to override the default
/// `Binary` type for the `payload` field, which remains wrapped in `Option`.
///
/// This is the `multi` feature version, which includes an additional `ValidSignatures` variant.
///
/// # Arguments
///
/// The macro accepts **zero or one type argument** via the attribute:
/// - **Payload type** (optional): The type for the `payload` field in the `ValidSignature` and
///   `ValidSignatures` variants (e.g., `CustomPayload`), wrapped in `Option`. Defaults to
///   `Option<cosmwasm_std::Binary>` if not provided.
///
/// # Generated Variants
///
/// The macro inserts the following query variants:
/// - `CanExecute`: Queries whether a message can be executed by a smart account.
/// - `ValidSignature`: Verifies a single signature against provided data and an optional payload.
/// - `ValidSignatures`: Verifies multiple signatures against a list of data and an optional payload.
///
/// # Notes
/// - The `#[account_query_multi]` attribute must be applied **before** `#[cw_serde]` or other derive
///   macros.
/// - The enum must derive `#[derive(QueryResponses)]` from `cosmwasm_schema` to support the
///   `#[returns(...)]` attributes used in the generated variants.
/// - This macro is part of the CW82 specification for smart accounts.
///
/// # Examples
///
/// ## Example 1: Basic usage with default payload type
///
/// ```rust,ignore
/// use cw82::account_query_multi;
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::{Binary, Empty};
///
/// #[account_query_multi]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]
/// //     CanExecute {
/// //         sender: String,
/// //         msg: ::cosmwasm_std::CosmosMsg,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<::cosmwasm_std::Binary>,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignaturesResponse)]
/// //     ValidSignatures {
/// //         data: Vec<::cosmwasm_std::Binary>,
/// //         signatures: Vec<::cosmwasm_std::Binary>,
/// //         payload: Option<::cosmwasm_std::Binary>,
/// //     },
/// // }
/// ```
///
/// ## Example 2: Usage with a custom payload type
///
/// ```rust,ignore
/// #[cw_serde]
/// pub struct CustomPayload {
///     pub metadata: String,
/// }
///
/// #[account_query_multi(CustomPayload)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]
/// //     CanExecute {
/// //         sender: String,
/// //         msg: ::cosmwasm_std::CosmosMsg,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<CustomPayload>,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignaturesResponse)]
/// //     ValidSignatures {
/// //         data: Vec<::cosmwasm_std::Binary>,
/// //         signatures: Vec<::cosmwasm_std::Binary>,
/// //         payload: Option<CustomPayload>,
/// //     },
/// // }
/// ```
///
/// ## Example 3: Generic enum with custom payload type
///
/// ```rust,ignore
/// #[account_query_multi(CustomPayload)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsgCustom<T> {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsgCustom<T> {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]
/// //     CanExecute {
/// //         sender: String,
/// //         msg: ::cosmwasm_std::CosmosMsg<T>,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<CustomPayload>,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignaturesResponse)]
/// //     ValidSignatures {
/// //         data: Vec<::cosmwasm_std::Binary>,
/// //         signatures: Vec<::cosmwasm_std::Binary>,
/// //         payload: Option<CustomPayload>,
/// //     },
/// // }
/// ```
///
/// # Errors
///
/// - Fails with a compile-time error if the attribute argument is invalid (e.g., not a type path or
///   more than one argument).
/// - Fails if the input is not a valid enum or if the merge with generated variants cannot be
///   performed.
///
/// This macro is part of the CW82 specification for smart accounts.
#[proc_macro_attribute]
pub fn account_query_multi(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);

    let payload_type = match args.len() {
        0 => quote!(Option<::cosmwasm_std::Binary>),
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => quote!(Option<#path>),
            other => return comp_err!(other, "Expected a type for the payload, like `CustomPayload`"),
        },
        _ => return comp_err!(&args[1], "Expected at most 1 argument"),
    };

    let cloned = input.clone();
    let ast: DeriveInput = parse_macro_input!(cloned as DeriveInput);
    let has_t = has_generic_t(&ast.generics);
    let msg_type = if has_t {
        quote!(::cosmwasm_std::CosmosMsg<T>)
    } else {
        quote!(::cosmwasm_std::CosmosMsg)
    };

    let right_enum = quote! {
        enum Right {
            #[returns(::cw82::CanExecuteResponse)]
            CanExecute {
                sender: String,
                msg: #msg_type
            },
            #[returns(::cw82::ValidSignatureResponse)]
            ValidSignature {
                data: ::cosmwasm_std::Binary,
                signature: ::cosmwasm_std::Binary,
                payload: #payload_type
            },
            #[returns(::cw82::ValidSignaturesResponse)]
            ValidSignatures {
                data: Vec<::cosmwasm_std::Binary>,
                signatures: Vec<::cosmwasm_std::Binary>,
                payload: #payload_type
            }
        }
    };

    merge_variants(input, right_enum.into())
}


/// Procedural macro to extend an enum with standardized smart account query variants.
///
/// This macro checks whether the input enum is generic over `T` and inserts variants accordingly. If
/// the enum is generic over `T`, the `CanExecute` variant uses `CosmosMsg<T>`; otherwise, it uses the
/// non-generic `CosmosMsg`. It supports an optional custom payload type to override the default
/// `Binary` type for the `payload` field, which remains wrapped in `Option`.
///
/// # Arguments
///
/// The macro accepts **zero or one type argument** via the attribute:
/// - **Payload type** (optional): The type for the `payload` field in the `ValidSignature` variant
///   (e.g., `CustomPayload`), wrapped in `Option`. Defaults to `Option<cosmwasm_std::Binary>` if not
///   provided.
///
/// # Generated Variants
///
/// The macro inserts the following query variants:
/// - `CanExecute`: Queries whether a message can be executed by a smart account.
/// - `ValidSignature`: Verifies a single signature against provided data and an optional payload.
///
/// # Notes
/// - The `#[account_query_one]` attribute must be applied **before** `#[cw_serde]` or other derive
///   macros.
/// - The enum must derive `#[derive(QueryResponses)]` from `cosmwasm_schema` to support the
///   `#[returns(...)]` attributes used in the generated variants.
/// - This macro is designed for single-signature validation, excluding the `ValidSignatures` variant
///   included in the `multi` version.
///
/// # Examples
///
/// ## Example 1: Basic usage with default payload type
///
/// ```rust,ignore
/// use cw82::account_query_one;
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::{Binary, Empty};
///
/// #[account_query_one]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]
/// //     CanExecute {
/// //         sender: String,
/// //         msg: ::cosmwasm_std::CosmosMsg,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<::cosmwasm_std::Binary>,
/// //     },
/// // }
/// ```
///
/// ## Example 2: Usage with a custom payload type
///
/// ```rust,ignore
/// #[cw_serde]
/// pub struct CustomPayload {
///     pub metadata: String,
/// }
///
/// #[account_query_one(CustomPayload)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]
/// //     CanExecute {
/// //         sender: String,
/// //         msg: ::cosmwasm_std::CosmosMsg,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<CustomPayload>,
/// //     },
/// // }
/// ```
///
/// ## Example 3: Generic enum with custom payload type
///
/// ```rust,ignore
/// #[account_query_one(CustomPayload)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsgCustom<T> {
///     // User-defined queries
/// }
///
/// // Generated:
/// // pub enum QueryMsgCustom<T> {
/// //     // User-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]
/// //     CanExecute {
/// //         sender: String,
/// //         msg: ::cosmwasm_std::CosmosMsg<T>,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data: ::cosmwasm_std::Binary,
/// //         signature: ::cosmwasm_std::Binary,
/// //         payload: Option<CustomPayload>,
/// //     },
/// // }
/// ```
///
/// # Errors
///
/// - Fails with a compile-time error if the attribute argument is invalid (e.g., not a type path or
///   more than one argument).
/// - Fails if the input is not a valid enum or if the merge with generated variants cannot be
///   performed.
///
/// This macro is part of the CW82 specification for smart accounts.
#[proc_macro_attribute]
pub fn account_query_one(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(metadata as AttributeArgs);

    let payload_type = match args.len() {
        0 => quote!(Option<::cosmwasm_std::Binary>),
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => quote!(Option<#path>),
            other => return comp_err!(other, "Expected a type for the payload, like `CustomPayload`"),
        },
        _ => return comp_err!(&args[1], "Expected at most 1 argument"),
    };

    let cloned = input.clone();
    let ast: DeriveInput = parse_macro_input!(cloned as DeriveInput);
    let has_t = has_generic_t(&ast.generics);
    let msg_type = if has_t {
        quote!(::cosmwasm_std::CosmosMsg<T>)
    } else {
        quote!(::cosmwasm_std::CosmosMsg)
    };

    let right_enum = quote! {
        enum Right {
            #[returns(::cw82::CanExecuteResponse)]
            CanExecute {
                sender: String,
                msg: #msg_type
            },
            #[returns(::cw82::ValidSignatureResponse)]
            ValidSignature {
                data: ::cosmwasm_std::Binary,
                signature: ::cosmwasm_std::Binary,
                payload: #payload_type
            }
        }
    };

    merge_variants(input, right_enum.into())
}


/// Procedural macro to extend an enum with a standardized smart account execution variant.
///
/// This macro checks whether the input enum is generic over `T` and inserts a variant accordingly. If
/// the enum is generic over `T`, the `Execute` variant uses `CosmosMsg<T>`; otherwise, it uses the
/// non-generic `CosmosMsg`. The macro is part of the CW82 specification for smart accounts and is
/// designed to standardize execution interfaces for smart accounts.
///
/// # Arguments
///
/// The macro does not accept any type arguments via the attribute.
///
/// # Generated Variants
///
/// The macro inserts the following execute variant:
/// - `Execute`: Executes a list of `CosmosMsg` messages by a smart account.
///
/// # Notes
/// - The `#[account_execute]` attribute must be applied **before** `#[cw_serde]` or other derive
///   macros.
/// - Unlike query macros, this macro does not require `#[derive(QueryResponses)]` since it targets
///   execute messages.
/// - This macro is designed for executing multiple `CosmosMsg` messages, suitable for batch
///   operations in smart accounts.
///
/// # Examples
///
/// ## Example 1: Basic usage with non-generic enum
///
/// ```rust,ignore
/// use cw82::account_execute;
/// use cosmwasm_schema::cw_serde;
///
/// #[account_execute]
/// #[cw_serde]
/// pub enum ExecuteMsg {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsg {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg>,
/// //     },
/// // }
/// ```
///
/// ## Example 2: Usage with a generic enum
///
/// ```rust,ignore
/// #[account_execute]
/// #[cw_serde]
/// pub enum ExecuteMsgCustom<T> {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsgCustom<T> {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg<T>>,
/// //     },
/// // }
/// ```
///
/// # Errors
///
/// - Fails with a compile-time error if the input is not a valid enum or if the merge with the
///   generated variant cannot be performed.
///
/// This macro is part of the CW82 specification for smart account execution.
#[proc_macro_attribute]
pub fn account_execute(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    let cloned = input.clone();
    let ast: DeriveInput = parse_macro_input!(cloned as DeriveInput);
    let has_t = has_generic_t(&ast.generics);
    let msg_type = if has_t {
        quote!(::cosmwasm_std::CosmosMsg<T>)
    } else {
        quote!(::cosmwasm_std::CosmosMsg)
    };
    let right_enum = quote! {
        enum Right {
            Execute {
                msgs: Vec<#msg_type>,
            },
        }
    };
    merge_variants(input, right_enum.into())
}


/// Procedural macro to extend an enum with a standardized `CreateAccount` execute variant
/// for registry-based smart accounts.
///
/// This macro injects a `CreateAccount(...)` variant into your execute enum,
/// where the inner value is `CreateAccountMsg<T>`, a message that allows
/// creating a smart account with optional registration metadata.
///
/// You can optionally specify a single type argument to customize the metadata payload (`T`).
/// If omitted, the type defaults to `Binary`.
///
/// # Examples
///
/// ```rust,ignore
/// use cw83::registry_execute;
/// use cosmwasm_schema::cw_serde;
/// use cosmwasm_std::Binary;
///
/// #[registry_execute]
/// #[cw_serde]
/// pub enum ExecuteMsg {
/// //
/// //    // user-defined variants
/// //
/// }
///
/// // Generated:
/// //
/// // pub struct CreateAccountMsg {
/// //     pub code_id      :  u64,
/// //     pub chain_id     :  String,
/// //     pub account_data :  Binary
/// // }
/// 
/// // pub enum ExecuteMsg {
/// //
/// //    // user-defined variants
/// //
/// //
/// //     CreateAccount(CreateAccountMsg),
/// // }
///
///
/// // With a custom metadata type:
///
/// // types taken from 
/// // pub use smart_account_auth::{CredentialData, Credential};
/// 
/// // mock type
/// pub struct Credential;
/// 
/// pub struct CredentialData {
///      pub credentials     :   Vec<Credential>,
///      pub with_native     :   Option<bool>,
///      pub primary_index   :   Option<u8>,
/// }
///
/// #[registry_execute(CredentialData)]
/// #[cw_serde]
/// pub enum ExecuteMsgData {
/// //
/// //    // user-defined variants
/// //
/// }
///
/// // Generated:
/// 
/// // pub struct CreateAccountMsgData {
/// //     pub code_id      :  u64,
/// //     pub chain_id     :  String,
/// //     pub account_data :  CredentialData
/// // }
/// //
/// // pub enum ExecuteMsgData {
/// //
/// //     // user-defined variants
/// //
/// //
/// //     CreateAccount(CreateAccountMsgData),
/// // }
/// ```
///
/// This macro is part of the CW83 spec for account registries.
#[proc_macro_attribute]
pub fn registry_execute(metadata: TokenStream, input: TokenStream) -> TokenStream {
    use syn::{parse_macro_input, AttributeArgs, NestedMeta};

    // ✅ Clone first
    let metadata_clone = metadata.clone();

    // ✅ Then parse using the clone
    let args = parse_macro_input!(metadata_clone as AttributeArgs);

    // 🧠 Extract the custom type or fallback to Binary
    let custom_type: proc_macro2::TokenStream = if args.is_empty() {
        quote!(::cosmwasm_std::Binary)
    } else if args.len() == 1 {
        match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => {
                quote!(#path)
            }
            other => {
                return syn::Error::new_spanned(
                    other,
                    "Expected a single type name like `CredentialData`",
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        return syn::Error::new_spanned(
            &args[1],
            "Expected at most one type argument",
        )
        .to_compile_error()
        .into();
    };

    // ✅ Inject the right variant
    let right_enum = quote! {
        enum Right {
            CreateAccount(::cw83::CreateAccountMsg<#custom_type>)
        }
    };

    // ✅ Use original metadata if needed
    merge_variants( input, right_enum.into())
}





/// Procedural macro for the CW83 standard that injects query variants for account registries into an enum.
///
/// # Notes
/// - This is a version with enabled `multi` flag that adds an additional variant and `Accounts { ... }`
/// - Requires `#[registry_query]` to be applied applied **before** `#[cw_serde]` or any other derive expressions.
/// - Your enum must derive `#[derive(QueryResponses)]` so that `#[returns(...)]` are recognized.
///
/// # Behavior
/// This macro generates two Query variants:
///
/// ```rust,ignore
/// #[returns(AccountResponse<T>)]
/// AccountInfo(QueryType),
///
/// #[returns(AccountsResponse<U>)]
/// Accounts {
///     query: QueryMultiType,
///     start_after: Option<String>,
///     skip: Option<u32>,
///     limit: Option<u32>,
/// }
/// ```
///
/// ## Type Parameters
///
/// The macro accepts **up to four optional positional type arguments**:
///
/// 1. **QueryType** - used as the type for the `AccountInfo(...)` variant  
///    _Defaults to `Binary`_
///
/// 2. **InfoType** - used inside `AccountResponse<InfoType>`  
///    _Defaults to `Option<Empty>`_
///
/// 3. **QueryMultiType** - used as the query object for `Accounts { query, ... }`  
///    _Defaults to `Option<QueryType>` if `QueryType` is not an `Option<_>`_ and same as `QueryType` otherwise_
///
/// 4. **InfoMultiType** - used inside `AccountsResponse<InfoMultiType>`  
///    _Defaults to same as `InfoType`_
///
/// ---
///
/// ## Example 1: Only a custom query type
///
/// ```rust,ignore
/// pub struct QueryParams {
///     pub custom_pubkey: Option<Binary>,
/// }
///
/// #[registry_query_multi(QueryParams)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum RegistryQueryMsg {
///     // custom user-defined queries
/// }
/// ```
///
/// ### Generated:
/// ```rust,ignore
/// #[returns(AccountResponse<Option<Empty>>)]
/// AccountInfo(QueryParams),
///
/// #[returns(AccountsResponse<Option<Empty>>)]
/// Accounts {
///     query: Option<QueryParams>,
///     start_after: Option<String>,
///     skip: Option<u32>,
///     limit: Option<u32>,
/// }
/// ```
///
/// ---
///
/// ## Example 2: Custom query and info types
///
/// ```rust,ignore
/// pub struct CustomInfo {
///     pub eth_address: String,
///     pub balances: Vec<cosmwasm_std::Coin>,
/// }
///
/// #[registry_query_multi(QueryParams, CustomInfo)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum RegistryQueryMsg {
///     // custom user-defined queries
/// }
/// ```
///
/// ### Generated:
/// ```rust,ignore
/// #[returns(AccountResponse<CustomInfo>)]
/// AccountInfo(QueryParams),
///
/// #[returns(AccountsResponse<CustomInfo>)]
/// Accounts {
///     query: Option<QueryParams>,
///     start_after: Option<String>,
///     skip: Option<u32>,
///     limit: Option<u32>,
/// }
/// ```
///
/// ---
///
/// ## Example 3: Fully customized query & response for single and multi account queries
///
/// ```rust,ignore
/// #[registry_query_multi(QueryParams, CustomInfo, MultiParams, MultiInfo)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum RegistryQueryMsg {
///     // user-defined queries
/// }
/// ```
///
/// ### Generated:
/// ```rust,ignore
/// #[returns(AccountResponse<CustomInfo>)]
/// AccountInfo(QueryParams),
///
/// #[returns(AccountsResponse<MultiInfo>)]
/// Accounts {
///     query: MultiParams,
///     start_after: Option<String>,
///     skip: Option<u32>,
///     limit: Option<u32>,
/// }
/// ```
///
/// This macro is part of the `CW83` registry spec and can be used to standardize query interfaces across smart accounts.
#[proc_macro_attribute]
pub fn registry_query_multi(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);


    // Helper to extract type paths from attributes
    let parse_type = |meta: &NestedMeta| {
        if let NestedMeta::Meta(syn::Meta::Path(path)) = meta {
            Ok(quote!(#path))
        } else {
            Err(syn::Error::new_spanned(meta, "Expected a type name").to_compile_error())
        }
    };

    let opt_empty = quote!(Option<::cosmwasm_std::Empty>);

    let (query_type, res_info_type, query_multi_type, res_info_multi_type) = match args.len() {
        0 => (
            quote!(::cosmwasm_std::Binary),
            opt_empty.clone(), 
            quote!(Option<::cosmwasm_std::Binary>), 
            opt_empty
        ),
        1 => match parse_type(&args[0]) {
            Ok(qt) => {
                let qt_string = qt.to_string();
                let is_option = qt_string.trim_start().starts_with("Option <") || qt_string.trim_start().starts_with("Option<");
                let qmt = if is_option {
                    qt.clone()
                } else {
                    quote!(Option<#qt>)
                };
                (qt, opt_empty.clone(), qmt, opt_empty)
            }
            Err(e) => return e.into(),
        },
        2 => match (parse_type(&args[0]), parse_type(&args[1])) {
            (Ok(qt), Ok(rt)) => {
                let qt_string = qt.to_string();
                let is_option = qt_string.trim_start().starts_with("Option <") || qt_string.trim_start().starts_with("Option<");
                let qmt = if is_option {
                    qt.clone()
                } else {
                    quote!(Option<#qt>)
                };
                (qt, rt.clone(), qmt, rt)
            }
            (Err(e1), Err(e2)) => return quote! { #e1 #e2 }.into(),
            (Err(e), _) | (_, Err(e)) => return e.into(),
        },
        3 => match (parse_type(&args[0]), parse_type(&args[1]), parse_type(&args[2])) {
            (Ok(qt), Ok(rt), Ok(qmt)) => (qt, rt.clone(), qmt, rt),
            _ => return quote! { #( #args.iter().map(parse_type).collect::<Result<Vec<_>, _>>().unwrap_err() )* }.into(),
        },
        4 => match (
            parse_type(&args[0]),
            parse_type(&args[1]),
            parse_type(&args[2]),
            parse_type(&args[3]),
        ) {
            (Ok(qt), Ok(qmt), Ok(rt), Ok(rmt)) => (qt, qmt, rt, rmt),
            (Err(e1), Err(e2), Err(e3), Err(e4)) => return quote! { #e1 #e2 #e3 #e4 }.into(),
            (Err(e), ..) | (_, Err(e), ..) | (_, _, Err(e), _) | (_, _, _, Err(e)) => return e.into(),
        },
        _ => {
            return syn::Error::new_spanned(
                &args[4],
                "Expected at most 4 type arguments: #[registry_query_multi(QueryType, QueryMultiType, InfoType, InfoMultiType)]"
            )
            .to_compile_error()
            .into();
        }
    };

    merge_variants(
        input,
        quote! {
            enum Right {
                #[returns(::cw83::AccountResponse<#res_info_type>)]
                AccountInfo(#query_type),

                #[returns(::cw83::AccountsResponse<#res_info_multi_type>)]
                Accounts {
                    query: #query_multi_type,
                    start_after: Option<String>,
                    skip: Option<u32>,
                    limit: Option<u32>,
                }
            }
        }
        .into(),
    )
}


/// Procedural macro for cw83 standard that automatically needed variants for query messages of account registries.
/// 
/// # Notes
/// - `#[registry_query]` must be applied *before* `#[cw_serde]` or other derives.
/// -  Enum must have `#[derive(QueryResponses)]` applied to make #[returns] properties valid.
/// 
/// This macro injects a `QueryMsg::AccountInfo(...)` variant into your query enum,
/// returning an `AccountResponse<T>` that contains metadata about a smart account.
///
/// Macto takes two optional arguments:
/// - The **first type** (optional) is used as the inner query type (e.g., `QueryParams`). Defaults to `Option<Binary>`.
/// - The **second type** (optional) is used as the info payload returned by `AccountResponse`. Must be serializable Defaults to `Option<Empty>`.
///
/// # Examples
///
/// ```rust,ignore
/// use cw83::registry_query;
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::{Binary, Empty};
///
/// #[registry_query]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum RegQuery {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated:
/// // pub struct AccountResponse<T> {
/// //     pub address: String,
/// //     pub info: Option<T>,
/// // }
/// //
/// // pub enum RegQuery {
/// //
/// //    // user-defined queries
/// //
/// //
/// //    #[returns(AccountResponse)]
/// //    AccountInfo(Option<Empty>),
/// // }
///
///
/// // With a custom query type:
///
/// pub struct QueryParams {
///     pub custom_pubkey: Option<Binary>,
/// }
///
/// #[registry_query(QueryParams)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum RegQueryParams {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated:
/// // pub enum RegQueryParams {
/// //
/// //    // user-defined queries
/// //
/// //
/// //     #[returns(AccountResponse)] // same as above
/// //     AccountInfo(QueryParams),
/// // }
///
/// // With custom query type and custom info payload:
///
/// // the macro call requires name only and doesn't support syntax of generics
/// type OptionalBin = Option<Binary>;
/// 
/// #[registry_query(QueryParams, OptionalBin)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum RegQueryCustom {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated:
/// // pub struct AccountResponse {
/// //     pub address: String,
/// //     pub info: Option<Binary>,  // Binary instead of Empty
/// // }
/// //
/// // pub enum RegQueryCustom {
/// //
/// //    // user-defined queries
/// //
/// //     #[returns(AccountResponse)]
/// //     AccountInfo(QueryParams),
/// // }
/// ```
///
/// This macro is part of the CW83 spec for account registries.
#[proc_macro_attribute]
pub fn registry_query_one(attr: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AttributeArgs);

    let (query_type, info_type): (TokenStream2, TokenStream2) = match args.len() {
        0 => (
            quote!(Option<::cosmwasm_std::Binary>),
            quote!(Option<::cosmwasm_std::Empty>)
        ),
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => (
                quote!(#path),
                quote!(Option<::cosmwasm_std::Empty>),
            ),
            other => {
                return syn::Error::new_spanned(other, "Expected a type name")
                    .to_compile_error()
                    .into();
            }
        },
        2 => {
            let get_path = |meta: &NestedMeta| {
                if let NestedMeta::Meta(syn::Meta::Path(path)) = meta {
                    Ok(quote!(#path))
                } else {
                    Err(syn::Error::new_spanned(meta, "Expected a type name").to_compile_error())
                }
            };

            match (get_path(&args[0]), get_path(&args[1])) {
                (Ok(qt), Ok(it)) => (quote!(#qt), quote!(#it)),
                (Err(e1), Err(e2)) => return quote! { #e1 #e2 }.into(),
                (Err(e), _) | (_, Err(e)) => return e.into(),
            }
        }
        _ => {
            return syn::Error::new_spanned(
                &args[2],
                "Expected at most two type arguments: `#[registry_query(QueryType, InfoType)]`",
            )
            .to_compile_error()
            .into();
        }
    };

    merge_variants(input, 
        quote! {
            enum Right {
                #[returns(::cw83::AccountResponse<#info_type>)]
                AccountInfo(#query_type)
            }
        }
        .into()
    )
    
}






/// Procedural macro for the CW84 standard that injects query variants for signed message validation
/// into a query enum, supporting generic and non-generic `CosmosMsg` types.
///
/// This macro extends a query enum with CW84-compliant variants for smart accounts. It checks if the
/// enum is generic over `T`, using `CosmosMsg<T>` for `CanExecute` if so, or `CosmosMsg` otherwise.
/// It supports custom types for the action message, signed data, and payload, enabling flexible
/// signed message validation with the `multi` feature for batch signature verification.
///
/// # Arguments
///
/// - **First type** (required): Inner action message type (e.g., `ExecuteMsg`) for `CanExecuteSigned`.
/// - **Second type** (optional): Signed data type (e.g., `SignedDataMsg`) for `CanExecuteSigned`.
///   Defaults to `cosmwasm_std::Binary`.
/// - **Third type** (optional): Payload type (e.g., `CustomPayload`) for `ValidSignature`, wrapped in
///   `Option`. Defaults to `Option<cosmwasm_std::Binary>`.
/// - **Fourth type** (optional): Payload type (e.g., `CustomMultiPayload`) for `ValidSignatures`, 
///   wrapped in `Option`. Defaults to the same as the third argument if not provided.
///
/// # Generated Variants
///
/// - `CanExecute`: Queries if a `CosmosMsg` can be executed.
/// - `CanExecuteSigned`: Queries if signed messages can be executed, using provided types.
/// - `ValidSignature`: Verifies a single signature against data and optional payload.
/// - `ValidSignatures`: Verifies multiple signatures against data and optional payload (may use different payload type).
///
/// # Notes
///
/// - Apply `#[signed_query]` before `#[cw_serde]` or other derives.
/// - Enum must derive `#[derive(QueryResponses)]` from `cosmwasm_schema` for `#[returns(...)]`.
/// - Part of the CW84 spec for smart account signed message validation.
///
/// # Examples
///
/// ## Basic usage
///
/// ```rust,ignore
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// #[cw_serde]
/// pub enum ExecuteMsg { Foo {} }
/// #[signed_query(ExecuteMsg)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg { /* User queries */ }
/// // Generates `CanExecute`, `CanExecuteSigned` (with `Binary` signed), `ValidSignature`, and
/// // `ValidSignatures` with `Option<Binary>` payload.
/// ```
///
/// ## Custom types with generic enum
///
/// ```rust,ignore
/// #[cw_serde]
/// pub struct SignedDataMsg { pub data: String, pub signature: String }
/// #[cw_serde]
/// pub struct CustomPayload { pub metadata: String }
/// #[signed_query(ExecuteMsg, SignedDataMsg, CustomPayload)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg<T> { /* User queries */ }
/// // Generates variants with `CosmosMsg<T>`, `SignedDataMsg` signed, and `Option<CustomPayload>`.
/// ```
///
/// # Errors
///
/// - Fails if arguments are invalid (e.g., not a type path or wrong count).
/// - Fails if input is not a valid enum or variant merge fails.
#[proc_macro_attribute]
pub fn signed_query_multi(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let cloned = input.clone();
    let ast: DeriveInput = parse_macro_input!(cloned as DeriveInput);
    let has_t = has_generic_t(&ast.generics);

    let msg_type = if has_t {
        quote!(::cosmwasm_std::CosmosMsg<T>)
    } else {
        quote!(::cosmwasm_std::CosmosMsg)
    };

    let args = parse_macro_input!(metadata as AttributeArgs);

    let (act_type, sign_type, pl_type, pl_multi_type) = match args.len() {
        4 => {
            let exec = match &args[0] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
            };
            let signed = match &args[1] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the signed message, like `SignedDataMsg`"),
            };
            let payload = match &args[2] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(Option<#path>),
                other => return comp_err!(other, "Expected a type for the payload, like `CustomPayload`"),
            };
            let payload_multi = match &args[3] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(Option<#path>),
                other => return comp_err!(other, "Expected a type for the multi payload, like `CustomMultiPayload`"),
            };
            (exec, signed, payload, payload_multi)
        },
        3 => {
            let exec = match &args[0] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
            };
            let signed = match &args[1] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the signed message, like `SignedDataMsg`"),
            };
            let payload = match &args[2] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(Option<#path>),
                other => return comp_err!(other, "Expected a type for the payload, like `CustomPayload`"),
            };
            (exec, signed, payload.clone(), payload)
        },
        2 => {
            let exec = match &args[0] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
            };
            let signed = match &args[1] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the signed message, like `SignedDataMsg`"),
            };
            let default_payload = quote!(Option<::cosmwasm_std::Binary>);
            (exec, signed, default_payload.clone(), default_payload)
        },
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => {
                let default_payload = quote!(Option<::cosmwasm_std::Binary>);
                (
                    quote!(#path),
                    quote!(::cosmwasm_std::Binary),
                    default_payload.clone(),
                    default_payload
                )
            },
            other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
        },
        _ => return comp_err!(&args.get(0), "Expected one to four arguments: `#[signed_query(ExecuteMsg[, SignedDataMsg][, CustomPayload][, CustomMultiPayload])]`"),
    };

    let right_enum = quote! {
        enum Right {
            #[returns(::cw84::CanExecuteResponse)]
            CanExecute {
                sender: String,
                msg: #msg_type
            },
            #[returns(::cw84::CanExecuteResponse)]
            CanExecuteNative {
                sender: String,
                msg: #msg_type
            },
            #[returns(::cw84::CanExecuteSignedResponse)]
            CanExecuteSigned {
                msgs: Vec<#act_type>,
                signed: #sign_type,
                nonce: Option<::cosmwasm_std::Uint64>,
            },
            #[returns(::cw84::ValidSignatureResponse)]
            ValidSignature {
                data: ::cosmwasm_std::Binary,
                signature: ::cosmwasm_std::Binary,
                payload: #pl_type
            },
            #[returns(::cw84::ValidSignaturesResponse)]
            ValidSignatures {
                data: Vec<::cosmwasm_std::Binary>,
                signatures: Vec<::cosmwasm_std::Binary>,
                payload: #pl_multi_type
            }
        }
    };
    merge_variants(input, right_enum.into())
}



/// Procedural macro for the CW84 standard that injects query variants for signed message validation
/// into a query enum, supporting generic and non-generic `CosmosMsg` types.
///
/// This macro extends a query enum with CW84-compliant variants for smart accounts. It checks if the
/// enum is generic over `T`, using `CosmosMsg<T>` for `CanExecute` if so, or `CosmosMsg` otherwise.
/// It supports custom types for the action message, signed data, and payload, enabling validation of
/// a single signed message.
///
/// # Arguments
///
/// - **First type** (required): Inner action message type (e.g., `ExecuteMsg`) for `CanExecuteSigned`.
/// - **Second type** (optional): Signed data type (e.g., `SignedDataMsg`) for `CanExecuteSigned`.
///   Defaults to `cosmwasm_std::Binary`.
/// - **Third type** (optional): Payload type (e.g., `CustomPayload`) for `ValidSignature`, wrapped in
///   `Option`. Defaults to `Option<cosmwasm_std::Binary>`.
///
/// # Generated Variants
///
/// - `CanExecute`: Queries if a `CosmosMsg` can be executed.
/// - `CanExecuteSigned`: Queries if a single signed message can be executed, using provided types.
/// - `ValidSignature`: Verifies a single signature against data and optional payload.
///
/// # Notes
///
/// - Apply `#[signed_query]` before `#[cw_serde]` or other derives.
/// - Enum must derive `#[derive(QueryResponses)]` from `cosmwasm_schema` for `#[returns(...)]`.
/// - Excludes multi-signature `ValidSignatures` variant, unlike `signed_query_multi`.
/// - Part of the CW84 spec for smart account signed message validation.
///
/// # Examples
///
/// ## Basic usage
///
/// ```rust,ignore
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// #[cw_serde]
/// pub enum ExecuteMsg { Foo {} }
/// #[signed_query(ExecuteMsg)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg { /* User queries */ }
/// // Generates `CanExecute`, `CanExecuteSigned` (with `Binary` signed), and `ValidSignature` with
/// // `Option<Binary>` payload.
/// ```
///
/// ## Custom types with generic enum
///
/// ```rust,ignore
/// #[cw_serde]
/// pub struct SignedDataMsg { pub data: String, pub signature: String }
/// #[cw_serde]
/// pub struct CustomPayload { pub metadata: String }
/// #[signed_query(ExecuteMsg, SignedDataMsg, CustomPayload)]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg<T> { /* User queries */ }
/// // Generates variants with `CosmosMsg<T>`, `SignedDataMsg` signed, and `Option<CustomPayload>`.
/// ```
///
/// # Errors
///
/// - Fails if arguments are invalid (e.g., not a type path or wrong count).
/// - Fails if input is not a valid enum or variant merge fails.
#[proc_macro_attribute]
pub fn signed_query_one(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let cloned = input.clone();
    let ast: DeriveInput = parse_macro_input!(cloned as DeriveInput);
    let has_t = has_generic_t(&ast.generics);

    let msg_type = if has_t {
        quote!(::cosmwasm_std::CosmosMsg<T>)
    } else {
        quote!(::cosmwasm_std::CosmosMsg)
    };

    let args = parse_macro_input!(metadata as AttributeArgs);

    let (exec_type, data_type, payload_type) = match args.len() {
        3 => {
            let exec = match &args[0] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
            };
            let signed = match &args[1] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the signed message, like `SignedDataMsg`"),
            };
            let payload = match &args[2] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(Option<#path>),
                other => return comp_err!(other, "Expected a type for the payload, like `CustomPayload`"),
            };
            (exec, signed, payload)
        },
        2 => {
            let exec = match &args[0] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
            };
            let signed = match &args[1] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the signed message, like `SignedDataMsg`"),
            };
            (exec, signed, quote!(Option<::cosmwasm_std::Binary>))
        },
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => (
                quote!(#path),
                quote!(::cosmwasm_std::Binary),
                quote!(Option<::cosmwasm_std::Binary>)
            ),
            other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
        },
        _ => return comp_err!(&args.get(0), "Expected one to three arguments: `#[signed_query(ExecuteMsg[, SignedDataMsg][, CustomPayload])]`"),
    };

    let right_enum = quote! {
        enum Right {
            #[returns(::cw84::CanExecuteResponse)]
            CanExecute {
                sender: String,
                msg: #msg_type
            },
            #[returns(::cw84::CanExecuteResponse)]
            CanExecuteNative {
                sender: String,
                msg: #msg_type
            },
            #[returns(::cw84::CanExecuteResponse)]
            CanExecuteSigned {
                msg: #exec_type,
                signed: #data_type,
                nonce: Option<::cosmwasm_std::Uint64>,
            },
            #[returns(::cw84::ValidSignatureResponse)]
            ValidSignature {
                data: ::cosmwasm_std::Binary,
                signature: ::cosmwasm_std::Binary,
                payload: #payload_type
            },
        }
    };
    merge_variants(input, right_enum.into())
}



/// Procedural macro for the CW84 standard that injects execute variants for signed message execution
/// into an execute enum, supporting both generic and non-generic `CosmosMsg` types.
///
/// This macro extends an enum with standardized execute variants for smart accounts as defined in the
/// CW84 specification. It checks if the input enum is generic over `T` and adjusts the `Execute`
/// variant to use `CosmosMsg<T>` or `CosmosMsg` accordingly. The macro supports custom types for the
/// inner action message and signed data, enabling execution of signed messages with multiple messages.
///
/// # Arguments
///
/// The macro accepts **zero to two type arguments** via the attribute:
/// - **First type** (optional): The inner action message type (e.g., `ExecuteMsg`) for the
///   `ExecuteSigned` and `Native` variants. Defaults to the enum's own type if not provided.
/// - **Second type** (optional): The signed data type (e.g., `SignedDataMsg`) for the
///   `ExecuteSigned` variant. Defaults to `cw84::Binary` if not provided.
///
/// # Generated Variants
///
/// The macro injects the following execute variants:
/// - `Execute`: Executes a list of `CosmosMsg` messages by a smart account.
/// - `ExecuteSigned`: Executes a list of signed messages, using the provided action and data types.
/// - `Native` (only if `exec_type` is specified): Executes a list of messages of the specified
///   `exec_type` directly, without requiring signed data.
///
/// # Notes
/// - The `#[signed_execute]` attribute must be applied **before** `#[cw_serde]` or other derive
///   macros.
/// - Unlike query macros, this macro does not require `#[derive(QueryResponses)]` since it targets
///   execute messages.
/// - This macro is designed for the `multi` feature, supporting execution of multiple messages.
///
/// # Examples
///
/// ## Example 1: Basic usage with no arguments
///
/// ```rust,ignore
/// use cosmwasm_schema::cw_serde;
///
/// #[cw_serde]
/// #[signed_execute]
/// pub enum ExecuteMsg {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsg {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg>,
/// //     },
/// //
/// //     ExecuteSigned {
/// //         msgs: Vec<ExecuteMsg>,
/// //         signed: ::cw84::Binary,
/// //         nonce: Option<::cosmwasm_std::Uint64>,
/// //     },
/// // }
/// ```
///
/// ## Example 2: Usage with a custom execute message type
///
/// ```rust,ignore
/// #[cw_serde]
/// pub enum ActionMsg {
///     Foo {},
/// }
///
/// #[signed_execute(ActionMsg)]
/// #[cw_serde]
/// pub enum ExecuteMsg {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsg {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg>,
/// //     },
/// //
/// //     ExecuteSigned {
/// //         msgs: Vec<ActionMsg>,
/// //         signed: ::cw84::Binary,
/// //         nonce: Option<::cosmwasm_std::Uint64>,
/// //     },
/// //
/// //     ExecuteNative {
/// //         msgs: Vec<ActionMsg>,
/// //     },
/// // }
/// ```
///
/// ## Example 3: Usage with custom execute and signed data types
///
/// ```rust,ignore
/// #[cw_serde]
/// pub struct SignedDataMsg {
///     pub data: String,
///     pub signature: String,
/// }
///
/// #[signed_execute(ActionMsg, SignedDataMsg)]
/// #[cw_serde]
/// pub enum ExecuteMsgSigned {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsgSigned {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg>,
/// //     },
/// //
/// //     ExecuteSigned {
/// //         msgs: Vec<ActionMsg>,
/// //         signed: SignedDataMsg,
/// //         nonce: Option<::cosmwasm_std::Uint64>,
/// //     },
/// //
/// //     ExecuteNative {
/// //         msgs: Vec<ActionMsg>,
/// //     },
/// // }
/// ```
///
/// ## Example 4: Generic enum with custom types
///
/// ```rust,ignore
/// #[signed_execute(ActionMsg, SignedDataMsg)]
/// #[cw_serde]
/// pub enum ExecuteMsgCustom<T> {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsgCustom<T> {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg<T>>,
/// //     },
/// //
/// //     ExecuteSigned {
/// //         msgs: Vec<ActionMsg>,
/// //         signed: SignedDataMsg,
/// //         nonce: Option<::cosmwasm_std::Uint64>,
/// //     },
/// //
/// //     ExecuteNative {
/// //         msgs: Vec<ActionMsg>,
/// //     },
/// // }
/// ```
///
/// # Errors
///
/// - Fails with a compile-time error if the attribute arguments are invalid (e.g., not a type path
///   or more than two arguments).
/// - Fails if the input is not a valid enum or if the merge with generated variants cannot be
///   performed.
///
/// This macro is part of the CW84 specification for smart account signed message execution.
#[proc_macro_attribute]
pub fn signed_execute_multi(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let cloned = input.clone();
    let ast: DeriveInput = parse_macro_input!(cloned as DeriveInput);
    let has_t = has_generic_t(&ast.generics);

    let cms_type = if has_t {
        quote!(::cosmwasm_std::CosmosMsg<T>)
    } else {
        quote!(::cosmwasm_std::CosmosMsg)
    };

    let args = parse_macro_input!(metadata as AttributeArgs);

    let (exec_type, sign_type) = match args.len() {
        2 => {
            let exec = match &args[0] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
            };
            let signed = match &args[1] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the signed message, like `SignedMsg`"),
            };
            (exec, signed)
        },
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => (quote!(#path), quote!(::cw84::Binary)),
            other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
        },
        0 => (
            // self type
            ast.ident.to_token_stream(),
            quote!(::cw84::Binary)
        ),
        _ => return comp_err!(&args[1], "Expected at most 2 arguments"),
    };

    let right_enum = if args.len() > 0 {
        // Include Native variant when exec_type is explicitly specified
        quote! {
            enum Right {
                Execute {
                    msgs: Vec<#cms_type>,
                    signed: Option<#sign_type>,
                },
                ExecuteSigned {
                    msgs: Vec<#exec_type>,
                    signed: #sign_type,
                    nonce: Option<::cosmwasm_std::Uint64>,
                },
                ExecuteNative {
                    msgs: Vec<#exec_type>,
                },
            }
        }
    } else {
        // Exclude Native variant when exec_type is not specified (uses self type)
        quote! {
            enum Right {
                Execute {
                    msgs: Vec<#cms_type>,
                    signed: Option<#sign_type>,
                },
                ExecuteSigned {
                    msgs: Vec<#exec_type>,
                    signed: #sign_type,
                    nonce: Option<::cosmwasm_std::Uint64>,
                },
            }
        }
    };

    merge_variants(input, right_enum.into())
}



/// Procedural macro for the CW84 standard that injects execute variants for signed message execution
/// into an execute enum, supporting both generic and non-generic `CosmosMsg` types.
///
/// This macro extends an enum with standardized execute variants for smart accounts as defined in the
/// CW84 specification. It checks if the input enum is generic over `T` and adjusts the `Execute`
/// variant to use `CosmosMsg<T>` or `CosmosMsg` accordingly. The macro supports custom types for the
/// inner action message and signed data, enabling execution of a single signed message.
///
/// # Arguments
///
/// The macro accepts **zero to two type arguments** via the attribute:
/// - **First type** (optional): The inner action message type (e.g., `ExecuteMsg`) for the
///   `ExecuteSigned` variant. Defaults to the enum's own type if not provided.
/// - **Second type** (optional): The signed data type (e.g., `SignedDataMsg`) for the
///   `ExecuteSigned` variant. Defaults to `cosmwasm_std::Binary` if not provided.
///
/// # Generated Variants
///
/// The macro injects the following execute variants:
/// - `Execute`: Executes a list of `CosmosMsg` messages by a smart account.
/// - `ExecuteSigned`: Executes a single signed message, using the provided action and data types.
///
/// # Notes
/// - The `#[signed_execute]` attribute must be applied **before** `#[cw_serde]` or other derive
///   macros.
/// - Unlike query macros, this macro does not require `#[derive(QueryResponses)]` since it targets
///   execute messages.
/// - This macro is designed for single-message execution, in contrast to `signed_execute_multi`, which
///   supports multiple messages in the `ExecuteSigned` variant.
///
/// # Examples
///
/// ## Example 1: Basic usage with no arguments
///
/// ```rust,ignore
/// use cosmwasm_schema::cw_serde;
///
/// #[cw_serde]
/// #[signed_execute]
/// pub enum ExecuteMsg {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsg {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg>,
/// //     },
/// //
/// //     ExecuteSigned {
/// //         msg: Box<ExecuteMsg>,
/// //         signed: ::cosmwasm_std::Binary,
/// //         nonce: Option<::cosmwasm_std::Uint64>,
/// //     },
/// // }
/// ```
///
/// ## Example 2: Usage with a custom execute message type
///
/// ```rust,ignore
/// #[cw_serde]
/// pub enum InnerActionMsg {
///     Foo {},
/// }
///
/// #[signed_execute(InnerActionMsg)]
/// #[cw_serde]
/// pub enum ExecuteMsg {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsg {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg>,
/// //     },
/// //
/// //     ExecuteSigned {
/// //         msg: InnerActionMsg,
/// //         signed: ::cosmwasm_std::Binary,
/// //         nonce: Option<::cosmwasm_std::Uint64>,
/// //     },
/// // }
/// ```
///
/// ## Example 3: Usage with custom execute and signed data types
///
/// ```rust,ignore
/// #[cw_serde]
/// pub struct SignedDataMsg {
///     pub data: String,
///     pub signature: String,
/// }
///
/// #[signed_execute(InnerActionMsg, SignedDataMsg)]
/// #[cw_serde]
/// pub enum ExecuteMsgSigned {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsgSigned {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg>,
/// //     },
/// //
/// //     ExecuteSigned {
/// //         msg: InnerActionMsg,
/// //         signed: SignedDataMsg,
/// //         nonce: Option<::cosmwasm_std::Uint64>,
/// //     },
/// // }
/// ```
///
/// ## Example 4: Generic enum with custom types
///
/// ```rust,ignore
/// #[signed_execute(InnerActionMsg, SignedDataMsg)]
/// #[cw_serde]
/// pub enum ExecuteMsgCustom<T> {
///     // User-defined execute messages
/// }
///
/// // Generated:
/// // pub enum ExecuteMsgCustom<T> {
/// //     // User-defined execute messages
/// //
/// //     Execute {
/// //         msgs: Vec<::cosmwasm_std::CosmosMsg<T>>,
/// //     },
/// //
/// //     ExecuteSigned {
/// //         msg: InnerActionMsg,
/// //         signed: SignedDataMsg,
/// //         nonce: Option<::cosmwasm_std::Uint64>,
/// //     },
/// // }
/// ```
///
/// # Errors
///
/// - Fails with a compile-time error if the attribute arguments are invalid (e.g., not a type path
///   or more than two arguments).
/// - Fails if the input is not a valid enum or if the merge with generated variants cannot be
///   performed.
///
/// This macro is part of the CW84 specification for smart account signed message execution.
#[proc_macro_attribute]
pub fn signed_execute_one(metadata: TokenStream, input: TokenStream) -> TokenStream {
    let cloned = input.clone();
    let ast: DeriveInput = parse_macro_input!(cloned as DeriveInput);
    let has_t = has_generic_t(&ast.generics);

    let cms_type = if has_t {
        quote!(::cosmwasm_std::CosmosMsg<T>)
    } else {
        quote!(::cosmwasm_std::CosmosMsg)
    };

    let args = parse_macro_input!(metadata as AttributeArgs);

    let (exec_type, sign_type) = match args.len() {
        2 => {
            let exec = match &args[0] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
            };
            let signed = match &args[1] {
                NestedMeta::Meta(syn::Meta::Path(path)) => quote!(#path),
                other => return comp_err!(other, "Expected a type for the signed message, like `SignedMsg`"),
            };
            (exec, signed)
        },
        1 => match &args[0] {
            NestedMeta::Meta(syn::Meta::Path(path)) => (quote!(#path), quote!(::cw84::Binary)),
            other => return comp_err!(other, "Expected a type for the inner action message, like `ExecuteMsg`"),
        },
        0 => (
            // self type
            ast.ident.to_token_stream(),
            quote!(::cw84::Binary)
        ),
        _ => return comp_err!(&args[1], "Expected at most 2 arguments"),
    };

    let right_enum = quote! {
        enum Right {
            Execute {
                msgs: Vec<#cms_type>,
                signed: Option<#sign_type>,
            },
            ExecuteSigned {
                msg:  Box<#exec_type>,
                signed: #sign_type,
                nonce: Option<::cosmwasm_std::Uint64>,
            },
        }
    };

    merge_variants(input, right_enum.into())
}