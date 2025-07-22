use crate::intermediate::{
    constraints::{Constraint, ElementOrSetOperation, SetOperation, SetOperator, SubtypeElements},
    error::{GrammarError, GrammarErrorType},
    types::{Choice, Enumerated},
    ASN1Type, ASN1Value, CharacterStringType,
};
use std::{borrow::Cow, collections::BTreeMap, ops::AddAssign};

pub fn to_per_visible(
    constraints: Vec<Constraint>,
    character_string_type: Option<CharacterStringType>,
) -> Result<
    (
        PerVisibleRangeConstraints,
        Option<PerVisibleAlphabetConstraints>,
    ),
    GrammarError,
> {
    Ok((
        constraints
            .iter()
            .try_fold(PerVisibleRangeConstraints::default(), |mut acc, curr| {
                let constraints = curr.try_into()?;
                acc += constraints;
                Ok(acc)
            })?,
        character_string_type
            .map(|c| {
                constraints.iter().try_fold(
                    PerVisibleAlphabetConstraints::default_for(c),
                    |mut acc, curr| {
                        if let Some(mut constraints) =
                            PerVisibleAlphabetConstraints::try_new(curr, c)?
                        {
                            acc += &mut constraints;
                        }
                        Ok(acc)
                    },
                )
            })
            .transpose()?,
    ))
}

trait PerVisible {
    fn per_visible(&self) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum CharsetSubset {
    Single(char),
    Range {
        from: Option<char>,
        to: Option<char>,
    },
}

#[derive(Debug, PartialEq)]
pub struct PerVisibleAlphabetConstraints {
    string_type: CharacterStringType,
    character_by_index: BTreeMap<usize, char>,
    index_by_character: Option<BTreeMap<char, usize>>,
    charset_subsets: Vec<CharsetSubset>,
}

impl PerVisibleAlphabetConstraints {
    pub fn try_new(
        constraint: &Constraint,
        string_type: CharacterStringType,
    ) -> Result<Option<Self>, GrammarError> {
        match constraint {
            Constraint::Subtype(c) => match &c.set {
                ElementOrSetOperation::Element(e) => Self::from_subtype_elem(Some(e), string_type),
                ElementOrSetOperation::SetOperation(s) => Self::from_subtype_elem(
                    fold_constraint_set(s, Some(&string_type.character_set()))?.as_ref(),
                    string_type,
                ),
            },
            _ => Ok(None),
        }
    }

    fn from_subtype_elem<'a>(
        element: Option<&SubtypeElements<'a>>,
        string_type: CharacterStringType,
    ) -> Result<Option<Self>, GrammarError> {
        match element {
            None => Ok(None),
            Some(SubtypeElements::PermittedAlphabet(elem_or_set)) => {
                let mut result = PerVisibleAlphabetConstraints::default_for(string_type);
                match &**elem_or_set {
                    ElementOrSetOperation::Element(e) => {
                        if let Some(mut p) = Self::from_subtype_elem(Some(e), string_type)? {
                            result += &mut p;
                        }
                    }
                    ElementOrSetOperation::SetOperation(s) => {
                        fn flatten_set<'a>(
                            elems: &mut Vec<SubtypeElements<'a>>,
                            set: &SetOperation<'a>,
                        ) {
                            elems.push(set.base.clone());
                            match &*set.operant {
                                ElementOrSetOperation::Element(e2) => elems.push(e2.clone()),
                                ElementOrSetOperation::SetOperation(inner) => {
                                    flatten_set(elems, inner)
                                }
                            }
                        }
                        let mut elems = Vec::new();
                        flatten_set(&mut elems, s);
                        for elem in elems {
                            if let Some(mut p) = Self::from_subtype_elem(Some(&elem), string_type)?
                            {
                                result += &mut p;
                            }
                        }
                    }
                }
                Ok(Some(result))
            }
            Some(SubtypeElements::SingleValue { value, extensible }) => match (value, extensible) {
                (ASN1Value::String(s), false) => {
                    let mut char_subset = s
                        .clone()
                        .chars()
                        .map(|c| find_char_index(&string_type.character_set(), c).map(|i| (i, c)))
                        .collect::<Result<Vec<(usize, char)>, _>>()?;
                    char_subset.sort_by(|(a, _), (b, _)| a.cmp(b));
                    Ok(Some(PerVisibleAlphabetConstraints {
                        string_type,
                        character_by_index: char_subset
                            .iter()
                            .map(|(_, c)| *c)
                            .enumerate()
                            .collect(),
                        index_by_character: None,
                        charset_subsets: s.chars().map(CharsetSubset::Single).collect(),
                    }))
                }
                _ => Ok(None),
            },
            Some(SubtypeElements::ValueRange {
                min,
                max,
                extensible,
            }) => {
                let char_set = string_type.character_set();
                if *extensible {
                    return Ok(None);
                }
                let (lower, upper) = match (min, max) {
                    (Some(ASN1Value::String(min)), Some(ASN1Value::String(max))) => (
                        find_string_index(min, &char_set)?,
                        find_string_index(max, &char_set)?,
                    ),
                    (None, Some(ASN1Value::String(max))) => (0, find_string_index(max, &char_set)?),
                    (Some(ASN1Value::String(min)), None) => {
                        (find_string_index(min, &char_set)?, char_set.len() - 1)
                    }
                    _ => (0, char_set.len() - 1),
                };
                if lower > upper {
                    return Err(GrammarError::new(&format!("Invalid range for permitted alphabet: Charset {char_set:?}; Range: {lower}..={upper}"), GrammarErrorType::UnpackingError
                    ));
                }
                Ok(Some(PerVisibleAlphabetConstraints {
                    string_type,
                    character_by_index: char_set
                        .iter()
                        .filter_map(|(i, c)| (lower..=upper).contains(i).then_some(*c))
                        .enumerate()
                        .collect(),
                    index_by_character: None,
                    charset_subsets: vec![CharsetSubset::Range {
                        from: char_set.get(&lower).copied(),
                        to: char_set.get(&upper).copied(),
                    }],
                }))
            }
            Some(SubtypeElements::ContainedSubtype {
                subtype,
                extensible: _,
            }) => {
                if let ASN1Type::CharacterString(c_string) = subtype {
                    let mut permitted_alphabet =
                        PerVisibleAlphabetConstraints::default_for(string_type);
                    for c in &c_string.constraints {
                        if let Some(mut p) = PerVisibleAlphabetConstraints::try_new(c, c_string.ty)?
                        {
                            permitted_alphabet += &mut p
                        }
                    }
                    Ok(Some(permitted_alphabet))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None),
        }
    }

