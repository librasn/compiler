#![allow(non_camel_case_types)]
use rasn_compiler_tests::e2e_pdu;

#[test]
fn t() {
    println!("{}",rasn_compiler::RasnCompiler::new().add_asn_literal("TestModule DEFINITIONS AUTOMATIC TAGS::= BEGIN Test-Enum ::= ENUMERATED {
        test-1(3),
        test-2(-7)
    }
    test-enum-val Test-Enum ::= test-2 END").compile_to_string().unwrap().0)
}

e2e_pdu!(
    boolean,
    "Test-Boolean ::= BOOLEAN",
    r#" #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
        #[rasn(delegate)]
        pub struct TestBoolean(pub bool);                                 "#
);

e2e_pdu!(
    integer,
    "Test-Int ::= INTEGER",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate)]
        pub struct TestInt(pub Integer);                                 "#
);

e2e_pdu!(
    integer_distinguished_values,
    "Test-Int ::= INTEGER {
        first(0),
        second(1),
        third (2)
    } (0..2)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("0..=2"))]
        pub struct TestInt(pub u8);                                 "#
);

e2e_pdu!(
    integer_const,
    r#" Test-Int ::= INTEGER
        test-int-val Test-Int ::= 4"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate)]
        pub struct TestInt(pub Integer);
        lazy_static!{
            static ref TEST_INT_VAL: TestInt = TestInt(Integer::from(4));
        }                                                                            "#
);

e2e_pdu!(
    integer_strict,
    "Test-Int ::= INTEGER (4)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("4"))]
        pub struct TestInt(pub u8);                                 "#
);

e2e_pdu!(
    integer_strict_ext,
    "Test-Int ::= INTEGER (4,...)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("4", extensible))]
        pub struct TestInt(pub Integer);                                 "#
);

e2e_pdu!(
    integer_strict_ext_const,
    r#" Test-Int ::= INTEGER (4,...)
        test-int-val Test-Int ::= 4"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("4", extensible))]
        pub struct TestInt(pub Integer);
        lazy_static!{
            static ref TEST_INT_VAL: TestInt = TestInt(Integer::from(4));
        }                                                                            "#
);

e2e_pdu!(
    integer_range,
    r#" Test-Int ::= INTEGER (4..6)
        test-int-val Test-Int ::= 5"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("4..=6"))]
        pub struct TestInt(pub u8);
        pub const TEST_INT_VAL: TestInt = TestInt(5);                                 "#
);

e2e_pdu!(
    integer_range_ext_const,
    r#" Test-Int ::= INTEGER (4..6,...)
        test-int-val Test-Int ::= 5"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(delegate, value("4..=6", extensible))]
        pub struct TestInt(pub Integer);
        lazy_static!{
            static ref TEST_INT_VAL: TestInt = TestInt(Integer::from(5));
        }                                                                            "#
);

e2e_pdu!(
    null,
    "Test-Int ::= NULL",
    r#" #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
        #[rasn(delegate)]
        pub struct TestInt(());                                 "#
);

e2e_pdu!(
    bit_string,
    r#" Test-Bits ::= BIT STRING
        test-bits-val Test-Bits ::= '10101010'B"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate)]
        pub struct TestBits(pub BitString);                     "#
);

e2e_pdu!(
    bit_string_named_bits,
    r#" Test-Bits ::= BIT STRING {
            first(0) -- first bit --,
            last(1),
        } SIZE(2)                                           "#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("2"))]
        pub struct TestBits(pub BitString);                     "#
);

e2e_pdu!(
    bit_string_strict,
    "Test-Bits ::= BIT STRING SIZE(4)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("4"))]
        pub struct TestBits(pub BitString);                                 "#
);

e2e_pdu!(
    bit_string_strict_ext,
    "Test-Bits ::= BIT STRING (SIZE(4,...))",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("4", extensible))]
        pub struct TestBits(pub BitString);                                 "#
);

e2e_pdu!(
    bit_string_range,
    r#" Test-Bits ::= BIT STRING (SIZE(4..6))
        test-bits-val Test-Bits ::= '10101'B"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("4..=6"))]
        pub struct TestBits(pub BitString);                                "#
);

e2e_pdu!(
    bit_string_range_ext,
    r#" Test-Bits ::= BIT STRING (SIZE(4..6,...))
        test-bits-val Test-Bits ::= '10101010'B"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("4..=6", extensible))]
        pub struct TestBits(pub BitString);                                 "#
);

e2e_pdu!(
    octet_string,
    r#" Test-Octets ::= OCTET STRING
        test-octets-val Test-Octets ::= '10101010'B"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate)]
        pub struct TestOctets(pub OctetString);                     "#
);

e2e_pdu!(
    octet_string_strict,
    "Test-Octets ::= OCTET STRING SIZE(4)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("4"))]
        pub struct TestOctets(pub OctetString);                                 "#
);

e2e_pdu!(
    octet_string_strict_ext,
    "Test-Octets ::= OCTET STRING (SIZE(4,...))",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("4", extensible))]
        pub struct TestOctets(pub OctetString);                                 "#
);

e2e_pdu!(
    octet_string_range,
    r#" Test-Octets ::= OCTET STRING (SIZE(4..6))
        test-octets-val Test-Octets ::= 'FF010201FF'H"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("4..=6"))]
        pub struct TestOctets(pub OctetString);                                "#
);

e2e_pdu!(
    octet_string_range_ext,
    r#" Test-Octets ::= OCTET STRING SIZE(4..6,...)
        test-octets-val Test-Octets ::= 'FF010201FF2EDD60'H"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(delegate, size("4..=6", extensible))]
        pub struct TestOctets(pub OctetString);                                 "#
);

e2e_pdu!(
    enumerated,
    r#" Test-Enum ::= ENUMERATED {
            test-1,
            test-2(7)
        }
        test-enum-val Test-Enum ::= test-2          "#,
    r#" #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(enumerated)]
        pub enum TestEnum {
            test_1 = 0,
            test_2 = 7,
        }
        pub const TEST_ENUM_VAL: TestEnum = TestEnum::test_2;                                "#
);

e2e_pdu!(
    enumerated_negative,
    r#" Test-Enum ::= ENUMERATED {
            test-1(3),
            test-2(-7)
        }
        test-enum-val Test-Enum ::= test-2          "#,
    r#" #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(enumerated)]
        pub enum TestEnum {
            test_1 = 3,
            test_2 = -7,
        }
        pub const TEST_ENUM_VAL: TestEnum = TestEnum::test_2;                                "#
);

e2e_pdu!(
    empty_enumerated,
    r#" Test-Enum ::= ENUMERATED {
        }                           "#,
    r#" #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(enumerated)]
        pub enum TestEnum {
        }                                               "#
);

e2e_pdu!(
    extended_enumerated,
    r#" Test-Enum ::= ENUMERATED {
            test-1,
            ...,
            test-2(7),
        }
        test-enum-val Test-Enum ::= test-2          "#,
    r#" #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
        #[rasn(enumerated)]
        #[non_exhaustive]
        pub enum TestEnum {
            test_1 = 0,
            #[rasn(extension_addition)]
            test_2 = 7,
        }
        pub const TEST_ENUM_VAL: TestEnum = TestEnum::test_2;                                "#
);

// REAL Types are currently not supported by rasn
