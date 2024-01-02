use proc_macro2::{Ident, Literal, TokenStream};
use quote::{quote, ToTokens};

use crate::intermediate::types::Enumerated;

pub fn typealias_template(
    comments: TokenStream,
    name: Ident,
    alias: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name (pub #alias);
    }
}

pub fn object_identifier_value_template(
    comments: TokenStream,
    name: Ident,
    value: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        pub const #name: &Oid = &Oid::const_new(&#value);
    }
}

pub fn integer_value_template(
    comments: TokenStream,
    name: Ident,
    vtype: TokenStream,
    value: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        pub const #name: #vtype = #value;

    }
}

pub fn integer_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
    integer_type: Ident,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #annotations
        pub struct #name (pub #integer_type);

    }
}

pub fn time_value_template(
    comments: TokenStream,
    name: Ident,
    type_name: Ident,
    value: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        pub const #name: #type_name = #value;
    }
}

pub fn generalized_time_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(pub GeneralizedTime);
    }
}

pub fn utc_time_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(pub UtcTime);
    }
}

pub fn bit_string_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(pub BitString);
    }
}

pub fn octet_string_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(pub OctetString);
    }
}

pub fn char_string_template(
    comments: TokenStream,
    name: Ident,
    string_type: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(pub #string_type);
    }
}

pub fn boolean_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(pub bool);
    }
}

pub fn null_value_template(comments: TokenStream, name: Ident) -> TokenStream {
    quote! {
        #comments
        pub const #name: () = ();
    }
}

pub fn enum_value_template(comments: TokenStream, name: Ident, enumerated: Ident, enumeral: Ident) -> TokenStream {
    quote! {
        #comments
        pub const #name: #enumerated = #enumerated::#enumeral;
    }
}

pub fn null_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(());
    }
}

pub fn any_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(Any);
    }
}

pub fn oid_template(
    comments: TokenStream,
    name: Ident,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #annotations
        pub struct #name(pub ObjectIdentifier);
    }
}

pub fn enumerated_template(
    comments: TokenStream,
    name: Ident,
    extensible: TokenStream,
    enum_members: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #annotations
        #extensible
        pub enum #name {
            #enum_members
        }
    }
}

pub fn sequence_or_set_template(
    comments: TokenStream,
    name: Ident,
    extensible: TokenStream,
    members: TokenStream,
    nested_members: Vec<TokenStream>,
    annotations: TokenStream,
    default_methods: TokenStream,
    new_impl: TokenStream,
    class_fields: TokenStream
) -> TokenStream {
    quote! {
            #(#nested_members)*
            #comments
            #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
            #annotations
            #extensible
            pub struct #name {
                #members
            }

            #new_impl

            #class_fields

            #default_methods
        }
}

pub fn sequence_or_set_of_template(
    is_set_of: bool,
    comments: TokenStream,
    name: Ident,
    anonymous_item: TokenStream,
    member_type: Ident,
    annotations: TokenStream,
) -> TokenStream {
    let generic_type = is_set_of.then(|| quote!(SetOf)).unwrap_or(quote!(SequenceOf));
    quote! {
            #anonymous_item
            #comments
            #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
            #annotations
            pub struct #name(pub #generic_type<#member_type>);
    }
}

pub fn choice_template(
    comments: TokenStream,
    name: Ident,
    extensible: TokenStream,
    options: TokenStream,
    nested_options: Vec<TokenStream>,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #(#nested_options)*
        #comments
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq)]
        #annotations
        #extensible
        pub enum #name {
            #options
        }
    }
}
