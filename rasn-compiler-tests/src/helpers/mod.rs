use std::num::ParseIntError;
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[macro_export]
macro_rules! e2e_pdu {
    ($suite:ident, $asn1:literal, $expected:literal) => {
        #[test]
        fn $suite() {
            rasn_compiler_derive::asn1!($asn1);
            assert_eq!(
                rasn_compiler::Compiler::new()
                    .add_asn_literal(&format!("TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN {} END", $asn1))
                    .compile_to_string()
                    .unwrap()
                    .generated
                    .replace(|c: char| c.is_whitespace(), "")
                    .replace("#[allow(non_camel_case_types,non_snake_case,non_upper_case_globals,unused)]pubmodtest_module{externcratealloc;use core::borrow::Borrow;uselazy_static::lazy_static;userasn::prelude::*;", ""),
                format!("{}}}", $expected)
                    .to_string()
                    .replace(|c: char| c.is_whitespace(), ""),
            )
        }
    };
}


#[macro_export]
macro_rules! e2e_module {
    ($suite:ident, $asn1:literal, $expected:literal) => {
        #[test]
        fn $suite() {
            assert_eq!(
                rasn_compiler::Compiler::new()
                    .add_asn_literal(asn1)
                    .compile_to_string()
                    .unwrap()
                    .0
                    .replace(|c: char| c.is_whitespace(), ""),
                $expected
                    .to_string()
                    .replace(|c: char| c.is_whitespace(), ""),
            )
        }
    };
}
