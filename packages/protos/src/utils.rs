use quote::quote;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DataEnum, DeriveInput};




pub (crate) fn merge_variants(left: TokenStream, right: TokenStream) -> TokenStream {
    use syn::Data::Enum;

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



pub(crate) fn has_generic_t(generics: &syn::Generics) -> bool {
    generics.type_params().any(|tp| tp.ident == "T")
}