    pub fn charset_subsets(&self) -> &Vec<CharsetSubset> {
        &self.charset_subsets
    }

    pub fn finalize(&mut self) {
        // sort charset_subsets by starting character to ensure consistent order
        self.charset_subsets.sort_by_key(|subset| match subset {
            CharsetSubset::Single(c) => *c,
            CharsetSubset::Range { from, .. } => from.unwrap_or('\0'),
        });
        self.index_by_character = Some(
            self.character_by_index
                .iter()
                .map(|(i, c)| (*c, *i))
                .collect(),
        );
    }

    pub fn default_for(string_type: CharacterStringType) -> Self {
        Self {
            character_by_index: BTreeMap::new(),
            string_type,
            index_by_character: None,
            charset_subsets: vec![],
        }
    }
}

fn find_string_index(value: &str, char_set: &BTreeMap<usize, char>) -> Result<usize, GrammarError> {
    let as_char = value.chars().next().unwrap();
    find_char_index(char_set, as_char)
}

fn find_char_index(char_set: &BTreeMap<usize, char>, as_char: char) -> Result<usize, GrammarError> {
    char_set
        .iter()
        .find_map(|(i, c)| (as_char == *c).then_some(*i))
        .ok_or(GrammarError::new(
            &format!("Character {as_char} is not in char set: {char_set:?}"),
            GrammarErrorType::UnpackingError,
        ))
}

impl AddAssign<&mut PerVisibleAlphabetConstraints> for PerVisibleAlphabetConstraints {
    fn add_assign(&mut self, rhs: &mut PerVisibleAlphabetConstraints) {
        // merge character index mappings
        self.character_by_index.append(&mut rhs.character_by_index);
        // merge charset subsets (ranges and singletons)
        self.charset_subsets.append(&mut rhs.charset_subsets);
    }
}

#[derive(Default)]
pub struct PerVisibleRangeConstraints {
    min: Option<i128>,
    max: Option<i128>,
    extensible: bool,
    is_size_constraint: bool,
}

impl PerVisibleRangeConstraints {
    pub fn default_unsigned() -> Self {
        Self {
            min: Some(0),
            max: None,
            extensible: false,
            is_size_constraint: false,
        }
    }

    pub fn is_extensible(&self) -> bool {
        self.extensible
    }

    pub fn min<I: num::Integer + num::FromPrimitive>(&self) -> Option<I> {
        self.min.and_then(|m| I::from_i128(m))
    }

    pub fn max<I: num::Integer + num::FromPrimitive>(&self) -> Option<I> {
        self.max.and_then(|m| I::from_i128(m))
    }

    pub fn is_size_constraint(&self) -> bool {
        self.is_size_constraint
    }
}

impl<'a> From<&Enumerated<'a>> for PerVisibleRangeConstraints {
    fn from(value: &Enumerated<'a>) -> Self {
        PerVisibleRangeConstraints {
            min: Some(0),
            max: Some(value.extensible.map_or(value.members.len() - 1, |i| i - 1) as i128),
            extensible: value.extensible.is_some(),
            is_size_constraint: false,
        }
    }
}

impl<'a> From<&Choice<'a>> for PerVisibleRangeConstraints {
    fn from(value: &Choice<'a>) -> Self {
        PerVisibleRangeConstraints {
            min: Some(0),
            max: Some(value.extensible.map_or(value.options.len() - 1, |i| i - 1) as i128),
            extensible: value.extensible.is_some(),
            is_size_constraint: false,
        }
    }
}

impl AddAssign<PerVisibleRangeConstraints> for PerVisibleRangeConstraints {
    fn add_assign(&mut self, rhs: PerVisibleRangeConstraints) {
        self.min = self.min.max(rhs.min);
        self.max = match (self.max, rhs.max) {
            (Some(m1), Some(m2)) => Some(m1.min(m2)),
            (None, Some(m)) | (Some(m), None) => Some(m),
            _ => None,
        };
        self.extensible = self.extensible || rhs.extensible;
        self.is_size_constraint = self.is_size_constraint || rhs.is_size_constraint;
    }
}

impl<'a> TryFrom<&Constraint<'a>> for PerVisibleRangeConstraints {
    type Error = GrammarError;

    fn try_from(value: &Constraint<'a>) -> Result<PerVisibleRangeConstraints, Self::Error> {
        match value {
            Constraint::Subtype(c) => {
                let mut per_visible: PerVisibleRangeConstraints = match &c.set {
                    ElementOrSetOperation::Element(e) => Some(e).try_into(),
                    ElementOrSetOperation::SetOperation(s) => {
                        fold_constraint_set(s, None)?.as_ref().try_into()
                    }
                }?;
                if let (PerVisibleRangeConstraints { min, max, .. }, true) =
                    (&mut per_visible, c.extensible)
                {
                    if min.or(*max).is_some() {
                        per_visible.extensible = true;
                    }
                }
                Ok(per_visible)
            }
            _ => Ok(Self::default()),
        }
    }
}

