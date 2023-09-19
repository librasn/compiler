use std::{collections::BTreeMap, error::Error};

use super::{
    error::{GrammarError, GrammarErrorType},
    information_object::{InformationObjectFields, ObjectSet},
    ASN1Type, ASN1Value, ToplevelDeclaration,
};

#[derive(Debug, PartialEq)]
pub struct OptionalMarker();

impl From<&str> for OptionalMarker {
    fn from(_: &str) -> Self {
        OptionalMarker()
    }
}

#[derive(Debug)]
pub struct RangeSeperator();

#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionMarker();

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    SubtypeConstraint(ElementSet),
    TableConstraint(TableConstraint),
    Parameter(Vec<Parameter>),
}

impl Constraint {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            Constraint::SubtypeConstraint(t) => t.set.link_cross_reference(identifier, tlds),
            _ => false,
        }
    }

    pub(super) fn has_cross_reference(&self) -> bool {
        if let Self::SubtypeConstraint(c) = self {
            c.set.has_cross_reference()
        } else {
            false
        }
    }

    pub fn unpack_as_value_range(
        &self,
    ) -> Result<(&Option<ASN1Value>, &Option<ASN1Value>, bool), GrammarError> {
        if let Constraint::SubtypeConstraint(set) = self {
            if let ElementOrSetOperation::Element(SubtypeElement::ValueRange {
                min,
                max,
                extensible,
            }) = &set.set
            {
                return Ok((min, max, *extensible));
            }
        }
        Err(GrammarError {
            details: format!(
                "Failed to unpack constraint as value range. Constraint: {:?}",
                self
            ),
            kind: GrammarErrorType::UnpackingError,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Parameter {
    ValueParameter(ASN1Value),
    TypeParameter(ASN1Type),
    InformationObjectParameter(InformationObjectFields),
    ObjectSetParameter(ObjectSet),
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetOperator {
    Intersection,
    Union,
    Except,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CompositeConstraint {
    pub base_constraint: Box<Constraint>,
    pub operation: Vec<(SetOperator, Box<Constraint>)>,
    pub extensible: bool,
}

impl
    From<(
        Constraint,
        Vec<(SetOperator, Constraint)>,
        Option<ExtensionMarker>,
    )> for CompositeConstraint
{
    fn from(
        value: (
            Constraint,
            Vec<(SetOperator, Constraint)>,
            Option<ExtensionMarker>,
        ),
    ) -> Self {
        Self {
            base_constraint: Box::new(value.0),
            operation: value
                .1
                .into_iter()
                .map(|(op, c)| (op, Box::new(c)))
                .collect(),
            extensible: value.2.is_some(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ComponentPresence {
    Absent,
    Present,
    Unspecified,
}

/// Representation of a component constraint used for subtyping
/// in ASN1 specifications
#[derive(Debug, Clone, PartialEq)]
pub struct InnerTypeConstraint {
    pub is_partial: bool,
    pub constraints: Vec<ConstrainedComponent>,
}

/// Representation of a single component within a component constraint
/// in ASN1 specifications
#[derive(Debug, Clone, PartialEq)]
pub struct ConstrainedComponent {
    pub identifier: String,
    pub constraints: Vec<Constraint>,
    pub presence: ComponentPresence,
}

/// Representation of a range constraint used for subtyping
/// in ASN1 specifications
#[derive(Debug, Clone, PartialEq)]
pub struct ValueConstraint {
    pub min_value: Option<ASN1Value>,
    pub max_value: Option<ASN1Value>,
    pub extensible: bool,
}

impl<'a> From<ASN1Value> for ValueConstraint {
    fn from(value: ASN1Value) -> Self {
        Self {
            min_value: Some(value.clone()),
            max_value: Some(value),
            extensible: false,
        }
    }
}

impl<'a> From<(ASN1Value, RangeSeperator, ASN1Value)> for ValueConstraint {
    fn from(value: (ASN1Value, RangeSeperator, ASN1Value)) -> Self {
        Self {
            min_value: Some(value.0),
            max_value: Some(value.2),
            extensible: false,
        }
    }
}

impl<'a> From<(ASN1Value, ExtensionMarker)> for ValueConstraint {
    fn from(value: (ASN1Value, ExtensionMarker)) -> Self {
        Self {
            min_value: Some(value.0.clone()),
            max_value: Some(value.0),
            extensible: true,
        }
    }
}

impl<'a> From<(ASN1Value, RangeSeperator, ASN1Value, ExtensionMarker)> for ValueConstraint {
    fn from(value: (ASN1Value, RangeSeperator, ASN1Value, ExtensionMarker)) -> Self {
        Self {
            min_value: Some(value.0),
            max_value: Some(value.2),
            extensible: true,
        }
    }
}

/// Representation of a table constraint used for subtyping
/// in ASN1 specifications
/// _See: ITU-T X.682 (02/2021) 10_
#[derive(Debug, Clone, PartialEq)]
pub struct TableConstraint {
    pub object_set: ObjectSet,
    pub linked_fields: Vec<RelationalConstraint>,
}

impl From<(ObjectSet, Option<Vec<RelationalConstraint>>)> for TableConstraint {
    fn from(value: (ObjectSet, Option<Vec<RelationalConstraint>>)) -> Self {
        Self {
            object_set: value.0,
            linked_fields: value.1.unwrap_or(vec![]),
        }
    }
}

/// Representation of a table's relational constraint
/// _See: ITU-T X.682 (02/2021) 10.7_
#[derive(Debug, Clone, PartialEq)]
pub struct RelationalConstraint {
    pub field_name: String,
    /// The level is null if the field is in the outermost object set of the declaration.
    /// The level is 1-n counting from the innermost object set of the declaration
    pub level: usize,
}

impl From<(usize, &str)> for RelationalConstraint {
    fn from(value: (usize, &str)) -> Self {
        Self {
            field_name: value.1.into(),
            level: value.0,
        }
    }
}

/// Representation of a pattern constraint
/// _See: ITU-T X.680 (02/2021) 51.9_
#[derive(Debug, Clone, PartialEq)]
pub struct PatternConstraint {
    pub pattern: String,
}

impl From<&str> for PatternConstraint {
    fn from(value: &str) -> Self {
        Self {
            pattern: value.into(),
        }
    }
}

/// Representation of a user-defined constraint
/// _See: ITU-T X.682 (02/2021) 9_
#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedConstraint {
    pub definition: String,
}

impl From<&str> for UserDefinedConstraint {
    fn from(value: &str) -> Self {
        Self {
            definition: value.into(),
        }
    }
}

/// Representation of a property settings constraint
/// _See: ITU-T X.680 (02/2021) 51.10_
#[derive(Debug, Clone, PartialEq)]
pub struct PropertySettings {
    pub property_settings_list: Vec<PropertyAndSettingsPair>,
}

impl From<Vec<&str>> for PropertySettings {
    fn from(value: Vec<&str>) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PropertyAndSettingsPair {
    Basic(BasicSettings),
    Date(DateSettings),
    Year(YearSettings),
    Time(TimeSettings),
    LocalOrUtc(LocalOrUtcSettings),
    IntervalType(IntervalTypeSettings),
    StartEndPoint(StartEndPointSettings),
    Recurrence(RecurrenceSettings),
    Midnight(MidnightSettings),
}

impl TryFrom<(&str, &str)> for PropertyAndSettingsPair {
    fn try_from(value: (&str, &str)) -> Result<PropertyAndSettingsPair, Box<dyn Error>> {
        match value.0 {
            BasicSettings::NAME => {
                BasicSettings::from_str(value.1).map(|settings| Self::Basic(settings))
            }
            DateSettings::NAME => {
                DateSettings::from_str(value.1).map(|settings| Self::Date(settings))
            }
            YearSettings::NAME => {
                YearSettings::from_str(value.1).map(|settings| Self::Year(settings))
            }
            TimeSettings::NAME => {
                TimeSettings::from_str(value.1).map(|settings| Self::Time(settings))
            }
            LocalOrUtcSettings::NAME => {
                LocalOrUtcSettings::from_str(value.1).map(|settings| Self::LocalOrUtc(settings))
            }
            IntervalTypeSettings::NAME => {
                IntervalTypeSettings::from_str(value.1).map(|settings| Self::IntervalType(settings))
            }
            StartEndPointSettings::NAME => StartEndPointSettings::from_str(value.1)
                .map(|settings| Self::StartEndPoint(settings)),
            RecurrenceSettings::NAME => {
                RecurrenceSettings::from_str(value.1).map(|settings| Self::Recurrence(settings))
            }
            MidnightSettings::NAME => {
                MidnightSettings::from_str(value.1).map(|settings| Self::Midnight(settings))
            }
            _ => Err("Unknown Settings value.".into())
        }
    }

    type Error = Box<dyn Error>;
}

pub trait PropertySetting {
    const NAME: &'static str;

    fn setting_name<'a>(&'a self) -> String;

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

#[derive(Debug, Clone, PartialEq)]
pub enum BasicSettings {
    Date,
    Time,
    DateTime,
    Interval,
    RecInterval,
}

impl PropertySetting for BasicSettings {
    const NAME: &'static str = "Basic";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            BasicSettings::Date => "Date".into(),
            BasicSettings::Time => "Time".into(),
            BasicSettings::DateTime => "Date-Time".into(),
            BasicSettings::Interval => "Interval".into(),
            BasicSettings::RecInterval => "Rec-Interval".into(),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Date" => Ok(BasicSettings::Date),
            "Time" => Ok(BasicSettings::Time),
            "Date-Time" => Ok(BasicSettings::DateTime),
            "Interval" => Ok(BasicSettings::Interval),
            "Rec-Interval" => Ok(BasicSettings::RecInterval),
            _ => Err("Unknown Settings value.".into())
        }
    }
}

impl PropertySetting for DateSettings {
    const NAME: &'static str = "Date";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            DateSettings::Century => "C".into(),
            DateSettings::Year => "Y".into(),
            DateSettings::YearMonth => "YM".into(),
            DateSettings::YearMonthDay => "YMD".into(),
            DateSettings::YearDay => "YD".into(),
            DateSettings::YearWeek => "YW".into(),
            DateSettings::YearWeekDay => "YWD".into(),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "C" => Ok(DateSettings::Century),
            "Y" => Ok(DateSettings::Year),
            "YM" => Ok(DateSettings::YearMonth),
            "YMD" => Ok(DateSettings::YearMonthDay),
            "YD" => Ok(DateSettings::YearDay),
            "YW" => Ok(DateSettings::YearWeek),
            "YWD" => Ok(DateSettings::YearWeekDay),
            _ => Err("Unknown Settings value.".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum DateSettings {
    Century,
    Year,
    YearMonth,
    YearMonthDay,
    YearDay,
    YearWeek,
    YearWeekDay,
}

impl PropertySetting for YearSettings {
    const NAME: &'static str = "Year";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            YearSettings::Basic => "Basic".into(),
            YearSettings::Proleptic => "Proleptic".into(),
            YearSettings::Negative => "Negative".into(),
            YearSettings::Large(i) => format!("L{i}"),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Basic" => Ok(YearSettings::Basic),
            "Proleptic" => Ok(YearSettings::Proleptic),
            "Negative" => Ok(YearSettings::Negative),
            s if s.starts_with("L") => Ok(s[1..].parse().map(|i| YearSettings::Large(i))?),
            _ => Err("Unknown Settings value.".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum YearSettings {
    Basic,
    Proleptic,
    Negative,
    Large(usize),
}

impl PropertySetting for TimeSettings {
    const NAME: &'static str = "Time";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            TimeSettings::Hour => "H".into(),
            TimeSettings::HourMinute => "HM".into(),
            TimeSettings::HourMinuteSecond => "HMS".into(),
            TimeSettings::HourDecimalFraction(i) => format!("HF{i}"),
            TimeSettings::HourMinuteFraction(i) => format!("HMF{i}"),
            TimeSettings::HourMinuteSecondFraction(i) => format!("HMSF{i}"),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "H" => Ok(TimeSettings::Hour),
            "HM" => Ok(TimeSettings::HourMinute),
            "HMS" => Ok(TimeSettings::HourMinuteSecond),
            s if s.starts_with("HF") => Ok(s[2..]
                .parse()
                .map(|i| TimeSettings::HourDecimalFraction(i))?),
            s if s.starts_with("HMF") => Ok(s[3..]
                .parse()
                .map(|i| TimeSettings::HourMinuteFraction(i))?),
            s if s.starts_with("HMSF") => Ok(s[4..]
                .parse()
                .map(|i| TimeSettings::HourMinuteSecondFraction(i))?),
                _ => Err("Unknown Settings value.".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TimeSettings {
    Hour,
    HourMinute,
    HourMinuteSecond,
    HourDecimalFraction(usize),
    HourMinuteFraction(usize),
    HourMinuteSecondFraction(usize),
}

impl PropertySetting for LocalOrUtcSettings {
    const NAME: &'static str = "Local-or-UTC";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            LocalOrUtcSettings::Local => "L".into(),
            LocalOrUtcSettings::Utc => "Z".into(),
            LocalOrUtcSettings::LocalAndDifference => "LD".into(),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "L" => Ok(LocalOrUtcSettings::Local),
            "Z" => Ok(LocalOrUtcSettings::Utc),
            "LD" => Ok(LocalOrUtcSettings::LocalAndDifference),
            _ => Err("Unknown Settings value.".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum LocalOrUtcSettings {
    Local,
    Utc,
    LocalAndDifference,
}

impl PropertySetting for IntervalTypeSettings {
    const NAME: &'static str = "Interval-type";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            IntervalTypeSettings::StartAndEnd => "SE".into(),
            IntervalTypeSettings::Duration => "D".into(),
            IntervalTypeSettings::StartAndDuration => "SD".into(),
            IntervalTypeSettings::DurationAndEnd => "DE".into(),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "SE" => Ok(IntervalTypeSettings::StartAndEnd),
            "D" => Ok(IntervalTypeSettings::Duration),
            "SD" => Ok(IntervalTypeSettings::StartAndDuration),
            "DE" => Ok(IntervalTypeSettings::DurationAndEnd),
            _ => Err("Unknown Settings value.".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntervalTypeSettings {
    StartAndEnd,
    Duration,
    StartAndDuration,
    DurationAndEnd,
}

impl PropertySetting for StartEndPointSettings {
    const NAME: &'static str = "SE-point";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            StartEndPointSettings::Date => "Date".into(),
            StartEndPointSettings::Time => "Time".into(),
            StartEndPointSettings::DateTime => "Date-Time".into(),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Date" => Ok(StartEndPointSettings::Date),
            "Time" => Ok(StartEndPointSettings::Time),
            "Date-Time" => Ok(StartEndPointSettings::DateTime),
            _ => Err("Unknown Settings value.".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum StartEndPointSettings {
    Date,
    Time,
    DateTime,
}

impl PropertySetting for RecurrenceSettings {
    const NAME: &'static str = "Recurrence";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            RecurrenceSettings::Unlimited => "Unlimited".into(),
            RecurrenceSettings::Recurrences(i) => format!("R{i}"),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Unlimited" => Ok(RecurrenceSettings::Unlimited),
            s if s.starts_with("R") => {
                Ok(s[1..].parse().map(|i| RecurrenceSettings::Recurrences(i))?)
            },
            _ => Err("Unknown Settings value.".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecurrenceSettings {
    Unlimited,
    Recurrences(usize),
}

impl PropertySetting for MidnightSettings {
    const NAME: &'static str = "Midnight";

    fn setting_name<'a>(&'a self) -> String {
        match self {
            MidnightSettings::StartOfDay => "Start".into(),
            MidnightSettings::EndOfDay => "End".into(),
        }
    }

    fn from_str<'a>(value: &'a str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Start" => Ok(MidnightSettings::StartOfDay),
            "End" => Ok(MidnightSettings::EndOfDay),
            _ => Err("Unknown Settings value.".into())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MidnightSettings {
    StartOfDay,
    EndOfDay,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubtypeElement {
    SingleValue {
        value: ASN1Value,
        extensible: bool,
    },
    ContainedSubtype {
        subtype: ASN1Type,
        extensible: bool,
    },
    ValueRange {
        min: Option<ASN1Value>,
        max: Option<ASN1Value>,
        extensible: bool,
    },
    PermittedAlphabet(Box<ElementOrSetOperation>),
    SizeConstraint(Box<ElementOrSetOperation>),
    TypeConstraint(ASN1Type),
    SingleTypeConstraint(InnerTypeConstraint),
    MultipleTypeConstraints(InnerTypeConstraint),
    PatternConstraint(PatternConstraint),
    UserDefinedConstraint(UserDefinedConstraint),
    PropertySettings(PropertySettings), // DurationRange
                                        // TimePointRange
                                        // RecurrenceRange
}

impl SubtypeElement {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            SubtypeElement::SingleValue {
                value,
                extensible: _,
            } => value.link_elsewhere_declared(identifier, tlds),
            SubtypeElement::PermittedAlphabet(e) => e.link_cross_reference(identifier, tlds),
            SubtypeElement::PatternConstraint(_) => false,
            SubtypeElement::UserDefinedConstraint(_) => false,
            SubtypeElement::PropertySettings(_) => false,
            SubtypeElement::ContainedSubtype {
                subtype,
                extensible: _,
            } => subtype.link_subtype_constraint(tlds),
            SubtypeElement::ValueRange {
                min,
                max,
                extensible: _,
            } => {
                let a = min
                    .as_mut()
                    .map_or(false, |m| m.link_elsewhere_declared(identifier, tlds));
                let b = max
                    .as_mut()
                    .map_or(false, |m| m.link_elsewhere_declared(identifier, tlds));
                a || b
            }
            SubtypeElement::SizeConstraint(s) => s.link_cross_reference(identifier, tlds),
            SubtypeElement::TypeConstraint(t) => t.link_constraint_reference(identifier, tlds),
            SubtypeElement::SingleTypeConstraint(s)
            | SubtypeElement::MultipleTypeConstraints(s) => s
                .constraints
                .iter_mut()
                .flat_map(|cc| &mut cc.constraints)
                .map(|c| c.link_cross_reference(identifier, tlds))
                .fold(false, |acc, b| acc || b),
        }
    }

    pub(super) fn has_cross_reference(&self) -> bool {
        match self {
            SubtypeElement::SingleValue {
                value,
                extensible: _,
            } => value.is_elsewhere_declared(),
            SubtypeElement::PatternConstraint(_) => false,
            SubtypeElement::UserDefinedConstraint(_) => false,
            SubtypeElement::PropertySettings(_) => false,
            SubtypeElement::PermittedAlphabet(e) => e.has_cross_reference(),
            SubtypeElement::ContainedSubtype {
                subtype,
                extensible: _,
            } => subtype.contains_constraint_reference(),
            SubtypeElement::ValueRange {
                min,
                max,
                extensible: _,
            } => {
                min.as_ref().map_or(false, |s| s.is_elsewhere_declared())
                    && max.as_ref().map_or(false, |s| s.is_elsewhere_declared())
            }
            SubtypeElement::SizeConstraint(s) => s.has_cross_reference(),
            SubtypeElement::TypeConstraint(t) => t.contains_class_field_reference(),
            SubtypeElement::MultipleTypeConstraints(s)
            | SubtypeElement::SingleTypeConstraint(s) => s
                .constraints
                .iter()
                .any(|cc| cc.constraints.iter().any(|c| c.has_cross_reference())),
        }
    }
}

impl From<(ASN1Value, Option<ExtensionMarker>)> for SubtypeElement {
    fn from(value: (ASN1Value, Option<ExtensionMarker>)) -> Self {
        Self::SingleValue {
            value: value.0,
            extensible: value.1.is_some(),
        }
    }
}

impl From<Constraint> for SubtypeElement {
    fn from(value: Constraint) -> Self {
        match value {
            Constraint::SubtypeConstraint(set) => Self::SizeConstraint(Box::new(set.set)),
            _ => unreachable!(),
        }
    }
}

impl
    From<(
        Option<ExtensionMarker>,
        Vec<(&str, Option<Vec<Constraint>>, Option<ComponentPresence>)>,
    )> for SubtypeElement
{
    fn from(
        value: (
            Option<ExtensionMarker>,
            Vec<(&str, Option<Vec<Constraint>>, Option<ComponentPresence>)>,
        ),
    ) -> Self {
        SubtypeElement::SingleTypeConstraint(InnerTypeConstraint {
            is_partial: value.0.is_some(),
            constraints: value
                .1
                .into_iter()
                .map(|(id, constraint, presence)| ConstrainedComponent {
                    identifier: String::from(id),
                    constraints: constraint.unwrap_or(vec![]),
                    presence: presence.unwrap_or(ComponentPresence::Unspecified),
                })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElementSet {
    pub set: ElementOrSetOperation,
    pub extensible: bool,
}

impl From<(ElementOrSetOperation, Option<ExtensionMarker>)> for ElementSet {
    fn from(value: (ElementOrSetOperation, Option<ExtensionMarker>)) -> Self {
        Self {
            set: value.0,
            extensible: value.1.is_some(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementOrSetOperation {
    Element(SubtypeElement),
    SetOperation(SetOperation),
}

impl ElementOrSetOperation {
    pub(super) fn link_cross_reference(
        &mut self,
        identifier: &String,
        tlds: &BTreeMap<String, ToplevelDeclaration>,
    ) -> bool {
        match self {
            ElementOrSetOperation::Element(e) => e.link_cross_reference(identifier, tlds),
            ElementOrSetOperation::SetOperation(s) => {
                let a = s.base.link_cross_reference(identifier, tlds);
                let b = s.operant.link_cross_reference(identifier, tlds);
                a || b
            }
        }
    }

    pub(super) fn has_cross_reference(&self) -> bool {
        match self {
            ElementOrSetOperation::Element(e) => e.has_cross_reference(),
            ElementOrSetOperation::SetOperation(s) => {
                s.base.has_cross_reference() || s.operant.has_cross_reference()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetOperation {
    pub base: SubtypeElement, //TODO: Handle exclusions
    pub operator: SetOperator,
    pub operant: Box<ElementOrSetOperation>,
}

impl From<(SubtypeElement, SetOperator, ElementOrSetOperation)> for SetOperation {
    fn from(value: (SubtypeElement, SetOperator, ElementOrSetOperation)) -> Self {
        Self {
            base: value.0,
            operator: value.1,
            operant: Box::new(value.2),
        }
    }
}
