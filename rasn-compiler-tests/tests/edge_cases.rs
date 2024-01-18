#![allow(non_camel_case_types)]
use rasn_compiler_tests::e2e_pdu;

#[test]
fn t() {
    println!(
        "{}",
        rasn_compiler::RasnCompiler::new()
            .add_asn_literal(
                r#" 
        TestModule DEFINITIONS AUTOMATIC TAGS ::= BEGIN

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

        END
        "#
            )
            .compile_to_string()
            .unwrap().0
    )
}

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
        } (1..10)"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("1..=10"))]
        pub struct Distinguished(pub u8);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, value("2..=8"))]
        pub struct Restricted(pub Distinguished);         "#
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
        } (1..10)"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("1..=10"))]
        pub struct Distinguished(pub u8);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        pub enum TestChoice {
            #[rasn(value("2..=8"))]
            restricted(Distinguished),
        }                                           "#
);