impl<'a> TryFrom<Option<&SubtypeElements<'a>>> for PerVisibleRangeConstraints {
    type Error = GrammarError;
    fn try_from(
        value: Option<&SubtypeElements>,
    ) -> Result<PerVisibleRangeConstraints, Self::Error> {
        match value {
            Some(SubtypeElements::PermittedAlphabet(_)) | None => Ok(Self::default()),
            Some(SubtypeElements::SingleValue { value, extensible }) => {
                let val = value.unwrap_as_integer().ok();
                Ok(Self {
                    min: val,
                    max: val,
                    extensible: *extensible,
                    is_size_constraint: false,
                })
            }
            Some(SubtypeElements::ValueRange {
                min,
                max,
                extensible,
            }) => Ok(Self {
                min: min.as_ref().and_then(|i| i.unwrap_as_integer().ok()),
                max: max.as_ref().and_then(|i| i.unwrap_as_integer().ok()),
                extensible: *extensible,
                is_size_constraint: false,
            }),
            Some(SubtypeElements::SizeConstraint(s)) => match &**s {
                ElementOrSetOperation::Element(e) => <Option<&SubtypeElements> as TryInto<
                    PerVisibleRangeConstraints,
                >>::try_into(Some(e))
                .map(|mut c| {
                    c.is_size_constraint = true;
                    c
                }),
                ElementOrSetOperation::SetOperation(s) => {
                    <Option<&SubtypeElements> as TryInto<PerVisibleRangeConstraints>>::try_into(
                        fold_constraint_set(s, None)?.as_ref(),
                    )
                    .map(|mut c| {
                        c.is_size_constraint = true;
                        c
                    })
                }
            },
            Some(SubtypeElements::ContainedSubtype {
                subtype,
                extensible: _,
            }) => per_visible_range_constraints(
                matches!(subtype, ASN1Type::Integer(_)),
                subtype.constraints().unwrap_or(&vec![]),
            ),
            x => {
                println!("{x:?}");
                unreachable!()
            }
        }
    }
}

impl<'a> PerVisible for Constraint<'a> {
    fn per_visible(&self) -> bool {
        match self {
            Constraint::Subtype(s) => s.set.per_visible(),
            _ => false,
        }
    }
}

impl<'a> PerVisible for ElementOrSetOperation<'a> {
    fn per_visible(&self) -> bool {
        match self {
            ElementOrSetOperation::Element(e) => e.per_visible(),
            ElementOrSetOperation::SetOperation(o) => {
                o.operant.per_visible() || o.operant.per_visible()
            }
        }
    }
}

impl<'a> PerVisible for SubtypeElements<'a> {
    fn per_visible(&self) -> bool {
        match self {
            SubtypeElements::SingleValue {
                value: _,
                extensible: _,
            } => true,
            SubtypeElements::ContainedSubtype {
                subtype: s,
                extensible: _,
            } => s
                .constraints()
                .is_some_and(|c| c.iter().any(|c| c.per_visible())),
            SubtypeElements::ValueRange {
                min: _,
                max: _,
                extensible: _,
            } => true,
            SubtypeElements::PermittedAlphabet(p) => p.per_visible(),
            SubtypeElements::SizeConstraint(s) => s.per_visible(),
            _ => false,
        }
    }
}

pub fn per_visible_range_constraints(
    signed: bool,
    constraint_list: &[Constraint],
) -> Result<PerVisibleRangeConstraints, GrammarError> {
    let mut constraints = if signed {
        PerVisibleRangeConstraints::default()
    } else {
        PerVisibleRangeConstraints::default_unsigned()
    };
    for c in constraint_list.iter().filter(|c| c.per_visible()) {
        constraints += c.try_into()?
    }
    Ok(constraints)
}

