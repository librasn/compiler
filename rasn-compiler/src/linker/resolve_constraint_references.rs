use std::{borrow::Cow, collections::HashMap};

use crate::{
    intermediate::{constraints::*, *},
    linker::{
        symbol_table::{Scope, SymbolId, UnnestedSymbols},
        SymbolTable,
    },
    prelude::{ir::*, LinkerError},
};

/// Traverses constraints and defaults to replace references to other symbols. 
/// An example would be the constraint of the `intercontinental` field in the
/// following example.
/// ```ignore
/// fifteen INTEGER = 15
///
/// Departures ::= SEQUENCE {
///   local SEQUENCE (SIZE(0..999)) OF Local,
///   continental SEQUENCE (SIZE(0..99)) OF Continental,
///   intercontinental SEQUENCE (SIZE(0..fifteen)) OF Intercontinental
/// }
/// ```
/// The trait handles linking of constraint and default references within a assignments.
pub(super) trait ResolveConstraintAndDefaultReferences<'a> {
    type ExtraArgs;

    fn resolve_constraint_and_default_references(
        &mut self,
        symbol_table: SymbolTable<'a, UnnestedSymbols>,
        extra_args: &Self::ExtraArgs,
    ) -> Result<SymbolTable<'a, UnnestedSymbols>, LinkerError>;
}

#[derive(Debug, Clone)]
pub(super) struct NameValueMappingExtraArgs<'a> {
    pub(super) module_name: Cow<'a, str>,
    pub(super) mapping: Option<HashMap<Cow<'a, str>, ASN1Value<'a>>>,
}

impl<'a> NameValueMappingExtraArgs<'a> {
    fn new(
        ty: &ASN1Type<'a>,
        module_name: Cow<'a, str>,
        symbol_table: &SymbolTable<'a, UnnestedSymbols>,
    ) -> NameValueMappingExtraArgs<'a> {
        let module_name = match ty {
            ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                identifier:
                    DefinedType::ExternalTypeReference {
                        modulereference,
                        ..
                    },
                ..
            }) => modulereference.clone(),
            _ => module_name,
        };
        let mapping = Self::mapping_for_type(ty, module_name.clone(), symbol_table);
        NameValueMappingExtraArgs {
            module_name,
            mapping,
        }
    }

    fn clone_with_type(
        &self,
        ty: &ASN1Type<'a>,
        symbol_table: &SymbolTable<'a, UnnestedSymbols>,
    ) -> Self {
        let mut clone = self.clone();
        clone.mapping = Self::mapping_for_type(&ty, self.module_name.clone(), symbol_table);
        clone
    }

    fn get_value(&self, name: &Cow<'_, str>) -> Option<ASN1Value<'a>> {
        self.mapping
            .as_ref()
            .and_then(|mapping| mapping.get(name).cloned())
    }

    fn mapping_for_type(
        ty: &ASN1Type<'a>,
        module_name: Cow<'a, str>,
        symbol_table: &SymbolTable<'a, UnnestedSymbols>,
    ) -> Option<HashMap<Cow<'a, str>, ASN1Value<'a>>> {
        match ty {
            ASN1Type::Integer(Integer {
                distinguished_values,
                ..
            }) => distinguished_values.as_ref().map(|dv| {
                dv.iter()
                    .map(|value| (value.name.clone(), ASN1Value::Integer(value.value)))
                    .collect()
            }),
            ASN1Type::Enumerated(Enumerated { members, .. }) => Some({
                members
                    .iter()
                    .map(|m| {
                        (
                            m.name.clone(),
                            ASN1Value::EnumeratedValue {
                                enumerated: todo!("Represent enumerated reference in EnumeratedValue as DefineType"),
                                enumerable: m.name.clone(),
                            },
                        )
                    })
                    .collect()
            }),
            ASN1Type::BitString(BitString {
                distinguished_values,
                ..
            }) => todo!("Return named bits in a linker name value mapping."),
            ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                identifier: DefinedType::TypeReference(typereference),
                ..
            }) => symbol_table
                .as_top_level_type(&SymbolId {
                    module_reference: module_name.clone(),
                    pdu_reference: typereference.clone(),
                    scope: Scope::Module,
                })
                .and_then(|assignment| {
                    Self::new(
                        &assignment.ty,
                        module_name.clone(),
                        &symbol_table,
                    )
                    .mapping
                }),
            ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
                identifier:
                    DefinedType::ExternalTypeReference {
                        modulereference,
                        typereference,
                    },
                ..
            }) => symbol_table
                .as_top_level_type(&SymbolId {
                    module_reference: modulereference.clone(),
                    pdu_reference: typereference.clone(),
                    scope: Scope::Module,
                })
                .and_then(|assignment| {
                    Self::new(
                        &assignment.ty,
                        modulereference.clone(),
                        &symbol_table,
                    )
                    .mapping
                }),
            _ => None,
        }
    }
}

