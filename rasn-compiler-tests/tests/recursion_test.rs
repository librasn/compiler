#![recursion_limit = "16"]
extern crate alloc;

use alloc::boxed::Box;
use rasn::prelude::*;
#[non_exhaustive]
pub enum TestChoice {
    Number1(()),
    Number2(bool),
    Number3(Box<TopLevel>),
}
impl rasn::AsnType for TestChoice {
    const TAG: rasn::Tag = { rasn::Tag::EOC };
    const TAG_TREE: rasn::TagTree = {
        const LIST: &'static [rasn::TagTree] = &[{
            const VARIANT_LIST: &'static [rasn::TagTree] = &[
                rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 0)),
                rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 1)),
                rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 2)),
            ];
            const VARIANT_TAG_TREE: rasn::TagTree = rasn::TagTree::Choice(VARIANT_LIST);
            const _: () = if !VARIANT_TAG_TREE.is_unique() {
                {
                    panic!(
                                        "TestChoice\'s variants is not unique, ensure that your variants\'s tags are correct.",
                                    
                                );
                }
            };
            VARIANT_TAG_TREE
        }];
        const TAG_TREE: rasn::TagTree = rasn::TagTree::Choice(LIST);
        const _: () = if !TAG_TREE.is_unique() {
            {
                panic!(
                                "TestChoice\'s variants is not unique, ensure that your variants\'s tags are correct.",
                        );
            }
        };
        TAG_TREE
    };
    const CONSTRAINTS: rasn::types::Constraints<'static> =
        rasn::types::Constraints::new(&[rasn::types::Constraint::Extensible]);
}
impl rasn::types::Choice for TestChoice {
    const VARIANTS: &'static [rasn::types::TagTree] = &[
        rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 0)),
        rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 1)),
        rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 2)),
    ];
    const EXTENDED_VARIANTS: &'static [rasn::types::TagTree] = &[];
}

// #[automatically_derived]
// impl rasn::types::DecodeChoice for TestChoice {
//     fn from_tag<D: rasn::Decoder>(decoder: &mut D, tag: rasn::Tag) -> Result<Self, D::Error> {
//         use rasn::de::Decode;
//         if rasn::TagTree::tag_contains(
//             &tag,
//             &[rasn::TagTree::Leaf(rasn::Tag::new(
//                 rasn::types::Class::Context,
//                 0,
//             ))],
//         ) {
//             return <_>::decode_with_tag(decoder, tag).map(Self::Number1);
//         }
//         if rasn::TagTree::tag_contains(
//             &tag,
//             &[rasn::TagTree::Leaf(rasn::Tag::new(
//                 rasn::types::Class::Context,
//                 1,
//             ))],
//         ) {
//             return <_>::decode_with_tag(decoder, tag).map(Self::Number2);
//         }
//         if rasn::TagTree::tag_contains(
//             &tag,
//             &[rasn::TagTree::Leaf(rasn::Tag::new(
//                 rasn::types::Class::Context,
//                 2,
//             ))],
//         ) {
//             return <_>::decode_with_tag(decoder, tag).map(Self::Number3);
//         }
//         Err(rasn::de::Error::no_valid_choice("TestChoice"))
//     }
// }
// #[automatically_derived]
// impl rasn::Decode for TestChoice {
//     fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
//         decoder: &mut D,
//         tag: rasn::Tag,
//         constraints: rasn::types::Constraints<'constraints>,
//     ) -> core::result::Result<Self, D::Error> {
//         decoder.decode_explicit_prefix(tag)
//     }
//     fn decode<D: rasn::Decoder>(decoder: &mut D) -> core::result::Result<Self, D::Error> {
//         decoder.decode_choice(Self::CONSTRAINTS)
//     }
// }
#[automatically_derived]
impl rasn::Encode for TestChoice {
    fn encode<E: rasn::Encoder>(&self, encoder: &mut E) -> core::result::Result<(), E::Error> {
        encoder
            .encode_choice::<Self>(Self::CONSTRAINTS, |encoder| match self {
                TestChoice::Number1(value) => rasn::Encode::encode_with_tag(
                    value,
                    encoder,
                    rasn::Tag::new(rasn::types::Class::Context, 0),
                )
                .map(|_| rasn::Tag::new(rasn::types::Class::Context, 0)),
                TestChoice::Number2(value) => rasn::Encode::encode_with_tag(
                    value,
                    encoder,
                    rasn::Tag::new(rasn::types::Class::Context, 1),
                )
                .map(|_| rasn::Tag::new(rasn::types::Class::Context, 1)),
                TestChoice::Number3(value) => rasn::Encode::encode_with_tag(
                    value,
                    encoder,
                    rasn::Tag::new(rasn::types::Class::Context, 2),
                )
                .map(|_| rasn::Tag::new(rasn::types::Class::Context, 2)),
            })
            .map(drop)
    }
    fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
        &self,
        encoder: &mut EN,
        tag: rasn::Tag,
        constraints: rasn::types::Constraints<'constraints>,
    ) -> core::result::Result<(), EN::Error> {
        encoder.encode_explicit_prefix(tag, self).map(drop)
    }
}

