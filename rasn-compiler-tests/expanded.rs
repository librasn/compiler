#![feature(prelude_import)]
#![recursion_limit = "32"]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub mod asn1 {
    pub mod recursive {
        extern crate alloc;
        use alloc::boxed::Box;
        use rasn::prelude::*;
        #[rasn(choice, automatic_tags)]
        #[non_exhaustive]
        pub enum TestChoice {
            Number1(()),
            Number2(bool),
            Number3(Box<TopLevel>),
        }
        impl rasn::AsnType for TestChoice {
            const TAG: rasn::Tag = { rasn::Tag::EOC };
            const TAG_TREE: rasn::TagTree = {
                const LIST: &'static [rasn::TagTree] = &[
                    {
                        const VARIANT_LIST: &'static [rasn::TagTree] = &[
                            rasn::TagTree::Leaf(
                                rasn::Tag::new(rasn::types::Class::Context, 0),
                            ),
                            rasn::TagTree::Leaf(
                                rasn::Tag::new(rasn::types::Class::Context, 1),
                            ),
                            rasn::TagTree::Leaf(
                                rasn::Tag::new(rasn::types::Class::Context, 2),
                            ),
                        ];
                        const VARIANT_TAG_TREE: rasn::TagTree = rasn::TagTree::Choice(
                            VARIANT_LIST,
                        );
                        const _: () = if !VARIANT_TAG_TREE.is_unique() {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!(
                                        "TestChoice\'s variants is not unique, ensure that your variants\'s tags are correct.",
                                    ),
                                );
                            }
                        };
                        VARIANT_TAG_TREE
                    },
                ];
                const TAG_TREE: rasn::TagTree = rasn::TagTree::Choice(LIST);
                const _: () = if !TAG_TREE.is_unique() {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "TestChoice\'s variants is not unique, ensure that your variants\'s tags are correct.",
                            ),
                        );
                    }
                };
                TAG_TREE
            };
            const CONSTRAINTS: rasn::types::Constraints<'static> = rasn::types::Constraints::new(
                &[rasn::types::Constraint::Extensible],
            );
        }
        impl rasn::types::Choice for TestChoice {
            const VARIANTS: &'static [rasn::types::TagTree] = &[
                rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 0)),
                rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 1)),
                rasn::TagTree::Leaf(rasn::Tag::new(rasn::types::Class::Context, 2)),
            ];
            const EXTENDED_VARIANTS: &'static [rasn::types::TagTree] = &[];
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TestChoice {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match self {
                    TestChoice::Number1(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Number1",
                            &__self_0,
                        )
                    }
                    TestChoice::Number2(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Number2",
                            &__self_0,
                        )
                    }
                    TestChoice::Number3(__self_0) => {
                        ::core::fmt::Formatter::debug_tuple_field1_finish(
                            f,
                            "Number3",
                            &__self_0,
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TestChoice {
            #[inline]
            fn clone(&self) -> TestChoice {
                match self {
                    TestChoice::Number1(__self_0) => {
                        TestChoice::Number1(::core::clone::Clone::clone(__self_0))
                    }
                    TestChoice::Number2(__self_0) => {
                        TestChoice::Number2(::core::clone::Clone::clone(__self_0))
                    }
                    TestChoice::Number3(__self_0) => {
                        TestChoice::Number3(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl rasn::types::DecodeChoice for TestChoice {
            fn from_tag<D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
            ) -> Result<Self, D::Error> {
                use rasn::de::Decode;
                if rasn::TagTree::tag_contains(
                    &tag,
                    &[
                        rasn::TagTree::Leaf(
                            rasn::Tag::new(rasn::types::Class::Context, 0),
                        ),
                    ],
                ) {
                    return <_>::decode_with_tag(decoder, tag).map(Self::Number1);
                }
                if rasn::TagTree::tag_contains(
                    &tag,
                    &[
                        rasn::TagTree::Leaf(
                            rasn::Tag::new(rasn::types::Class::Context, 1),
                        ),
                    ],
                ) {
                    return <_>::decode_with_tag(decoder, tag).map(Self::Number2);
                }
                if rasn::TagTree::tag_contains(
                    &tag,
                    &[
                        rasn::TagTree::Leaf(
                            rasn::Tag::new(rasn::types::Class::Context, 2),
                        ),
                    ],
                ) {
                    return <_>::decode_with_tag(decoder, tag).map(Self::Number3);
                }
                Err(rasn::de::Error::no_valid_choice("TestChoice"))
            }
        }
        #[automatically_derived]
        impl rasn::Decode for TestChoice {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                decoder.decode_explicit_prefix(tag)
            }
            fn decode<D: rasn::Decoder>(
                decoder: &mut D,
            ) -> core::result::Result<Self, D::Error> {
                decoder.decode_choice(Self::CONSTRAINTS)
            }
        }
        #[automatically_derived]
        impl rasn::Encode for TestChoice {
            fn encode<E: rasn::Encoder>(
                &self,
                encoder: &mut E,
            ) -> core::result::Result<(), E::Error> {
                encoder
                    .encode_choice::<
                        Self,
                    >(
                        Self::CONSTRAINTS,
                        |encoder| match self {
                            TestChoice::Number1(value) => {
                                rasn::Encode::encode_with_tag(
                                        value,
                                        encoder,
                                        rasn::Tag::new(rasn::types::Class::Context, 0),
                                    )
                                    .map(|_| rasn::Tag::new(rasn::types::Class::Context, 0))
                            }
                            TestChoice::Number2(value) => {
                                rasn::Encode::encode_with_tag(
                                        value,
                                        encoder,
                                        rasn::Tag::new(rasn::types::Class::Context, 1),
                                    )
                                    .map(|_| rasn::Tag::new(rasn::types::Class::Context, 1))
                            }
                            TestChoice::Number3(value) => {
                                rasn::Encode::encode_with_tag(
                                        value,
                                        encoder,
                                        rasn::Tag::new(rasn::types::Class::Context, 2),
                                    )
                                    .map(|_| rasn::Tag::new(rasn::types::Class::Context, 2))
                            }
                        },
                    )
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
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TestChoice {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TestChoice {
            #[inline]
            fn eq(&self, other: &TestChoice) -> bool {
                let __self_tag = ::core::intrinsics::discriminant_value(self);
                let __arg1_tag = ::core::intrinsics::discriminant_value(other);
                __self_tag == __arg1_tag
                    && match (self, other) {
                        (
                            TestChoice::Number1(__self_0),
                            TestChoice::Number1(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            TestChoice::Number2(__self_0),
                            TestChoice::Number2(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        (
                            TestChoice::Number3(__self_0),
                            TestChoice::Number3(__arg1_0),
                        ) => *__self_0 == *__arg1_0,
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        #[rasn(automatic_tags)]
        pub struct TopLevel {
            #[rasn(value("1..=8"))]
            pub test: u8,
            pub choice: TestChoice,
        }
        #[automatically_derived]
        impl rasn::types::Constructed for TopLevel {
            const FIELDS: rasn::types::fields::Fields = rasn::types::fields::Fields::from_static(
                &[
                    {
                        rasn::types::fields::Field::new_required(
                            rasn::Tag::new(rasn::types::Class::Context, 0usize as u32),
                            rasn::TagTree::Leaf(
                                rasn::Tag::new(rasn::types::Class::Context, 0usize as u32),
                            ),
                        )
                    },
                    {
                        rasn::types::fields::Field::new_required(
                            rasn::Tag::new(rasn::types::Class::Context, 1usize as u32),
                            rasn::TagTree::Leaf(
                                rasn::Tag::new(rasn::types::Class::Context, 1usize as u32),
                            ),
                        )
                    },
                ],
            );
            const EXTENDED_FIELDS: Option<rasn::types::fields::Fields> = None;
        }
        #[automatically_derived]
        impl rasn::AsnType for TopLevel {
            const TAG: rasn::Tag = { rasn::Tag::SEQUENCE };
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for TopLevel {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "TopLevel",
                    "test",
                    &self.test,
                    "choice",
                    &&self.choice,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for TopLevel {
            #[inline]
            fn clone(&self) -> TopLevel {
                TopLevel {
                    test: ::core::clone::Clone::clone(&self.test),
                    choice: ::core::clone::Clone::clone(&self.choice),
                }
            }
        }
        impl rasn::Decode for TopLevel {
            fn decode_with_tag_and_constraints<'constraints, D: rasn::Decoder>(
                decoder: &mut D,
                tag: rasn::Tag,
                constraints: rasn::types::Constraints<'constraints>,
            ) -> core::result::Result<Self, D::Error> {
                decoder
                    .decode_sequence(
                        tag,
                        |decoder| {
                            Ok(Self {
                                test: {
                                    <_>::decode_with_tag_and_constraints(
                                            decoder,
                                            rasn::Tag::new(rasn::types::Class::Context, 0usize as u32),
                                            <u8 as rasn::AsnType>::CONSTRAINTS
                                                .override_constraints(
                                                    rasn::types::Constraints::new(
                                                        &[
                                                            rasn::types::Constraint::Value(
                                                                rasn::types::constraints::Extensible::new(
                                                                        rasn::types::constraints::Value::new(
                                                                            rasn::types::constraints::Bounded::const_new(
                                                                                1i128 as i128,
                                                                                8i128 as i128,
                                                                            ),
                                                                        ),
                                                                    )
                                                                    .set_extensible(false),
                                                            ),
                                                        ],
                                                    ),
                                                ),
                                        )
                                        .map_err(|error| rasn::de::Error::field_error(
                                            "TopLevel.test",
                                            error,
                                        ))?
                                },
                                choice: {
                                    <_>::decode_with_tag(
                                            decoder,
                                            rasn::Tag::new(rasn::types::Class::Context, 1usize as u32),
                                        )
                                        .map_err(|error| rasn::de::Error::field_error(
                                            "TopLevel.choice",
                                            error,
                                        ))?
                                },
                            })
                        },
                    )
            }
        }
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
                    .encode_sequence::<
                        Self,
                        _,
                    >(
                        tag,
                        |encoder| {
                            self.test
                                .encode_with_tag_and_constraints(
                                    encoder,
                                    rasn::Tag::new(rasn::types::Class::Context, 0usize as u32),
                                    <u8 as rasn::AsnType>::CONSTRAINTS
                                        .override_constraints(
                                            rasn::types::Constraints::new(
                                                &[
                                                    rasn::types::Constraint::Value(
                                                        rasn::types::constraints::Extensible::new(
                                                                rasn::types::constraints::Value::new(
                                                                    rasn::types::constraints::Bounded::const_new(
                                                                        1i128 as i128,
                                                                        8i128 as i128,
                                                                    ),
                                                                ),
                                                            )
                                                            .set_extensible(false),
                                                    ),
                                                ],
                                            ),
                                        ),
                                )?;
                            self.choice
                                .encode_with_tag(
                                    encoder,
                                    rasn::Tag::new(rasn::types::Class::Context, 1usize as u32),
                                )?;
                            Ok(())
                        },
                    )
                    .map(drop)
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for TopLevel {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for TopLevel {
            #[inline]
            fn eq(&self, other: &TopLevel) -> bool {
                self.test == other.test && self.choice == other.choice
            }
        }
        impl TopLevel {
            pub fn new(test: u8, choice: TestChoice) -> Self {
                Self { test, choice }
            }
        }
    }
}
pub mod helpers {
    use std::num::ParseIntError;
    pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16)).collect()
    }
}
