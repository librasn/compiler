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

e2e_pdu! {
parameterized_type,
r#"
        ParamType { INTEGER: lower, BOOLEAN: flag } ::= SEQUENCE {
            int-value INTEGER (lower..12),
            bool-value BOOLEAN DEFAULT flag
        }
        ImplType ::= ParamType { 2, TRUE }
    "#,
r#"
        #[derive (AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn (automatic_tags)]
        pub struct ImplType {
            #[rasn(value("2..=12"))]
            pub int_value : u8,
            #[rasn (default = "impl_type_bool_value_default")]
            pub bool_value : bool,
        }

        impl ImplType {
            pub fn new (int_value: u8 , bool_value: bool) -> Self {
                Self { int_value , bool_value, }
            }
        }

        fn impl_type_bool_value_default () -> bool {
            true
        }
    "#
}
