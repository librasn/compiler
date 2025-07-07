#![allow(non_camel_case_types)]
use rasn_compiler_tests::e2e_pdu;

e2e_pdu!(
    boolean,
    "Test-Boolean ::= BOOLEAN",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
        #[rasn(delegate, identifier = "Test-Boolean")]
        pub struct TestBoolean(pub bool);                                 "#
);

e2e_pdu!(
    boolean_value,
    "test-boolean BOOLEAN ::= TRUE",
    r#" pub const TEST_BOOLEAN: bool = true;                               "#
);

e2e_pdu!(
    integer,
    "Test-Int ::= INTEGER",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Int")]
        pub struct TestInt(pub Integer);                                 "#
);

e2e_pdu!(
    integer_value,
    "test-int INTEGER ::= 4",
    r#" pub static TEST_INT: LazyLock<Integer> = LazyLock::new(|| Integer::from(4));                           "#
);

e2e_pdu!(
    integer_value_constrained,
    "test-int INTEGER(0..255) ::= 4",
    r#"pub const TEST_INT: u8 = 4;"#
);

e2e_pdu!(
    integer_value_large_constrained,
    "test-int INTEGER(0..MAX) ::= 4",
    r#"pub static TEST_INT: LazyLock<Integer> = LazyLock::new(|| Integer::from(4));"#
);

e2e_pdu!(
    integer_distinguished_values,
    "Test-Int ::= INTEGER {
        first(0),
        second(1),
        third (2)
    } (0..2)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Int", value("0..=2"))]
        pub struct TestInt(pub u8);                                 "#
);

e2e_pdu!(
    integer_const,
    r#" Test-Int ::= INTEGER
        test-int-val Test-Int ::= 4"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Int")]
        pub struct TestInt(pub Integer);
        pub static TEST_INT_VAL: LazyLock<TestInt> = LazyLock::new(||
            TestInt(Integer::from(4))
        );                                                                            "#
);

e2e_pdu!(
    integer_strict,
    "Test-Int ::= INTEGER (4)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Int", value("4"))]
        pub struct TestInt(pub u8);                                 "#
);

e2e_pdu!(
    integer_strict_ext,
    "Test-Int ::= INTEGER (4,...)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Int", value("4", extensible))]
        pub struct TestInt(pub Integer);                                 "#
);

e2e_pdu!(
    integer_strict_ext_const,
    r#" Test-Int ::= INTEGER (4,...)
        test-int-val Test-Int ::= 4"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Int", value("4", extensible))]
        pub struct TestInt(pub Integer);
        pub static TEST_INT_VAL: LazyLock<TestInt> = LazyLock::new(||
            TestInt(Integer::from(4))
        );                                                                            "#
);

e2e_pdu!(
    integer_range,
    r#" Test-Int ::= INTEGER (4..6)
        test-int-val Test-Int ::= 5"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Int", value("4..=6"))]
        pub struct TestInt(pub u8);
        pub const TEST_INT_VAL: TestInt = TestInt(5);                                 "#
);

e2e_pdu!(
    integer_range_ext_const,
    r#" Test-Int ::= INTEGER (4..6,...)
        test-int-val Test-Int ::= 5"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Int", value("4..=6", extensible))]
        pub struct TestInt(pub Integer);
        pub static TEST_INT_VAL: LazyLock<TestInt> = LazyLock::new(||
            TestInt(Integer::from(5))
        );                                                                            "#
);

e2e_pdu!(
    null,
    "Test-Int ::= NULL",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
        #[rasn(delegate, identifier = "Test-Int")]
        pub struct TestInt(pub ());                                 "#
);

e2e_pdu!(
    null_value,
    "test-null NULL ::= NULL",
    r#" pub const TEST_NULL: () = ();                           "#
);

e2e_pdu!(
    bit_string,
    r#" Test-Bits ::= BIT STRING
        test-bits-val Test-Bits ::= '10101010'B"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Bits")]
        pub struct TestBits(pub BitString);
        pub static TEST_BITS_VAL: LazyLock<TestBits> = LazyLock::new(|| {
            TestBits(
                [true,false,true,false,true,false,true,false].into_iter().collect(),
            )
        });                                                       "#
);

e2e_pdu!(
    bit_string_value,
    "test-bits BIT STRING ::= '1010'B",
    r#" pub static TEST_BITS: LazyLock<BitString> = LazyLock::new(|| [true,false,true,false].into_iter().collect());                           "#
);

e2e_pdu!(
    bit_string_value_hex,
    "test-bits BIT STRING ::= 'FF'H",
    r#" pub static TEST_BITS: LazyLock<BitString> = LazyLock::new(|| { [true,true,true,true,true,true,true,true].into_iter().collect() });                           "#
);

