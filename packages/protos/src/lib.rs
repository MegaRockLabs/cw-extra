mod utils;
use quote::quote;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, NestedMeta};
use utils::{merge_variants, has_generic_t};





/// Procedural macro to extend an enum with standardized signature validation variants.
///
/// This macro inserts additional variants to your `QueryMsg` used for signing-in with smart contracts:
///
/// - `ValidSignature`: to verify a single signature.
/// - `ValidSignatures`: to verify multiple signatures at once (only in the multi variant).
///
/// # Notes
/// -  The is version with `multi` feature enabled, which adds `ValidSignatures` for batch verification.
/// - `#[valid_signature_query]` must be applied *before* `#[cw_serde]` or other derives.
/// -  Enum must have `#[derive(QueryResponses)]` applied to make #[returns] properties valid.
///
/// # Examples
///
/// ```rust,ignore
/// use cw81::{valid_signature_multi, valid_signature_one};
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::Binary;
///
/// #[valid_signature_multi]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated version:
/// //
/// // pub enum QueryMsg {
/// //
/// //     // user-defined queries
/// //
/// //     #[returns(::cw81::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data:        :    Binary,
/// //         signature    :    Binary,
/// //         payload      :    Option<Binary>,
/// //     },
/// //     #[returns(::cw81::ValidSignaturesResponse)]
/// //     ValidSignatures {
/// //         data         :    Vec<Binary>,
/// //         signatures   :    Vec<Binary>,
/// //         payload      :    Option<Binary>,
/// //     },
/// // }
/// ```
///
/// This macro is part of the CW81 spec for signature validation.
#[proc_macro_attribute]
pub fn valid_signature_multi(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        input,
        quote! {
            enum Right {
                #[returns(::cw81::ValidSignatureResponse)]
                ValidSignature {
                    data: ::cosmwasm_std::Binary,
                    signature: ::cosmwasm_std::Binary,
                    payload: Option<::cosmwasm_std::Binary>
                },
                #[returns(::cw81::ValidSignaturesResponse)]
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


/// Procedural macro to extend an enum with standardized signature validation variants.
///
/// This macro inserts variants for verifying cryptographic signatures in queries:
///
/// - `ValidSignature`: to verify signature validation for smart contracts
///
/// # Notes
/// - `#[valid_signature_query]` must be applied *before* `#[cw_serde]` or other derives.
/// -  Enum must have `#[derive(QueryResponses)]` applied to make #[returns] properties valid.
///
/// # Examples
///
/// ```rust,ignore
/// use cw81::{valid_signature_multi, valid_signature_one};
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::Binary;
///
/// #[valid_signature_multi]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated version:
/// //
/// // pub enum QueryMsg {
/// //
/// //     // user-defined queries
/// //
/// //     #[returns(::cw81::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data:        :    Binary,
/// //         signature    :    Binary,
/// //         payload      :    Option<Binary>,
/// //     },
/// // }
/// ```
///
/// This macro is part of the CW81 spec for signature validation.
#[proc_macro_attribute]
pub fn valid_signature_one(_metadata: TokenStream, input: TokenStream) -> TokenStream {
    merge_variants(
        input,
        quote! {
            enum Right {
                #[returns(::cw81::ValidSignatureResponse)]
                ValidSignature {
                    data: ::cosmwasm_std::Binary,
                    signature: ::cosmwasm_std::Binary,
                    payload: Option<::cosmwasm_std::Binary>
                },
            }
        }
        .into(),
    )
}



/// Procedural macro to extend an enum with standardized smart account query variants.
///
/// This macro checks whether the input enum is generic over `T`, and inserts variants
/// accordingly. If the enum is generic over `T`, the inserted `CanExecute` variant
/// uses `CosmosMsg<T>`; otherwise, it uses the non-generic `CosmosMsg`.
/// 
/// This is version with `multi` feature tag enabled, which adds an additional `ValidSignatures` variant.
/// All inserted variants are:
///
/// - `CanExecute`: to query whether a message can be executed by a smart account.
/// - `ValidSignature`: to verify a single signature.
/// - `ValidSignatures`: to verify multiple signatures at once.
///
/// Uses #[return] property macros from `QueryResponses` of `cosmwasm_schema`, 
/// so the user must add #[derive(QueryResponses)] to their enum.
///
/// # Examples
///
/// ```rust,ignore
/// use cw82::smart_account_query;
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::{Binary, Empty};
///
/// #[smart_account_query]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //
/// //     // user-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]  // same as ::cw1::CanExecuteResponse
/// //     CanExecute {
/// //         sender       :    String,
/// //         msg          :    CosmosMsg, // CosmosMsg<Empty>
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data:        :    Binary,
/// //         signature    :    Binary,
/// //         payload      :    Option<Binary>,
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignaturesResponse)]
/// //     ValidSignatures {
/// //         data         :    Vec<Binary>,
/// //         signatures   :    Vec<Binary>,
/// //         payload      :    Option<Binary>,
/// //     },
/// // }
///
///
/// #[smart_account_query]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsgCustom<T> {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated (generic version):
/// // pub enum QueryMsgCustom<T> {
/// //
/// //     // user-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]
/// //     CanExecute {
/// //         sender       :    String,
/// //         msg          :    CosmosMsg<T>,
/// //     },
/// //
/// //     // ---  same valid signature variants as above ---
/// // }
/// ```
/// This macro is part of the CW82 spec for smart accounts.
#[proc_macro_attribute]
pub fn account_query_multi(_metadata: TokenStream, input: TokenStream) -> TokenStream {
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
                payload: Option<::cosmwasm_std::Binary>
            },

            #[returns(::cw82::ValidSignaturesResponse)]
            ValidSignatures {
                data: Vec<::cosmwasm_std::Binary>,
                signatures: Vec<::cosmwasm_std::Binary>,
                payload: Option<::cosmwasm_std::Binary>
            }
        }
    };
    merge_variants( input, right_enum.into())
}


