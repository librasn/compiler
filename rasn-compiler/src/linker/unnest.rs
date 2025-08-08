use std::borrow::Cow;

use crate::{
    intermediate::{
        ASN1Type, ASN1Value, DeclarationElsewhere, DefinedType, TypeAssignment, ValueAssignment,
    },
    linker::symbol_table::{GeneratedSymbols, SymbolId, SymbolTableEntry},
    prelude::{
        ir::{
            ASN1Information, ClassLink, InformationObject, InformationObjectField,
            InformationObjectFields, IterMembers, MemberOrOption, ObjectClassAssignment,
            ObjectOrObjectSetAssignment, ObjectSet, ObjectSetValue, SyntaxApplication,
        },
        Assignment,
    },
};

/// ## What will not be unnested
/// Nested fixed value fields in information objects cannot be unnested because
/// the linker does not have enough information about the associated type.
pub(super) trait Unnest<'a> {
    type ExtraArgs;

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a>;
}

impl<'a> Unnest<'a> for ASN1Type<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a> {
        match self {
            ASN1Type::Choice(c) => c.unnest(parent_symbol, extra_args),
            ASN1Type::Sequence(s) | ASN1Type::Set(s) => s.unnest(parent_symbol, extra_args),
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => {
                if s.element_type.is_constrained_type() || s.element_type.is_constructed_type() {
                    let id = parent_symbol.locally_scoped(Cow::Borrowed("item"));
                    let name = Cow::<'a, str>::Owned(id.to_string());
                    let ty_reference = ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                        parent: None,
                        identifier: DefinedType::TypeReference(name.clone()),
                        constraints: Vec::new(),
                    });
                    let ty = std::mem::replace(&mut s.element_type, Box::new(ty_reference));
                    GeneratedSymbols::single(SymbolTableEntry {
                        assignment: Assignment::Type(TypeAssignment::with_name_and_type(name, *ty)),
                        id,
                    })
                } else {
                    GeneratedSymbols::empty()
                }
            }
            _ => GeneratedSymbols::empty(),
        }
    }
}

impl<'a, I: IterMembers<'a>> Unnest<'a> for I {
    type ExtraArgs = ();

    fn unnest(&mut self, parent_symbol: SymbolId<'a>, _: Self::ExtraArgs) -> GeneratedSymbols<'a> {
        GeneratedSymbols::new(
            self.iter_mut_members()
                .filter_map(|m| {
                    if m.ty().is_constrained_type() || m.ty().is_constructed_type() {
                        let id = parent_symbol.locally_scoped(m.name().clone());
                        let name = Cow::<'a, str>::Owned(id.to_string());
                        let ty_reference = ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                            parent: None,
                            identifier: DefinedType::TypeReference(name.clone()),
                            constraints: Vec::new(),
                        });
                        let ty = std::mem::replace(m.ty_mut(), ty_reference);
                        Some(SymbolTableEntry {
                            assignment: Assignment::Type(TypeAssignment::with_name_and_type(
                                name, ty,
                            )),
                            id,
                        })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>(),
        )
    }
}

impl<'a> Unnest<'a> for Assignment<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        symbol_id: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a> {
        let mut stage_result = match self {
            Assignment::Type(t) => t.unnest(symbol_id, extra_args),
            Assignment::Class(c) => c.unnest(symbol_id, extra_args),
            Assignment::Object(o) => o.unnest(symbol_id, extra_args),
            Assignment::Value(v) => v.unnest(symbol_id, extra_args),
            _ => GeneratedSymbols::empty(),
        };
        if !stage_result.is_empty() {
            let mut next_stage_result = GeneratedSymbols::empty();
            for unnested in stage_result.iter_mut() {
                next_stage_result += unnested.assignment.unnest(unnested.id.clone(), extra_args);
            }
            stage_result += next_stage_result;
            stage_result
        } else {
            GeneratedSymbols::empty()
        }
    }
}

impl<'a> Unnest<'a> for TypeAssignment<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        symbol_id: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a> {
        self.ty.unnest(symbol_id.clone(), extra_args)
    }
}

