use std::{borrow::Cow, collections::BTreeMap};

use crate::{
    intermediate::{
        ASN1Type, ASN1Value, DeclarationElsewhere, DefinedType, TypeAssignment, ValueAssignment,
    },
    linker::{
        symbol_table::{GeneratedSymbols, Scope, SymbolId, SymbolTableEntry, UnnestedSymbols},
        SymbolTable,
    },
    prelude::{
        ir::{
            ASN1Information, ClassLink, Constraint, InformationObject, ObjectOrObjectSetAssignment,
            ObjectSet, ObjectSetValue, Parameter, ParameterGovernor, Parameterization,
            ParameterizationArgument, TableConstraint,
        },
        Assignment, LinkerError, LinkerErrorType,
    },
};

pub(super) struct SymbolTableExtraParams<'a, 'b>
where
    'a: 'b,
{
    pub symbol_table: &'b SymbolTable<'a, UnnestedSymbols>,
    pub current_context: SymbolId<'a>,
}

struct ImplReferenceWithGenerated<'a> {
    pub impl_ref: Cow<'a, str>,
    pub generated: GeneratedSymbols<'a>,
}

pub(super) trait ResolveParameters<'a> {
    fn resolve_parameters<'b>(
        &mut self,
        extra_params: &SymbolTableExtraParams<'a, 'b>,
    ) -> Result<GeneratedSymbols<'a>, LinkerError>
    where
        'a: 'b;
}

impl<'a> ResolveParameters<'a> for ASN1Type<'a> {
    fn resolve_parameters<'b>(
        &mut self,
        extra_params: &SymbolTableExtraParams<'a, 'b>,
    ) -> Result<GeneratedSymbols<'a>, LinkerError>
    where
        'a: 'b,
    {
        if let ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere { identifier, .. }) = self {
            identifier.resolve_parameters(extra_params)
        } else {
            Ok(GeneratedSymbols::empty())
        }
    }
}

impl<'a> ResolveParameters<'a> for DefinedType<'a> {
    fn resolve_parameters<'b>(
        &mut self,
        extra_params: &SymbolTableExtraParams<'a, 'b>,
    ) -> Result<GeneratedSymbols<'a>, LinkerError>
    where
        'a: 'b,
    {
        let (id, args) = match self {
        DefinedType::ParameterizedTypeOrValueSetType {
            simple_defined_type,
            actual_parameter_list,
        } => {
            match &**simple_defined_type {
                DefinedType::ExternalTypeReference {
                    modulereference,
                    typereference,
                } => (SymbolId {
                    module_reference: modulereference.clone(),
                    pdu_reference: typereference.clone(),
                    scope: Scope::Module,
                }, actual_parameter_list),
                DefinedType::TypeReference(t) => (SymbolId {
                    module_reference: extra_params.current_context.module_reference.clone(),
                    pdu_reference: t.clone(),
                    scope: Scope::Module,
                }, actual_parameter_list),
                _ => return Err(LinkerError { pdu: None, details: String::from("ITU-T X.683 (02/2021) requires SimpleDefinedType ::= ExternalTypeReference | typereference"), kind: LinkerErrorType::GrammarViolation })
            }
        }
        _ => return Ok(GeneratedSymbols::empty()),
    };
        if let Some(Assignment::Type(TypeAssignment {
            ty,
            parameterization: Some(param),
            ..
        })) = extra_params.symbol_table.get(&id)
        {
            let impl_with_gen =
                resolve_parameters_with_type(ty, param, args, &extra_params.current_context)?;
            *self = DefinedType::TypeReference(impl_with_gen.impl_ref);
            Ok(impl_with_gen.generated)
        } else {
            Err(LinkerError {
                pdu: None,
                details: format!(
                    "Failed to resolve supertype {id} of parameterized implementation.",
                ),
                kind: LinkerErrorType::MissingDependency,
            })
        }
    }
}

