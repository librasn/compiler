#[cfg(test)]
use internal_macros::EnumDebug;
use std::error::Error;

use crate::lexer::Span;

use super::{
    error::{GrammarError, GrammarErrorType},
    information_object::{InformationObjectFields, ObjectSet},
    ASN1Type, ASN1Value, IntegerType,
};

#[derive(Debug, PartialEq)]
pub struct OptionalMarker();

impl From<Span<'_>> for OptionalMarker {
    fn from(_: Span) -> Self {
        OptionalMarker()
    }
}

#[derive(Debug)]
pub struct RangeSeperator();

#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionMarker();

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Constraint {
    SubtypeConstraint(ElementSet),
    TableConstraint(TableConstraint),
    Parameter(Vec<Parameter>),
    ContentConstraint(ContentConstraint),
}

impl Constraint {
    /// Returns the type of integer that should be used in a representation when applying the Constraint
    /// ### Example
    pub fn integer_constraints(&self) -> IntegerType {
        let (mut min, mut max, mut is_extensible) = (i128::MAX, i128::MIN, false);
        if let Ok((cmin, cmax, extensible)) = self.unpack_as_value_range() {
            is_extensible = is_extensible || extensible;
            if let Some(ASN1Value::Integer(i)) = cmin {
                min = (*i).min(min);
            };
            if let Some(ASN1Value::Integer(i)) = cmax {
                max = (*i).max(max);
            };
        } else if let Ok((val, extensible)) = self.unpack_as_strict_value() {
            is_extensible = is_extensible || extensible;
            if let ASN1Value::Integer(i) = val {
                min = (*i).min(min);
                max = (*i).max(max);
            };
        };
        if min > max || is_extensible {
            IntegerType::Unbounded
        } else if min >= 0 {
            match max {
                r if r <= u8::MAX.into() => IntegerType::Uint8,
                r if r <= u16::MAX.into() => IntegerType::Uint16,
                r if r <= u32::MAX.into() => IntegerType::Uint32,
                r if r <= u64::MAX.into() => IntegerType::Uint64,
                _ => IntegerType::Unbounded,
            }
        } else {
            match (min, max) {
                (mi, ma) if mi >= i8::MIN.into() && ma <= i8::MAX.into() => IntegerType::Int8,
                (mi, ma) if mi >= i16::MIN.into() && ma <= i16::MAX.into() => IntegerType::Int16,
                (mi, ma) if mi >= i32::MIN.into() && ma <= i32::MAX.into() => IntegerType::Int32,
                (mi, ma) if mi >= i64::MIN.into() && ma <= i64::MAX.into() => IntegerType::Int64,
                _ => IntegerType::Unbounded,
            }
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

    pub fn unpack_as_strict_value(&self) -> Result<(&ASN1Value, bool), GrammarError> {
        if let Constraint::SubtypeConstraint(set) = self {
            if let ElementOrSetOperation::Element(SubtypeElement::SingleValue {
                value,
                extensible,
            }) = &set.set
            {
                return Ok((value, *extensible));
            }
        }
        Err(GrammarError {
            details: format!(
                "Failed to unpack constraint as strict value. Constraint: {:?}",
                self
            ),
            kind: GrammarErrorType::UnpackingError,
        })
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ContentConstraint {
    Containing(ASN1Type),
    EncodedBy(ASN1Value),
    ContainingEncodedBy {
        containing: ASN1Type,
        encoded_by: ASN1Value,
    },
}

impl From<ASN1Type> for ContentConstraint {
    fn from(value: ASN1Type) -> Self {
        ContentConstraint::Containing(value)
    }
}

impl From<ASN1Value> for ContentConstraint {
    fn from(value: ASN1Value) -> Self {
        ContentConstraint::EncodedBy(value)
    }
}

impl From<(ASN1Type, ASN1Value)> for ContentConstraint {
    fn from(value: (ASN1Type, ASN1Value)) -> Self {
        ContentConstraint::ContainingEncodedBy {
            containing: value.0,
            encoded_by: value.1,
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Parameter {
    ValueParameter(ASN1Value),
    TypeParameter(ASN1Type),
    InformationObjectParameter(InformationObjectFields),
    ObjectSetParameter(ObjectSet),
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
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

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
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

impl From<ASN1Value> for ValueConstraint {
    fn from(value: ASN1Value) -> Self {
        Self {
            min_value: Some(value.clone()),
            max_value: Some(value),
            extensible: false,
        }
    }
}

impl From<(ASN1Value, RangeSeperator, ASN1Value)> for ValueConstraint {
    fn from(value: (ASN1Value, RangeSeperator, ASN1Value)) -> Self {
        Self {
            min_value: Some(value.0),
            max_value: Some(value.2),
            extensible: false,
        }
    }
}

impl From<(ASN1Value, ExtensionMarker)> for ValueConstraint {
    fn from(value: (ASN1Value, ExtensionMarker)) -> Self {
        Self {
            min_value: Some(value.0.clone()),
            max_value: Some(value.0),
            extensible: true,
        }
    }
}

impl From<(ASN1Value, RangeSeperator, ASN1Value, ExtensionMarker)> for ValueConstraint {
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
            linked_fields: value.1.unwrap_or_default(),
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

impl From<(usize, Span<'_>)> for RelationalConstraint {
    fn from(value: (usize, Span)) -> Self {
        Self {
            field_name: value.1.to_string(),
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

impl From<Span<'_>> for PatternConstraint {
    fn from(value: Span) -> Self {
        Self {
            pattern: value.to_string(),
        }
    }
}

/// Representation of a user-defined constraint
/// _See: ITU-T X.682 (02/2021) 9_
#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedConstraint {
    pub definition: String,
}

impl From<Span<'_>> for UserDefinedConstraint {
    fn from(value: Span) -> Self {
        Self {
            definition: value.to_string(),
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
    fn from(_value: Vec<&str>) -> Self {
        todo!()
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
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

impl TryFrom<(Span<'_>, Span<'_>)> for PropertyAndSettingsPair {
    fn try_from(value: (Span, Span)) -> Result<PropertyAndSettingsPair, Box<dyn Error>> {
        match *value.0 {
            BasicSettings::NAME => BasicSettings::from_str(*value.1).map(Self::Basic),
            DateSettings::NAME => DateSettings::from_str(*value.1).map(Self::Date),
            YearSettings::NAME => YearSettings::from_str(*value.1).map(Self::Year),
            TimeSettings::NAME => TimeSettings::from_str(*value.1).map(Self::Time),
            LocalOrUtcSettings::NAME => {
                LocalOrUtcSettings::from_str(*value.1).map(Self::LocalOrUtc)
            }
            IntervalTypeSettings::NAME => {
                IntervalTypeSettings::from_str(*value.1).map(Self::IntervalType)
            }
            StartEndPointSettings::NAME => {
                StartEndPointSettings::from_str(*value.1).map(Self::StartEndPoint)
            }
            RecurrenceSettings::NAME => {
                RecurrenceSettings::from_str(*value.1).map(Self::Recurrence)
            }
            MidnightSettings::NAME => MidnightSettings::from_str(*value.1).map(Self::Midnight),
            _ => Err("Unknown Settings value.".into()),
        }
    }

    type Error = Box<dyn Error>;
}

pub trait PropertySetting {
    const NAME: &'static str;

    fn setting_name(&self) -> String;

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized;
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum BasicSettings {
    Date,
    Time,
    DateTime,
    Interval,
    RecInterval,
}

impl PropertySetting for BasicSettings {
    const NAME: &'static str = "Basic";

    fn setting_name(&self) -> String {
        match self {
            BasicSettings::Date => "Date".into(),
            BasicSettings::Time => "Time".into(),
            BasicSettings::DateTime => "Date-Time".into(),
            BasicSettings::Interval => "Interval".into(),
            BasicSettings::RecInterval => "Rec-Interval".into(),
        }
    }

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Date" => Ok(BasicSettings::Date),
            "Time" => Ok(BasicSettings::Time),
            "Date-Time" => Ok(BasicSettings::DateTime),
            "Interval" => Ok(BasicSettings::Interval),
            "Rec-Interval" => Ok(BasicSettings::RecInterval),
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

impl PropertySetting for DateSettings {
    const NAME: &'static str = "Date";

    fn setting_name(&self) -> String {
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

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "C" => Ok(DateSettings::Century),
            "Y" => Ok(DateSettings::Year),
            "YM" => Ok(DateSettings::YearMonth),
            "YMD" => Ok(DateSettings::YearMonthDay),
            "YD" => Ok(DateSettings::YearDay),
            "YW" => Ok(DateSettings::YearWeek),
            "YWD" => Ok(DateSettings::YearWeekDay),
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
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

    fn setting_name(&self) -> String {
        match self {
            YearSettings::Basic => "Basic".into(),
            YearSettings::Proleptic => "Proleptic".into(),
            YearSettings::Negative => "Negative".into(),
            YearSettings::Large(i) => format!("L{i}"),
        }
    }

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Basic" => Ok(YearSettings::Basic),
            "Proleptic" => Ok(YearSettings::Proleptic),
            "Negative" => Ok(YearSettings::Negative),
            s if s.starts_with('L') => Ok(s[1..].parse().map(YearSettings::Large)?),
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum YearSettings {
    Basic,
    Proleptic,
    Negative,
    Large(usize),
}

impl PropertySetting for TimeSettings {
    const NAME: &'static str = "Time";

    fn setting_name(&self) -> String {
        match self {
            TimeSettings::Hour => "H".into(),
            TimeSettings::HourMinute => "HM".into(),
            TimeSettings::HourMinuteSecond => "HMS".into(),
            TimeSettings::HourDecimalFraction(i) => format!("HF{i}"),
            TimeSettings::HourMinuteFraction(i) => format!("HMF{i}"),
            TimeSettings::HourMinuteSecondFraction(i) => format!("HMSF{i}"),
        }
    }

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "H" => Ok(TimeSettings::Hour),
            "HM" => Ok(TimeSettings::HourMinute),
            "HMS" => Ok(TimeSettings::HourMinuteSecond),
            s if s.starts_with("HF") => {
                Ok(s[2..].parse().map(TimeSettings::HourDecimalFraction)?)
            }
            s if s.starts_with("HMF") => {
                Ok(s[3..].parse().map(TimeSettings::HourMinuteFraction)?)
            }
            s if s.starts_with("HMSF") => {
                Ok(s[4..].parse().map(TimeSettings::HourMinuteSecondFraction)?)
            }
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
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

    fn setting_name(&self) -> String {
        match self {
            LocalOrUtcSettings::Local => "L".into(),
            LocalOrUtcSettings::Utc => "Z".into(),
            LocalOrUtcSettings::LocalAndDifference => "LD".into(),
        }
    }

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "L" => Ok(LocalOrUtcSettings::Local),
            "Z" => Ok(LocalOrUtcSettings::Utc),
            "LD" => Ok(LocalOrUtcSettings::LocalAndDifference),
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum LocalOrUtcSettings {
    Local,
    Utc,
    LocalAndDifference,
}

impl PropertySetting for IntervalTypeSettings {
    const NAME: &'static str = "Interval-type";

    fn setting_name(&self) -> String {
        match self {
            IntervalTypeSettings::StartAndEnd => "SE".into(),
            IntervalTypeSettings::Duration => "D".into(),
            IntervalTypeSettings::StartAndDuration => "SD".into(),
            IntervalTypeSettings::DurationAndEnd => "DE".into(),
        }
    }

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "SE" => Ok(IntervalTypeSettings::StartAndEnd),
            "D" => Ok(IntervalTypeSettings::Duration),
            "SD" => Ok(IntervalTypeSettings::StartAndDuration),
            "DE" => Ok(IntervalTypeSettings::DurationAndEnd),
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum IntervalTypeSettings {
    StartAndEnd,
    Duration,
    StartAndDuration,
    DurationAndEnd,
}

impl PropertySetting for StartEndPointSettings {
    const NAME: &'static str = "SE-point";

    fn setting_name(&self) -> String {
        match self {
            StartEndPointSettings::Date => "Date".into(),
            StartEndPointSettings::Time => "Time".into(),
            StartEndPointSettings::DateTime => "Date-Time".into(),
        }
    }

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Date" => Ok(StartEndPointSettings::Date),
            "Time" => Ok(StartEndPointSettings::Time),
            "Date-Time" => Ok(StartEndPointSettings::DateTime),
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum StartEndPointSettings {
    Date,
    Time,
    DateTime,
}

impl PropertySetting for RecurrenceSettings {
    const NAME: &'static str = "Recurrence";

    fn setting_name(&self) -> String {
        match self {
            RecurrenceSettings::Unlimited => "Unlimited".into(),
            RecurrenceSettings::Recurrences(i) => format!("R{i}"),
        }
    }

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Unlimited" => Ok(RecurrenceSettings::Unlimited),
            s if s.starts_with('R') => Ok(s[1..].parse().map(RecurrenceSettings::Recurrences)?),
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum RecurrenceSettings {
    Unlimited,
    Recurrences(usize),
}

impl PropertySetting for MidnightSettings {
    const NAME: &'static str = "Midnight";

    fn setting_name(&self) -> String {
        match self {
            MidnightSettings::StartOfDay => "Start".into(),
            MidnightSettings::EndOfDay => "End".into(),
        }
    }

    fn from_str(value: &str) -> Result<Self, Box<dyn Error>> {
        match value {
            "Start" => Ok(MidnightSettings::StartOfDay),
            "End" => Ok(MidnightSettings::EndOfDay),
            _ => Err("Unknown Settings value.".into()),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum MidnightSettings {
    StartOfDay,
    EndOfDay,
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
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
        Vec<(Span<'_>, Option<Vec<Constraint>>, Option<ComponentPresence>)>,
    )> for SubtypeElement
{
    fn from(
        value: (
            Option<ExtensionMarker>,
            Vec<(Span, Option<Vec<Constraint>>, Option<ComponentPresence>)>,
        ),
    ) -> Self {
        SubtypeElement::SingleTypeConstraint(InnerTypeConstraint {
            is_partial: value.0.is_some(),
            constraints: value
                .1
                .into_iter()
                .map(|(id, constraint, presence)| ConstrainedComponent {
                    identifier: String::from(*id),
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

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ElementOrSetOperation {
    Element(SubtypeElement),
    SetOperation(SetOperation),
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