/// 10.3.21 If a constraint that is PER-visible is part of an INTERSECTION construction,
/// then the resulting constraint is PER-visible, and consists of the INTERSECTION of
/// all PER-visible parts (with the non-PER-visible parts ignored).
/// If a constraint which is not PER-visible is part of a UNION construction,
/// then the resulting constraint is not PER-visible.
/// If a constraint has an EXCEPT clause, the EXCEPT and the following value set is completely ignored,
/// whether the value set following the EXCEPT is PER-visible or not.
fn fold_constraint_set<'a>(
    set: &SetOperation<'a>,
    char_set: Option<&'a BTreeMap<usize, char>>,
) -> Result<Option<SubtypeElements<'a>>, GrammarError> {
    let folded_operant = match &*set.operant {
        ElementOrSetOperation::Element(e) => e.per_visible().then(|| e.clone()),
        ElementOrSetOperation::SetOperation(s) => fold_constraint_set(s, char_set)?,
    };
    match (&set.base, &folded_operant) {
        (base, Some(SubtypeElements::PermittedAlphabet(elem_or_set)))
        | (SubtypeElements::PermittedAlphabet(elem_or_set), Some(base))
        | (base, Some(SubtypeElements::SizeConstraint(elem_or_set)))
        | (SubtypeElements::SizeConstraint(elem_or_set), Some(base)) => {
            return fold_constraint_set(
                &SetOperation {
                    base: base.clone(),
                    operator: set.operator.clone(),
                    operant: elem_or_set.clone(),
                },
                char_set,
            )
        }
        (
            SubtypeElements::ContainedSubtype {
                subtype: _,
                extensible: _,
            },
            None,
        )
        | (
            SubtypeElements::ContainedSubtype {
                subtype: _,
                extensible: _,
            },
            Some(SubtypeElements::ContainedSubtype {
                subtype: _,
                extensible: _,
            }),
        ) => return Ok(None),
        (
            SubtypeElements::ContainedSubtype {
                subtype: _,
                extensible: _,
            },
            Some(c),
        )
        | (
            c,
            Some(SubtypeElements::ContainedSubtype {
                subtype: _,
                extensible: _,
            }),
        ) => return Ok(Some(c.clone())),
        (SubtypeElements::PermittedAlphabet(elem_or_set), None)
        | (SubtypeElements::SizeConstraint(elem_or_set), None) => {
            return match &**elem_or_set {
                ElementOrSetOperation::Element(e) => Ok(Some(e.clone())),
                ElementOrSetOperation::SetOperation(s) => fold_constraint_set(s, char_set),
            }
        }
        _ => (),
    }

    match set.operator {
        SetOperator::Intersection => match (&set.base, &folded_operant) {
            (b, _) if !b.per_visible() => Ok(None),
            (b, None) => Ok(Some(b.clone())),
            (b, Some(f)) if !f.per_visible() => Ok(Some(b.clone())),
            (
                SubtypeElements::SingleValue {
                    value: v1,
                    extensible: x1,
                },
                Some(SubtypeElements::SingleValue {
                    value: v2,
                    extensible: x2,
                }),
            ) => match (v1, v2, char_set.is_some()) {
                (ASN1Value::Integer(_), ASN1Value::String(_), false)
                | (ASN1Value::String(_), ASN1Value::Integer(_), true) => Ok(Some(set.base.clone())),
                (ASN1Value::String(_), ASN1Value::Integer(_), false)
                | (ASN1Value::Integer(_), ASN1Value::String(_), true) => Ok(folded_operant),
                (ASN1Value::Integer(i1), ASN1Value::Integer(i2), _) => {
                    if *i1 != *i2 {
                        Err(GrammarError::new(
                            &format!(
                                "Empty intersection result for {:?} and {:?}",
                                v1,
                                ASN1Value::Integer(*i2)
                            ),
                            GrammarErrorType::UnpackingError,
                        ))
                    } else {
                        Ok(Some(SubtypeElements::SingleValue {
                            value: ASN1Value::Integer(*i2),
                            extensible: *x1 || *x2,
                        }))
                    }
                }
                (ASN1Value::String(s1), ASN1Value::String(s2), _) => {
                    if *x1 || *x2 {
                        Ok(None)
                    } else {
                        let permitted: String = s2.chars().filter(|c| s1.contains(*c)).collect();
                        if permitted.is_empty() {
                            return Err(GrammarError::new(
                                &format!(
                                    "Empty intersection result for {:?} and {:?}",
                                    v1,
                                    ASN1Value::String(s2.clone())
                                ),
                                GrammarErrorType::UnpackingError,
                            ));
                        }
                        Ok(Some(SubtypeElements::SingleValue {
                            value: ASN1Value::String(Cow::Owned(permitted)),
                            extensible: false,
                        }))
                    }
                }
                (v1, v2, _) => Err(GrammarError::new(
                    &format!("Unsupported operation for ASN1Values {v1:?} and {v2:?}"),
                    GrammarErrorType::UnpackingError,
                )),
            },
            (
                SubtypeElements::SingleValue {
                    value,
                    extensible: x1,
                },
                Some(SubtypeElements::ValueRange {
                    min,
                    max,
                    extensible: x2,
                }),
            ) => intersect_single_and_range(value, min.as_ref(), max.as_ref(), *x1, *x2, char_set),
            (
                SubtypeElements::ValueRange {
                    min,
                    max,
                    extensible: x2,
                },
                Some(SubtypeElements::SingleValue {
                    value,
                    extensible: x1,
                }),
            ) => intersect_single_and_range(value, min.as_ref(), max.as_ref(), *x1, *x2, char_set),
            (
                _,
                Some(SubtypeElements::SingleValue {
                    value: v,
                    extensible: x,
                }),
            ) => Ok(Some(SubtypeElements::SingleValue {
                value: v.clone(),
                extensible: *x,
            })),
            (
                SubtypeElements::ValueRange {
                    min: min1,
                    max: max1,
                    extensible: x1,
                },
                Some(SubtypeElements::ValueRange {
                    min: min2,
                    max: max2,
                    extensible: x2,
                }),
            ) => {
                match (min1, max1, &min2, &max2) {
                    (Some(ASN1Value::Integer(_)), _, Some(ASN1Value::String(_)), _)
                    | (_, Some(ASN1Value::Integer(_)), Some(ASN1Value::String(_)), _)
                    | (Some(ASN1Value::Integer(_)), _, _, Some(ASN1Value::String(_)))
                    | (_, Some(ASN1Value::Integer(_)), _, Some(ASN1Value::String(_))) => {
                        return if char_set.is_none() {
                            Ok(Some(set.base.clone()))
                        } else if !x2 {
                            Ok(folded_operant.clone())
                        } else {
                            Ok(None)
                        }
                    }
                    (Some(ASN1Value::String(_)), _, Some(ASN1Value::Integer(_)), _)
                    | (_, Some(ASN1Value::String(_)), Some(ASN1Value::Integer(_)), _)
                    | (Some(ASN1Value::String(_)), _, _, Some(ASN1Value::Integer(_)))
                    | (_, Some(ASN1Value::String(_)), _, Some(ASN1Value::Integer(_))) => {
                        return if char_set.is_none() {
                            Ok(folded_operant)
                        } else if !x1 {
                            Ok(Some(set.base.clone()))
                        } else {
                            Ok(None)
                        }
                    }
                    _ => (),
                };
                let min = compare_optional_asn1values(min1.as_ref(), min2.as_ref(), |m1, m2| {
                    m1.max(m2, char_set)
                })?;
                let max = compare_optional_asn1values(max1.as_ref(), max2.as_ref(), |m1, m2| {
                    m1.min(m2, char_set)
                })?;
                Ok(Some(SubtypeElements::ValueRange {
                    min,
                    max,
                    extensible: *x1 || *x2,
                }))
            }
            _ => unreachable!(),
        },
        SetOperator::Union => match (&set.base, folded_operant) {
            (b, _) if !b.per_visible() => Ok(None),
            (_, None) => Ok(None),
            (_, Some(f)) if !f.per_visible() => Ok(None),
            (
                SubtypeElements::SingleValue {
                    value: v1,
                    extensible: x1,
                },
                Some(SubtypeElements::SingleValue {
                    value: v2,
                    extensible: x2,
                }),
            ) => match (v1, &v2) {
                (ASN1Value::String(_), ASN1Value::Integer(_))
                | (ASN1Value::Integer(_), ASN1Value::String(_)) => Ok(None),
                (ASN1Value::Integer(v1_int), ASN1Value::Integer(v2_int)) => {
                    Ok(Some(SubtypeElements::ValueRange {
                        min: Some(ASN1Value::Integer(*v2_int.min(v1_int))),
                        max: Some(ASN1Value::Integer(*v2_int.max(v1_int))),
                        extensible: *x1 || x2,
                    }))
                }
                (ASN1Value::String(v1_str), ASN1Value::String(v2_str)) => {
                    let mut v2_clone = (*v2_str).to_string();
                    v2_clone.extend(v1_str.chars().filter(|c| !v2_str.contains(*c)));
                    Ok(Some(SubtypeElements::SingleValue {
                        value: ASN1Value::String(Cow::Owned(v2_clone)),
                        extensible: *x1 || x2,
                    }))
                }
                _ => Err(GrammarError::new(
                    &format!("Unsupported operation for ASN1Values {v1:?} and {v2:?}"),
                    GrammarErrorType::UnpackingError,
                )),
            },
            (
                SubtypeElements::ValueRange {
                    min,
                    max,
                    extensible: x1,
                },
                Some(SubtypeElements::SingleValue {
                    value: v,
                    extensible: x2,
                }),
            ) => union_single_and_range(&v, min.as_ref(), char_set, max.as_ref(), *x1, x2),
            (
                SubtypeElements::SingleValue {
                    value: v,
                    extensible: x1,
                },
                Some(SubtypeElements::ValueRange {
                    min,
                    max,
                    extensible: x2,
                }),
            ) => union_single_and_range(v, min.as_ref(), char_set, max.as_ref(), *x1, x2),
            (
                SubtypeElements::ValueRange {
                    min: min1,
                    max: max1,
                    extensible: x1,
                },
                Some(SubtypeElements::ValueRange {
                    min: min2,
                    max: max2,
                    extensible: x2,
                }),
            ) => {
                match (min1, max1, &min2, &max2) {
                    (Some(ASN1Value::Integer(_)), _, Some(ASN1Value::String(_)), _)
                    | (Some(ASN1Value::String(_)), _, Some(ASN1Value::Integer(_)), _)
                    | (_, Some(ASN1Value::Integer(_)), Some(ASN1Value::String(_)), _)
                    | (_, Some(ASN1Value::String(_)), Some(ASN1Value::Integer(_)), _)
                    | (Some(ASN1Value::Integer(_)), _, _, Some(ASN1Value::String(_)))
                    | (Some(ASN1Value::String(_)), _, _, Some(ASN1Value::Integer(_)))
                    | (_, Some(ASN1Value::Integer(_)), _, Some(ASN1Value::String(_)))
                    | (_, Some(ASN1Value::String(_)), _, Some(ASN1Value::Integer(_))) => {
                        return Ok(None)
                    }
                    _ => (),
                };
                let min = compare_optional_asn1values(min1.as_ref(), min2.as_ref(), |m1, m2| {
                    m1.min(m2, char_set)
                })?;
                let max = compare_optional_asn1values(max1.as_ref(), max2.as_ref(), |m1, m2| {
                    m1.max(m2, char_set)
                })?;
                Ok(Some(SubtypeElements::ValueRange {
                    min,
                    max,
                    extensible: *x1 || x2,
                }))
            }
            _ => unreachable!(),
        },
        SetOperator::Except => {
            if set.base.per_visible() {
                Ok(Some(set.base.clone()))
            } else {
                Ok(None)
            }
        }
    }
}

