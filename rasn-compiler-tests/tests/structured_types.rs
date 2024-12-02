use rasn_compiler_tests::e2e_pdu;

e2e_pdu!(
    sequence_of_primitive_value,
    r#" value SEQUENCE OF INTEGER ::= { 1, 2, 3 }"#,
    r#" lazy_static! {
            pub static ref VALUE: Vec<Integer> = alloc::vec![Integer::from(1), Integer::from(2), Integer::from(3)];
        }                                                       "#
);

e2e_pdu!(
    sequence_of_in_set_field,
    r#"PersonnelRecord ::=  SET {
        title VisibleString,
        children SEQUENCE OF VisibleString DEFAULT {}
    }"#,
    r#"
        #[derive (AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn (set , automatic_tags)]
        pub struct PersonnelRecord {
            pub title : VisibleString,
            #[rasn (default = "personnel_record_children_default")]
            pub children : SequenceOf<VisibleString>,
        }

        impl PersonnelRecord {
            pub fn new (title: VisibleString , children: SequenceOf<VisibleString>) -> Self {
                Self { title , children }
            }
        }

        fn personnel_record_children_default () -> SequenceOf<VisibleString> {
            alloc::vec![]
        }                          "#
);

e2e_pdu!(
    sized_sequence_of_in_sequence_field,
    r#"PersonnelRecord ::=  SET {
        title VisibleString,
        children SEQUENCE SIZE(0..4) OF VisibleString DEFAULT {}
    }"#,
    r#"
        #[derive (AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn (set , automatic_tags)]
        pub struct PersonnelRecord {
            pub title : VisibleString,
            #[rasn(size("0..=4"), default = "personnel_record_children_default")]
            pub children : SequenceOf<VisibleString>,
        }

        impl PersonnelRecord {
            pub fn new (title: VisibleString , children: SequenceOf<VisibleString>) -> Self {
                Self { title , children }
            }
        }

        fn personnel_record_children_default () -> SequenceOf<VisibleString> {
            alloc::vec![]
        }                          "#
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
    "#,
    r#"
        #[doc = "Inner type"]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice, automatic_tags)]
        pub enum NestedTypeChoiceField {
            one(Integer),
            two(bool),
        }
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags)]
        pub struct NestedType{
            #[rasn(identifier="choiceField")]
            pub choice_field: NestedTypeChoiceField,
        }
        impl NestedType {
            pub fn new(choice_field: NestedTypeChoiceField) -> Self {
                Self { choice_field }
            }
        }         
        
        lazy_static! {
            pub static ref NESTED_TYPE_VAL: NestedType = NestedType::new(NestedTypeChoiceField::one(Integer::from(4)));
        }          "#
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
    "#,
    r#"
        #[doc = "Inner type"]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice, automatic_tags)]
        pub enum NestedTypeChoiceField {
            one(Integer),
            two(bool),
        }
        impl From<Integer> for NestedTypeChoiceField {
            fn from(value: Integer) -> Self {
                Self::one(value)
            }
        }
        impl From<bool> for NestedTypeChoiceField {
            fn from(value: bool) -> Self {
                Self::two(value)
            }
        }

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags)]
        pub struct NestedType{
            #[rasn(identifier="choiceField")]
            pub choice_field: NestedTypeChoiceField,
        }
        impl NestedType {
            pub fn new(choice_field: NestedTypeChoiceField) -> Self {
                Self { choice_field }
            }
        }

        lazy_static! {
            pub static ref NESTED_TYPE_VAL: NestedType = NestedType::new(NestedTypeChoiceField::one(Integer::from(4)));
        }          "#
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
    "#,
    r#"
        #[doc = " Anonymous SEQUENCE OF member "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags, identifier = "SEQUENCE")]
        pub struct AnonymousTFCSReconfAddR12CtfcSizeCtfc8Bit {
            #[rasn(value("0..=255"))]
            pub ctfc8: u8,
        }
        impl AnonymousTFCSReconfAddR12CtfcSizeCtfc8Bit {
            pub fn new(ctfc8: u8) -> Self {
                Self { ctfc8 }
            }
        }
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, size("1..=1024"))]
        pub struct TFCSReconfAddR12CtfcSizeCtfc8Bit(
            pub SequenceOf<AnonymousTFCSReconfAddR12CtfcSizeCtfc8Bit>,
        );
        #[doc = " Anonymous SEQUENCE OF member "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags, identifier = "SEQUENCE")]
        pub struct AnonymousTFCSReconfAddR12CtfcSizeCtfc16Bit {
            #[rasn(value("0..=65535"))]
            pub ctfc16: u16,
        }
        impl AnonymousTFCSReconfAddR12CtfcSizeCtfc16Bit {
            pub fn new(ctfc16: u16) -> Self {
                Self { ctfc16 }
            }
        }
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, size("1..=1024"))]
        pub struct TFCSReconfAddR12CtfcSizeCtfc16Bit(
            pub SequenceOf<AnonymousTFCSReconfAddR12CtfcSizeCtfc16Bit>,
        );
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice, automatic_tags)]
        pub enum TFCSReconfAddR12CtfcSize {
            #[rasn(size("1..=1024"))]
            ctfc8Bit(SequenceOf<TFCSReconfAddR12CtfcSizeCtfc8Bit>),
            #[rasn(size("1..=1024"))]
            ctfc16Bit(SequenceOf<TFCSReconfAddR12CtfcSizeCtfc16Bit>),
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags, identifier = "TFCS-ReconfAdd-r12")]
        pub struct TFCSReconfAddR12 {
            #[rasn(identifier = "ctfcSize")]
            pub ctfc_size: TFCSReconfAddR12CtfcSize,
        }
        impl TFCSReconfAddR12 {
            pub fn new(ctfc_size: TFCSReconfAddR12CtfcSize) -> Self {
                Self { ctfc_size }
            }
        }
        lazy_static! {
            pub static ref MAX_TFC: Integer = Integer::from(1024);
        }
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
    "#,
    r#"
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate)]
        pub struct Color(pub BitString);
        
        #[doc="Inner type"]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(tag(context, 0))]
        pub struct TestParameters {
            #[rasn(tag(context, 0), default="test_parameters_color_default")]
            pub color: Color,
        }
        
        impl TestParameters {
            pub fn new(color: Color) -> Self {
                Self { color }
            }
        }
        
        fn test_parameters_color_default() -> Color {
            Color([false, true, false].into_iter().collect())
        }
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(set)]
        pub struct Test {
            #[rasn(tag(context, 0))]
            pub parameters: TestParameters,
        }
        
        impl Test {
            pub fn new(parameters: TestParameters) -> Self {
                Self { parameters }
            }
        }
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
    "#,
    r#"
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate)]
        pub struct Color(pub BitString);
        
        #[doc="Inner type"]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(tag(context, 0))]
        pub struct TestParameters {
            #[rasn(tag(context, 0), default="test_parameters_color_default")]
            pub color: Color,
        }
        
        impl TestParameters {
            pub fn new(color: Color) -> Self {
                Self { color }
            }
        }
        
        fn test_parameters_color_default() -> Color {
            Color([false, true, true].into_iter().collect())
        }
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(set)]
        pub struct Test {
            #[rasn(tag(context, 0))]
            pub parameters: TestParameters,
        }
        
        impl Test {
            pub fn new(parameters: TestParameters) -> Self {
                Self { parameters }
            }
        }
    "#
);
