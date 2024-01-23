extern crate proc_macro;

use proc_macro::TokenStream;
use syn::{parse::Parse, parse_macro_input, LitStr};

const DUMMY_HEADER: &str = r#"asn1 { dummy(999) header(999) }

DEFINITIONS AUTOMATIC TAGS::= BEGIN
"#;
const DUMMY_FOOTER: &str = r#"END"#;

struct MacroInput {
    asn: LitStr,
}

impl Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            asn: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn asn1(input: TokenStream) -> TokenStream {
    let config = parse_macro_input!(input as MacroInput);

    let literal_asn1 = match config.asn.value() {
        v if v.contains("BEGIN") => v,
        v => String::from(DUMMY_HEADER) + &v + DUMMY_FOOTER,
    };

    rasn_compiler::Compiler::new()
        .add_asn_literal(literal_asn1)
        .compile_to_string()
        .unwrap()
        .generated
        .parse()
        .unwrap()
}