fn intersect_single_and_range<'a>(
    value: &ASN1Value<'a>,
    min: Option<&ASN1Value<'a>>,
    max: Option<&ASN1Value<'a>>,
    x1: bool,
    x2: bool,
    char_set: Option<&BTreeMap<usize, char>>,
) -> Result<Option<SubtypeElements<'a>>, GrammarError> {
    match (value, min, max, x1 || x2, char_set) {
        (ASN1Value::Integer(_), _, Some(ASN1Value::String(_)), _, Some(_))
        | (ASN1Value::Integer(_), Some(ASN1Value::String(_)), _, _, Some(_)) => {
            if x2 {
                Ok(None)
            } else {
                Ok(Some(SubtypeElements::ValueRange {
                    min: min.cloned(),
                    max: max.cloned(),
                    extensible: false,
                }))
            }
        }
        (ASN1Value::String(_), Some(ASN1Value::Integer(_)), _, _, Some(_))
        | (ASN1Value::String(_), _, Some(ASN1Value::Integer(_)), _, Some(_)) => {
            if x1 {
                Ok(None)
            } else {
                Ok(Some(SubtypeElements::SingleValue {
                    value: value.clone(),
                    extensible: false,
                }))
            }
        }
        (ASN1Value::Integer(_), _, Some(ASN1Value::String(_)), _, None)
        | (ASN1Value::Integer(_), Some(ASN1Value::String(_)), _, _, None) => {
            Ok(Some(SubtypeElements::SingleValue {
                value: value.clone(),
                extensible: x1,
            }))
        }
        (ASN1Value::String(_), Some(ASN1Value::Integer(_)), _, _, None)
        | (ASN1Value::String(_), _, Some(ASN1Value::Integer(_)), _, None) => {
            Ok(Some(SubtypeElements::ValueRange {
                min: min.cloned(),
                max: max.cloned(),
                extensible: x2,
            }))
        }
        (ASN1Value::Integer(v), _, _, extensible, _) => Ok(Some(SubtypeElements::SingleValue {
            value: ASN1Value::Integer(*v),
            extensible,
        })),
        (_, _, _, true, _) => Ok(None),
        (ASN1Value::String(s1), _, _, _, Some(chars)) => {
            let indices = s1
                .chars()
                .map(|c| find_char_index(chars, c).map(|i| (c, i)))
                .collect::<Result<Vec<(char, usize)>, _>>()?;
            let s_min = indices
                .iter()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(c, _)| ASN1Value::String(Cow::Owned(c.to_string())));
            let s_max = indices
                .iter()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(c, _)| ASN1Value::String(Cow::Owned(c.to_string())));
            Ok(Some(SubtypeElements::ValueRange {
                min: compare_optional_asn1values(s_min.as_ref(), min, |a, b| a.max(b, char_set))?,
                max: compare_optional_asn1values(s_max.as_ref(), max, |a, b| a.min(b, char_set))?,
                extensible: false,
            }))
        }
        _ => Err(GrammarError::new(
            &format!("Unsupported operation for ASN1Values {value:?} and {min:?}..{max:?}"),
            GrammarErrorType::UnpackingError,
        )),
    }
}

