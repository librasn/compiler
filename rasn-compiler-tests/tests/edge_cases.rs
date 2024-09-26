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
        } (1..10)"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, value("1..=10"))]
        pub struct Distinguished(pub u8);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
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
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, value("1..=10"))]
        pub struct Distinguished(pub u8);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        pub enum TestChoice {
            #[rasn(value("2..=8"))]
            restricted(Distinguished),
        }                                           "#
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
        } (1..10)"#,
    r#" #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, value("1..=10"))]
        pub struct Distinguished(pub u8);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        pub enum TestChoice {
            #[rasn(value("2..=8"))]
            restricted(Distinguished),
        }
        impl From<Distinguished> for TestChoice {
            fn from(value: Distinguished) -> Self {
                Self::restricted(value)
            }
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
        #[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(enumerated)]
        pub enum EnumWithDefault {
            first = 1,
            second = 2,
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, value("1..=10"))]
        pub struct IntWithDefault(pub u8);
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags)]
        pub struct Test {
            #[rasn(default = "test_int_default")]
            pub int: IntWithDefault,
            #[rasn(default = "test_r_enum_default", identifier = "enum")]
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
    "#,
    r#"
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice,automatic_tags)]
        pub enum ChoiceType {
            number(Integer),
            anotherNumber(Integer),
            bool(bool),
            oneMoreNumber(Integer),
            aString(Ia5String),
        }
        impl From<bool> for ChoiceType {
            fn from(value:bool) -> Self {
                Self::bool(value)
            }
        }
        impl From<Ia5String> for ChoiceType {
            fn from(value:Ia5String) -> Self {
                Self::aString(value)
            }
        }                                           "#
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
    "#,
    r#"
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate)]
        pub struct AssertionValue(pub OctetString);

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate)]
        pub struct AttributeDescription(pub LDAPString);

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags)]
        pub struct AttributeValueAssertion {
            #[rasn(identifier = "attributeDesc")]
            pub attribute_desc: AttributeDescription,
            #[rasn(identifier = "assertionValue")]
            pub assertion_value: AssertionValue,
        }

        impl AttributeValueAssertion {
            pub fn new(attribute_desc: AttributeDescription, assertion_value: AssertionValue) -> Self {
                Self {
                    attribute_desc,
                    assertion_value,
                }
            }
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        pub enum Filter {
            #[rasn(size("1.."), tag(context, 0))]
            and(SetOf<Filter>),
            #[rasn(size("1.."), tag(context, 1))]
            or(SetOf<Filter>),
            #[rasn(tag(context, 2))]
            not(Box<Filter>),
            #[rasn(tag(context, 3))]
            equalityMatch(AttributeValueAssertion),
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, tag(universal, 4))]
        pub struct LDAPString(pub Utf8String);
                           "#
);