impl<'a> Unnest<'a> for ValueAssignment<'a> {
    type ExtraArgs = ();

    fn unnest(&mut self, symbol_id: SymbolId<'a>, _: Self::ExtraArgs) -> GeneratedSymbols<'a> {
        fn unnest_iter_members<'a, I: IterMembers<'a>>(
            symbol_id: &SymbolId<'a>,
            member_name: &Cow<'a, str>,
            inner_value: &mut Box<ASN1Value<'a>>,
            iterable: &I,
        ) -> SymbolTableEntry<'a> {
            let id = symbol_id.locally_scoped(member_name.clone());
            let name = Cow::<'a, str>::Owned(id.to_string());
            let unnested_value = std::mem::replace(
                inner_value,
                Box::new(ASN1Value::ElsewhereDeclaredValue {
                    parent: None,
                    identifier: name.clone(),
                }),
            );
            let replacement = SymbolTableEntry {
                id,
                assignment: Assignment::Value(ValueAssignment::with_name_value_type(
                    name,
                    *unnested_value,
                    iterable
                        .iter_members()
                        .find(|o| &o.name() == member_name)
                        .expect("Unnesting: No matching member name found!")
                        .ty()
                        .clone(),
                )),
            };
            replacement
        }

        if self.value.is_constructed_value() {
            match (&mut self.value, &self.associated_type){
                (ASN1Value::Choice {
                    variant_name,
                    inner_value,
                    ..
                }, ASN1Type::Choice(c)) => {
                    if inner_value.is_constructed_value() {
                        let replacement = unnest_iter_members(&symbol_id, variant_name, inner_value, c);
                        GeneratedSymbols::single(replacement)
                    } else {
                        GeneratedSymbols::empty()
                    }
                },
                (ASN1Value::SequenceOrSet(items),ASN1Type::Sequence(s))
                | (ASN1Value::SequenceOrSet(items),ASN1Type::Set(s)) => {
                    let mut unnested = Vec::new();
                    for (field_name, inner_value) in items {
                        match (field_name, inner_value.is_constructed_value()) {
                            (Some(field_name), true) => {
                                unnested.push(unnest_iter_members(&symbol_id, &field_name, inner_value, s));
                            },
                            _ => ()
                        }
                    }
                    GeneratedSymbols::new(unnested)
                },
                (ASN1Value::SequenceOrSet(items),ASN1Type::SequenceOf(s))
                | (ASN1Value::SequenceOrSet(items),ASN1Type::SetOf(s)) => {
                    let mut unnested = Vec::new();
                    for (i, (_, inner_value)) in items.iter_mut().enumerate() {
                        if inner_value.is_constructed_value() {
                            let id = symbol_id.locally_scoped(Cow::Owned(i.to_string()));
                            let name = Cow::<'a, str>::Owned(id.to_string());
                            let unnested_value = std::mem::replace(
                                inner_value,
                                Box::new(ASN1Value::ElsewhereDeclaredValue {
                                    parent: None,
                                    identifier: name.clone(),
                                }),
                            );
                            let replacement = SymbolTableEntry {
                                id,
                                assignment: Assignment::Value(ValueAssignment::with_name_value_type(
                                    name,
                                    *unnested_value,
                                    *s.element_type.clone(),
                                )),
                            };
                            unnested.push(replacement);
                        }
                    }
                    GeneratedSymbols::new(unnested)
                }
                _ => unreachable!("Only plain Choice, Sequence, Set, SequenceOf, or SetOf values should be defined"),
            }
        } else {
            GeneratedSymbols::empty()
        }
    }
}

impl<'a> Unnest<'a> for ObjectClassAssignment<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        symbol_id: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a> {
        self.definition
            .fields
            .iter_mut()
            .filter_map(|field| {
                field
                    .ty
                    .as_mut()
                    .map(|t| t.unnest(symbol_id.clone(), extra_args))
            })
            .collect()
    }
}

