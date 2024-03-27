use rasn_compiler_tests::e2e_pdu;

#[test]
fn param_type() {
    println!(
        "{:#?}",
        rasn_compiler::Compiler::new()
            .add_asn_literal(
                r#"
    NGAP-Test DEFINITIONS AUTOMATIC TAGS ::= BEGIN
        NGAP-PROTOCOL-EXTENSION ::= CLASS {
            &id				INTEGER			UNIQUE,
            &criticality	INTEGER,
            &Extension,
            &presence		BOOLEAN
        }
        WITH SYNTAX {
            ID				&id
            CRITICALITY		&criticality
            EXTENSION		&Extension
            PRESENCE		&presence
        }

        ProtocolExtensionContainer {NGAP-PROTOCOL-EXTENSION : ExtensionSetParam} ::= 
            SEQUENCE (SIZE (1..maxProtocolExtensions)) OF
            ProtocolExtensionField {{ExtensionSetParam}}

        ProtocolExtensionField {NGAP-PROTOCOL-EXTENSION : ExtensionSetParam} ::= SEQUENCE {
            id					NGAP-PROTOCOL-EXTENSION.&id				({ExtensionSetParam}),
            criticality			NGAP-PROTOCOL-EXTENSION.&criticality	({ExtensionSetParam}{@id}),
            extensionValue		NGAP-PROTOCOL-EXTENSION.&Extension		({ExtensionSetParam}{@id})
        }

        A2X-PC5-FlowBitRates ::= SEQUENCE {
            a2X-GuaranteedFlowBitRate		BOOLEAN,
            iE-Extensions		ProtocolExtensionContainer { {A2X-PC5-FlowBitRates-ExtIEs} }	OPTIONAL,
            ...
        }

        A2X-PC5-FlowBitRates-ExtIEs NGAP-PROTOCOL-EXTENSION ::= {
            ...
        }
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