fn union_single_and_range<'a>(
    v: &ASN1Value<'a>,
    min: Option<&ASN1Value<'a>>,
    char_set: Option<&'a BTreeMap<usize, char>>,
    max: Option<&ASN1Value<'a>>,
    x1: bool,
    x2: bool,
) -> Result<Option<SubtypeElements<'a>>, GrammarError> {
    match (v, min, max, x1 || x2, char_set) {
        (ASN1Value::Integer(_), _, Some(ASN1Value::String(_)), _, _)
        | (ASN1Value::Integer(_), Some(ASN1Value::String(_)), _, _, _)
        | (ASN1Value::String(_), Some(ASN1Value::Integer(_)), _, _, _)
        | (ASN1Value::String(_), _, Some(ASN1Value::Integer(_)), _, _) => Ok(None),
        (ASN1Value::Integer(_), _, _, extensible, _) => Ok(Some(SubtypeElements::ValueRange {
            min: compare_optional_asn1values(Some(v), min, |a, b| a.min(b, char_set))?,
            max: compare_optional_asn1values(Some(v), max, |a, b| a.max(b, char_set))?,
            extensible,
        })),
        (_, _, _, true, _) => Ok(None),
        (ASN1Value::String(s1), _, _, _, Some(chars)) => {
            let indices = s1
                .chars()
                .map(|c| find_char_index(chars, c).map(|i| (c, i)))
                .collect::<Result<Vec<(char, usize)>, _>>()?;
            let s_min = indices
                .iter()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(c, _)| ASN1Value::String(Cow::Owned(c.to_string())));
            let s_max = indices
                .iter()
                .max_by(|(_, a), (_, b)| a.cmp(b))
                .map(|(c, _)| ASN1Value::String(Cow::Owned(c.to_string())));
            Ok(Some(SubtypeElements::ValueRange {
                min: compare_optional_asn1values(s_min.as_ref(), min, |a, b| a.min(b, char_set))?,
                max: compare_optional_asn1values(s_max.as_ref(), max, |a, b| a.max(b, char_set))?,
                extensible: false,
            }))
        }
        _ => Err(GrammarError::new(
            &format!("Unsupported operation for values {v:?} and {min:?}..{max:?}"),
            GrammarErrorType::UnpackingError,
        )),
    }
}