pub struct TopLevel {
    pub test: u8,
    pub choice: TestChoice,
}
#[automatically_derived]
impl rasn::types::Constructed for TopLevel {
    const FIELDS: rasn::types::fields::Fields = rasn::types::fields::Fields::from_static(&[
        {
            rasn::types::fields::Field::new_required(
                rasn::Tag::new(rasn::types::Class::Context, 0usize as u32),
                rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 0usize as u32)),
            )
        },
        {
            rasn::types::fields::Field::new_required(
                rasn::Tag::new(rasn::types::Class::Context, 1usize as u32),
                rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 1usize as u32)),
            )
        },
    ]);
    const EXTENDED_FIELDS: Option<rasn::types::fields::Fields> = None;
}
#[automatically_derived]
impl rasn::AsnType for TopLevel {
    const TAG: rasn::Tag = { rasn::Tag::SEQUENCE };
}
// impl rasn::Decode for TopLevel {
//     fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
//         decoder: &mut D,
//         tag: rasn::Tag,
//         constraints: rasn::types::Constraints<'constraints>,
//     ) -> core::result::Result<Self, D::Error> {
//         decoder.decode_sequence(tag, |decoder| {
//             Ok(Self {
//                 test: {
//                     <_>::decode_with_tag_and_constraints(
//                         decoder,
//                         rasn::Tag::new(rasn::types::Class::Context, 0usize as u32),
//                         <u8 as rasn::AsnType>::CONSTRAINTS.override_constraints(
//                             rasn::types::Constraints::new(&[rasn::types::Constraint::Value(
//                                 rasn::types::constraints::Extensible::new(
//                                     rasn::types::constraints::Value::new(
//                                         rasn::types::constraints::Bounded::const_new(
//                                             1i128 as i128,
//                                             8i128 as i128,
//                                         ),
//                                     ),
//                                 )
//                                 .set_extensible(false),
//                             )]),
//                         ),
//                     )
//                     .map_err(|error| rasn::de::Error::field_error("TopLevel.test", error))?
//                 },
//                 choice: {
//                     <_>::decode_with_tag(
//                         decoder,
//                         rasn::Tag::new(rasn::types::Class::Context, 1usize as u32),
//                     )
//                     .map_err(|error| rasn::de::Error::field_error("TopLevel.choice", error))?
//                 },
//             })
//         })
//     }
// }
impl rasn::Encode for TopLevel {
    fn encode_with_tag_and_constraints<'constraints, EN: rasn::Encoder>(
        &self,
        encoder: &mut EN,
        tag: rasn::Tag,
        constraints: rasn::types::Constraints<'constraints>,
    ) -> core::result::Result<(), EN::Error> {
        #[allow(unused)]
        let test = &self.test;
        #[allow(unused)]
        let choice = &self.choice;
        encoder
            .encode_sequence::<Self, _>(tag, |encoder| {
                self.test.encode_with_tag_and_constraints(
                    encoder,
                    rasn::Tag::new(rasn::types::Class::Context, 0usize as u32),
                    <u8 as rasn::AsnType>::CONSTRAINTS.override_constraints(
                        rasn::types::Constraints::new(&[rasn::types::Constraint::Value(
                            rasn::types::constraints::Extensible::new(
                                rasn::types::constraints::Value::new(
                                    rasn::types::constraints::Bounded::const_new(
                                        1i128 as i128,
                                        8i128 as i128,
                                    ),
                                ),
                            )
                            .set_extensible(false),
                        )]),
                    ),
                )?;
                self.choice.encode_with_tag(
                    encoder,
                    rasn::Tag::new(rasn::types::Class::Context, 1usize as u32),
                )?;
                Ok(())
            })
            .map(drop)
    }
}
impl TopLevel {
    pub fn new(test: u8, choice: TestChoice) -> Self {
        Self { test, choice }
    }
}

#[test]
fn recursive_type_test() {
    let mut encoder = rasn::uper::Encoder::new(rasn::uper::enc::EncoderOptions::unaligned());
    let tl = TopLevel::new(
        1,
        TestChoice::Number3(Box::new(TopLevel {
            test: 2,
            choice: TestChoice::Number1(()),
        })),
    );
    tl.encode(&mut encoder).unwrap();
    let encoded = encoder.output();
    println!("{:?}", encoded);
}
