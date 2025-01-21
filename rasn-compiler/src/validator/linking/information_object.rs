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
                match resolve_and_link(&mut o.fields, class, tlds)? {
                    Some(ToplevelInformationDefinition {
                        value: ASN1Information::Object(obj),
                        ..
                    }) => {
                        self.value = ASN1Information::ObjectSet(ObjectSet {
                            values: vec![ObjectSetValue::Inline(obj.fields.clone())],
                            extensible: None,
                        });
                    }
                    Some(ToplevelInformationDefinition {
                        value: ASN1Information::ObjectSet(set),
                        ..
                    }) => {
                        self.value = ASN1Information::ObjectSet(set.clone());
                    }
                    _ => (),
                }
                Ok(())
            }
            (ASN1Information::ObjectSet(ref mut o), Some(ClassLink::ByReference(class))) => {
                o.values.iter_mut().try_for_each(|value| match value {
                    ObjectSetValue::Reference(_) => Ok(()),
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

fn resolve_and_link(
    fields: &mut InformationObjectFields,
    class: &InformationObjectClass,
    tlds: &BTreeMap<String, ToplevelDefinition>,
) -> Result<Option<ToplevelInformationDefinition>, GrammarError> {
    match resolve_custom_syntax(fields, class) {
        Ok(()) => link_object_fields(fields, class, tlds).map(|_| None),
        Err(
            err @ GrammarError {
                kind: GrammarErrorType::SyntaxMismatch,
                ..
            },
        ) => {
            if let InformationObjectFields::CustomSyntax(c) = &fields {
                if let Some(id) = c.first().and_then(SyntaxApplication::as_str_or_none) {
                    if let Some(ToplevelDefinition::Information(tld)) = tlds.get(id) {
                        let mut tld_clone = tld.clone().resolve_class_reference(tlds);
                        tld_clone.collect_supertypes(tlds)?;
                        return Ok(Some(tld_clone));
                    }
                }
            }
            Err(err)
        }
        Err(e) => Err(e),
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
                    .ok_or_else(|| {
                        GrammarError::new(
                            &format!(
                                "Could not determine type of fixed value field {}",
                                fixed.identifier
                            ),
                            GrammarErrorType::LinkerError,
                        )
                    })
                    .and_then(|ty| {
                        fixed
                            .value
                            .link_with_type(tlds, ty, Some(&ty.as_str().to_string()))
                    }),
                InformationObjectField::ObjectSetField(_) => Err(GrammarError::new(
                    "Linking object set fields is not yet supported!",
                    GrammarErrorType::NotYetInplemented,
                )),
                _ => Ok(()),
            })
        }
        InformationObjectFields::CustomSyntax(_) => Err(GrammarError::new(
            "Unexpectedly encountered unresolved custom syntax linking information object",
            GrammarErrorType::LinkerError,
        )),
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
    ) -> Option<&'a InformationObjectClassField> {
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
    ) -> Option<Vec<ObjectSetValue>> {
        match self {
            ObjectSetValue::Reference(id) => match tlds.get(id) {
                Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                    value: ASN1Information::Object(obj),
                    ..
                })) => {
                    *self = ObjectSetValue::Inline(obj.fields.clone());
                    None
                }
                Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                    value: ASN1Information::ObjectSet(obj),
                    ..
                })) => Some(obj.values.clone()),
                _ => None,
            },
            ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(c)) => {
                c.iter_mut()
                    .any(|field| field.link_object_set_reference(tlds));
                None
            }
            ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(d)) => {
                d.iter_mut()
                    .any(|field| field.link_object_set_reference(tlds));
                None
            }
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
        let mut flattened: Vec<_> = self
            .values
            .iter_mut()
            .flat_map(|val| val.link_object_set_reference(tlds).unwrap_or_default())
            .collect();
        self.values.append(&mut flattened);
        true
    }

    pub fn references_object_set_by_name(&self) -> bool {
        self.values
            .iter()
            .any(|val| val.references_object_set_by_name())
    }

    pub fn resolve_object_set_references(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDefinition>,
    ) -> Result<(), GrammarError> {
        let mut flattened_members = Vec::new();
        let mut needs_recursing = false;
        'resolving_references: for mut value in std::mem::take(&mut self.values) {
            if let ObjectSetValue::Reference(id) = value {
                match tlds.get(&id) {
                    Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                        value: ASN1Information::ObjectSet(set),
                        ..
                    })) => {
                        set.values
                            .iter()
                            .for_each(|v| flattened_members.push(v.clone()));
                        needs_recursing = true;
                        continue 'resolving_references;
                    }
                    Some(ToplevelDefinition::Information(ToplevelInformationDefinition {
                        value: ASN1Information::Object(obj),
                        ..
                    })) => value = ObjectSetValue::Inline(obj.fields.clone()),
                    _ => {
                        return Err(GrammarError::new(
                            "Failed to resolve reference in object set.",
                            GrammarErrorType::LinkerError,
                        ))
                    }
                }
            }
            flattened_members.push(value)
        }
        self.values = flattened_members;
        if needs_recursing {
            self.resolve_object_set_references(tlds)
        } else {
            Ok(())
        }
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