fn compare_optional_asn1values<'a>(
    first: Option<&ASN1Value<'a>>,
    second: Option<&ASN1Value<'a>>,
    predicate: impl Fn(&ASN1Value<'a>, &ASN1Value<'a>) -> Result<ASN1Value<'a>, GrammarError>,
) -> Result<Option<ASN1Value<'a>>, GrammarError> {
    match (first, second) {
        (Some(f), Some(s)) => Ok(Some(predicate(f, s)?)),
        (None, Some(s)) => Ok(Some(s.clone())),
        (Some(f), None) => Ok(Some(f.clone())),
        _ => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use crate::intermediate::{constraints::*, *};

    use super::*;

    #[test]
    fn initializes_per_visible_alphabet_from_single_value() {
        assert_eq!(
            PerVisibleAlphabetConstraints::try_new(
                &Constraint::Subtype(ElementSetSpecs {
                    extensible: false,
                    set: ElementOrSetOperation::Element(SubtypeElements::SingleValue {
                        value: ASN1Value::String("ABCDEF".into()),
                        extensible: false
                    })
                }),
                CharacterStringType::UTF8String
            )
            .unwrap()
            .unwrap(),
            PerVisibleAlphabetConstraints {
                string_type: CharacterStringType::UTF8String,
                character_by_index: [(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D'), (4, 'E'), (5, 'F')]
                    .into_iter()
                    .collect(),
                index_by_character: None,
                charset_subsets: vec![
                    CharsetSubset::Single('A'),
                    CharsetSubset::Single('B'),
                    CharsetSubset::Single('C'),
                    CharsetSubset::Single('D'),
                    CharsetSubset::Single('E'),
                    CharsetSubset::Single('F')
                ]
            }
        );
        assert_eq!(
            PerVisibleAlphabetConstraints::try_new(
                &Constraint::Subtype(ElementSetSpecs {
                    extensible: false,
                    set: ElementOrSetOperation::Element(SubtypeElements::SingleValue {
                        value: ASN1Value::String("132".into()),
                        extensible: false
                    })
                }),
                CharacterStringType::NumericString
            )
            .unwrap()
            .unwrap(),
            PerVisibleAlphabetConstraints {
                string_type: CharacterStringType::NumericString,
                character_by_index: [(0, '1'), (2, '3'), (1, '2')].into_iter().collect(),
                index_by_character: None,
                charset_subsets: vec![
                    CharsetSubset::Single('1'),
                    CharsetSubset::Single('3'),
                    CharsetSubset::Single('2')
                ]
            }
        )
    }

    #[test]
    fn initializes_per_visible_alphabet_from_range_value() {
        assert_eq!(
            PerVisibleAlphabetConstraints::try_new(
                &Constraint::Subtype(ElementSetSpecs {
                    extensible: false,
                    set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                        min: Some(ASN1Value::String("A".into())),
                        max: Some(ASN1Value::String("F".into())),
                        extensible: false
                    })
                }),
                CharacterStringType::UTF8String
            )
            .unwrap()
            .unwrap(),
            PerVisibleAlphabetConstraints {
                string_type: CharacterStringType::UTF8String,
                character_by_index: [(0, 'A'), (1, 'B'), (2, 'C'), (3, 'D'), (4, 'E'), (5, 'F')]
                    .into_iter()
                    .collect(),
                index_by_character: None,
                charset_subsets: vec![CharsetSubset::Range {
                    from: Some('A'),
                    to: Some('F')
                }]
            }
        );
        assert_eq!(
            PerVisibleAlphabetConstraints::try_new(
                &Constraint::Subtype(ElementSetSpecs {
                    extensible: false,
                    set: ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                        min: None,
                        max: Some(ASN1Value::String("3".into())),
                        extensible: false
                    })
                }),
                CharacterStringType::NumericString
            )
            .unwrap()
            .unwrap(),
            PerVisibleAlphabetConstraints {
                string_type: CharacterStringType::NumericString,
                character_by_index: [(0, ' '), (1, '0'), (2, '1'), (3, '2'), (4, '3')]
                    .into_iter()
                    .collect(),
                index_by_character: None,
                charset_subsets: vec![CharsetSubset::Range {
                    from: Some(' '),
                    to: Some('3')
                }]
            }
        )
    }

    #[test]
    fn folds_single_value_alphabet_constraints() {
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::SingleValue {
                        value: ASN1Value::String("ABC".into()),
                        extensible: false
                    },
                    operator: SetOperator::Intersection,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleValue {
                            value: ASN1Value::String("CDE".into()),
                            extensible: false
                        }
                    ))
                },
                Some(&CharacterStringType::IA5String.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::SingleValue {
                value: ASN1Value::String("C".into()),
                extensible: false
            }
        );
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::SingleValue {
                        value: ASN1Value::String("ABC".into()),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleValue {
                            value: ASN1Value::String("CDE".into()),
                            extensible: false
                        }
                    ))
                },
                Some(&CharacterStringType::IA5String.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::SingleValue {
                value: ASN1Value::String("CDEAB".into()),
                extensible: false
            }
        )
    }

    #[test]
    fn folds_range_value_alphabet_constraints() {
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::String("A".into())),
                        max: Some(ASN1Value::String("C".into())),
                        extensible: false
                    },
                    operator: SetOperator::Intersection,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleValue {
                            value: ASN1Value::String("CDE".into()),
                            extensible: false
                        }
                    ))
                },
                Some(&CharacterStringType::PrintableString.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::String("C".into())),
                max: Some(ASN1Value::String("C".into())),
                extensible: false
            }
        );
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::String("A".into())),
                        max: Some(ASN1Value::String("C".into())),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleValue {
                            value: ASN1Value::String("CDE".into()),
                            extensible: false
                        }
                    ))
                },
                Some(&CharacterStringType::PrintableString.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::String("A".into())),
                max: Some(ASN1Value::String("E".into())),
                extensible: false
            }
        )
    }

    #[test]
    fn folds_range_values_alphabet_constraints() {
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::String("A".into())),
                        max: Some(ASN1Value::String("C".into())),
                        extensible: false
                    },
                    operator: SetOperator::Intersection,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::ValueRange {
                            min: Some(ASN1Value::String("C".into())),
                            max: Some(ASN1Value::String("E".into())),
                            extensible: false
                        }
                    ))
                },
                Some(&CharacterStringType::VisibleString.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::String("C".into())),
                max: Some(ASN1Value::String("C".into())),
                extensible: false
            }
        );
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::String("A".into())),
                        max: Some(ASN1Value::String("C".into())),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::ValueRange {
                            min: Some(ASN1Value::String("C".into())),
                            max: Some(ASN1Value::String("E".into())),
                            extensible: false
                        }
                    ))
                },
                Some(&CharacterStringType::PrintableString.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::String("A".into())),
                max: Some(ASN1Value::String("E".into())),
                extensible: false
            }
        )
    }

    #[test]
    fn folds_single_value_numeric_constraints() {
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::SingleValue {
                        value: ASN1Value::Integer(4),
                        extensible: false
                    },
                    operator: SetOperator::Intersection,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleValue {
                            value: ASN1Value::Integer(4),
                            extensible: true
                        }
                    ))
                },
                None
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::SingleValue {
                value: ASN1Value::Integer(4),
                extensible: true
            }
        );
    }

    #[test]
    fn folds_range_value_integer_constraints() {
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::Integer(-1)),
                        max: Some(ASN1Value::Integer(3)),
                        extensible: false
                    },
                    operator: SetOperator::Intersection,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleValue {
                            value: ASN1Value::Integer(2),
                            extensible: false
                        }
                    ))
                },
                None
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::SingleValue {
                value: ASN1Value::Integer(2),
                extensible: false
            }
        );
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::Integer(-1)),
                        max: Some(ASN1Value::Integer(5)),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::SingleValue {
                            value: ASN1Value::Integer(-3),
                            extensible: false
                        }
                    ))
                },
                None
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::Integer(-3)),
                max: Some(ASN1Value::Integer(5)),
                extensible: false
            }
        )
    }

    #[test]
    fn folds_range_values_numeric_constraints() {
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::Integer(-2)),
                        max: Some(ASN1Value::Integer(3)),
                        extensible: false
                    },
                    operator: SetOperator::Intersection,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::ValueRange {
                            min: Some(ASN1Value::Integer(-5)),
                            max: Some(ASN1Value::Integer(1)),
                            extensible: false
                        }
                    ))
                },
                None
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::Integer(-2)),
                max: Some(ASN1Value::Integer(1)),
                extensible: false
            }
        );
        assert_eq!(
            fold_constraint_set(
                &SetOperation {
                    base: SubtypeElements::ValueRange {
                        min: Some(ASN1Value::Integer(-2)),
                        max: Some(ASN1Value::Integer(3)),
                        extensible: false
                    },
                    operator: SetOperator::Union,
                    operant: Box::new(ElementOrSetOperation::Element(
                        SubtypeElements::ValueRange {
                            min: Some(ASN1Value::Integer(-1)),
                            max: Some(ASN1Value::Integer(5)),
                            extensible: false
                        }
                    ))
                },
                None
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::Integer(-2)),
                max: Some(ASN1Value::Integer(5)),
                extensible: false
            }
        )
    }

    #[test]
    fn folds_single_value_mixed_constraints() {
        let set_op = |op: SetOperator| SetOperation {
            base: SubtypeElements::SingleValue {
                value: ASN1Value::Integer(4),
                extensible: false,
            },
            operator: op,
            operant: Box::new(ElementOrSetOperation::Element(
                SubtypeElements::SingleValue {
                    value: ASN1Value::String("abc".into()),
                    extensible: false,
                },
            )),
        };
        assert_eq!(
            fold_constraint_set(&set_op(SetOperator::Intersection), None)
                .unwrap()
                .unwrap(),
            SubtypeElements::SingleValue {
                value: ASN1Value::Integer(4),
                extensible: false
            }
        );
        assert_eq!(
            fold_constraint_set(
                &set_op(SetOperator::Intersection),
                Some(&CharacterStringType::IA5String.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::SingleValue {
                value: ASN1Value::String("abc".into()),
                extensible: false
            }
        );
        assert_eq!(
            fold_constraint_set(&set_op(SetOperator::Union), None).unwrap(),
            None
        );
        assert_eq!(
            fold_constraint_set(
                &set_op(SetOperator::Union),
                Some(&CharacterStringType::IA5String.character_set())
            )
            .unwrap(),
            None
        );
    }

    #[test]
    fn folds_range_value_mixed_constraints() {
        let set_op = |op| SetOperation {
            base: SubtypeElements::ValueRange {
                min: Some(ASN1Value::Integer(-1)),
                max: Some(ASN1Value::Integer(3)),
                extensible: false,
            },
            operator: op,
            operant: Box::new(ElementOrSetOperation::Element(
                SubtypeElements::SingleValue {
                    value: ASN1Value::String("ABC".into()),
                    extensible: false,
                },
            )),
        };
        assert_eq!(
            fold_constraint_set(
                &set_op(SetOperator::Intersection),
                Some(&CharacterStringType::PrintableString.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::SingleValue {
                value: ASN1Value::String("ABC".into()),
                extensible: false,
            }
        );
        assert_eq!(
            fold_constraint_set(
                &set_op(SetOperator::Union),
                Some(&CharacterStringType::PrintableString.character_set())
            )
            .unwrap(),
            None
        );
        assert_eq!(
            fold_constraint_set(&set_op(SetOperator::Intersection), None)
                .unwrap()
                .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::Integer(-1)),
                max: Some(ASN1Value::Integer(3)),
                extensible: false,
            }
        );
        assert_eq!(
            fold_constraint_set(&set_op(SetOperator::Union), None).unwrap(),
            None
        );
    }

    #[test]
    fn folds_range_values_mixed_constraints() {
        let set_op = |op| SetOperation {
            base: SubtypeElements::ValueRange {
                min: Some(ASN1Value::Integer(-2)),
                max: Some(ASN1Value::Integer(3)),
                extensible: false,
            },
            operator: op,
            operant: Box::new(ElementOrSetOperation::Element(
                SubtypeElements::ValueRange {
                    min: Some(ASN1Value::String("A".into())),
                    max: Some(ASN1Value::String("C".into())),
                    extensible: false,
                },
            )),
        };
        assert_eq!(
            fold_constraint_set(
                &set_op(SetOperator::Intersection),
                Some(&CharacterStringType::PrintableString.character_set())
            )
            .unwrap()
            .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::String("A".into())),
                max: Some(ASN1Value::String("C".into())),
                extensible: false,
            }
        );
        assert_eq!(
            fold_constraint_set(&set_op(SetOperator::Intersection), None)
                .unwrap()
                .unwrap(),
            SubtypeElements::ValueRange {
                min: Some(ASN1Value::Integer(-2)),
                max: Some(ASN1Value::Integer(3)),
                extensible: false,
            }
        );
        assert_eq!(
            fold_constraint_set(
                &set_op(SetOperator::Union),
                Some(&CharacterStringType::PrintableString.character_set())
            )
            .unwrap(),
            None
        );
        assert_eq!(
            fold_constraint_set(&set_op(SetOperator::Union), None).unwrap(),
            None
        );
    }
}
