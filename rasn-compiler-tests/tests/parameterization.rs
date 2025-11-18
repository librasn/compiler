use rasn_compiler_tests::e2e_pdu;

e2e_pdu! {
parameterized_type,
r#"
        ParamType { INTEGER: lower, BOOLEAN: flag } ::= SEQUENCE {
            int-value INTEGER (lower..12),
            bool-value BOOLEAN DEFAULT flag
        }
        ImplType ::= ParamType { 2, TRUE }
    "#
}

e2e_pdu! {
parameterized_information_object_classes,
rasn_compiler::prelude::RasnConfig {
    opaque_open_types: false,
    ..Default::default()
},
r#"
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
    "#
}

e2e_pdu! {
anonymous_type_param,
r#"
            SetupRelease { ElementTypeParam } ::= CHOICE { 
                release         NULL,
                setup           ElementTypeParam 
            }

            LocationMeasurementInfo ::= SEQUENCE {
                test BOOLEAN
            }

            LocationMeasurementIndication-IEs ::=       SEQUENCE { 
                measurementIndication      SetupRelease { LocationMeasurementInfo }, 
                lateNonCriticalExtension   OCTET STRING OPTIONAL,
                nonCriticalExtension       SEQUENCE{} OPTIONAL 
            }
        "#
}