e2e_pdu!(
    bit_string_named_bits,
    r#" Test-Bits ::= BIT STRING {
            first(0) -- first bit --,
            last(1),
        } SIZE(2)                                           "#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Bits")]
        pub struct TestBits(pub FixedBitString<2usize>);                     "#
);

e2e_pdu!(
    bit_string_strict,
    "Test-Bits ::= BIT STRING SIZE(4)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Bits")]
        pub struct TestBits(pub FixedBitString<4usize>);                                 "#
);

e2e_pdu!(
    bit_string_strict_ext,
    "Test-Bits ::= BIT STRING (SIZE(4,...))",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Bits", size("4", extensible))]
        pub struct TestBits(pub BitString);                                 "#
);

e2e_pdu!(
    bit_string_range,
    r#" Test-Bits ::= BIT STRING (SIZE(4..6))
        test-bits-val Test-Bits ::= '10101'B"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Bits", size("4..=6"))]
        pub struct TestBits(pub BitString);
        pub static TEST_BITS_VAL: LazyLock<TestBits> = LazyLock::new(||
            TestBits(
                [true,false,true,false,true].into_iter().collect()
            )
        );                                                                   "#
);

e2e_pdu!(
    bit_string_range_ext,
    r#" Test-Bits ::= BIT STRING (SIZE(4..6,...))
        test-bits-val Test-Bits ::= 'D5'H"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Bits", size("4..=6", extensible))]
        pub struct TestBits(pub BitString);
        pub static TEST_BITS_VAL: LazyLock<TestBits> = LazyLock::new(|| {
            TestBits(
                [true,true,false,true,false,true,false,true].into_iter().collect(),
            )
        });                                                                          "#
);

e2e_pdu!(
    octet_string,
    r#" Test-Octets ::= OCTET STRING
        test-octets-val Test-Octets ::= '10101010'B"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Octets")]
        pub struct TestOctets(pub OctetString);
        pub static TEST_OCTETS_VAL: LazyLock<TestOctets> = LazyLock::new(||
            TestOctets(
                <OctetStringasFrom<&'static[u8]>>::from(&[170])
            )
        );                                                           "#
);

e2e_pdu!(
    octet_string_value,
    "test-bytes OCTET STRING ::= 'FF'H",
    r#" pub static TEST_BYTES: LazyLock<OctetString> = LazyLock::new(|| <OctetString as From<&'static[u8]>>::from(&[255]));                           "#
);

e2e_pdu!(
    octet_string_value_binary,
    "test-bytes OCTET STRING ::= '11111111'B",
    r#"pub static TEST_BYTES: LazyLock<OctetString> = LazyLock::new(|| <OctetString as From<&'static[u8]>>::from(&[255]));                           "#
);

e2e_pdu!(
    octet_string_strict,
    "Test-Octets ::= OCTET STRING SIZE(4)",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Octets")]
        pub struct TestOctets(pub FixedOctetString<4usize>);                                 "#
);

e2e_pdu!(
    octet_string_strict_ext,
    "Test-Octets ::= OCTET STRING (SIZE(4,...))",
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Octets", size("4", extensible))]
        pub struct TestOctets(pub OctetString);                                 "#
);

e2e_pdu!(
    octet_string_range,
    r#" Test-Octets ::= OCTET STRING (SIZE(4..6))
        test-octets-val Test-Octets ::= 'FF010201FF'H"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Octets", size("4..=6"))]
        pub struct TestOctets(pub OctetString);
        pub static TEST_OCTETS_VAL: LazyLock<TestOctets> = LazyLock::new(|| {
            TestOctets(
                <OctetString as From<&'static[u8]>>::from(&[255, 1, 2, 1, 255,])
            )
        });                                                                       "#
);

e2e_pdu!(
    octet_string_range_ext,
    r#" Test-Octets ::= OCTET STRING SIZE(4..6,...)
        test-octets-val Test-Octets ::= 'FF010201FF2EDD60'H"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-Octets", size("4..=6", extensible))]
        pub struct TestOctets(pub OctetString);
        pub static TEST_OCTETS_VAL: LazyLock<TestOctets> = LazyLock::new(|| {
            TestOctets(
                <OctetString as From<&'static[u8]>>::from(&[255, 1, 2, 1, 255, 46, 221, 96,])
            )
        });                                                                       "#
);

