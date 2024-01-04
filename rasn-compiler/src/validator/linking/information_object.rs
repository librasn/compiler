use std::collections::BTreeMap;

use crate::intermediate::{*, information_object::*};

use super::utils::walk_object_field_ref_path;

impl ToplevelInformationDeclaration {
    pub fn resolve_class_reference(mut self, tlds: &BTreeMap<String, ToplevelDeclaration>) -> Self {
        if let Some(ClassLink::ByName(name)) = &self.class {
            if let Some(ToplevelDeclaration::Information(ToplevelInformationDeclaration {
                value: ASN1Information::ObjectClass(c),
                ..
            })) = tlds.get(name)
            {
                self.class = Some(ClassLink::ByReference(c.clone()));
            }
        }
        self
    }
}

impl ASN1Information {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
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
        tlds: &BTreeMap<String, ToplevelDeclaration>,
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
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match &mut self.fields {
            InformationObjectFields::DefaultSyntax(d) => d.iter_mut().fold(false, |acc, field| {
                acc || field.link_object_set_reference(tlds)
            }),
            InformationObjectFields::CustomSyntax(c) => c.iter_mut().fold(false, |acc, field| {
                acc || field.link_object_set_reference(tlds)
            }),
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match &self.fields {
            InformationObjectFields::DefaultSyntax(d) => d.iter().fold(false, |acc, field| {
                acc || field.references_object_set_by_name()
            }),
            InformationObjectFields::CustomSyntax(c) => c.iter().fold(false, |acc, field| {
                acc || field.references_object_set_by_name()
            }),
        }
    }
}

impl ObjectSetValue {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            ObjectSetValue::Reference(id) => {
                if let Some(ToplevelDeclaration::Information(ToplevelInformationDeclaration {
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
            ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(c)) => {
                c.iter_mut().fold(false, |acc, field| {
                    acc || field.link_object_set_reference(tlds)
                })
            }
            ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(d)) => {
                d.iter_mut().fold(false, |acc, field| {
                    acc || field.link_object_set_reference(tlds)
                })
            }
        }
    }

    pub fn references_object_set_by_name(&self) -> bool {
        match self {
            ObjectSetValue::Reference(_) => true,
            ObjectSetValue::Inline(InformationObjectFields::CustomSyntax(c)) => {
                c.iter().fold(false, |acc, field| {
                    acc || field.references_object_set_by_name()
                })
            }
            ObjectSetValue::Inline(InformationObjectFields::DefaultSyntax(d)) => {
                d.iter().fold(false, |acc, field| {
                    acc || field.references_object_set_by_name()
                })
            }
        }
    }
}

impl ObjectSet {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        self.values
            .iter_mut()
            .fold(false, |acc, val| acc || val.link_object_set_reference(tlds))
    }

    pub fn references_object_set_by_name(&self) -> bool {
        self.values
            .iter()
            .fold(false, |acc, val| acc || val.references_object_set_by_name())
    }
}

impl InformationObjectField {
    pub fn link_object_set_reference(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
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