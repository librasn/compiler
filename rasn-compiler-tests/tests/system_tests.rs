#![allow(non_camel_case_types)]
use rasn_compiler::prelude::{LexerError, LexerErrorType, ReportData};
use rasn_compiler_derive::asn1;
#[allow(unused_imports)]
use rasn_compiler_tests::e2e_pdu;

#[test]
fn trailing_comments() {
    asn1!(
        r#"
            -- preludio
            TestModuleA DEFINITIONS AUTOMATIC TAGS::= BEGIN
                Hello ::= INTEGER (4..8)
            END
            -- intermezzo
            TestModuleB DEFINITIONS AUTOMATIC TAGS::= BEGIN
                World ::= INTEGER (0..4)
            END
            -- finale

            -- postludio
    "#
    );
    assert_eq!(test_module_a::Hello(4).0, 4);
    assert_eq!(test_module_b::World(2).0, 2);
}

#[test]
fn multi_module_error() {
    assert!(matches!(
        rasn_compiler::Compiler::<rasn_compiler::prelude::RasnBackend, _>::new()
            .add_asn_literal(
                r#"
            -- preludio
            TestModuleA DEFINITIONS AUTOMATIC TAGS::= BEGIN
                Hello ::= INTEGER (4..8)
            END
            -- intermezzo
            TestModuleB DEFINITIONS AUTOMATIC TAGS::= BEGIN
                World ::= INTEGER (0..4)
            END
            -- finale
            TestModuleC DEFINITIONS AUTOMATIC TAGS::= BEGIN
                Fail :: INTGER (0..4)
            -- postludio
        "#,
            )
            .compile_to_string()
            .unwrap_err(),
        rasn_compiler::prelude::CompilerError::Lexer(LexerError {
            kind: LexerErrorType::MatchingError(ReportData { line: 12, .. })
        })
    ))
}

#[test]
fn custom_imports_on_path_asn() {
    let generated = rasn_compiler::Compiler::<rasn_compiler::prelude::RasnBackend, _>::new()
        .add_asn_by_path_with_custom_imports(
            "tests/modules/itu-t_x_x692_2008_Example6-ASN1-Module.asn1",
            ["my::special::import::*", "my::custom::Struct"],
        )
        .compile_to_string()
        .unwrap()
        .generated;
    assert!(generated.contains("use my::custom::Struct;"));
    assert!(generated.contains("use my::special::import::*;"));
}

#[test]
fn custom_imports_on_literal() {
    let generated = rasn_compiler::Compiler::<rasn_compiler::prelude::RasnBackend, _>::new()
        .add_asn_literal_with_custom_imports(
            r#"
                TestModuleA DEFINITIONS AUTOMATIC TAGS::= BEGIN
                    Hello ::= INTEGER (4..8)
                END"#,
            ["my::special::import::*", "my::custom::Struct"],
        )
        .compile_to_string()
        .unwrap()
        .generated;
    assert!(generated.contains("use my::custom::Struct;"));
    assert!(generated.contains("use my::special::import::*;"));
}