fn resolve_parameters_with_type<'a>(
    ty: &ASN1Type<'a>,
    parameterization: &Parameterization<'a>,
    args: &[Parameter<'a>],
    ctx: &SymbolId<'a>,
) -> Result<ImplReferenceWithGenerated<'a>, LinkerError> {
    let mut impl_template = ty.clone();
    let GeneratedSymbolsWithTableConstraintReplacements {
        mut generated_symbols,
        table_constraint_replacements,
    } = generate_parameter_impl_symbols(parameterization, args, ctx)?;
    for (dummy_reference, osv) in table_constraint_replacements {
        reassign_table_constraint(&mut impl_template, &dummy_reference, &osv)?;
    }
    let impl_id = SymbolId {
        module_reference: ctx.module_reference.clone(),
        pdu_reference: ctx.pdu_reference.clone(),
        scope: ctx.scope.clone() + Scope::Local(Cow::Borrowed("impl")),
    };
    generated_symbols += GeneratedSymbols::single(SymbolTableEntry {
        id: impl_id.clone(),
        assignment: Assignment::Type(TypeAssignment::with_name_and_type(
            Cow::Owned(impl_id.to_string()),
            impl_template,
        )),
    });
    Ok(ImplReferenceWithGenerated {
        impl_ref: Cow::Owned(impl_id.to_string()),
        generated: generated_symbols,
    })
}

