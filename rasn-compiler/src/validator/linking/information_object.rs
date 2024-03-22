use std::collections::BTreeMap;

use crate::intermediate::{information_object::*, *};

use super::{
    utils::{resolve_custom_syntax, walk_object_field_ref_path},
    GrammarError, GrammarErrorType,
};

impl ToplevelInformationDefinition {
    pub fn resolve_class_reference(mut self, tlds: &BTreeMap<String, ToplevelDefinition>) -> Self {
        if let Some(ClassLink::ByName(name)) = &self.class {
            if let Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                value: ASN1Information::ObjectClass(c),
                ..
            })) = tlds.get(name)
            {
                self.class = Some(ClassLink::ByReference(c.clone()));
            }
        }
        self
    }

    /// Collects supertypes of ASN1 values.
    /// In `ToplevelTypeDefinition`s, values will appear only as `DEFAULT`
    /// values in `SET`s or `SEQUENCE`s.
    pub fn collect_supertypes(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        match (&mut self.value, &self.class) {
            (ASN1Information::Object(ref mut o), Some(ClassLink::ByReference(class))) => {
                resolve_custom_syntax(&mut o.fields, class)?;
                link_object_fields(&mut o.fields, class, tlds)
            }
            (ASN1Information::ObjectSet(ref mut o), Some(ClassLink::ByReference(class))) => {
                o.values.iter_mut().try_for_each(|value| match value {
                    ObjectSetValue::Reference(_) => Err(GrammarError { details: "Collecting supertypes of information object set values is currently unsupported!".into(), kind: GrammarErrorType::NotYetInplemented }),
                    ObjectSetValue::Inline(ref mut fields) => {
                        resolve_custom_syntax(fields, class)?;
                        link_object_fields(fields, class, tlds)
                    }
                })
            }
            _ => Ok(()),
        }
    }
}

fn link_object_fields(
    fields: &mut InformationObjectFields,
    class: &InformationObjectClass,
    tlds: &BTreeMap<String, ToplevelDefinition>,
) -> Result<(), GrammarError> {
    match fields {
        InformationObjectFields::DefaultSyntax(ref mut fields) => {
            fields.iter_mut().try_for_each(|field| match field {
                InformationObjectField::FixedValueField(fixed) => class
                    .fields
                    .iter()
                    .find_map(|f| {
                        (f.identifier
                            == ObjectFieldIdentifier::SingleValue(fixed.identifier.clone()))
                        .then_some(f.ty.as_ref())
                    })
                    .flatten()
                    .ok_or_else(|| GrammarError {
                        details: format!(
                            "Could not determine type of fixed value field {}",
                            fixed.identifier,
                        ),
                        kind: GrammarErrorType::LinkerError,
                    })
                    .and_then(|ty| fixed.value.link_with_type(tlds, ty)),
                InformationObjectField::ObjectSetField(_) => todo!(),
                _ => Ok(()),
            })
        }
        InformationObjectFields::CustomSyntax(_) => Err(GrammarError {
            details: "Unexpectedly encountered unresolved custom syntax linking information object"
                .to_string(),
            kind: GrammarErrorType::LinkerError,
        }),
    }
}

impl ASN1Information {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> bool {
        match self {
            ASN1Information::ObjectSet(s) => s.link_object_set_reference(tlds),
            ASN1Information::Object(o) => o.link_object_set_reference(tlds),
            ASN1Information::ObjectClass(_) => false,
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            ASN1Information::ObjectSet(s) => s.references_object_set_by_name(),
            ASN1Information::Object(o) => o.references_object_set_by_name(),
            ASN1Information::ObjectClass(_) => false,
        }
    }
}

impl SyntaxApplication {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> bool {
        match self {
            SyntaxApplication::ObjectSetDeclaration(o) => o.link_object_set_reference(tlds),
            _ => false,
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            SyntaxApplication::ObjectSetDeclaration(o) => o.references_object_set_by_name(),
            _ => false,
        }
    }
}

impl InformationObjectClass {
    pub fn get_field<'a>(
        &'a self,
        path: &'a Vec<ObjectFieldIdentifier>,
    ) -> Option<&InformationObjectClassField> {
        walk_object_field_ref_path(&self.fields, path, 0)
    }
}

impl InformationObject {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> bool {
        match &mut self.fields {
            InformationObjectFields::DefaultSyntax(d) => d
                .iter_mut()
                .any(|field| field.link_object_set_reference(tlds)),
            InformationObjectFields::CustomSyntax(c) => c
                .iter_mut()
                .any(|field| field.link_object_set_reference(tlds)),
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match &self.fields {
            InformationObjectFields::DefaultSyntax(d) => {
                d.iter().any(|field| field.references_object_set_by_name())
            }
            InformationObjectFields::CustomSyntax(c) => {
                c.iter().any(|field| field.references_object_set_by_name())
            }
        }
    }
}

impl ObjectSetValue {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> bool {
        match self {
            ObjectSetValue::Reference(id) => {
                if let Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                    value: ASN1Information::Object(obj),
                    ..
                })) = tlds.get(id)
                {
                    *self = ObjectSetValue::Inline(obj.fields.clone());
                    true
                } else {
                    false
                }
            }
            ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(c)) => c
                .iter_mut()
                .any(|field| field.link_object_set_reference(tlds)),
            ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(d)) => d
                .iter_mut()
                .any(|field| field.link_object_set_reference(tlds)),
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            ObjectSetValue::Reference(_) => true,
            ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(c)) => {
                c.iter().any(|field| field.references_object_set_by_name())
            }
            ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(d)) => {
                d.iter().any(|field| field.references_object_set_by_name())
            }
        }
    }
}

impl ObjectSet {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> bool {
        self.values
            .iter_mut()
            .any(|val| val.link_object_set_reference(tlds))
    }

    pub fn references_object_set_by_name(&self) -> bool {
        self.values
            .iter()
            .any(|val| val.references_object_set_by_name())
    }
}

impl InformationObjectField {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> bool {
        match self {
            InformationObjectField::ObjectSetField(ObjectSetField { value, .. }) => {
                value.link_object_set_reference(tlds)
            }
            _ => false,
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            InformationObjectField::ObjectSetField(ObjectSetField { value, .. }) => {
                value.references_object_set_by_name()
            }
            _ => false,
        }
    }
}