impl<'a> Unnest<'a> for ObjectOrObjectSetAssignment<'a> {
    type ExtraArgs = ();

    fn unnest(&mut self, parent_symbol: SymbolId<'a>, _: Self::ExtraArgs) -> GeneratedSymbols<'a> {
        let class_name = match &self.class {
            ClassLink::ByName(name) => name.clone(),
            ClassLink::ByReference(_) => {
                unreachable!("Classes should be only linked by name at this stage.")
            }
        };
        self.value
            .unnest(parent_symbol, ExtraClassReferenceArg { class_name })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(super) struct ExtraClassReferenceArg<'a> {
    class_name: Cow<'a, str>,
}

impl<'a> Unnest<'a> for ASN1Information<'a> {
    type ExtraArgs = ExtraClassReferenceArg<'a>;

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a> {
        match self {
            ASN1Information::ObjectSet(object_set) => object_set.unnest(parent_symbol, extra_args),
            ASN1Information::Object(information_object) => {
                information_object.unnest(parent_symbol, extra_args)
            }
        }
    }
}

impl<'a> Unnest<'a> for InformationObject<'a> {
    type ExtraArgs = ExtraClassReferenceArg<'a>;

    fn unnest(&mut self, parent_symbol: SymbolId<'a>, _: Self::ExtraArgs) -> GeneratedSymbols<'a> {
        self.fields.unnest(
            parent_symbol,
            ExtraClassReferenceArg {
                class_name: self.class_name.clone(),
            },
        )
    }
}

impl<'a> Unnest<'a> for InformationObjectFields<'a> {
    type ExtraArgs = ExtraClassReferenceArg<'a>;

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a> {
        match self {
            InformationObjectFields::DefaultSyntax(fields) => fields
                .iter_mut()
                .map(|field| field.unnest(parent_symbol.clone(), extra_args.clone()))
                .collect(),
            InformationObjectFields::CustomSyntax(stx) => stx
                .iter_mut()
                .map(|elem| {
                    if let SyntaxApplication::ObjectSetDeclaration(o) = elem {
                        o.unnest(parent_symbol.clone(), extra_args.clone())
                    } else {
                        GeneratedSymbols::empty()
                    }
                })
                .collect(),
        }
    }
}

impl<'a> Unnest<'a> for InformationObjectField<'a> {
    type ExtraArgs = ExtraClassReferenceArg<'a>;

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a> {
        match self {
            InformationObjectField::TypeField(type_field) => {
                type_field.ty.unnest(parent_symbol, ())
            }
            InformationObjectField::FixedValueField(_) => GeneratedSymbols::empty(),
            InformationObjectField::ObjectSetField(object_set_field) => {
                object_set_field.value.unnest(parent_symbol, extra_args)
            }
        }
    }
}