/// Procedural macro to extend an enum with standardized smart account query variants.
///
/// This macro checks whether the input enum is generic over `T`, and inserts variants
/// accordingly. If the enum is generic over `T`, the inserted `CanExecute` variant
/// uses `CosmosMsg<T>`; otherwise, it uses the non-generic `CosmosMsg`.
/// 
/// The inserted variants are:
///
/// - `CanExecute`: to query whether a message can be executed by a smart account.
/// - `ValidSignature`: for checking whether the given data and signature are valid.
///
/// Uses #[return] property macros from `QueryResponses` of `cosmwasm_schema`, 
/// so the user must add #[derive(QueryResponses)] to their enum.
///
/// # Examples
///
/// ```rust,ignore
/// use cw82::smart_account_query;
/// use cosmwasm_schema::{cw_serde, QueryResponses};
/// use cosmwasm_std::{Binary, Empty};
///
/// #[smart_account_query]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsg {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated:
/// // pub enum QueryMsg {
/// //
/// //     // user-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]  // same as ::cw1::CanExecuteResponse
/// //     CanExecute {
/// //         sender       :    String,
/// //         msg          :    CosmosMsg, // CosmosMsg<Empty>
/// //     },
/// //
/// //     #[returns(::cw82::ValidSignatureResponse)]
/// //     ValidSignature {
/// //         data:        :    Binary,
/// //         signature    :    Binary,
/// //         payload      :    Option<Binary>,
/// //     }
/// // }
///
///
/// #[smart_account_query]
/// #[cw_serde]
/// #[derive(QueryResponses)]
/// pub enum QueryMsgCustom<T> {
/// //
/// //    // user-defined queries
/// //
/// }
///
/// // Generated (generic version):
/// // pub enum QueryMsgCustom<T> {
/// //
/// //     // user-defined queries
/// //
/// //     #[returns(::cw82::CanExecuteResponse)]
/// //     CanExecute {
/// //         sender       :    String,
/// //         msg          :    CosmosMsg<T>,
/// //     },
/// //
/// //     // ---  same valid signature variants as above ---
/// // }
/// ```
/// This macro is part of the CW82 spec for smart accounts.
#[proc_macro_attribute]
pub fn account_query_one(_metadata: TokenStream, input: TokenStream) -> TokenStream {
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
                payload: Option<::cosmwasm_std::Binary>
            }
        }
    };
    merge_variants( input, right_enum.into())
}



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
    merge_variants( input, right_enum.into())
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

