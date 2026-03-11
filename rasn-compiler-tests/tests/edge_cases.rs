#![allow(non_camel_case_types)]
use rasn_compiler_tests::e2e_pdu;

e2e_pdu!(
    distinguished_value_range,
    r#" Restricted ::= Distinguished (second|fourth..sixth|eighth)
        Distinguished ::= INTEGER {
            first(1),
            second(2),
            third(3),
            fourth(4),
            fifth(5),
            sixth(6),
            seventh(7),
            eighth(8),
            ninth(9),
            tenth(10),
        } (1..10)"#
);

e2e_pdu!(
    distinguished_value_range_in_choice,
    r#"
        TestChoice ::= CHOICE {
            restricted Distinguished (second|fourth..sixth|eighth),
            ...
        }
        Distinguished ::= INTEGER {
            first(1),
            second(2),
            third(3),
            fourth(4),
            fifth(5),
            sixth(6),
            seventh(7),
            eighth(8),
            ninth(9),
            tenth(10),
        } (1..10)"#
);

e2e_pdu!(
    distinguished_value_range_in_choice_from_impl,
    rasn_compiler::prelude::RasnConfig {
        generate_from_impls: true,
        ..Default::default()
    },
    r#"
        TestChoice ::= CHOICE {
            restricted Distinguished (second|fourth..sixth|eighth),
            ...
        }
        Distinguished ::= INTEGER {
            first(1),
            second(2),
            third(3),
            fourth(4),
            fifth(5),
            sixth(6),
            seventh(7),
            eighth(8),
            ninth(9),
            tenth(10),
        } (1..10)"#
);

e2e_pdu!(
    enum_and_distinguished_defaults,
    r#"
        Test ::= SEQUENCE {
            int IntWithDefault DEFAULT first,
            enum EnumWithDefault DEFAULT first,
        }

        IntWithDefault ::= INTEGER {
            first(1),
            second(2)
        } (1..10)

        EnumWithDefault ::= ENUMERATED {
            first(1),
            second(2)
        }
    "#
);

e2e_pdu!(
    same_variant_name,
    rasn_compiler::prelude::RasnConfig {
        generate_from_impls: true,
        ..Default::default()
    },
    r#"
        ChoiceType ::= CHOICE {
            number INTEGER,
            anotherNumber INTEGER,
            bool BOOLEAN,
            oneMoreNumber INTEGER,
            aString IA5String
        }
    "#
);

e2e_pdu!(
    recursive_type,
    rasn_compiler::prelude::RasnConfig {
        ..Default::default()
    },
    r#"
        Filter ::= CHOICE {
            and             [0] SET SIZE (1..MAX) OF filter Filter,
            or              [1] SET SIZE (1..MAX) OF filter Filter,
            not             [2] Filter,
            equalityMatch   [3] AttributeValueAssertion,
            ...
        }

        AttributeValueAssertion ::= SEQUENCE {
            attributeDesc   AttributeDescription,
            assertionValue  AssertionValue }

        AssertionValue ::= OCTET STRING

        AttributeDescription ::= LDAPString

        LDAPString ::= [UNIVERSAL 4] IMPLICIT UTF8String
    "#
);

// Regression test for issue #167.2 (Bug B): a type alias used as a SEQUENCE field type
// with a plain integer literal DEFAULT must generate a newtype-wrapped value, e.g.
// `MyUInt8(1)` rather than bare `1`.
e2e_pdu!(
    integer_type_alias_with_literal_default,
    r#"
        MySeq ::= SEQUENCE { count MyUInt8 DEFAULT 1 }
        MyUInt8 ::= INTEGER (0..255)
    "#
);