impl<'a> Unnest<'a> for ObjectSet<'a> {
    type ExtraArgs = ExtraClassReferenceArg<'a>;

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> GeneratedSymbols<'a> {
        self.values
            .iter_mut()
            .enumerate()
            .map(|(i, val)| {
                if let ObjectSetValue::Inline(_) = val {
                    let id = parent_symbol.locally_scoped(Cow::Owned(i.to_string()));
                    let name = Cow::<'a, str>::Owned(id.to_string());
                    let val_reference = ObjectSetValue::Reference(name.clone());
                    if let ObjectSetValue::Inline(fields) = std::mem::replace(val, val_reference) {
                        GeneratedSymbols::single(SymbolTableEntry {
                            id,
                            assignment: Assignment::Object(
                                ObjectOrObjectSetAssignment::with_name_class_value(
                                    name,
                                    ClassLink::ByName(extra_args.class_name.clone()),
                                    ASN1Information::Object(InformationObject {
                                        class_name: extra_args.class_name.clone(),
                                        fields,
                                    }),
                                ),
                            ),
                        })
                    } else {
                        unreachable!(
                            "Moved object has already been matched to ObjectSetValue::Inline"
                        )
                    }
                } else {
                    GeneratedSymbols::empty()
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use insta::assert_debug_snapshot;

    use crate::{
        intermediate::ASN1Type,
        linker::{
            symbol_table::{Scope, SymbolId},
            unnest::Unnest,
        },
        prelude::ir::*,
    };

    fn test_sequence(members: Vec<ASN1Type>) -> ASN1Type {
        ASN1Type::Sequence(SequenceOrSet {
            components_of: Vec::new(),
            extensible: None,
            constraints: Vec::new(),
            members: members
                .into_iter()
                .enumerate()
                .map(|(i, m)| SequenceOrSetMember {
                    name: Cow::Owned(format!("m{i}")),
                    tag: None,
                    ty: m,
                    optionality: Optionality::Required,
                    is_recursive: false,
                })
                .collect(),
        })
    }

    fn test_choice(opts: Vec<ASN1Type>) -> ASN1Type {
        ASN1Type::Choice(Choice {
            extensible: None,
            options: opts
                .into_iter()
                .enumerate()
                .map(|(i, o)| ChoiceOption {
                    name: Cow::Owned(format!("o{i}")),
                    tag: None,
                    ty: o,
                    is_recursive: false,
                })
                .collect(),
            constraints: Vec::new(),
        })
    }

    fn test_set_of(elem: ASN1Type) -> ASN1Type {
        ASN1Type::SetOf(SequenceOrSetOf {
            constraints: Vec::new(),
            element_type: Box::new(elem),
            element_tag: None,
            is_recursive: false,
        })
    }

    fn test_constrained_int<'a>() -> ASN1Type<'a> {
        ASN1Type::Integer(Integer {
            constraints: vec![Constraint::value_range(1..4, false)],
            distinguished_values: None,
        })
    }

    fn test_type_assignment<'a>(a: ASN1Type<'a>) -> Assignment<'a> {
        Assignment::Type(TypeAssignment::with_name_and_type("Test".into(), a))
    }

    fn test_object<'a>(fields: Vec<InformationObjectField<'a>>) -> ASN1Information<'a> {
        ASN1Information::Object(InformationObject {
            class_name: "TEST-CLASS".into(),
            fields: InformationObjectFields::DefaultSyntax(fields),
        })
    }

    fn test_object_set<'a>(values: Vec<ASN1Information<'a>>) -> ASN1Information<'a> {
        let values = values
            .into_iter()
            .map(|v| match v {
                ASN1Information::Object(o) => ObjectSetValue::Inline(o.fields),
                _ => panic!("expected object values"),
            })
            .collect();
        ASN1Information::ObjectSet(ObjectSet {
            values,
            extensible: None,
        })
    }

    fn test_type_object_field<'a>(field: ASN1Type<'a>) -> InformationObjectField<'a> {
        InformationObjectField::TypeField(TypeField {
            identifier: "test-field".into(),
            ty: field,
        })
    }

    fn test_value_object_field<'a>(field: ASN1Value<'a>) -> InformationObjectField<'a> {
        InformationObjectField::FixedValueField(FixedValueField {
            identifier: "test-value".into(),
            value: field,
        })
    }

    fn test_object_set_object_field<'a>(field: ASN1Information<'a>) -> InformationObjectField<'a> {
        match field {
            ASN1Information::ObjectSet(o) => {
                InformationObjectField::ObjectSetField(ObjectSetField {
                    identifier: "test-set".into(),
                    value: o,
                })
            }
            _ => panic!("expected object set values"),
        }
    }

    fn test_object_assignment<'a>(info: ASN1Information<'a>) -> Assignment<'a> {
        Assignment::Object(ObjectOrObjectSetAssignment::with_name_class_value(
            "Test".into(),
            ClassLink::ByName("TEST-CLASS".into()),
            info,
        ))
    }

    fn test_sequence_value<'a>(members: Vec<(&'a str, ASN1Value<'a>)>) -> ASN1Value<'a> {
        ASN1Value::SequenceOrSet(
            members
                .into_iter()
                .map(|(id, m)| (Some(id.into()), Box::new(m)))
                .collect(),
        )
    }

    fn test_choice_value<'a>(opt: &'a str, val: ASN1Value<'a>) -> ASN1Value<'a> {
        ASN1Value::Choice {
            type_name: None,
            variant_name: opt.into(),
            inner_value: Box::new(val),
        }
    }

    fn test_value_assignment<'a>(val: ASN1Value<'a>, ty: ASN1Type<'a>) -> Assignment<'a> {
        Assignment::Value(ValueAssignment::with_name_value_type(
            "test".into(),
            val,
            ty,
        ))
    }

    #[test]
    fn unnests_mixed_nested_sequence() {
        let mut test_sequence = test_type_assignment(test_sequence(vec![
            test_choice(vec![
                ASN1Type::Boolean(Boolean {
                    constraints: Vec::new(),
                }),
                test_constrained_int(),
            ]),
            test_set_of(ASN1Type::Null),
            test_sequence(vec![test_sequence(vec![
                ASN1Type::Boolean(Boolean {
                    constraints: Vec::new(),
                }),
                ASN1Type::BitString(BitString {
                    constraints: vec![Constraint::fixed_size(3, false)],
                    distinguished_values: None,
                }),
            ])]),
        ]));
        let id = SymbolId {
            module_reference: "test-module".into(),
            pdu_reference: "Test".into(),
            scope: Scope::Module,
        };
        assert_debug_snapshot!(test_sequence.unnest(id, ()));
        assert_debug_snapshot!(test_sequence);
    }

    #[test]
    fn unnests_mixed_nested_set_of() {
        let mut test_set = test_type_assignment(test_set_of(test_set_of(test_choice(vec![
            ASN1Type::Boolean(Boolean {
                constraints: Vec::new(),
            }),
            test_constrained_int(),
            test_set_of(ASN1Type::Null),
            test_sequence(vec![test_sequence(vec![
                ASN1Type::Boolean(Boolean {
                    constraints: Vec::new(),
                }),
                ASN1Type::BitString(BitString {
                    constraints: vec![Constraint::fixed_size(3, false)],
                    distinguished_values: None,
                }),
            ])]),
        ]))));
        let id = SymbolId {
            module_reference: "test-module".into(),
            pdu_reference: "Test".into(),
            scope: Scope::Module,
        };
        assert_debug_snapshot!(test_set.unnest(id, ()));
        assert_debug_snapshot!(test_set);
    }

    #[test]
    fn unnests_nested_value() {
        let mut test_val = test_value_assignment(
            test_sequence_value(vec![
                ("m0", test_choice_value("o0", ASN1Value::Integer(42))),
                ("m1", ASN1Value::Null),
            ]),
            test_sequence(vec![
                test_choice(vec![test_constrained_int(), ASN1Type::Null]),
                ASN1Type::Null,
            ]),
        );
        let id = SymbolId {
            module_reference: "test-module".into(),
            pdu_reference: "test".into(),
            scope: Scope::Module,
        };
        assert_debug_snapshot!(test_val.unnest(id, ()));
        assert_debug_snapshot!(test_val);
    }

    #[test]
    fn unnests_inline_information_objects() {
        let mut test_obj = test_object_assignment(test_object_set(vec![test_object(vec![
            test_type_object_field(test_sequence(vec![test_constrained_int()])),
            test_value_object_field(test_sequence_value(vec![(
                "m0",
                test_choice_value("o0", ASN1Value::Integer(42)),
            )])),
            test_object_set_object_field(test_object_set(vec![test_object(vec![
                test_type_object_field(ASN1Type::BitString(BitString {
                    constraints: vec![Constraint::fixed_size(1, true)],
                    distinguished_values: None,
                })),
            ])])),
        ])]));
        let id = SymbolId {
            module_reference: "test-module".into(),
            pdu_reference: "Test".into(),
            scope: Scope::Module,
        };
        assert_debug_snapshot!(test_obj.unnest(id, ()));
        assert_debug_snapshot!(test_obj);
    }
}
