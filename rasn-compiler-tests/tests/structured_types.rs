use rasn_compiler_tests::e2e_pdu;

e2e_pdu!(
    sequence_of_primitive_value,
    r#" value SEQUENCE OF INTEGER ::= { 1, 2, 3 }"#
);

e2e_pdu!(
    sequence_of_in_set_field,
    r#"PersonnelRecord ::=  SET {
        title VisibleString,
        children SEQUENCE OF VisibleString DEFAULT {}
    }"#
);

e2e_pdu!(
    sized_sequence_of_in_sequence_field,
    r#"PersonnelRecord ::=  SET {
        title VisibleString,
        children SEQUENCE SIZE(0..4) OF VisibleString DEFAULT {}
    }"#
);

e2e_pdu!(
    nested_choice_value,
    r#"
        NestedType ::= SEQUENCE {
            choiceField CHOICE {
                one INTEGER,
                two BOOLEAN
            }
        }

        nestedTypeVal NestedType ::= { choiceField one:4 }
    "#
);

e2e_pdu!(
    nested_choice_value_from_impl,
    rasn_compiler::prelude::RasnConfig {
        generate_from_impls: true,
        ..Default::default()
    },
    r#"
        NestedType ::= SEQUENCE {
            choiceField CHOICE {
                one INTEGER,
                two BOOLEAN
            }
        }

        nestedTypeVal NestedType ::= { choiceField one:4 }
    "#
);

e2e_pdu!(
    deeply_nested_type,
    r#"
        TFCS-ReconfAdd-r12 ::= SEQUENCE {
            ctfcSize							CHOICE {
                ctfc8Bit							SEQUENCE (SIZE (1..maxTFC)) OF SEQUENCE {
                    ctfc8								INTEGER (0..255),
                },
                ctfc16Bit							SEQUENCE (SIZE (1..maxTFC)) OF SEQUENCE {
                    ctfc16								INTEGER(0..65535),
                }
            }
        }

        maxTFC						INTEGER ::= 1024
    "#
);

e2e_pdu!(
    single_named_bit_default,
    r#"
        Test::= SET {
            parameters[0] IMPLICIT SEQUENCE {
                        color [0] IMPLICIT Color DEFAULT {blue}
            }
        }

        Color ::= BIT STRING {red(0), blue(1), yellow(2)}
    "#
);

e2e_pdu!(
    multiple_named_bits_default,
    r#"
        Test::= SET {
            parameters[0] IMPLICIT SEQUENCE {
                        color [0] IMPLICIT Color DEFAULT {blue, yellow}
            }
        }

        Color ::= BIT STRING {red(0), blue(1), yellow(2)}
    "#
);

e2e_pdu!(
    anonymous_sequence_of_item_in_sequence_member,
    r#"
    Ticket ::= SEQUENCE {
        ages		SEQUENCE OF INTEGER (1..5),	 
        passenger	Passenger OPTIONAL
    }

    Passenger ::= ENUMERATED {
        adult	(0),
        youth	(1),
        ...
    }
    "#
);

e2e_pdu!(
    anonymous_set_of_item_in_choice_option,
    r#"
    Ticket ::= CHOICE {
        age-set		SET (SIZE (1..4)) OF INTEGER (1..5)
    }
    "#
);

e2e_pdu!(
    nested_recursion,
    r#"
        TypeDescription ::= CHOICE {
            boolean [0] IMPLICIT BOOLEAN,
            string [2] IMPLICIT UTF8String,
            array [3] IMPLICIT SEQUENCE {
                size [0] IMPLICIT INTEGER (0..MAX),
                element-type [1] TypeDescription
            }
        }
    "#
);

e2e_pdu!(
    nested_recursion_elsewhere,
    r#"
        TypeSpecification ::= CHOICE {
            array			[1] IMPLICIT SEQUENCE
            {
                elementType		[2] TypeSpecification
            }
        }

        GetVariableAccessAttributesResponse ::= SEQUENCE
        {
            typeSpecification	[2] TypeSpecification
        }
    "#
);

e2e_pdu!(
    nested_recursion_ping_pong,
    r#"
    TypeDescription ::= CHOICE {
        array [1] IMPLICIT SEQUENCE {
            elementType [2] TypeSpecification
        },
        structure [2] IMPLICIT SEQUENCE {
            components [1] IMPLICIT SEQUENCE OF SEQUENCE {
                componentType [1] TypeSpecification
            }
        }
    }

    TypeSpecification ::= CHOICE {
        typeDescription TypeDescription
    }

    VariableSpecification ::= CHOICE {
        variableDescription [2] IMPLICIT SEQUENCE {
            typeSpecification TypeSpecification
        }
    }
    "#
);

e2e_pdu!(
    tagged_prefix_type,
    r#"
        A ::= SEQUENCE {
            member SEQUENCE OF [0] INTEGER,
        }
    "#
);
