#![allow(non_camel_case_types)]
use rasn_compiler_tests::e2e_pdu;

e2e_pdu!(
    boolean,
    r#" Test-Boolean ::= BOOLEAN
        Wrapping-Boolean ::= Test-Boolean
        value Wrapping-Boolean ::= FALSE"#
);

e2e_pdu!(
    integer,
    r#" Test-Int ::= INTEGER (0..123723)
        Wrapping-Int ::= Test-Int (0..123)
        value Wrapping-Int ::= 4"#
);

e2e_pdu!(
    sequence,
    r#" Test-Int ::= INTEGER (0..123723)
        Wrapping-Int ::= Test-Int (0..123)
        Test-Boolean ::= BOOLEAN
        Wrapping-Boolean ::= Test-Boolean
        Test-Sequence ::= SEQUENCE {
            int Wrapping-Int DEFAULT 5,
            boolean Wrapping-Boolean,
        }
        value Test-Sequence ::= { boolean TRUE }"#
);

e2e_pdu!(
    constraint_cross_reference,
    r#" Test-Int ::= INTEGER (0..123723)
        Wrapping-Int ::= Test-Int (0..value)
        value Test-Int ::= 5"#
);
