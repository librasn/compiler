use std::borrow::Cow;

use crate::{
    intermediate::{
        ASN1Type, ASN1Value, DeclarationElsewhere, DefinedType, TypeAssignment, ValueAssignment,
    },
    linker::symbol_table::{Scope, SymbolId},
    prelude::{ir::{IterMembers, MemberOrOption, ObjectClassAssignment, ObjectOrObjectSetAssignment, SequenceOrSet}, Assignment},
};

#[derive(Debug, Clone, PartialEq)]
pub(super) struct Unnested<'a> {
    pub id: SymbolId<'a>,
    pub assignment: Assignment<'a>,
}

trait Unnest<'a> {
    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
    ) -> Option<Vec<Unnested<'a>>>;
}

impl<'a> Unnest<'a> for ASN1Type<'a> {
    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
    ) -> Option<Vec<Unnested<'a>>> {
            match self {
                ASN1Type::Choice(c) => c.unnest(parent_symbol),
                ASN1Type::Sequence(s)
                | ASN1Type::Set(s) => s.unnest(parent_symbol),
                ASN1Type::SequenceOf(s)
                | ASN1Type::SetOf(s) => todo!(),
                _ => None,
            }
    }
}


impl<'a, I: IterMembers<'a>> Unnest<'a> for I {
    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
    ) -> Option<Vec<Unnested<'a>>> {
        Some(self.iter_mut_members().filter_map(|m| {
        if m.ty().is_constrained_type() || m.ty().is_constructed_type() {
            let id = SymbolId {
                module_reference: parent_symbol.module_reference.clone(),
                type_reference: parent_symbol.type_reference.clone(),
                scope: parent_symbol.scope.clone() + Scope::Local(m.name().clone()),
            };
            let name =  Cow::<'a, str>::Owned(id.to_string());
            let ty_reference = ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                parent: None,
                identifier: DefinedType::TypeReference(name.clone()),
                constraints: Vec::new(),
            });
            let ty = std::mem::replace(m.ty_mut(), ty_reference);
            Some(Unnested {
                assignment: Assignment::Type(TypeAssignment::with_name_and_type(name, ty)),
                id,
            })
        } else {
            None
        }
    }).collect())
    }
}

impl<'a> Unnest<'a> for Assignment<'a> {
    fn unnest(&mut self, symbol_id: SymbolId<'a>) -> Option<Vec<Unnested<'a>>> {
        match self {
            Assignment::Type(t) => t.unnest(symbol_id),
            _ => None,
        }
    }
}

impl<'a> Unnest<'a> for TypeAssignment<'a> {
    fn unnest(&mut self, symbol_id: SymbolId<'a>) -> Option<Vec<Unnested<'a>>> {
        let stage_result= self.ty.unnest(symbol_id.clone());
        if let Some(mut result) = stage_result {
            let mut next_stage_result = Vec::new();
            for unnested in &mut result {
                next_stage_result.append(
                    &mut unnested.assignment.unnest(unnested.id.clone()
                ).unwrap_or_default());
            }
            result.append(&mut next_stage_result); 
            Some(result)
        } else {
            None
        }
    }
}

impl<'a> ValueAssignment<'a> {
    fn unnest(&mut self, module_reference: Cow<'a, str>) -> Option<Vec<Unnested<'a>>> {
        if self.value.is_constructed_value() {
            match (&mut self.value, &self.associated_type){
                (ASN1Value::Choice {
                    variant_name,
                    inner_value,
                    ..
                }, ASN1Type::Choice(c)) => {
                    if inner_value.is_constructed_value() {
                        let id = SymbolId {
                            module_reference,
                            type_reference: self.name.clone(),
                            scope: Scope::Local(variant_name.clone()),
                        };
                        let name = Cow::<'a, str>::Owned(id.to_string());
                        let unnested_value = std::mem::replace(
                            inner_value,
                            Box::new(ASN1Value::ElsewhereDeclaredValue { 
                                parent: None,
                                identifier: name.clone()
                            }
                        ));
                        let replacement = Unnested {
                            id,
                            assignment: Assignment::Value(
                                ValueAssignment::with_name_value_type(
                                    name,
                                    *unnested_value,
                                    c.options.iter()
                                        .find(|o| 
                                            &o.name == variant_name
                                        )
                                        .expect("Unnesting: No matching variant name found!")
                                        .ty
                                        .clone()
                                    )
                                )
                            };
                        Some(vec![replacement])
                    } else {
                        None
                    }
                },
                (ASN1Value::SequenceOrSet(items),_) => todo!(),
                _ => unreachable!("Only plain Choice, Sequence, Set, SequenceOf, or SetOf values should be defined"),
            }
        } else {
            None
        }
    }
}

impl<'a> Unnest<'a> for ObjectClassAssignment<'a> {
    fn unnest(&mut self, symbol_id: SymbolId<'a>) -> Option<Vec<Unnested<'a>>> {
        Some(self.definition.fields.iter_mut().filter_map(|field| {
            field.ty.as_mut().and_then(|t| t.unnest(symbol_id.clone()))
        }).flatten().collect())
    }
}

impl<'a> Unnest<'a> for ObjectOrObjectSetAssignment<'a> {
    fn unnest(
        &mut self,
        parent_symbol: SymbolId<'a>,
    ) -> Option<Vec<Unnested<'a>>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use crate::{intermediate::ASN1Type, linker::{symbol_table::{Scope, SymbolId}, unnest::Unnest}, prelude::ir::*};

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
                    constraints: vec![
                        Constraint::Subtype(
                            ElementSetSpecs {
                                set: ElementOrSetOperation::Element(
                                    SubtypeElements::SingleValue {
                                        value: ASN1Value::Boolean(true),
                                        extensible: false 
                                    }
                                ),
                                extensible: false 
                            }
                        )
                    ] 
                }),
                optionality: Optionality::Required,
                is_recursive: false,
                constraints: Vec::new()
            }]
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
                            constraints: Vec::new()
                        }],
                    }),
                    optionality: Optionality::Required,
                    is_recursive: false,
                    constraints: Vec::new()
                }],
            }),
        ));
        let result = input.unnest(SymbolId { module_reference: "test-module".into(), type_reference: "test-type".into(), scope: Scope::Module }).unwrap();
        println!("{result:#?}");
    }
}