/// In certain parameterization cases, the constraining object set of a table constraint
/// has to be reassigned. Consider the following example:
/// ```ignore
/// ProtocolExtensionContainer {NGAP-PROTOCOL-EXTENSION : ExtensionSetParam} ::=
///     SEQUENCE (SIZE (1..4)) OF
///     ProtocolExtensionField {{ExtensionSetParam}}
///
/// ProtocolExtensionField {NGAP-PROTOCOL-EXTENSION : ExtensionSetParam} ::= SEQUENCE {
///     id                    NGAP-PROTOCOL-EXTENSION.&id                ({ExtensionSetParam}),
///     extensionValue        NGAP-PROTOCOL-EXTENSION.&Extension        ({ExtensionSetParam}{@id})
/// }
///
/// ActualExtensions ::= ProtocolExtensionContainer { {ApplicableSet} }
/// ApplicableSet NGAP-PROTOCOL-EXTENSION ::= { ... }
/// ```
/// Since the compiler only creates bindings for actual implementations of abstract items,
/// the `ExtensionSetParam` references in `ProtocolExtensionField`'s table constraints need
/// to be reassigned to the actual object sets that are passed in by the implementations of
/// the abstract classes.
fn reassign_table_constraint<'a>(
    ty: &mut ASN1Type<'a>,
    reference_id_before: &str,
    replacement: &ObjectSetValue<'a>,
) -> Result<(), LinkerError> {
    match ty {
        ASN1Type::Sequence(s) | ASN1Type::Set(s) => {
            for m in &mut s.members {
                if let Some(constraints) = m.ty.constraints_mut() {
                    for c in constraints {
                        if let Constraint::Table(TableConstraint {
                            object_set: ObjectSet { values, .. },
                            ..
                        }) = c
                        {
                            for value in values {
                                match value {
                                    ObjectSetValue::Reference(r) if r == reference_id_before => {
                                        *value = replacement.clone();
                                    }
                                    _ => (),
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }
        ASN1Type::SequenceOf(s) | ASN1Type::SetOf(s) => {
            reassign_table_constraint(&mut s.element_type, reference_id_before, replacement)
        }
        _ => Ok(()),
    }
}

fn resolve_parameters_with_value<'a>(
    val: &ASN1Value<'a>,
    parameterization: &Parameterization<'a>,
    args: &[Parameter<'a>],
    ctx: &SymbolId<'a>,
) -> Result<ImplReferenceWithGenerated<'a>, LinkerError> {
    let mut impl_template = val.clone();
    let GeneratedSymbolsWithTableConstraintReplacements {
        mut generated_symbols, ..
    } = generate_parameter_impl_symbols(parameterization, args, ctx)?;
    let impl_id = SymbolId {
        module_reference: ctx.module_reference.clone(),
        pdu_reference: ctx.pdu_reference.clone(),
        scope: ctx.scope.clone() + Scope::Local(Cow::Borrowed("impl")),
    };
    generated_symbols += GeneratedSymbols::single(SymbolTableEntry {
        id: impl_id.clone(),
        assignment: Assignment::Value(ValueAssignment::with_name_value_type(
            Cow::Owned(impl_id.to_string()),
            impl_template,
        )),
    });
    Ok(ImplReferenceWithGenerated {
        impl_ref: Cow::Owned(impl_id.to_string()),
        generated: generated_symbols,
    })
}

struct GeneratedSymbolsWithTableConstraintReplacements<'a> {
    pub generated_symbols: GeneratedSymbols<'a>,
    pub table_constraint_replacements: BTreeMap<Cow<'a, str>, ObjectSetValue<'a>>,
}

fn generate_parameter_impl_symbols<'a>(
    parameterization: &Parameterization<'a>,
    args: &[Parameter<'a>],
    ctx: &SymbolId<'a>,
) -> Result<GeneratedSymbolsWithTableConstraintReplacements<'a>, LinkerError> {
    let mut generated_symbols = GeneratedSymbols::empty();
    let mut table_constaint_replacements = BTreeMap::new();
    for (
        index,
        ParameterizationArgument {
            dummy_reference,
            param_governor,
        },
    ) in parameterization.parameters.iter().enumerate()
    {
        let arg = args.get(index).ok_or_else(|| LinkerError {
            pdu: None,
            details: format!("Did not find an argument for parameter {dummy_reference}"),
            kind: crate::prelude::LinkerErrorType::MissingDependency,
        })?;
        match (arg, param_governor) {
            (Parameter::ValueParameter(v), ParameterGovernor::TypeOrClass(gov)) => {
                generated_symbols += GeneratedSymbols::single(SymbolTableEntry {
                    id: SymbolId {
                        module_reference: ctx.module_reference.clone(),
                        pdu_reference: ctx.module_reference.clone(),
                        scope: ctx.scope.clone() + Scope::Local(dummy_reference.clone()),
                    },
                    assignment: Assignment::Value(ValueAssignment::with_name_value_type(
                        dummy_reference.clone(),
                        v.clone(),
                        gov.clone(),
                    )),
                });
            }
            (Parameter::TypeParameter(t), _) => {
                generated_symbols += GeneratedSymbols::single(SymbolTableEntry {
                    id: SymbolId {
                        module_reference: ctx.module_reference.clone(),
                        pdu_reference: ctx.module_reference.clone(),
                        scope: ctx.scope.clone() + Scope::Local(dummy_reference.clone()),
                    },
                    assignment: Assignment::Type(TypeAssignment::with_name_and_type(
                        dummy_reference.clone(),
                        t.clone(),
                    )),
                });
            }
            (Parameter::InformationObjectParameter(f), ParameterGovernor::Class(c)) => {
                generated_symbols += GeneratedSymbols::single(SymbolTableEntry {
                    id: SymbolId {
                        module_reference: ctx.module_reference.clone(),
                        pdu_reference: ctx.module_reference.clone(),
                        scope: ctx.scope.clone() + Scope::Local(dummy_reference.clone()),
                    },
                    assignment: Assignment::Object(
                        ObjectOrObjectSetAssignment::with_name_class_value(
                            dummy_reference.clone(),
                            ClassLink::ByName(c.clone()),
                            ASN1Information::Object(InformationObject {
                                class_name: c.clone(),
                                fields: f.clone(),
                            }),
                        ),
                    ),
                });
            }
            (Parameter::ObjectSetParameter(o), ParameterGovernor::Class(c)) => {
                match &o.values.first() {
                        Some(osv) if o.values.len() == 1 => {
                            table_constaint_replacements.insert(dummy_reference.clone(), (*osv).clone());
                        }
                        _ => return Err(
                            LinkerError {
                            pdu: None,
                            details: format!(
                                "Expected object set value argument to contain single object set value!"
                            ),
                            kind: crate::prelude::LinkerErrorType::MissingDependency,
                        }),
                    }
                let info = ASN1Information::ObjectSet(o.clone());
                generated_symbols += GeneratedSymbols::single(SymbolTableEntry {
                    id: SymbolId {
                        module_reference: ctx.module_reference.clone(),
                        pdu_reference: ctx.module_reference.clone(),
                        scope: ctx.scope.clone() + Scope::Local(dummy_reference.clone()),
                    },
                    assignment: Assignment::Object(
                        ObjectOrObjectSetAssignment::with_name_class_value(
                            dummy_reference.clone(),
                            ClassLink::ByName(c.clone()),
                            info,
                        ),
                    ),
                });
            }
            _ => {
                return Err(LinkerError {
                    pdu: None,
                    details: format!("Mismatching argument for parameter {dummy_reference}"),
                    kind: crate::prelude::LinkerErrorType::MissingDependency,
                })
            }
        }
    }
    Ok(GeneratedSymbolsWithTableConstraintReplacements {
        generated_symbols,
        table_constaint_replacements,
    })
}
