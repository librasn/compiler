use std::borrow::Cow;

use crate::{
    intermediate::{
        ASN1Type, ASN1Value, DeclarationElsewhere, DefinedType, TypeAssignment, ValueAssignment,
    },
    linker::symbol_table::SymbolId,
    prelude::{
        ir::{
            ASN1Information, ClassLink, InformationObject, InformationObjectField,
            InformationObjectFields, IterMembers, MemberOrOption, ObjectClassAssignment,
            ObjectOrObjectSetAssignment, ObjectSet, ObjectSetValue, SyntaxApplication,
        },
        Assignment,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Unnested<'a> {
    pub id: SymbolId<'a>,
    pub assignment: Assignment<'a>,
}


/// ## What will not be unnested
/// Nested fixed value fields in information objects cannot be unnested because
/// the linker does not have enough information about the associated type.
pub(super) trait Unnest<'a> {
    type ExtraArgs;

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>>;
}

impl<'a> Unnest<'a> for ASN1Type<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>> {
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
                        Some(vec![Unnested {
                            assignment: Assignment::Type(TypeAssignment::with_name_and_type(
                                name, *ty,
                            )),
                            id,
                        }])
                    } else {
                        None
                    }
            },
            _ => None,
        }
    }
}

impl<'a, I: IterMembers<'a>> Unnest<'a> for I {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        _: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>> {
        Some(
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
                        Some(Unnested {
                            assignment: Assignment::Type(TypeAssignment::with_name_and_type(
                                name, ty,
                            )),
                            id,
                        })
                    } else {
                        None
                    }
                })
                .collect(),
        )
    }
}

impl<'a> Unnest<'a> for Assignment<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        symbol_id: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>> {
        let stage_result = match self {
            Assignment::Type(t) => t.unnest(symbol_id, extra_args),
            Assignment::Class(c) => c.unnest(symbol_id, extra_args),
            Assignment::Object(o) => o.unnest(symbol_id, extra_args),
            Assignment::Value(v) => v.unnest(symbol_id, extra_args),
            _ => None,
        };
        if let Some(mut result) = stage_result {
            let mut next_stage_result = Vec::new();
            for unnested in &mut result {
                next_stage_result.append(
                    &mut unnested
                        .assignment
                        .unnest(unnested.id.clone(), extra_args)
                        .unwrap_or_default(),
                );
            }
            result.append(&mut next_stage_result);
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> Unnest<'a> for TypeAssignment<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        symbol_id: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>> {
        self.ty.unnest(symbol_id.clone(), extra_args)
    }
}

impl<'a> Unnest<'a> for ValueAssignment<'a> {
    type ExtraArgs = ();

    fn unnest(&mut self, symbol_id: SymbolId<'a>, _: Self::ExtraArgs) -> Option<Vec<Unnested<'a>>> {
        fn unnest_iter_members<'a, I: IterMembers<'a>>(
            symbol_id: &SymbolId<'a>,
            member_name: &Cow<'a, str>,
            inner_value: &mut Box<ASN1Value<'a>>,
            iterable: &I,
        ) -> Unnested<'a> {
            let id = symbol_id.locally_scoped(member_name.clone());
            let name = Cow::<'a, str>::Owned(id.to_string());
            let unnested_value = std::mem::replace(
                inner_value,
                Box::new(ASN1Value::ElsewhereDeclaredValue {
                    parent: None,
                    identifier: name.clone(),
                }),
            );
            let replacement = Unnested {
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
                        Some(vec![replacement])
                    } else {
                        None
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
                    Some(unnested)
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
                            let replacement = Unnested {
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
                    Some(unnested)
                }
                _ => unreachable!("Only plain Choice, Sequence, Set, SequenceOf, or SetOf values should be defined"),
            }
        } else {
            None
        }
    }
}

impl<'a> Unnest<'a> for ObjectClassAssignment<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        symbol_id: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>> {
        Some(
            self.definition
                .fields
                .iter_mut()
                .filter_map(|field| {
                    field
                        .ty
                        .as_mut()
                        .and_then(|t| t.unnest(symbol_id.clone(), extra_args))
                })
                .flatten()
                .collect(),
        )
    }
}

