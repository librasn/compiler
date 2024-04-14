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
        #[derive (AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[derive (AsnType, Debug, Clone, Decode, Encode, PartialEq)]
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
        #[derive(AsnType,Debug,Clone,Decode,Encode,PartialEq)]
        #[rasn(choice, automatic_tags)]
        pub enum NestedTypeChoiceField {
            one(Integer),
            two(bool),
        }
        
        #[derive(AsnType,Debug,Clone,Decode,Encode,PartialEq)]
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