e2e_pdu!(
    enumerated,
    r#" Test-Enum ::= ENUMERATED {
            test-1,
            test-2(7)
        }
        test-enum-val Test-Enum ::= test-2          "#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
        #[rasn(enumerated, identifier = "Test-Enum")]
        pub enum TestEnum {
            #[rasn(identifier = "test-1")]
            test_1 = 0,
            #[rasn(identifier = "test-2")]
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
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
        #[rasn(enumerated, identifier = "Test-Enum")]
        pub enum TestEnum {
            #[rasn(identifier = "test-1")]
            test_1 = 3,
            #[rasn(identifier = "test-2")]
            test_2 = -7,
        }
        pub const TEST_ENUM_VAL: TestEnum = TestEnum::test_2;                                "#
);

e2e_pdu!(
    empty_enumerated,
    r#" Test-Enum ::= ENUMERATED {
        }                           "#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
        #[rasn(enumerated, identifier = "Test-Enum")]
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
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
        #[rasn(enumerated, identifier = "Test-Enum")]
        #[non_exhaustive]
        pub enum TestEnum {
            #[rasn(identifier = "test-1")]
            test_1 = 0,
            #[rasn(extension_addition, identifier = "test-2")]
            test_2 = 7,
        }
        pub const TEST_ENUM_VAL: TestEnum = TestEnum::test_2;                                "#
);

// REAL Types are currently not supported by rasn

