use std::collections::BTreeMap;

use crate::intermediate::{
    error::{GrammarError, GrammarErrorType},
    types::DefaultValue,
    ToplevelDeclaration,
};

impl DefaultValue {
    /// Builds a chain of typereferences (see [AssociatedValue])
    pub fn build_typereference_chain(
        &mut self,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> Result<(), GrammarError> {
        match self {
            DefaultValue::WithTypereference {
                typereference,
                value,
            } => {
                if let Some(ToplevelDeclaration::Type(tld)) = tlds.get(typereference) {
                    let (typereferences, base_type) =
                        tld.build_typereference_chain(tlds, (vec![], None))?;
                    *self = DefaultValue::WithTypereferenceChain {
                        typereferences,
                        base_type: base_type.ok_or_else(|| GrammarError {
                            details: format!(
                                "Failed to build typereference chain for {typereference}"
                            ),
                            kind: GrammarErrorType::LinkerError,
                        })?,
                        value: *value,
                    };
                    Ok(())
                } else {
                    Err(GrammarError {
                        details: format!("Failed to build typereference chain for {typereference}"),
                        kind: GrammarErrorType::LinkerError,
                    })
                }
            }
            _ => Ok(()),
        }
    }
}
