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
            ctfc8Bit(TFCSReconfAddR12CtfcSizeCtfc8Bit),
            ctfc16Bit(TFCSReconfAddR12CtfcSizeCtfc16Bit),
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
    "#,
    r#"
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash, Copy)]
        #[rasn(enumerated)]
        #[non_exhaustive]
        pub enum Passenger{
            adult = 0,
            youth = 1,
        }

        #[doc = " Anonymous SEQUENCE OF member "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "INTEGER", value("1..=5"))]
        pub struct AnonymousTicketAges(pub u8);

        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate)]
        pub struct TicketAges(pub SequenceOf<AnonymousTicketAges>);

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags)]
        pub struct Ticket {
            pub ages: TicketAges,
            pub passenger: Option<Passenger>,
        }

        impl Ticket {
            pub fn new(ages: TicketAges, passenger: Option<Passenger>) -> Self {
                Self { ages, passenger }
            }
        }
    "#
);

e2e_pdu!(
    anonymous_set_of_item_in_choice_option,
    r#"
    Ticket ::= CHOICE {
        age-set		SET (SIZE (1..4)) OF INTEGER (1..5)
    }
    "#,
    r#"
        #[doc = " Anonymous SET OF member "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "INTEGER", value("1..=5"))]
        pub struct AnonymousTicketAgeSet(pub u8);

        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, size("1..=4"))]
        pub struct TicketAgeSet(pub SetOf<AnonymousTicketAgeSet>);

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice, automatic_tags)]
        pub enum Ticket {
            #[rasn(identifier = "age-set")]
            age_set(TicketAgeSet),
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
    "#,
    r#"
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        pub struct TypeDescriptionArray {
            #[rasn(value("0.."), tag(context, 0))]
            pub size: Integer,
            #[rasn(tag(context, 1), identifier = "element-type")]
            pub element_type: TypeDescription,
        }
        impl TypeDescriptionArray {
            pub fn new(size: Integer, element_type: TypeDescription) -> Self {
                Self { size, element_type }
            }
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice)]
        pub enum TypeDescription {
            #[rasn(tag(context, 0))]
            boolean(bool),
            #[rasn(tag(context, 2))]
            string(Utf8String),
            #[rasn(tag(context, 3))]
            array(Box<TypeDescriptionArray>),
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
    "#,
    r#"
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        pub struct GetVariableAccessAttributesResponse {
            #[rasn(tag(context, 2), identifier = "typeSpecification")]
            pub type_specification: TypeSpecification,
        }
        impl GetVariableAccessAttributesResponse {
            pub fn new(type_specification: TypeSpecification) -> Self {
                Self { type_specification }
            }
        }
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        pub struct TypeSpecificationArray {
            #[rasn(tag(context, 2), identifier = "elementType")]
            pub element_type: TypeSpecification,
        }
        impl TypeSpecificationArray {
            pub fn new(element_type: TypeSpecification) -> Self {
                Self { element_type }
            }
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice)]
        pub enum TypeSpecification {
            #[rasn(tag(context, 1))]
            array(Box<TypeSpecificationArray>),
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
    "#,
    r#"
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        pub struct TypeDescriptionArray {
            #[rasn(tag(context, 2), identifier = "elementType")]
            pub element_type: TypeSpecification,
        }
        impl TypeDescriptionArray {
            pub fn new(element_type: TypeSpecification) -> Self {
                Self { element_type }
            }
        }
        #[doc = " Anonymous SEQUENCE OF member "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(identifier = "SEQUENCE")]
        pub struct AnonymousTypeDescriptionStructureComponents {
            #[rasn(tag(context, 1), identifier = "componentType")]
            pub component_type: TypeSpecification,
        }
        impl AnonymousTypeDescriptionStructureComponents {
            pub fn new(component_type: TypeSpecification) -> Self {
                Self { component_type }
            }
        }
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate)]
        pub struct TypeDescriptionStructureComponents(
            pub SequenceOf<AnonymousTypeDescriptionStructureComponents>,
        );
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        pub struct TypeDescriptionStructure {
            #[rasn(tag(context, 1))]
            pub components: TypeDescriptionStructureComponents,
        }
        impl TypeDescriptionStructure {
            pub fn new(components: TypeDescriptionStructureComponents) -> Self {
                Self { components }
            }
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice)]
        pub enum TypeDescription {
            #[rasn(tag(context, 1))]
            array(TypeDescriptionArray),
            #[rasn(tag(context, 2))]
            structure(TypeDescriptionStructure),
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice, automatic_tags)]
        pub enum TypeSpecification {
            typeDescription(Box<TypeDescription>),
        }
        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags)]
        pub struct VariableSpecificationVariableDescription {
            #[rasn(identifier = "typeSpecification")]
            pub type_specification: TypeSpecification,
        }
        impl VariableSpecificationVariableDescription {
            pub fn new(type_specification: TypeSpecification) -> Self {
                Self { type_specification }
            }
        }
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(choice)]
        pub enum VariableSpecification {
            #[rasn(tag(context, 2))]
            variableDescription(VariableSpecificationVariableDescription),
        }
    "#
);

e2e_pdu!(
    tagged_prefix_type,
    r#"
        A ::= SEQUENCE {
            member SEQUENCE OF [0] INTEGER,
        }
    "#,
    r#"
        #[doc = " Anonymous SEQUENCE OF member "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate, identifier = "INTEGER")]
        pub struct AnonymousAMember(pub Integer);

        #[doc = " Inner type "]
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(delegate)]
        pub struct AMember(pub SequenceOf<AnonymousAMember>);

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
        #[rasn(automatic_tags)]
        pub struct A {
            pub member: AMember,
        }

        impl A {
            pub fn new(member: AMember) -> Self {
                Self { member }
            }
        }
    "#
);
