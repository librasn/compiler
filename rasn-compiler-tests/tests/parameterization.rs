use rasn_compiler_tests::e2e_pdu;

#[test]
fn param_type() {
    println!(
        "{:#?}",
        rasn_compiler::Compiler::new()
            .add_asn_literal(
                r#"
    Param-Test DEFINITIONS AUTOMATIC TAGS ::= BEGIN
    ParamType { INTEGER: lower, BOOLEAN: flag } ::= SEQUENCE {
            int-value INTEGER (lower..12),
            bool-value BOOLEAN DEFAULT flag
    }
    ImplType ::= ParamType { 2, TRUE }
    END
    "#
            )
            .compile_to_string()
            .unwrap()
    );
}

// e2e_pdu!(
//     parameterized_type
//     r#"
//     ParamType { INTEGER: lower, BOOLEAN: flag } ::= SEQUENCE {
//         int-value INTEGER (lower..12),
//         bool-value BOOLEAN DEFAULT flag
//     }
//     ImplType ::= ParamType { 2, TRUE }
//     "#,
//     r#""#
// )
