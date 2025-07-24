use proc_macro2::{Ident, TokenStream};
use quote::quote;

pub fn typealias_template(
    comments: TokenStream,
    name: TokenStream,
    alias: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name (pub #alias);
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

pub fn lazy_static_value_template(
    comments: TokenStream,
    name: Ident,
    vtype: TokenStream,
    value: TokenStream,
    no_std_compliant: bool,
) -> TokenStream {
    if no_std_compliant {
        quote! {
            lazy_static! {
                #comments
                pub static ref #name: #vtype = #value;
            }
        }
    } else {
        quote! {
            #comments
            pub static #name: LazyLock< #vtype > = LazyLock::new(||
                #value
            );
        }
    }
}

pub fn integer_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
    integer_type: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name (pub #integer_type);

    }
}

pub fn generalized_time_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub GeneralizedTime);
    }
}

pub fn utc_time_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub UtcTime);
    }
}

pub fn bit_string_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub BitString);
    }
}

pub fn fixed_bit_string_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
    size: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub FixedBitString<#size>);
    }
}

pub fn octet_string_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub OctetString);
    }
}

pub fn fixed_octet_string_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
    size: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub FixedOctetString<#size>);
    }
}

pub fn char_string_template(
    comments: TokenStream,
    name: TokenStream,
    string_type: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub #string_type);
    }
}

pub fn boolean_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub bool);
    }
}

pub fn primitive_value_template(
    comments: TokenStream,
    name: Ident,
    type_name: TokenStream,
    assignment: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        pub const #name: #type_name = #assignment;
    }
}

pub fn enum_value_template(
    comments: TokenStream,
    name: Ident,
    enumerated: TokenStream,
    enumeral: Ident,
) -> TokenStream {
    quote! {
        #comments
        pub const #name: #enumerated = #enumerated::#enumeral;
    }
}

pub fn null_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub ());
    }
}

pub fn any_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub Any);
    }
}

pub fn oid_template(
    comments: TokenStream,
    name: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        pub struct #name(pub ObjectIdentifier);
    }
}

pub fn enumerated_template(
    comments: TokenStream,
    name: TokenStream,
    extensible: TokenStream,
    enum_members: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #comments
        #annotations
        #extensible
        pub enum #name {
            #enum_members
        }
    }
}

pub fn sequence_or_set_value_template(
    comments: TokenStream,
    name: Ident,
    vtype: TokenStream,
    members: TokenStream,
    no_std_compliant: bool,
) -> TokenStream {
    if no_std_compliant {
        quote! {
        lazy_static! {
                #comments
                pub static ref #name: #vtype = #vtype ::new(
                    #members
                );
            }
        }
    } else {
        quote! {
            #comments
            pub static #name: LazyLock< #vtype > = LazyLock::new(||
                #vtype ::new(
                    #members
                )
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn sequence_or_set_template(
    comments: TokenStream,
    name: TokenStream,
    extensible: TokenStream,
    members: TokenStream,
    nested_members: Vec<TokenStream>,
    annotations: TokenStream,
    default_methods: TokenStream,
    new_impl: TokenStream,
    default_impl: TokenStream,
    class_fields: TokenStream,
) -> TokenStream {
    quote! {
        #(#nested_members)*
        #comments
        #annotations
        #extensible
        pub struct #name {
            #members
        }

        #new_impl

        #default_impl

        #class_fields

        #default_methods
    }
}

pub fn sequence_or_set_of_template(
    is_set_of: bool,
    comments: TokenStream,
    name: TokenStream,
    anonymous_item: TokenStream,
    member_type: TokenStream,
    annotations: TokenStream,
) -> TokenStream {
    let generic_type = is_set_of
        .then(|| quote!(SetOf))
        .unwrap_or(quote!(SequenceOf));
    quote! {
            #anonymous_item
            #comments
            #annotations
            pub struct #name(pub #generic_type<#member_type>);
    }
}

pub fn choice_value_template(
    comments: TokenStream,
    name: Ident,
    type_id: TokenStream,
    choice_name: Ident,
    inner_decl: TokenStream,
    no_std_compliant: bool,
) -> TokenStream {
    if no_std_compliant {
        quote! {
            lazy_static! {
                #comments
                pub static ref #name: #type_id = #type_id :: #choice_name (#inner_decl);
            }
        }
    } else {
        quote! {
            #comments
            pub static #name: LazyLock< #type_id > = LazyLock::new(||
                #type_id :: #choice_name (#inner_decl)
            );
        }
    }
}

pub fn const_choice_value_template(
    comments: TokenStream,
    name: Ident,
    type_id: TokenStream,
    choice_name: Ident,
    inner_decl: TokenStream,
) -> TokenStream {
    quote! {
            #comments
            pub const #name: #type_id = #type_id :: #choice_name (#inner_decl);
    }
}

pub fn choice_template(
    comments: TokenStream,
    name: &TokenStream,
    extensible: TokenStream,
    options: TokenStream,
    nested_options: Vec<TokenStream>,
    annotations: TokenStream,
) -> TokenStream {
    quote! {
        #(#nested_options)*
        #comments
        #annotations
        #extensible
        pub enum #name {
            #options
        }
    }
}

pub fn choice_from_impl_template(
    name: &TokenStream,
    variant: Ident,
    wrapped: TokenStream,
) -> TokenStream {
    quote! {
        impl From<#wrapped> for #name {
            fn from(value: #wrapped) -> Self {
                Self::#variant(value)
            }
        }
    }
}
