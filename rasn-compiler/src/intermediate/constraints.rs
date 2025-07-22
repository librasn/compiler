#[cfg(test)]
use internal_macros::EnumDebug;
use std::{borrow::Cow, error::Error};

use super::{
    error::{GrammarError, GrammarErrorType},
    information_object::{InformationObjectFields, ObjectSet},
    ASN1Type, ASN1Value, IntegerType,
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

/// X.680 49.6 Constraint specification.
///
/// _See X.682 (02/2021) 8_
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Constraint<'a> {
    Subtype(ElementSetSpecs<'a>),
    /// A TableConstraint as specified in X.682 9.
    Table(TableConstraint<'a>),
    Parameter(Vec<Parameter<'a>>),
    /// A ContentsConstraint as specified in X.682 11.
    Content(ContentConstraint<'a>),
}

impl<'a> Constraint<'a> {
    /// Returns the type of integer that should be used in a representation when applying the
    /// GeneralConstraint.
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
    ) -> Result<(&Option<ASN1Value<'a>>, &Option<ASN1Value<'a>>, bool), GrammarError> {
        if let Constraint::Subtype(set) = self {
            if let ElementOrSetOperation::Element(SubtypeElements::ValueRange {
                min,
                max,
                extensible,
            }) = &set.set
            {
                return Ok((min, max, *extensible));
            }
        }
        Err(GrammarError::new(
            &format!("Failed to unpack constraint as value range. Constraint: {self:?}"),
            GrammarErrorType::UnpackingError,
        ))
    }

    pub fn unpack_as_strict_value(&self) -> Result<(&ASN1Value<'a>, bool), GrammarError> {
        if let Constraint::Subtype(set) = self {
            if let ElementOrSetOperation::Element(SubtypeElements::SingleValue {
                value,
                extensible,
            }) = &set.set
            {
                return Ok((value, *extensible));
            }
        }
        Err(GrammarError::new(
            &format!("Failed to unpack constraint as strict value. Constraint: {self:?}"),
            GrammarErrorType::UnpackingError,
        ))
    }
}

/// A ContentConstraint.
///
/// _See: ITU-T X.682 (02/2021) 11_
#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ContentConstraint<'a> {
    /// **X.682 11.4** _The abstract value of the octet string or bit string is the encoding of an
    /// (any) abstract value of "Type" that is produced by the encoding rules that are applied to
    /// the octet string or bit string._
    Containing(ASN1Type<'a>),
    /// **X.682 11.5** _The procedures identified by the object identifier value "Value" shall be
    /// used to produce and to interpret the contents of the bit string or octet string. If the bit
    /// string or octet string is already constrained, it is a specification error if these
    /// procedures do not produce encodings that satisfy the constraint._
    EncodedBy(ASN1Value<'a>),
    /// **X.682 11.6** _The abstract value of the octet string or bit string is the encoding of an
    /// (any) abstract value of "Type" that is produced by the encoding rules identified by the
    /// object identifier value "Value"._
    ContainingEncodedBy {
        containing: ASN1Type<'a>,
        encoded_by: ASN1Value<'a>,
    },
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum Parameter<'a> {
    ValueParameter(ASN1Value<'a>),
    TypeParameter(ASN1Type<'a>),
    InformationObjectParameter(InformationObjectFields<'a>),
    ObjectSetParameter(ObjectSet<'a>),
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
pub struct CompositeConstraint<'a> {
    pub base_constraint: Box<Constraint<'a>>,
    pub operation: Vec<(SetOperator, Box<Constraint<'a>>)>,
    pub extensible: bool,
}

impl<'a>
    From<(
        Constraint<'a>,
        Vec<(SetOperator, Constraint<'a>)>,
        Option<ExtensionMarker>,
    )> for CompositeConstraint<'a>
{
    fn from(
        value: (
            Constraint<'a>,
            Vec<(SetOperator, Constraint<'a>)>,
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
pub struct InnerTypeConstraint<'a> {
    pub is_partial: bool,
    pub constraints: Vec<NamedConstraint<'a>>,
}

/// Representation of a single component within a component constraint
/// in ASN1 specifications
#[derive(Debug, Clone, PartialEq)]
pub struct NamedConstraint<'a> {
    pub identifier: Cow<'a, str>,
    pub constraints: Vec<Constraint<'a>>,
    pub presence: ComponentPresence,
}

/// Representation of a range constraint used for subtyping
/// in ASN1 specifications
#[derive(Debug, Clone, PartialEq)]
pub struct ValueConstraint<'a> {
    pub min_value: Option<ASN1Value<'a>>,
    pub max_value: Option<ASN1Value<'a>>,
    pub extensible: bool,
}

impl<'a> From<ASN1Value<'a>> for ValueConstraint<'a> {
    fn from(value: ASN1Value<'a>) -> Self {
        Self {
            min_value: Some(value.clone()),
            max_value: Some(value),
            extensible: false,
        }
    }
}

impl<'a> From<(ASN1Value<'a>, RangeSeperator, ASN1Value<'a>)> for ValueConstraint<'a> {
    fn from(value: (ASN1Value<'a>, RangeSeperator, ASN1Value<'a>)) -> Self {
        Self {
            min_value: Some(value.0),
            max_value: Some(value.2),
            extensible: false,
        }
    }
}

impl<'a> From<(ASN1Value<'a>, ExtensionMarker)> for ValueConstraint<'a> {
    fn from(value: (ASN1Value<'a>, ExtensionMarker)) -> Self {
        Self {
            min_value: Some(value.0.clone()),
            max_value: Some(value.0),
            extensible: true,
        }
    }
}

impl<'a>
    From<(
        ASN1Value<'a>,
        RangeSeperator,
        ASN1Value<'a>,
        ExtensionMarker,
    )> for ValueConstraint<'a>
{
    fn from(
        value: (
            ASN1Value<'a>,
            RangeSeperator,
            ASN1Value<'a>,
            ExtensionMarker,
        ),
    ) -> Self {
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
pub struct TableConstraint<'a> {
    pub object_set: ObjectSet<'a>,
    pub linked_fields: Vec<RelationalConstraint<'a>>,
}

impl<'a> From<(ObjectSet<'a>, Option<Vec<RelationalConstraint<'a>>>)> for TableConstraint<'a> {
    fn from(value: (ObjectSet<'a>, Option<Vec<RelationalConstraint<'a>>>)) -> Self {
        Self {
            object_set: value.0,
            linked_fields: value.1.unwrap_or_default(),
        }
    }
}

/// Representation of a table's relational constraint
/// _See: ITU-T X.682 (02/2021) 10.7_
#[derive(Debug, Clone, PartialEq)]
pub struct RelationalConstraint<'a> {
    pub field_name: Cow<'a, str>,
    /// The level is null if the field is in the outermost object set of the declaration.
    /// The level is 1-n counting from the innermost object set of the declaration
    pub level: usize,
}

impl<'a> From<(usize, &'a str)> for RelationalConstraint<'a> {
    fn from(value: (usize, &'a str)) -> Self {
        Self {
            field_name: Cow::Borrowed(value.1),
            level: value.0,
        }
    }
}

/// Representation of a pattern constraint
/// _See: ITU-T X.680 (02/2021) 51.9_
#[derive(Debug, Clone, PartialEq)]
pub struct PatternConstraint<'a> {
    pub pattern: Cow<'a, str>,
}

impl<'a> From<&'a str> for PatternConstraint<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            pattern: Cow::Borrowed(value),
        }
    }
}

/// Representation of a user-defined constraint
/// _See: ITU-T X.682 (02/2021) 9_
#[derive(Debug, Clone, PartialEq)]
pub struct UserDefinedConstraint<'a> {
    pub definition: Cow<'a, str>,
}

impl<'a> From<&'a str> for UserDefinedConstraint<'a> {
    fn from(value: &'a str) -> Self {
        Self {
            definition: Cow::Borrowed(value),
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

impl TryFrom<(&str, &str)> for PropertyAndSettingsPair {
    fn try_from(value: (&str, &str)) -> Result<PropertyAndSettingsPair, Box<dyn Error>> {
        match value.0 {
            BasicSettings::NAME => BasicSettings::from_str(value.1).map(Self::Basic),
            DateSettings::NAME => DateSettings::from_str(value.1).map(Self::Date),
            YearSettings::NAME => YearSettings::from_str(value.1).map(Self::Year),
            TimeSettings::NAME => TimeSettings::from_str(value.1).map(Self::Time),
            LocalOrUtcSettings::NAME => LocalOrUtcSettings::from_str(value.1).map(Self::LocalOrUtc),
            IntervalTypeSettings::NAME => {
                IntervalTypeSettings::from_str(value.1).map(Self::IntervalType)
            }
            StartEndPointSettings::NAME => {
                StartEndPointSettings::from_str(value.1).map(Self::StartEndPoint)
            }
            RecurrenceSettings::NAME => RecurrenceSettings::from_str(value.1).map(Self::Recurrence),
            MidnightSettings::NAME => MidnightSettings::from_str(value.1).map(Self::Midnight),
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
pub enum SubtypeElements<'a> {
    SingleValue {
        value: ASN1Value<'a>,
        extensible: bool,
    },
    ContainedSubtype {
        subtype: ASN1Type<'a>,
        extensible: bool,
    },
    ValueRange {
        min: Option<ASN1Value<'a>>,
        max: Option<ASN1Value<'a>>,
        extensible: bool,
    },
    PermittedAlphabet(Box<ElementOrSetOperation<'a>>),
    SizeConstraint(Box<ElementOrSetOperation<'a>>),
    TypeConstraint(ASN1Type<'a>),
    SingleTypeConstraint(Vec<Constraint<'a>>),
    MultipleTypeConstraints(InnerTypeConstraint<'a>),
    PatternConstraint(PatternConstraint<'a>),
    UserDefinedConstraint(UserDefinedConstraint<'a>),
    PropertySettings(PropertySettings), // DurationRange
                                        // TimePointRange
                                        // RecurrenceRange
}

impl<'a> From<(ASN1Value<'a>, Option<ExtensionMarker>)> for SubtypeElements<'a> {
    fn from(value: (ASN1Value<'a>, Option<ExtensionMarker>)) -> Self {
        Self::SingleValue {
            value: value.0,
            extensible: value.1.is_some(),
        }
    }
}

impl<'a> From<Constraint<'a>> for SubtypeElements<'a> {
    fn from(value: Constraint<'a>) -> Self {
        match value {
            Constraint::Subtype(set) => Self::SizeConstraint(Box::new(set.set)),
            _ => unreachable!(),
        }
    }
}

impl<'a> From<(Option<ExtensionMarker>, Vec<NamedConstraint<'a>>)> for SubtypeElements<'a> {
    fn from(value: (Option<ExtensionMarker>, Vec<NamedConstraint<'a>>)) -> Self {
        SubtypeElements::MultipleTypeConstraints(InnerTypeConstraint {
            is_partial: value.0.is_some(),
            constraints: value.1,
        })
    }
}

/// X.680 50. Element set specification
///
/// *50.1* _In some notations a set of elements of some identified type or information object class
/// (the governor) can be specified. In such cases, the notation "ElementSetSpec" is used._
#[derive(Debug, Clone, PartialEq)]
pub struct ElementSetSpecs<'a> {
    pub set: ElementOrSetOperation<'a>,
    pub extensible: bool,
}

impl<'a> From<(ElementOrSetOperation<'a>, Option<ExtensionMarker>)> for ElementSetSpecs<'a> {
    fn from(value: (ElementOrSetOperation<'a>, Option<ExtensionMarker>)) -> Self {
        Self {
            set: value.0,
            extensible: value.1.is_some(),
        }
    }
}

#[cfg_attr(test, derive(EnumDebug))]
#[cfg_attr(not(test), derive(Debug))]
#[derive(Clone, PartialEq)]
pub enum ElementOrSetOperation<'a> {
    Element(SubtypeElements<'a>),
    SetOperation(SetOperation<'a>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetOperation<'a> {
    pub base: SubtypeElements<'a>, //TODO: Handle exclusions
    pub operator: SetOperator,
    pub operant: Box<ElementOrSetOperation<'a>>,
}

impl<'a> From<(SubtypeElements<'a>, SetOperator, ElementOrSetOperation<'a>)> for SetOperation<'a> {
    fn from(value: (SubtypeElements<'a>, SetOperator, ElementOrSetOperation<'a>)) -> Self {
        Self {
            base: value.0,
            operator: value.1,
            operant: Box::new(value.2),
        }
    }
}
