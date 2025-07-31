use std::borrow::Cow;

use crate::{
    intermediate::{constraints::*, error::*, *},
    linker::{symbol_table::UnnestedSymbols, SymbolTable},
    prelude::LinkerError,
};

/// Traverses an to replace references to other symbols
/// in a constraint. An example would be the constraint of the `intercontinental` field in the
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
/// The trait handles linking of multiple constraint references within a assignments.
///
/// ## Parameterization
/// This linking step also resolves implementations of parameterized types.
/// The compiler does not create representations of abstract parameterized types
/// but only of actual implementations. For example, no rust output
/// will be generated for
/// ```ignore
/// ParamType { INTEGER: lower, BOOLEAN: flag } ::= SEQUENCE {
///     int-value INTEGER (lower..12),
///     bool-value BOOLEAN DEFAULT flag
/// }
/// ```
/// but an implementing type such as
/// ```ignore
/// ImplType ::= ParamType { 2, TRUE }
/// ```
/// will be represented in the generated rust bindings.
pub(super) trait ResolveConstraitReferences<'a> {
    type ExtraArgs;

    fn resolve_constraint_references(
        &mut self,
        symbol_table: SymbolTable<'a, UnnestedSymbols>,
        extra_args: Self::ExtraArgs,
    ) -> Result<(), LinkerError>;
}

pub(super) struct TypeNameExtraArgs<'a> {
    type_name: Cow<'a, str>,
}

impl<'a> ResolveConstraitReferences<'a> for SubtypeElements<'a> {
    type ExtraArgs = TypeNameExtraArgs<'a>;

    fn resolve_constraint_references(
        &mut self,
        symbol_table: SymbolTable<'a, UnnestedSymbols>,
        extra_args: Self::ExtraArgs,
    ) -> Result<(), LinkerError> {
        match self {
            SubtypeElements::SingleValue { value, .. } => {
                todo!()
            }
            SubtypeElements::ContainedSubtype { subtype, .. } => todo!(),
            SubtypeElements::ValueRange { min, max, .. } => {
                todo!()
            }
            SubtypeElements::PermittedAlphabet(element_or_set_operation) => todo!(),
            SubtypeElements::SizeConstraint(element_or_set_operation) => todo!(),
            SubtypeElements::TypeConstraint(asn1_type) => todo!(),
            _ => Ok(()),
        }
    }
}
