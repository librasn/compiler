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

use rasn::prelude::*;
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("2"))]
pub struct FixedSizeAliasSequence(pub [IntegerAlias; 2usize]);

#[doc = " Anonymous SEQUENCE OF member "]
#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("1..=4"), identifier = "INTEGER")]
pub struct AnonymousFixedSizeIntegerSequence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, size("4"))]
pub struct FixedSizeIntegerSequence(pub [AnonymousFixedSizeIntegerSequence; 4usize]);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Eq, Hash)]
#[rasn(delegate, value("10..=14"))]
pub struct IntegerAlias(pub u8);

e2e_pdu!(
    fixed_size_sequence_of,
    r#"
    FixedSizeIntegerSequence ::= SEQUENCE SIZE(4) OF INTEGER (1..4)
    IntegerAlias ::= INTEGER (10..14)
    FixedSizeAliasSequence ::= SEQUENCE (SIZE (2)) OF IntegerAlias
    "#,
    ""
);