e2e_pdu!(
    bmp,
    r#" Test-String ::= BMPString
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String")]
        pub struct TestString(pub BmpString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(BmpString::try_from("012345").unwrap())
        );"#
);
e2e_pdu!(
    bmp_strict,
    r#" Test-String ::= BMPString SIZE (4)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4"))]
        pub struct TestString(pub BmpString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                BmpString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    bmp_strict_ext,
    r#" Test-String ::= BMPString SIZE (4,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4", extensible))]
        pub struct TestString(pub BmpString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                BmpString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    bmp_range,
    r#" Test-String ::= BMPString SIZE (4..6)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6"))]
        pub struct TestString(pub BmpString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                BmpString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    bmp_range_ext,
    r#" Test-String ::= BMPString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6", extensible))]
        pub struct TestString(pub BmpString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                BmpString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    numeric,
    r#" Test-String ::= NumericString
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String")]
        pub struct TestString(pub NumericString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                NumericString::try_from("012345").unwrap()
            )
        );                                                           "#
);
e2e_pdu!(
    numeric_strict,
    r#" Test-String ::= NumericString SIZE (4)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4"))]
        pub struct TestString(pub NumericString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                NumericString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    numeric_strict_ext,
    r#" Test-String ::= NumericString SIZE (4,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4", extensible))]
        pub struct TestString(pub NumericString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                NumericString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    numeric_range,
    r#" Test-String ::= NumericString SIZE (4..6)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6"))]
        pub struct TestString(pub NumericString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                NumericString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    numeric_range_ext,
    r#" Test-String ::= NumericString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6", extensible))]
        pub struct TestString(pub NumericString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                NumericString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    ia5,
    r#" Test-String ::= IA5String
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String")]
        pub struct TestString(pub Ia5String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                Ia5String::try_from("012345").unwrap()
            )
        );                                                           "#
);
e2e_pdu!(
    ia5_strict,
    r#" Test-String ::= IA5String SIZE (4)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4"))]
        pub struct TestString(pub Ia5String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                Ia5String::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    ia5_strict_ext,
    r#" Test-String ::= IA5String SIZE (4,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4", extensible))]
        pub struct TestString(pub Ia5String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                Ia5String::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    ia5_range,
    r#" Test-String ::= IA5String SIZE (4..6)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6"))]
        pub struct TestString(pub Ia5String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                Ia5String::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    ia5_range_ext,
    r#" Test-String ::= IA5String SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6", extensible))]
        pub struct TestString(pub Ia5String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                Ia5String::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    printable,
    r#" Test-String ::= PrintableString
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String")]
        pub struct TestString(pub PrintableString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                PrintableString::try_from("012345").unwrap()
            )
        );                                                           "#
);
e2e_pdu!(
    printable_strict,
    r#" Test-String ::= PrintableString SIZE (4)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4"))]
        pub struct TestString(pub PrintableString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                PrintableString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    printable_strict_ext,
    r#" Test-String ::= PrintableString SIZE (4,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4", extensible))]
        pub struct TestString(pub PrintableString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(
                PrintableString::try_from("012345").unwrap()
            )
        );                                                           "#
);

e2e_pdu!(
    printable_range,
    r#" Test-String ::= PrintableString SIZE (4..6)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6"))]
        pub struct TestString(pub PrintableString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(PrintableString::try_from("012345").unwrap())
        );                                                           "#
);

e2e_pdu!(
    printable_range_ext,
    r#" Test-String ::= PrintableString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6", extensible))]
        pub struct TestString(pub PrintableString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(PrintableString::try_from("012345").unwrap())
        );                                                           "#
);

e2e_pdu!(
    general,
    r#" Test-String ::= GeneralString
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String")]
        pub struct TestString(pub GeneralString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GeneralString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);
e2e_pdu!(
    general_strict,
    r#" Test-String ::= GeneralString SIZE (4)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4"))]
        pub struct TestString(pub GeneralString);
        pub static  TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GeneralString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    general_strict_ext,
    r#" Test-String ::= GeneralString SIZE (4,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4", extensible))]
        pub struct TestString(pub GeneralString);
        pub static  TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GeneralString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    general_range,
    r#" Test-String ::= GeneralString SIZE (4..6)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6"))]
        pub struct TestString(pub GeneralString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GeneralString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    general_range_ext,
    r#" Test-String ::= GeneralString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6", extensible))]
        pub struct TestString(pub GeneralString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GeneralString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    graphic,
    r#" Test-String ::= GraphicString
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String")]
        pub struct TestString(pub GraphicString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GraphicString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    graphic_strict,
    r#" Test-String ::= GraphicString SIZE (4)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4"))]
        pub struct TestString(pub GraphicString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GraphicString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    graphic_strict_ext,
    r#" Test-String ::= GraphicString SIZE (4,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4", extensible))]
        pub struct TestString(pub GraphicString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GraphicString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    graphic_range,
    r#" Test-String ::= GraphicString SIZE (4..6)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6"))]
        pub struct TestString(pub GraphicString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GraphicString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    graphic_range_ext,
    r#" Test-String ::= GraphicString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6", extensible))]
        pub struct TestString(pub GraphicString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(GraphicString::try_from(String::from("012345")).unwrap())
        );                                                           "#
);

e2e_pdu!(
    utf8,
    r#" Test-String ::= UTF8String
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String")]
        pub struct TestString(pub Utf8String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(String::from("012345"))
        );                                                          "#
);
e2e_pdu!(
    utf8_strict,
    r#" Test-String ::= UTF8String SIZE (4)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4"))]
        pub struct TestString(pub Utf8String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(String::from("012345"))
        );                                                          "#
);

e2e_pdu!(
    utf8_strict_ext,
    r#" Test-String ::= UTF8String SIZE (4,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4", extensible))]
        pub struct TestString(pub Utf8String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(String::from("012345"))
        );                                                          "#
);

e2e_pdu!(
    utf8_range,
    r#" Test-String ::= UTF8String SIZE (4..6)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6"))]
        pub struct TestString(pub Utf8String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(String::from("012345"))
        );                                                          "#
);

e2e_pdu!(
    utf8_range_ext,
    r#" Test-String ::= UTF8String SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6", extensible))]
        pub struct TestString(pub Utf8String);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(String::from("012345"))
        );                                                          "#
);

e2e_pdu!(
    visible,
    r#" Test-String ::= VisibleString
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String")]
        pub struct TestString(pub VisibleString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(VisibleString::try_from("012345").unwrap())
        );                                                          "#
);
e2e_pdu!(
    visible_strict,
    r#" Test-String ::= VisibleString SIZE (4)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4"))]
        pub struct TestString(pub VisibleString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(VisibleString::try_from("012345").unwrap())
        );                                                          "#
);

e2e_pdu!(
    visible_strict_ext,
    r#" Test-String ::= VisibleString SIZE (4,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4", extensible))]
        pub struct TestString(pub VisibleString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(VisibleString::try_from("012345").unwrap())
        );                                                          "#
);

e2e_pdu!(
    visible_range,
    r#" Test-String ::= VisibleString SIZE (4..6)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6"))]
        pub struct TestString(pub VisibleString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(VisibleString::try_from("012345").unwrap())
        );                                                          "#
);

e2e_pdu!(
    visible_range_ext,
    r#" Test-String ::= VisibleString SIZE (4..6,...)
        test-string-val Test-String ::= "012345""#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "Test-String", size("4..=6", extensible))]
        pub struct TestString(pub VisibleString);
        pub static TEST_STRING_VAL: LazyLock<TestString> = LazyLock::new(||
            TestString(VisibleString::try_from("012345").unwrap())
        );                                                          "#
);

e2e_pdu!(
    visible_from,
    r#" FQDN ::= VisibleString(FROM ("a".."z" | "A".."Z" | "0".."9" | ".-")) (SIZE (1..255))"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, size("1..=255"), from("\u{2d}", "\u{2e}", "\u{30}..=\u{39}", "\u{41}..=\u{5a}", "\u{61}..=\u{7a}"))]
        pub struct FQDN(pub VisibleString);                             "#
);