impl<'a> ResolveConstraintAndDefaultReferences<'a> for ASN1Type<'a> {
    type ExtraArgs = NameValueMappingExtraArgs<'a>;

    fn resolve_constraint_and_default_references(
        &mut self,
        mut symbol_table: SymbolTable<'a, UnnestedSymbols>,
        extra_args: &Self::ExtraArgs,
    ) -> Result<SymbolTable<'a, UnnestedSymbols>, LinkerError> {
        let name_value_mapping = extra_args.clone_with_type(self, &symbol_table);
        if let Some(constraints) = self.constraints_mut() {
            for constraint in constraints {
                symbol_table =
                    constraint.resolve_constraint_and_default_references(symbol_table, &name_value_mapping)?;
            }
        }
        match self {
            ASN1Type::Choice(choice) => {
                for option in choice.iter_mut_members() {
                    symbol_table = option
                        .ty
                        .resolve_constraint_and_default_references(symbol_table, &name_value_mapping)?;
                }
            }
            ASN1Type::Sequence(s) | ASN1Type::Set(s) => {
                for field in s.members.iter_mut() {
                    symbol_table = field
                        .ty
                        .resolve_constraint_and_default_references(symbol_table, &name_value_mapping)?;
                }
            }
            ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => {
                symbol_table = s
                    .element_type
                    .resolve_constraint_and_default_references(symbol_table, &name_value_mapping)?;
            }
            _ => (),
        }

        Ok(symbol_table)
    }
}

impl<'a> ResolveConstraintAndDefaultReferences<'a> for Constraint<'a> {
    type ExtraArgs = NameValueMappingExtraArgs<'a>;

    fn resolve_constraint_and_default_references(
        &mut self,
        symbol_table: SymbolTable<'a, UnnestedSymbols>,
        extra_args: &Self::ExtraArgs,
    ) -> Result<SymbolTable<'a, UnnestedSymbols>, LinkerError> {
        match self {
            Constraint::Subtype(t) => t
                .set
                .resolve_constraint_and_default_references(symbol_table, extra_args),
            _ => Ok(symbol_table),
        }
    }
}

impl<'a> ResolveConstraintAndDefaultReferences<'a> for SubtypeElements<'a> {
    type ExtraArgs = NameValueMappingExtraArgs<'a>;

    fn resolve_constraint_and_default_references(
        &mut self,
        mut symbol_table: SymbolTable<'a, UnnestedSymbols>,
        extra_args: &Self::ExtraArgs,
    ) -> Result<SymbolTable<'a, UnnestedSymbols>, LinkerError> {
        match self {
            SubtypeElements::SingleValue { value, .. } => {
                symbol_table = value.resolve_constraint_and_default_references(symbol_table, extra_args)?;
                Ok(symbol_table)
            }
            SubtypeElements::ContainedSubtype { subtype, .. } => {
                symbol_table = subtype.resolve_constraint_and_default_references(symbol_table, extra_args)?;
                Ok(symbol_table)
            }
            SubtypeElements::ValueRange { min, max, .. } => {
                if let Some(min_val) = min.as_mut() {
                    symbol_table =
                        min_val.resolve_constraint_and_default_references(symbol_table, extra_args)?;
                }
                if let Some(max_val) = max.as_mut() {
                    symbol_table =
                        max_val.resolve_constraint_and_default_references(symbol_table, extra_args)?;
                }
                Ok(symbol_table)
            }
            SubtypeElements::PermittedAlphabet(element_or_set_operation) => {
                element_or_set_operation.resolve_constraint_and_default_references(symbol_table, extra_args)
            }
            SubtypeElements::SizeConstraint(element_or_set_operation) => {
                element_or_set_operation.resolve_constraint_and_default_references(symbol_table, extra_args)
            }
            SubtypeElements::TypeConstraint(asn1_type) => {
                symbol_table = asn1_type.resolve_constraint_and_default_references(symbol_table, extra_args)?;
                Ok(symbol_table)
            }
            SubtypeElements::SingleTypeConstraint(constraints) => {
                for c in constraints {
                    symbol_table = c.resolve_constraint_and_default_references(symbol_table, extra_args)?;
                }
                Ok(symbol_table)
            }
            SubtypeElements::MultipleTypeConstraints(s) => {
                for constraint_container in s.constraints.iter_mut() {
                    for c in constraint_container.constraints.iter_mut() {
                        symbol_table = c.resolve_constraint_and_default_references(symbol_table, extra_args)?;
                    }
                }
                Ok(symbol_table)
            }
            _ => Ok(symbol_table),
        }
    }
}

