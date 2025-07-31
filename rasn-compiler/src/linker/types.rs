use crate::{
    intermediate::{ASN1Type, DeclarationElsewhere},
    linker::symbol_table::GeneratedSymbols,
    prelude::{
        ir::{
            BitString, Boolean, CharacterString, Constraint, ElementOrSetOperation,
            ElementSetSpecs, Enumerated, GeneralizedTime, Integer, IterMembers, MemberOrOption,
            ObjectClassFieldType, ObjectIdentifier, OctetString, Real, SubtypeElements, Time,
            UTCTime,
        },
        LinkerError,
    },
};

impl<'a> ASN1Type<'a> {
    pub(super) fn apply_resursively<F, E>(
        &mut self,
        f: &mut F,
    ) -> Result<GeneratedSymbols<'a>, LinkerError>
    where
        F: FnMut(&mut ASN1Type<'a>, E) -> Result<GeneratedSymbols<'a>, LinkerError>,
    {
        fn apply_to_subtype_elem<'a, F, E>(
            e: &mut SubtypeElements<'a>,
            f: &mut F,
        ) -> Result<GeneratedSymbols<'a>, LinkerError>
        where
            F: FnMut(&mut ASN1Type<'a>, E) -> Result<GeneratedSymbols<'a>, LinkerError>,
        {
            match e {
                SubtypeElements::ContainedSubtype { subtype: ty, .. }
                | SubtypeElements::TypeConstraint(ty) => ty.apply_resursively(f),
                SubtypeElements::PermittedAlphabet(elem)
                | SubtypeElements::SizeConstraint(elem) => apply_to_elem_or_set_op(elem, f),
                _ => Ok(GeneratedSymbols::empty()),
            }
        }

        fn apply_to_elem_or_set_op<'a, F, E>(
            e: &mut ElementOrSetOperation<'a>,
            f: &mut F,
        ) -> Result<GeneratedSymbols<'a>, LinkerError>
        where
            F: FnMut(&mut ASN1Type<'a>, E) -> Result<GeneratedSymbols<'a>, LinkerError>,
        {
            match e {
                ElementOrSetOperation::Element(elem) => apply_to_subtype_elem(elem, f),
                ElementOrSetOperation::SetOperation(set_operation) => {
                    apply_to_subtype_elem(&mut set_operation.base, f)?;
                    apply_to_elem_or_set_op(&mut set_operation.operant, f)
                }
            }
        }

        fn apply_to_constraints<'a, F, E>(
            constraints: &mut Vec<Constraint<'a>>,
            f: &mut F,
        ) -> Result<GeneratedSymbols<'a>, LinkerError>
        where
            F: FnMut(&mut ASN1Type<'a>, E) -> Result<GeneratedSymbols<'a>, LinkerError>,
        {
            constraints
                .iter_mut()
                .map(|c| match c {
                    Constraint::Subtype(ElementSetSpecs { set, .. }) => {
                        apply_to_elem_or_set_op(set, f)
                    }
                    _ => Ok(GeneratedSymbols::empty()),
                })
                .collect()
        }

        fn apply_to_members<'a, I, F, E>(
            iter: &mut I,
            f: &mut F,
        ) -> Result<GeneratedSymbols<'a>, LinkerError>
        where
            I: IterMembers<'a>,
            F: FnMut(&mut ASN1Type<'a>, E) -> Result<GeneratedSymbols<'a>, LinkerError>,
        {
            iter.iter_mut_members()
                .map(|m| m.ty_mut().apply_resursively(f))
                .collect()
        }

        match self {
            ASN1Type::ChoiceSelectionType(_) |
            ASN1Type::EmbeddedPdv |
            ASN1Type::External |
            ASN1Type::Null => Ok(GeneratedSymbols::empty()),
            ASN1Type::Boolean(Boolean { constraints })
            | ASN1Type::Integer(Integer { constraints, .. })
            | ASN1Type::Time(Time { constraints, .. })
            | ASN1Type::GeneralizedTime(GeneralizedTime { constraints, .. })
            | ASN1Type::ObjectIdentifier(ObjectIdentifier { constraints, .. })
            | ASN1Type::UTCTime(UTCTime { constraints, .. })
            | ASN1Type::ObjectClassField(ObjectClassFieldType { constraints, .. })
            | ASN1Type::ElsewhereDeclaredType(DeclarationElsewhere { constraints, .. })
            | ASN1Type::Real(Real { constraints, .. })
            | ASN1Type::OctetString(OctetString { constraints, .. })
            | ASN1Type::CharacterString(CharacterString { constraints, .. })
            | ASN1Type::Enumerated(Enumerated { constraints, .. })
            | ASN1Type::BitString(BitString { constraints, .. }) => {
                apply_to_constraints(constraints, f)
            }
            ASN1Type::Choice(choice) => {
                let mut generated = apply_to_constraints(&mut choice.constraints, f)?;
                let gen_from_members = apply_to_members(choice, f)?;
                generated += gen_from_members;
                Ok(generated)
            }
            ASN1Type::Sequence(sequence_or_set) | ASN1Type::Set(sequence_or_set) => {
                let mut generated = apply_to_constraints(&mut sequence_or_set.constraints, f)?;
                let gen_from_members = apply_to_members(sequence_or_set, f)?;
                generated += gen_from_members;
                Ok(generated)
            }
            ASN1Type::SequenceOf(sequence_or_set_of) | ASN1Type::SetOf(sequence_or_set_of) => {
                let mut generated = apply_to_constraints(&mut sequence_or_set_of.constraints, f)?;
                let gen_from_items = sequence_or_set_of.element_type.apply_resursively(f)?;
                generated += gen_from_items;
                Ok(generated)
            }
        }
    }
}
