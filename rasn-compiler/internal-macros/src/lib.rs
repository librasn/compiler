extern crate quote;
extern crate syn;

use quote::{format_ident, quote, ToTokens};
use syn::{Data, DataEnum, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam};

/// Extension of the standard `Debug` derive macro.
/// Prefixes the variant identifier with the enum identifier separated by `::`.
/// This extension serves the `rasn_compiler::tests::generate_stepwise_e2e_test` test generator.
#[proc_macro_derive(EnumDebug)]
pub fn enum_debug_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let DeriveInput {
        ident,
        data: Data::Enum(DataEnum { variants, .. }),
        generics,
        ..
    } = syn::parse_macro_input!(input as syn::DeriveInput)
    {
        let arms = variants.iter().map(|v| {
            let variant_name = &v.ident;
            let variant_literal = v.ident.to_string();
            match &v.fields {
                Fields::Unit => quote!(#ident::#variant_name => f.write_str(#variant_literal)),
                Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                    let field_names = (0..unnamed.len()).map(|i| format_ident!("f{i}"));
                    let field_debugs = (0..unnamed.len()).map(|i| {
                        let field_name = format_ident!("f{i}");
                        quote!(.field(&#field_name))
                    });
                    quote! {
                        Self::#variant_name(#(#field_names),*) =>
                            f.debug_tuple(#variant_literal)#(#field_debugs)*.finish()
                    }
                }
                Fields::Named(FieldsNamed { named, .. }) => {
                    let field_names = named.iter().map(|f| f.ident.clone().unwrap());
                    let field_debugs = named.iter().map(|f| {
                        let field_name = f.ident.clone().unwrap();
                        let field_literal = f.ident.clone().unwrap().to_string();
                        quote!(.field(#field_literal, &#field_name))
                    });
                    quote! {
                        Self::#variant_name { #(#field_names),* } =>
                            f.debug_struct(#variant_literal)#(#field_debugs)*.finish()
                    }
                }
            }
        });
        let ident_literal = format!("{ident}::");
        quote! {
            impl #generics std::fmt::Debug for #ident #generics {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    f.write_str(#ident_literal)?;
                    match self {
                        #(#arms),*
                    }
                }
            }
        }
        .into()
    } else {
        proc_macro::TokenStream::new()
    }
}