impl<'a> ResolveConstraintAndDefaultReferences<'a> for ElementOrSetOperation<'a> {
    type ExtraArgs = NameValueMappingExtraArgs<'a>;

    fn resolve_constraint_and_default_references(
        &mut self,
        mut symbol_table: SymbolTable<'a, UnnestedSymbols>,
        extra_args: &Self::ExtraArgs,
    ) -> Result<SymbolTable<'a, UnnestedSymbols>, LinkerError> {
        match self {
            ElementOrSetOperation::Element(e) => {
                e.resolve_constraint_and_default_references(symbol_table, extra_args)
            }
            ElementOrSetOperation::SetOperation(s) => {
                symbol_table = s
                    .base
                    .resolve_constraint_and_default_references(symbol_table, extra_args)?;
                s.operant
                    .resolve_constraint_and_default_references(symbol_table, extra_args)
            }
        }
    }
}

impl<'a> ResolveConstraintAndDefaultReferences<'a> for ASN1Value<'a> {
    type ExtraArgs = NameValueMappingExtraArgs<'a>;

    fn resolve_constraint_and_default_references(
        &mut self,
        symbol_table: SymbolTable<'a, UnnestedSymbols>,
        extra_args: &Self::ExtraArgs,
    ) -> Result<SymbolTable<'a, UnnestedSymbols>, LinkerError> {
        let replacement = match self {
            ASN1Value::ElsewhereDeclaredValue { identifier, .. }
            | ASN1Value::EnumeratedValue {
                enumerable: identifier,
                ..
            } => extra_args.get_value(&identifier).or(symbol_table
                .as_top_level_value(&SymbolId {
                    module_reference: extra_args.module_name.clone(),
                    pdu_reference: identifier.clone(),
                    scope: Scope::Module,
                })
                .map(|value_assignment| value_assignment.value.clone())),
            _ => None,
        };
        if let Some(value) = replacement {
            *self = value;
        }
        Ok(symbol_table)
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;
    use crate::{
        intermediate::{AsnModule, ModuleHeader, TypeAssignment},
        prelude::{Assignment, ExtensibilityEnvironment, TaggingEnvironment},
    };

    #[test]
    fn resolves_integer_value_reference() {
        // fifteen INTEGER = 15
        // Departures ::= SEQUENCE {
        //    intercontinental SEQUENCE (SIZE(0..fifteen)) OF UTF8String (SIZE(fifteen..30))
        // }

        let fifteen_value = Assignment::Value(ValueAssignment::with_name_value_type(
            "fifteen".into(),
            ASN1Value::Integer(15),
            ASN1Type::Integer(Integer {
                constraints: vec![],
                distinguished_values: None,
            }),
        ));

        let inner_string_constraints = vec![Constraint::Subtype(ElementSetSpecs {
            set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(Box::new(
                ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                    min: Some(ASN1Value::ElsewhereDeclaredValue {
                        parent: None,
                        identifier: "fifteen".into(),
                    }),
                    max: Some(ASN1Value::Integer(30)),
                    extensible: false,
                }),
            ))),
            extensible: false,
        })];

        let mut departures_type = ASN1Type::Sequence(SequenceOrSet {
            components_of: Vec::new(),
            extensible: None,
            constraints: Vec::new(),
            members: vec![SequenceOrSetMember {
                name: "intercontinental".into(),
                tag: None,
                ty: ASN1Type::SequenceOf(SequenceOrSetOf {
                    element_tag: None,
                    is_recursive: false,
                    constraints: vec![Constraint::Subtype(ElementSetSpecs {
                        set: ElementOrSetOperation::Element(SubtypeElements::SizeConstraint(
                            Box::new(ElementOrSetOperation::Element(
                                SubtypeElements::ValueRange {
                                    min: Some(ASN1Value::Integer(0)),
                                    max: Some(ASN1Value::ElsewhereDeclaredValue {
                                        parent: None,
                                        identifier: "fifteen".into(),
                                    }),
                                    extensible: false,
                                },
                            )),
                        )),
                        extensible: false,
                    })],
                    element_type: Box::new(ASN1Type::CharacterString(CharacterString {
                        constraints: inner_string_constraints,
                        ty: CharacterStringType::UTF8String,
                    })),
                }),
                optionality: Optionality::Required,
                is_recursive: false,
            }],
        });

        let symbol_table = SymbolTable::<UnnestedSymbols>::with_state(vec![AsnModule {
            module_header: ModuleHeader {
                name: "test-module".into(),
                module_identifier: None,
                encoding_reference_default: None,
                tagging_environment: TaggingEnvironment::Automatic,
                extensibility_environment: ExtensibilityEnvironment::Explicit,
                imports: Vec::new(),
                exports: None,
            },
            assignments: vec![fifteen_value],
        }])
        .unwrap();

        let extra_args = NameValueMappingExtraArgs {
            module_name: "test-module".into(),
            mapping: None,
        };

        let result = departures_type.resolve_constraint_and_default_references(symbol_table, &extra_args);

        assert!(result.is_ok());
        assert_debug_snapshot!(departures_type);
    }

    #[test]
    fn resolves_distinguished_integer_value_reference() {
        // Int ::= INTEGER { zero(0), five(5), ten(10) }
        // UpToFive ::= Int(zero..five)

        let int_type = Assignment::Type(TypeAssignment {
            comments: "".into(),
            tag: None,
            name: "Int".into(),
            ty: ASN1Type::Integer(Integer {
                constraints: vec![],
                distinguished_values: Some(vec![
                    DistinguishedValue {
                        name: "zero".into(),
                        value: 0,
                    },
                    DistinguishedValue {
                        name: "five".into(),
                        value: 5,
                    },
                    DistinguishedValue {
                        name: "ten".into(),
                        value: 10,
                    },
                ]),
            }),
            parameterization: None,
            module_header: None,
        });

        let mut up_to_five_type = ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere {
            parent: None,
            identifier: DefinedType::TypeReference("Int".into()),
            constraints: vec![Constraint::Subtype(ElementSetSpecs {
                set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                    min: Some(ASN1Value::ElsewhereDeclaredValue {
                        parent: None,
                        identifier: "zero".into(),
                    }),
                    max: Some(ASN1Value::ElsewhereDeclaredValue {
                        parent: None,
                        identifier: "five".into(),
                    }),
                    extensible: false,
                }),
                extensible: false,
            })],
        });

        let symbol_table = SymbolTable::<UnnestedSymbols>::with_state(vec![AsnModule {
            module_header: ModuleHeader {
                name: "test-module".into(),
                module_identifier: None,
                encoding_reference_default: None,
                tagging_environment: TaggingEnvironment::Automatic,
                extensibility_environment: ExtensibilityEnvironment::Explicit,
                imports: Vec::new(),
                exports: None,
            },
            assignments: vec![int_type],
        }])
        .unwrap();

        let extra_args = NameValueMappingExtraArgs {
            module_name: "test-module".into(),
            mapping: None,
        };

        let result = up_to_five_type.resolve_constraint_and_default_references(symbol_table, &extra_args);

        assert!(result.is_ok());
        assert_debug_snapshot!(up_to_five_type);
    }
}