impl<'a> Unnest<'a> for ObjectOrObjectSetAssignment<'a> {
    type ExtraArgs = ();

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        _: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>> {
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
    ) -> Option<Vec<Unnested<'a>>> {
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

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        _: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>> {
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
    ) -> Option<Vec<Unnested<'a>>> {
        match self {
            InformationObjectFields::DefaultSyntax(fields) => Some(
                fields
                    .iter_mut()
                    .flat_map(|field| {
                        field
                            .unnest(parent_symbol.clone(), extra_args.clone())
                            .unwrap_or_default()
                    })
                    .collect(),
            ),
            InformationObjectFields::CustomSyntax(stx) => {
                Some(stx.iter_mut().filter_map(|elem| {
                    if let SyntaxApplication::ObjectSetDeclaration(o) = elem {
                        o.unnest(parent_symbol.clone(), extra_args.clone())
                    } else {
                        None
                    }
                }).flatten().collect())
            },
        }
    }
}

impl<'a> Unnest<'a> for InformationObjectField<'a> {
    type ExtraArgs = ExtraClassReferenceArg<'a>;

    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
        extra_args: Self::ExtraArgs,
    ) -> Option<Vec<Unnested<'a>>> {
        match self {
            InformationObjectField::TypeField(type_field) => {
                type_field.ty.unnest(parent_symbol, ())
            }
            InformationObjectField::FixedValueField(_) => None,
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
    ) -> Option<Vec<Unnested<'a>>> {
        self.values
            .iter_mut()
            .enumerate()
            .map(|(i, val)| {
                if let ObjectSetValue::Inline(_) = val {
                    let id = parent_symbol.locally_scoped(Cow::Owned(i.to_string()));
                    let name = Cow::<'a, str>::Owned(id.to_string());
                    let val_reference = ObjectSetValue::Reference(name.clone());
                    if let ObjectSetValue::Inline(fields) = std::mem::replace(val, val_reference) {
                        Some(Unnested {
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
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        intermediate::ASN1Type,
        linker::{
            symbol_table::{Scope, SymbolId},
            unnest::Unnest,
        },
        prelude::ir::*,
    };

    #[test]
    fn unnests_nested_sequence() {
        let nested = ASN1Type::Sequence(SequenceOrSet {
            components_of: Vec::new(),
            extensible: None,
            constraints: Vec::new(),
            members: vec![SequenceOrSetMember {
                name: "m1".into(),
                tag: None,
                ty: ASN1Type::Boolean(Boolean {
                    constraints: vec![Constraint::Subtype(ElementSetSpecs {
                        set: ElementOrSetOperation::Element(SubtypeElements::SingleValue {
                            value: ASN1Value::Boolean(true),
                            extensible: false,
                        }),
                        extensible: false,
                    })],
                }),
                optionality: Optionality::Required,
                is_recursive: false,
                constraints: Vec::new(),
            }],
        });
        let mut input = Assignment::Type(TypeAssignment::with_name_and_type(
            "test-nested".into(),
            ASN1Type::Sequence(SequenceOrSet {
                components_of: Vec::new(),
                extensible: None,
                constraints: Vec::new(),
                members: vec![SequenceOrSetMember {
                    name: "m2".into(),
                    tag: None,
                    ty: ASN1Type::Sequence(SequenceOrSet {
                        components_of: Vec::new(),
                        extensible: None,
                        constraints: Vec::new(),
                        members: vec![SequenceOrSetMember {
                            name: "m3".into(),
                            tag: None,
                            ty: nested,
                            optionality: Optionality::Required,
                            is_recursive: false,
                            constraints: Vec::new(),
                        }],
                    }),
                    optionality: Optionality::Required,
                    is_recursive: false,
                    constraints: Vec::new(),
                }],
            }),
        ));
        let result = input
            .unnest(
                SymbolId {
                    module_reference: "test-module".into(),
                    type_reference: "test-type".into(),
                    scope: Scope::Module,
                },
                (),
            )
            .unwrap();
        println!("{result:#?}");
    }
}
