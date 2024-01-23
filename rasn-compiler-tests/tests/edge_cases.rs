#![allow(non_camel_case_types)]
use rasn_compiler_tests::e2e_pdu;

#[test]
fn t() {
    println!(
        "{}",
        rasn_compiler::Compiler::new()
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
            .unwrap().generated
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
    "#,
    r#" 
        #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(enumerated)]
        pub enum EnumWithDefault {
            first = 1,
            second = 2,
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("1..=10"))]
        pub struct IntWithDefault(pub u8);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Test {
            #[rasn(default = "test_int_default")]
            pub int: IntWithDefault,
            #[rasn(default = "test_r_enum_default")]
            pub r_enum: EnumWithDefault,
        }
        impl Test {
            pub fn new(int: IntWithDefault, r_enum: EnumWithDefault) -> Self {
                Self { int, r_enum }
            }
        }
        fn test_int_default() -> IntWithDefault {
            IntWithDefault(1)
        }
        fn test_r_enum_default() -> EnumWithDefault {
            EnumWithDefault::first
        }                                           "#
);