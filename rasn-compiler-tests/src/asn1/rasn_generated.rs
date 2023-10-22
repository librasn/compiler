extern crate alloc;

use rasn::prelude::*;
// =====================================================
// Etsi-Schema
// { iso(1) standard(0) signalizedIntersection(19091) profilec(2) addgrpc(0) version2(2) }
// =====================================================

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ACKResponseService {
    #[rasn(value("-32768..=32767"))]
    pub ack_resp_delay_adjust: i16,
    #[rasn(value("0..=65535"))]
    pub ack_resp_delay_std_dev: u16,
}

impl ACKResponseService {
    pub fn new(ack_resp_delay_adjust: i16, ack_resp_delay_std_dev: u16) -> Self {
        Self {
            ack_resp_delay_adjust,
            ack_resp_delay_std_dev,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AbsolutePosition {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

impl AbsolutePosition {
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AbsolutePositionWAltitude {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub altitude: Altitude,
}

impl AbsolutePositionWAltitude {
    pub fn new(latitude: Latitude, longitude: Longitude, altitude: Altitude) -> Self {
        Self {
            latitude,
            longitude,
            altitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct AbsolutePositions(pub SequenceOf<AbsolutePosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct AbsolutePositionsWithAltitude(pub SequenceOf<AbsolutePositionWAltitude>);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum AccelOrDecel {
    Accelerate = 0,
    Decelerate = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AccelerationChangeIndication {
    pub accel_or_decel: AccelOrDecel,
    pub action_delta_time: ActionDeltaTime,
}

impl AccelerationChangeIndication {
    pub fn new(accel_or_decel: AccelOrDecel, action_delta_time: ActionDeltaTime) -> Self {
        Self {
            accel_or_decel,
            action_delta_time,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=102"))]
pub struct AccelerationConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("7"))]
pub struct AccelerationControl(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AccidentSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum AckNackIndication {
    ACK = 0,
    NACK = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=127"))]
pub struct ActionDeltaTime(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ActionID {
    pub originating_station_i_d: StationID,
    pub sequence_number: SequenceNumber,
}

impl ActionID {
    pub fn new(originating_station_i_d: StationID, sequence_number: SequenceNumber) -> Self {
        Self {
            originating_station_i_d,
            sequence_number,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionAdhesionSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionExtremeWeatherConditionSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionPrecipitationSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionVisibilitySubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32"))]
pub struct AdvertiserIdentifier(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct AdvertiserPermissions(pub SequenceOf<ChannelIdentifier>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct AdvisorySpeedRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct AdvisorySpeed {
    pub r_type: AdvisorySpeedType,
    pub speed: Option<SpeedAdvice>,
    pub confidence: Option<SAESpeedConfidence>,
    pub distance: Option<ZoneLength>,
    pub class: Option<RestrictionClassID>,
    pub regional: Option<AdvisorySpeedRegional>,
}

impl AdvisorySpeed {
    pub fn new(
        r_type: AdvisorySpeedType,
        speed: Option<SpeedAdvice>,
        confidence: Option<SAESpeedConfidence>,
        distance: Option<ZoneLength>,
        class: Option<RestrictionClassID>,
        regional: Option<AdvisorySpeedRegional>,
    ) -> Self {
        Self {
            r_type,
            speed,
            confidence,
            distance,
            class,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct AdvisorySpeedList(pub SequenceOf<AdvisorySpeed>);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum AdvisorySpeedType {
    None = 0,
    Greenwave = 1,
    EcoDrive = 2,
    Transit = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct AlacarteContainer {
    pub lane_position: Option<LanePosition>,
    pub impact_reduction: Option<ImpactReductionContainer>,
    pub external_temperature: Option<Temperature>,
    pub road_works: Option<RoadWorksContainerExtended>,
    pub positioning_solution: Option<PositioningSolutionType>,
    pub stationary_vehicle: Option<StationaryVehicleContainer>,
    #[rasn(extension_addition)]
    pub pre_crash_container: Option<PreCrashContainer>,
}

impl AlacarteContainer {
    pub fn new(
        lane_position: Option<LanePosition>,
        impact_reduction: Option<ImpactReductionContainer>,
        external_temperature: Option<Temperature>,
        road_works: Option<RoadWorksContainerExtended>,
        positioning_solution: Option<PositioningSolutionType>,
        stationary_vehicle: Option<StationaryVehicleContainer>,
        pre_crash_container: Option<PreCrashContainer>,
    ) -> Self {
        Self {
            lane_position,
            impact_reduction,
            external_temperature,
            road_works,
            positioning_solution,
            stationary_vehicle,
            pre_crash_container,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("12"))]
pub struct AllowedManeuvers(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Altitude {
    pub altitude_value: AltitudeValue,
    pub altitude_confidence: AltitudeConfidence,
}

impl Altitude {
    pub fn new(altitude_value: AltitudeValue, altitude_confidence: AltitudeConfidence) -> Self {
        Self {
            altitude_value,
            altitude_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum AltitudeConfidence {
    Alt00001 = 0,
    Alt00002 = 1,
    Alt00005 = 2,
    Alt00010 = 3,
    Alt00020 = 4,
    Alt00050 = 5,
    Alt00100 = 6,
    Alt00200 = 7,
    Alt00500 = 8,
    Alt01000 = 9,
    Alt02000 = 10,
    Alt05000 = 11,
    Alt10000 = 12,
    Alt20000 = 13,
    OutOfRange = 14,
    Unavailable = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-100000..=800001"))]
pub struct AltitudeValue(pub i32);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum AmbientOrRoadConditionPictogram {
    AmbientCondition = 0,
    RoadCondition = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=28800"))]
pub struct Angle(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct AngleConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AnimalSubclass {
    #[rasn(default = "animal_subclass_r_type_default")]
    pub r_type: AnimalSubclassType,
    #[rasn(default = "animal_subclass_confidence_default")]
    pub confidence: ClassConfidence,
}

impl AnimalSubclass {
    pub fn new(r_type: AnimalSubclassType, confidence: ClassConfidence) -> Self {
        Self { r_type, confidence }
    }
}

fn animal_subclass_r_type_default() -> AnimalSubclassType {
    AnimalSubclassType(0).into()
}

fn animal_subclass_confidence_default() -> ClassConfidence {
    ClassConfidence(0).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct AnimalSubclassType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AntennaOffsetSet {
    pub ant_offset_x: OffsetB12,
    pub ant_offset_y: OffsetB09,
    pub ant_offset_z: OffsetB10,
}

impl AntennaOffsetSet {
    pub fn new(ant_offset_x: OffsetB12, ant_offset_y: OffsetB09, ant_offset_z: OffsetB10) -> Self {
        Self {
            ant_offset_x,
            ant_offset_y,
            ant_offset_z,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AnyCatalogue {
    pub owner: Provider,
    #[rasn(value("0..=255"))]
    pub version: u8,
    #[rasn(value("0..=65535"))]
    pub pictogram_code: u16,
    #[rasn(value("0..=65535"))]
    pub value: Option<u16>,
    pub unit: Option<RSCUnit>,
    pub attributes: Option<ISO14823Attributes>,
}

impl AnyCatalogue {
    pub fn new(
        owner: Provider,
        version: u8,
        pictogram_code: u16,
        value: Option<u16>,
        unit: Option<RSCUnit>,
        attributes: Option<ISO14823Attributes>,
    ) -> Self {
        Self {
            owner,
            version,
            pictogram_code,
            value,
            unit,
            attributes,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct ApproachID(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AreaCircular {
    pub node_center_point: Option<OffsetPoint>,
    pub radius: Radius,
}

impl AreaCircular {
    pub fn new(node_center_point: Option<OffsetPoint>, radius: Radius) -> Self {
        Self {
            node_center_point,
            radius,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AreaEllipse {
    pub node_center_point: Option<OffsetPoint>,
    pub semi_minor_range_length: SemiRangeLength,
    pub semi_major_range_length: SemiRangeLength,
    pub semi_major_range_orientation: WGS84AngleValue,
    pub semi_height: Option<SemiRangeLength>,
}

impl AreaEllipse {
    pub fn new(
        node_center_point: Option<OffsetPoint>,
        semi_minor_range_length: SemiRangeLength,
        semi_major_range_length: SemiRangeLength,
        semi_major_range_orientation: WGS84AngleValue,
        semi_height: Option<SemiRangeLength>,
    ) -> Self {
        Self {
            node_center_point,
            semi_minor_range_length,
            semi_major_range_length,
            semi_major_range_orientation,
            semi_height,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AreaPolygon {
    pub poly_point_list: PolyPointList,
}

impl AreaPolygon {
    pub fn new(poly_point_list: PolyPointList) -> Self {
        Self { poly_point_list }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct AreaRadial {
    pub range: Range,
    pub stationary_horizontal_opening_angle_start: WGS84AngleValue,
    pub stationary_horizontal_opening_angle_end: WGS84AngleValue,
    pub vertical_opening_angle_start: Option<CartesianAngleValue>,
    pub vertical_opening_angle_end: Option<CartesianAngleValue>,
    pub sensor_position_offset: Option<OffsetPoint>,
    pub sensor_height: Option<SensorHeight>,
}

impl AreaRadial {
    pub fn new(
        range: Range,
        stationary_horizontal_opening_angle_start: WGS84AngleValue,
        stationary_horizontal_opening_angle_end: WGS84AngleValue,
        vertical_opening_angle_start: Option<CartesianAngleValue>,
        vertical_opening_angle_end: Option<CartesianAngleValue>,
        sensor_position_offset: Option<OffsetPoint>,
        sensor_height: Option<SensorHeight>,
    ) -> Self {
        Self {
            range,
            stationary_horizontal_opening_angle_start,
            stationary_horizontal_opening_angle_end,
            vertical_opening_angle_start,
            vertical_opening_angle_end,
            sensor_position_offset,
            sensor_height,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AreaRectangle {
    pub node_center_point: Option<OffsetPoint>,
    pub semi_major_range_length: SemiRangeLength,
    pub semi_minor_range_length: SemiRangeLength,
    pub semi_major_range_orientation: WGS84AngleValue,
    pub semi_height: Option<SemiRangeLength>,
}

impl AreaRectangle {
    pub fn new(
        node_center_point: Option<OffsetPoint>,
        semi_major_range_length: SemiRangeLength,
        semi_minor_range_length: SemiRangeLength,
        semi_major_range_orientation: WGS84AngleValue,
        semi_height: Option<SemiRangeLength>,
    ) -> Self {
        Self {
            node_center_point,
            semi_major_range_length,
            semi_minor_range_length,
            semi_major_range_orientation,
            semi_height,
        }
    }
}

/// Anonymous SEQUENCE OF member

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=127", extensible))]
pub struct AnonymousAttributeIdList(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=127", extensible))]
pub struct AttributeIdList(pub SequenceOf<AnonymousAttributeIdList>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct AxleWeightLimits {
    pub max_ladenweight_on_axle1: Int2,
    pub max_ladenweight_on_axle2: Int2,
    pub max_ladenweight_on_axle3: Int2,
    pub max_ladenweight_on_axle4: Int2,
    pub max_ladenweight_on_axle5: Int2,
}

impl AxleWeightLimits {
    pub fn new(
        max_ladenweight_on_axle1: Int2,
        max_ladenweight_on_axle2: Int2,
        max_ladenweight_on_axle3: Int2,
        max_ladenweight_on_axle4: Int2,
        max_ladenweight_on_axle5: Int2,
    ) -> Self {
        Self {
            max_ladenweight_on_axle1,
            max_ladenweight_on_axle2,
            max_ladenweight_on_axle3,
            max_ladenweight_on_axle4,
            max_ladenweight_on_axle5,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum BasePublicEncryptionKey {
    EciesNistP256(EccP256CurvePoint),
    EciesBrainpoolP256r1(EccP256CurvePoint),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct BasicContainer {
    pub station_type: StationType,
    pub reference_position: ReferencePosition,
}

impl BasicContainer {
    pub fn new(station_type: StationType, reference_position: ReferencePosition) -> Self {
        Self {
            station_type,
            reference_position,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct BasicVehicleContainerHighFrequency {
    pub heading: Heading,
    pub speed: Speed,
    pub drive_direction: DriveDirection,
    pub vehicle_length: VehicleLength,
    pub vehicle_width: VehicleWidth,
    pub longitudinal_acceleration: LongitudinalAcceleration,
    pub curvature: Curvature,
    pub curvature_calculation_mode: CurvatureCalculationMode,
    pub yaw_rate: YawRate,
    pub acceleration_control: Option<AccelerationControl>,
    pub lane_position: Option<LanePosition>,
    pub steering_wheel_angle: Option<SteeringWheelAngle>,
    pub lateral_acceleration: Option<LateralAcceleration>,
    pub vertical_acceleration: Option<VerticalAcceleration>,
    pub performance_class: Option<PerformanceClass>,
    pub cen_dsrc_tolling_zone: Option<CenDsrcTollingZone>,
}

impl BasicVehicleContainerHighFrequency {
    pub fn new(
        heading: Heading,
        speed: Speed,
        drive_direction: DriveDirection,
        vehicle_length: VehicleLength,
        vehicle_width: VehicleWidth,
        longitudinal_acceleration: LongitudinalAcceleration,
        curvature: Curvature,
        curvature_calculation_mode: CurvatureCalculationMode,
        yaw_rate: YawRate,
        acceleration_control: Option<AccelerationControl>,
        lane_position: Option<LanePosition>,
        steering_wheel_angle: Option<SteeringWheelAngle>,
        lateral_acceleration: Option<LateralAcceleration>,
        vertical_acceleration: Option<VerticalAcceleration>,
        performance_class: Option<PerformanceClass>,
        cen_dsrc_tolling_zone: Option<CenDsrcTollingZone>,
    ) -> Self {
        Self {
            heading,
            speed,
            drive_direction,
            vehicle_length,
            vehicle_width,
            longitudinal_acceleration,
            curvature,
            curvature_calculation_mode,
            yaw_rate,
            acceleration_control,
            lane_position,
            steering_wheel_angle,
            lateral_acceleration,
            vertical_acceleration,
            performance_class,
            cen_dsrc_tolling_zone,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct BasicVehicleContainerLowFrequency {
    pub vehicle_role: VehicleRole,
    pub exterior_lights: ExteriorLights,
    pub path_history: PathHistory,
}

impl BasicVehicleContainerLowFrequency {
    pub fn new(
        vehicle_role: VehicleRole,
        exterior_lights: ExteriorLights,
        path_history: PathHistory,
    ) -> Self {
        Self {
            vehicle_role,
            exterior_lights,
            path_history,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum BasicVehicleRole {
    BasicVehicle = 0,
    PublicTransport = 1,
    SpecialTransport = 2,
    DangerousGoods = 3,
    RoadWork = 4,
    RoadRescue = 5,
    Emergency = 6,
    SafetyCar = 7,
    NoneUnknown = 8,
    Truck = 9,
    Motorcycle = 10,
    RoadSideSource = 11,
    Police = 12,
    Fire = 13,
    Ambulance = 14,
    Dot = 15,
    Transit = 16,
    SlowMoving = 17,
    StopNgo = 18,
    Cyclist = 19,
    Pedestrian = 20,
    NonMotorized = 21,
    Military = 22,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum BatteryStatus {
    Unknown = 0,
    Critical = 1,
    Low = 2,
    Good = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=31"))]
pub struct BitmapSsp(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct BitmapSspRange {
    #[rasn(size("1..=32"))]
    pub ssp_value: OctetString,
    #[rasn(size("1..=32"))]
    pub ssp_bitmask: OctetString,
}

impl BitmapSspRange {
    pub fn new(ssp_value: OctetString, ssp_bitmask: OctetString) -> Self {
        Self {
            ssp_value,
            ssp_bitmask,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CAM {
    pub header: ItsPduHeader,
    pub cam: CoopAwareness,
}

impl CAM {
    pub fn new(header: ItsPduHeader, cam: CoopAwareness) -> Self {
        Self { header, cam }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CPM {
    pub header: ItsPduHeader,
    pub cpm: CollectivePerceptionMessage,
}

impl CPM {
    pub fn new(header: ItsPduHeader, cpm: CollectivePerceptionMessage) -> Self {
        Self { header, cpm }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CS5 {
    pub vin: VisibleString,
    #[rasn(size("9"))]
    pub fill: BitString,
}

impl CS5 {
    pub fn new(vin: VisibleString, fill: BitString) -> Self {
        Self { vin, fill }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CamParameters {
    pub basic_container: BasicContainer,
    pub high_frequency_container: HighFrequencyContainer,
    pub low_frequency_container: Option<LowFrequencyContainer>,
    pub special_vehicle_container: Option<SpecialVehicleContainer>,
}

impl CamParameters {
    pub fn new(
        basic_container: BasicContainer,
        high_frequency_container: HighFrequencyContainer,
        low_frequency_container: Option<LowFrequencyContainer>,
        special_vehicle_container: Option<SpecialVehicleContainer>,
    ) -> Self {
        Self {
            basic_container,
            high_frequency_container,
            low_frequency_container,
            special_vehicle_container,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CartesianAngle {
    pub value: CartesianAngleValue,
    pub confidence: AngleConfidence,
}

impl CartesianAngle {
    pub fn new(value: CartesianAngleValue, confidence: AngleConfidence) -> Self {
        Self { value, confidence }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3601"))]
pub struct CartesianAngleValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CauseCode {
    pub cause_code: CauseCodeType,
    pub sub_cause_code: SubCauseCodeType,
}

impl CauseCode {
    pub fn new(cause_code: CauseCodeType, sub_cause_code: SubCauseCodeType) -> Self {
        Self {
            cause_code,
            sub_cause_code,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct CauseCodeType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CenDsrcTollingZone {
    pub protected_zone_latitude: Latitude,
    pub protected_zone_longitude: Longitude,
    pub cen_dsrc_tolling_zone_i_d: Option<CenDsrcTollingZoneID>,
}

impl CenDsrcTollingZone {
    pub fn new(
        protected_zone_latitude: Latitude,
        protected_zone_longitude: Longitude,
        cen_dsrc_tolling_zone_i_d: Option<CenDsrcTollingZoneID>,
    ) -> Self {
        Self {
            protected_zone_latitude,
            protected_zone_longitude,
            cen_dsrc_tolling_zone_i_d,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct CenDsrcTollingZoneID(pub ProtectedZoneID);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ChInfoOptions {
    pub option1: Option<()>,
    pub option2: Option<()>,
    pub option3: Option<()>,
    pub option4: Option<()>,
    pub option5: Option<()>,
    pub option6: Option<()>,
    pub option7: Option<()>,
    pub extensions: Option<ChannelInfoExts>,
}

impl ChInfoOptions {
    pub fn new(
        option1: Option<()>,
        option2: Option<()>,
        option3: Option<()>,
        option4: Option<()>,
        option5: Option<()>,
        option6: Option<()>,
        option7: Option<()>,
        extensions: Option<ChannelInfoExts>,
    ) -> Self {
        Self {
            option1,
            option2,
            option3,
            option4,
            option5,
            option6,
            option7,
            extensions,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct ChannelAccess80211(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ChannelIdentifier {
    #[rasn(size("3"))]
    pub country_string: OctetString,
    pub operating_class: Uint8,
    pub channel_number: Uint8,
}

impl ChannelIdentifier {
    pub fn new(country_string: OctetString, operating_class: Uint8, channel_number: Uint8) -> Self {
        Self {
            country_string,
            operating_class,
            channel_number,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=31"))]
pub struct ChannelIndex(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ChannelInfo {
    pub operating_class: OperatingClass80211,
    pub channel_number: ChannelNumber80211,
    pub power_level: TXpower80211,
    pub data_rate: WsaChInfoDataRate,
    pub extensions: ChInfoOptions,
}

impl ChannelInfo {
    pub fn new(
        operating_class: OperatingClass80211,
        channel_number: ChannelNumber80211,
        power_level: TXpower80211,
        data_rate: WsaChInfoDataRate,
        extensions: ChInfoOptions,
    ) -> Self {
        Self {
            operating_class,
            channel_number,
            power_level,
            data_rate,
            extensions,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ChannelInfoExt(pub Extension);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ChannelInfoExts(pub SequenceOf<ChannelInfoExt>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ChannelInfos(pub SequenceOf<ChannelInfo>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct ChannelNumber80211(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ChannelOptions {
    pub system_service: Option<SystemService>,
    pub service_provider_port: Option<ReplyAddress>,
    pub extensions: Option<ServiceInfoExts>,
}

impl ChannelOptions {
    pub fn new(
        system_service: Option<SystemService>,
        service_provider_port: Option<ReplyAddress>,
        extensions: Option<ServiceInfoExts>,
    ) -> Self {
        Self {
            system_service,
            service_provider_port,
            extensions,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ChannelSpecificProviderPermission {
    pub channel_id: ChannelIdentifier,
    pub permitted_psids: Option<SequenceOfPsid>,
    pub permitted_edca_parameters: Option<SequenceOfEdcaIdentifier>,
    pub maximum_transmit_power: Option<Uint8>,
}

impl ChannelSpecificProviderPermission {
    pub fn new(
        channel_id: ChannelIdentifier,
        permitted_psids: Option<SequenceOfPsid>,
        permitted_edca_parameters: Option<SequenceOfEdcaIdentifier>,
        maximum_transmit_power: Option<Uint8>,
    ) -> Self {
        Self {
            channel_id,
            permitted_psids,
            permitted_edca_parameters,
            maximum_transmit_power,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ChargingSpotType(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CircularRegion {
    pub center: Dot2TwoDLocation,
    pub radius: Uint16,
}

impl CircularRegion {
    pub fn new(center: Dot2TwoDLocation, radius: Uint16) -> Self {
        Self { center, radius }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=101"))]
pub struct ClassConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ClosedLanes {
    pub innerhard_shoulder_status: Option<HardShoulderStatus>,
    pub outerhard_shoulder_status: Option<HardShoulderStatus>,
    pub driving_lane_status: Option<DrivingLaneStatus>,
}

impl ClosedLanes {
    pub fn new(
        innerhard_shoulder_status: Option<HardShoulderStatus>,
        outerhard_shoulder_status: Option<HardShoulderStatus>,
        driving_lane_status: Option<DrivingLaneStatus>,
    ) -> Self {
        Self {
            innerhard_shoulder_status,
            outerhard_shoulder_status,
            driving_lane_status,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ClusterBoundingBoxShape {
    ClusterRectangle(AreaRectangle),
    ClusterCircle(AreaCircular),
    ClusterPolygon(AreaPolygon),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ClusterBreakupInfo {
    pub cluster_breakup_reason: ClusterBreakupReason,
    pub breakup_time: VruClusterOpTimestamp,
}

impl ClusterBreakupInfo {
    pub fn new(
        cluster_breakup_reason: ClusterBreakupReason,
        breakup_time: VruClusterOpTimestamp,
    ) -> Self {
        Self {
            cluster_breakup_reason,
            breakup_time,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum ClusterBreakupReason {
    NotProvided = 0,
    ClusteringPurposeCompleted = 1,
    LeaderMovedOutOfClusterBoundingBox = 2,
    JoiningAnotherCluster = 3,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct ClusterId(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ClusterJoinInfo {
    pub cluster_id: ClusterId,
    pub join_time: VruClusterOpTimestamp,
}

impl ClusterJoinInfo {
    pub fn new(cluster_id: ClusterId, join_time: VruClusterOpTimestamp) -> Self {
        Self {
            cluster_id,
            join_time,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ClusterLeaveInfo {
    pub cluster_id: ClusterId,
    pub cluster_leave_reason: ClusterLeaveReason,
}

impl ClusterLeaveInfo {
    pub fn new(cluster_id: ClusterId, cluster_leave_reason: ClusterLeaveReason) -> Self {
        Self {
            cluster_id,
            cluster_leave_reason,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum ClusterLeaveReason {
    NotProvided = 0,
    ClusterLeaderLost = 1,
    ClusterDisbandedByLeader = 2,
    OutOfClusterBoundingBox = 3,
    OutOfClusterSpeedRange = 4,
    JoiningAnotherCluster = 5,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("4"))]
pub struct ClusterProfiles(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Code {
    ViennaConvention(VcCode),
    Iso14823(ISO14823Code),
    #[rasn(value("0..=65535"))]
    ItisCodes(u16),
    AnyCatalogue(AnyCatalogue),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CollectivePerceptionMessage {
    pub generation_delta_time: GenerationDeltaTime,
    pub cpm_parameters: CpmParameters,
}

impl CollectivePerceptionMessage {
    pub fn new(generation_delta_time: GenerationDeltaTime, cpm_parameters: CpmParameters) -> Self {
        Self {
            generation_delta_time,
            cpm_parameters,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct CollisionRiskSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct ComparisonOperator(pub u8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=3"))]
pub struct CompleteVehicleCharacteristicsTrailer(pub SequenceOf<TrailerCharacteristics>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CompleteVehicleCharacteristics {
    pub tractor: Option<TractorCharacteristics>,
    pub trailer: Option<CompleteVehicleCharacteristicsTrailer>,
    pub train: Option<TrainCharacteristics>,
}

impl CompleteVehicleCharacteristics {
    pub fn new(
        tractor: Option<TractorCharacteristics>,
        trailer: Option<CompleteVehicleCharacteristicsTrailer>,
        train: Option<TrainCharacteristics>,
    ) -> Self {
        Self {
            tractor,
            trailer,
            train,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct ComputedLaneRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ComputedLane {
    pub reference_lane_id: LaneID,
    pub offset_xaxis: OffsetXaxis,
    pub offset_yaxis: OffsetYaxis,
    pub rotate_x_y: Option<Angle>,
    pub scale_xaxis: Option<ScaleB12>,
    pub scale_yaxis: Option<ScaleB12>,
    pub regional: Option<ComputedLaneRegional>,
}

impl ComputedLane {
    pub fn new(
        reference_lane_id: LaneID,
        offset_xaxis: OffsetXaxis,
        offset_yaxis: OffsetYaxis,
        rotate_x_y: Option<Angle>,
        scale_xaxis: Option<ScaleB12>,
        scale_yaxis: Option<ScaleB12>,
        regional: Option<ComputedLaneRegional>,
    ) -> Self {
        Self {
            reference_lane_id,
            offset_xaxis,
            offset_yaxis,
            rotate_x_y,
            scale_xaxis,
            scale_yaxis,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ComputedSegment {
    pub zone_id: Zid,
    pub lane_number: LanePosition,
    pub lane_width: IVILaneWidth,
    #[rasn(value("-32768..=32767"))]
    pub offset_distance: Option<i16>,
    pub offset_position: Option<DeltaReferencePosition>,
}

impl ComputedSegment {
    pub fn new(
        zone_id: Zid,
        lane_number: LanePosition,
        lane_width: IVILaneWidth,
        offset_distance: Option<i16>,
        offset_position: Option<DeltaReferencePosition>,
    ) -> Self {
        Self {
            zone_id,
            lane_number,
            lane_width,
            offset_distance,
            offset_position,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ConnectingLane {
    pub lane: LaneID,
    pub maneuver: Option<AllowedManeuvers>,
}

impl ConnectingLane {
    pub fn new(lane: LaneID, maneuver: Option<AllowedManeuvers>) -> Self {
        Self { lane, maneuver }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Connection {
    pub connecting_lane: ConnectingLane,
    pub remote_intersection: Option<IntersectionReferenceID>,
    pub signal_group: Option<SignalGroupID>,
    pub user_class: Option<RestrictionClassID>,
    pub connection_i_d: Option<LaneConnectionID>,
}

impl Connection {
    pub fn new(
        connecting_lane: ConnectingLane,
        remote_intersection: Option<IntersectionReferenceID>,
        signal_group: Option<SignalGroupID>,
        user_class: Option<RestrictionClassID>,
        connection_i_d: Option<LaneConnectionID>,
    ) -> Self {
        Self {
            connecting_lane,
            remote_intersection,
            signal_group,
            user_class,
            connection_i_d,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct ConnectionManeuverAssistRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ConnectionManeuverAssist {
    pub connection_i_d: LaneConnectionID,
    pub queue_length: Option<ZoneLength>,
    pub available_storage_length: Option<ZoneLength>,
    pub wait_on_stop: Option<WaitOnStopline>,
    pub ped_bicycle_detect: Option<PedestrianBicycleDetect>,
    pub regional: Option<ConnectionManeuverAssistRegional>,
}

impl ConnectionManeuverAssist {
    pub fn new(
        connection_i_d: LaneConnectionID,
        queue_length: Option<ZoneLength>,
        available_storage_length: Option<ZoneLength>,
        wait_on_stop: Option<WaitOnStopline>,
        ped_bicycle_detect: Option<PedestrianBicycleDetect>,
        regional: Option<ConnectionManeuverAssistRegional>,
    ) -> Self {
        Self {
            connection_i_d,
            queue_length,
            available_storage_length,
            wait_on_stop,
            ped_bicycle_detect,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ConnectionManeuverAssistAddGrpC {
    pub its_station_position: Option<ItsStationPositionList>,
}

impl ConnectionManeuverAssistAddGrpC {
    pub fn new(its_station_position: Option<ItsStationPositionList>) -> Self {
        Self {
            its_station_position,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ConnectionTrajectoryAddGrpC {
    pub nodes: NodeSetXY,
    pub connection_i_d: LaneConnectionID,
}

impl ConnectionTrajectoryAddGrpC {
    pub fn new(nodes: NodeSetXY, connection_i_d: LaneConnectionID) -> Self {
        Self {
            nodes,
            connection_i_d,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct ConnectsToList(pub SequenceOf<Connection>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CoopAwareness {
    pub generation_delta_time: GenerationDeltaTime,
    pub cam_parameters: CamParameters,
}

impl CoopAwareness {
    pub fn new(generation_delta_time: GenerationDeltaTime, cam_parameters: CamParameters) -> Self {
        Self {
            generation_delta_time,
            cam_parameters,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum CopValue {
    NoEntry = 0,
    Co2class1 = 1,
    Co2class2 = 2,
    Co2class3 = 3,
    Co2class4 = 4,
    Co2class5 = 5,
    Co2class6 = 6,
    Co2class7 = 7,
    ReservedforUse = 8,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CountryAndRegions {
    pub country_only: CountryOnly,
    pub regions: SequenceOfUint8,
}

impl CountryAndRegions {
    pub fn new(country_only: CountryOnly, regions: SequenceOfUint8) -> Self {
        Self {
            country_only,
            regions,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct CountryAndSubregions {
    pub country: CountryOnly,
    pub region_and_subregions: SequenceOfRegionAndSubregions,
}

impl CountryAndSubregions {
    pub fn new(country: CountryOnly, region_and_subregions: SequenceOfRegionAndSubregions) -> Self {
        Self {
            country,
            region_and_subregions,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("10"))]
pub struct CountryCode(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct CountryOnly(pub Uint16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CpmManagementContainer {
    pub station_type: StationType,
    pub perceived_object_container_segment_info: Option<PerceivedObjectContainerSegmentInfo>,
    pub reference_position: ReferencePosition,
}

impl CpmManagementContainer {
    pub fn new(
        station_type: StationType,
        perceived_object_container_segment_info: Option<PerceivedObjectContainerSegmentInfo>,
        reference_position: ReferencePosition,
    ) -> Self {
        Self {
            station_type,
            perceived_object_container_segment_info,
            reference_position,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct CpmParameters {
    pub management_container: CpmManagementContainer,
    pub station_data_container: Option<StationDataContainer>,
    pub sensor_information_container: Option<SensorInformationContainer>,
    pub perceived_object_container: Option<PerceivedObjectContainer>,
    pub free_space_addendum_container: Option<FreeSpaceAddendumContainer>,
    pub number_of_perceived_objects: NumberOfPerceivedObjects,
}

impl CpmParameters {
    pub fn new(
        management_container: CpmManagementContainer,
        station_data_container: Option<StationDataContainer>,
        sensor_information_container: Option<SensorInformationContainer>,
        perceived_object_container: Option<PerceivedObjectContainer>,
        free_space_addendum_container: Option<FreeSpaceAddendumContainer>,
        number_of_perceived_objects: NumberOfPerceivedObjects,
    ) -> Self {
        Self {
            management_container,
            station_data_container,
            sensor_information_container,
            perceived_object_container,
            free_space_addendum_container,
            number_of_perceived_objects,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct CrlSeries(pub Uint16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct CtxRef(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Curvature {
    pub curvature_value: CurvatureValue,
    pub curvature_confidence: CurvatureConfidence,
}

impl Curvature {
    pub fn new(curvature_value: CurvatureValue, curvature_confidence: CurvatureConfidence) -> Self {
        Self {
            curvature_value,
            curvature_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum CurvatureCalculationMode {
    YawRateUsed = 0,
    YawRateNotUsed = 1,
    Unavailable = 2,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum CurvatureConfidence {
    OnePerMeter000002 = 0,
    OnePerMeter00001 = 1,
    OnePerMeter00005 = 2,
    OnePerMeter0002 = 3,
    OnePerMeter001 = 4,
    OnePerMeter01 = 5,
    OutOfRange = 6,
    Unavailable = 7,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1023..=1023"))]
pub struct CurvatureValue(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct DBV(pub Distance);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct DDDIoList(pub SequenceOf<DDDIO>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DDD {
    #[rasn(value("1..=128"))]
    pub dcj: Option<u8>,
    #[rasn(value("1..=128"))]
    pub dcr: Option<u8>,
    #[rasn(value("1..=128"))]
    pub tpl: Option<u8>,
    pub io_list: DDDIoList,
}

impl DDD {
    pub fn new(dcj: Option<u8>, dcr: Option<u8>, tpl: Option<u8>, io_list: DDDIoList) -> Self {
        Self {
            dcj,
            dcr,
            tpl,
            io_list,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15", extensible))]
pub struct DDDDEP(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15", extensible))]
pub struct DDDDER(pub Integer);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct DDDIODp(pub SequenceOf<DestinationPlace>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct DDDIODr(pub SequenceOf<DestinationRoad>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DDDIO {
    #[rasn(value("0..=7"))]
    pub drn: u8,
    pub dp: Option<DDDIODp>,
    pub dr: Option<DDDIODr>,
    #[rasn(value("1..=999"))]
    pub rne: Option<u16>,
    #[rasn(value("1..=999"))]
    pub stn_id: Option<u16>,
    pub stn_text: Option<Utf8String>,
    pub dcp: Option<DistanceOrDuration>,
    pub ddp: Option<DistanceOrDuration>,
}

impl DDDIO {
    pub fn new(
        drn: u8,
        dp: Option<DDDIODp>,
        dr: Option<DDDIODr>,
        rne: Option<u16>,
        stn_id: Option<u16>,
        stn_text: Option<Utf8String>,
        dcp: Option<DistanceOrDuration>,
        ddp: Option<DistanceOrDuration>,
    ) -> Self {
        Self {
            drn,
            dp,
            dr,
            rne,
            stn_id,
            stn_text,
            dcp,
            ddp,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DDateTime {
    pub year: Option<DYear>,
    pub month: Option<DMonth>,
    pub day: Option<DDay>,
    pub hour: Option<DHour>,
    pub minute: Option<DMinute>,
    pub second: Option<DSecond>,
    pub offset: Option<DOffset>,
}

impl DDateTime {
    pub fn new(
        year: Option<DYear>,
        month: Option<DMonth>,
        day: Option<DDay>,
        hour: Option<DHour>,
        minute: Option<DMinute>,
        second: Option<DSecond>,
        offset: Option<DOffset>,
    ) -> Self {
        Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            offset,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=31"))]
pub struct DDay(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DENM {
    pub header: ItsPduHeader,
    pub denm: DecentralizedEnvironmentalNotificationMessage,
}

impl DENM {
    pub fn new(header: ItsPduHeader, denm: DecentralizedEnvironmentalNotificationMessage) -> Self {
        Self { header, denm }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=8"))]
pub struct DFL(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=31"))]
pub struct DHour(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=60"))]
pub struct DMinute(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=12"))]
pub struct DMonth(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-840..=840"))]
pub struct DOffset(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=32767"))]
pub struct DSRCmsgID(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct DSecond(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DTM {
    pub year: Option<DtmYear>,
    pub month_day: Option<DtmMonthDay>,
    pub pmd: Option<PMD>,
    pub hour_minutes: Option<DtmHourMinutes>,
    pub day_of_week: Option<DayOfWeek>,
    pub period: Option<HoursMinutes>,
}

impl DTM {
    pub fn new(
        year: Option<DtmYear>,
        month_day: Option<DtmMonthDay>,
        pmd: Option<PMD>,
        hour_minutes: Option<DtmHourMinutes>,
        day_of_week: Option<DayOfWeek>,
        period: Option<HoursMinutes>,
    ) -> Self {
        Self {
            year,
            month_day,
            pmd,
            hour_minutes,
            day_of_week,
            period,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4095"))]
pub struct DYear(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct DangerousEndOfQueueSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum DangerousGoodsBasic {
    Explosives1 = 0,
    Explosives2 = 1,
    Explosives3 = 2,
    Explosives4 = 3,
    Explosives5 = 4,
    Explosives6 = 5,
    FlammableGases = 6,
    NonFlammableGases = 7,
    ToxicGases = 8,
    FlammableLiquids = 9,
    FlammableSolids = 10,
    SubstancesLiableToSpontaneousCombustion = 11,
    SubstancesEmittingFlammableGasesUponContactWithWater = 12,
    OxidizingSubstances = 13,
    OrganicPeroxides = 14,
    ToxicSubstances = 15,
    InfectiousSubstances = 16,
    RadioactiveMaterial = 17,
    CorrosiveSubstances = 18,
    MiscellaneousDangerousSubstances = 19,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DangerousGoodsContainer {
    pub dangerous_goods_basic: DangerousGoodsBasic,
}

impl DangerousGoodsContainer {
    pub fn new(dangerous_goods_basic: DangerousGoodsBasic) -> Self {
        Self {
            dangerous_goods_basic,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DangerousGoodsExtended {
    pub dangerous_goods_type: DangerousGoodsBasic,
    #[rasn(value("0..=9999"))]
    pub un_number: u16,
    pub elevated_temperature: bool,
    pub tunnels_restricted: bool,
    pub limited_quantity: bool,
    #[rasn(size("1..=24"))]
    pub emergency_action_code: Option<Ia5String>,
    pub phone_number: Option<PhoneNumber>,
    #[rasn(size("1..=24"))]
    pub company_name: Option<Utf8String>,
}

impl DangerousGoodsExtended {
    pub fn new(
        dangerous_goods_type: DangerousGoodsBasic,
        un_number: u16,
        elevated_temperature: bool,
        tunnels_restricted: bool,
        limited_quantity: bool,
        emergency_action_code: Option<Ia5String>,
        phone_number: Option<PhoneNumber>,
        company_name: Option<Utf8String>,
    ) -> Self {
        Self {
            dangerous_goods_type,
            un_number,
            elevated_temperature,
            tunnels_restricted,
            limited_quantity,
            emergency_action_code,
            phone_number,
            company_name,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct DangerousSituationSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct DataParameters {
    #[rasn(size("1..=255"))]
    pub process_method: Option<Ia5String>,
    #[rasn(size("1..=255"))]
    pub process_agency: Option<Ia5String>,
    #[rasn(size("1..=255"))]
    pub last_checked_date: Option<Ia5String>,
    #[rasn(size("1..=255"))]
    pub geoid_used: Option<Ia5String>,
}

impl DataParameters {
    pub fn new(
        process_method: Option<Ia5String>,
        process_agency: Option<Ia5String>,
        last_checked_date: Option<Ia5String>,
        geoid_used: Option<Ia5String>,
    ) -> Self {
        Self {
            process_method,
            process_agency,
            last_checked_date,
            geoid_used,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct DataRate80211(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct DayOfWeek(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DecentralizedEnvironmentalNotificationMessage {
    pub management: ManagementContainer,
    pub situation: Option<SituationContainer>,
    pub location: Option<LocationContainer>,
    pub alacarte: Option<AlacarteContainer>,
}

impl DecentralizedEnvironmentalNotificationMessage {
    pub fn new(
        management: ManagementContainer,
        situation: Option<SituationContainer>,
        location: Option<LocationContainer>,
        alacarte: Option<AlacarteContainer>,
    ) -> Self {
        Self {
            management,
            situation,
            location,
            alacarte,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-12700..=12800"))]
pub struct DeltaAltitude(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-150..=150"))]
pub struct DeltaAngle(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-131071..=131072"))]
pub struct DeltaLatitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-131071..=131072"))]
pub struct DeltaLongitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DeltaPosition {
    pub delta_latitude: DeltaLatitude,
    pub delta_longitude: DeltaLongitude,
}

impl DeltaPosition {
    pub fn new(delta_latitude: DeltaLatitude, delta_longitude: DeltaLongitude) -> Self {
        Self {
            delta_latitude,
            delta_longitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32", extensible))]
pub struct DeltaPositions(pub SequenceOf<DeltaPosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32", extensible))]
pub struct DeltaPositionsWithAltitude(pub SequenceOf<DeltaReferencePosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DeltaReferencePosition {
    pub delta_latitude: DeltaLatitude,
    pub delta_longitude: DeltaLongitude,
    pub delta_altitude: DeltaAltitude,
}

impl DeltaReferencePosition {
    pub fn new(
        delta_latitude: DeltaLatitude,
        delta_longitude: DeltaLongitude,
        delta_altitude: DeltaAltitude,
    ) -> Self {
        Self {
            delta_latitude,
            delta_longitude,
            delta_altitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-122..=121"))]
pub struct DeltaTime(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=63"))]
pub struct DescriptiveName(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DestinationPlace {
    pub dep_type: DDDDEP,
    pub dep_r_s_code: Option<ISO14823Code>,
    pub dep_blob: Option<OctetString>,
    #[rasn(value("1..=999"))]
    pub pln_id: Option<u16>,
    pub pln_text: Option<Utf8String>,
}

impl DestinationPlace {
    pub fn new(
        dep_type: DDDDEP,
        dep_r_s_code: Option<ISO14823Code>,
        dep_blob: Option<OctetString>,
        pln_id: Option<u16>,
        pln_text: Option<Utf8String>,
    ) -> Self {
        Self {
            dep_type,
            dep_r_s_code,
            dep_blob,
            pln_id,
            pln_text,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DestinationRoad {
    pub der_type: DDDDER,
    #[rasn(value("1..=999"))]
    pub ron_id: Option<u16>,
    pub ron_text: Option<Utf8String>,
}

impl DestinationRoad {
    pub fn new(der_type: DDDDER, ron_id: Option<u16>, ron_text: Option<Utf8String>) -> Self {
        Self {
            der_type,
            ron_id,
            ron_text,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum DetectionArea {
    VehicleSensor(VehicleSensor),
    StationarySensorRadial(AreaRadial),
    StationarySensorPolygon(AreaPolygon),
    StationarySensorCircular(AreaCircular),
    StationarySensorEllipse(AreaEllipse),
    StationarySensorRectangle(AreaRectangle),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DieselEmissionValues {
    pub particulate: Particulate,
    pub absorption_coeff: Int2,
}

impl DieselEmissionValues {
    pub fn new(particulate: Particulate, absorption_coeff: Int2) -> Self {
        Self {
            particulate,
            absorption_coeff,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=256"))]
pub struct DigitalMap(pub SequenceOf<ReferencePosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct Direction(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Distance {
    #[rasn(value("1..=16384"))]
    pub value: u16,
    pub unit: RSCUnit,
}

impl Distance {
    pub fn new(value: u16, unit: RSCUnit) -> Self {
        Self { value, unit }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=102"))]
pub struct DistanceConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DistanceOrDuration {
    #[rasn(value("1..=16384"))]
    pub value: u16,
    pub unit: RSCUnit,
}

impl DistanceOrDuration {
    pub fn new(value: u16, unit: RSCUnit) -> Self {
        Self { value, unit }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-132768..=132767"))]
pub struct DistanceValue(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Dot2Elevation(pub ElevInt);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Dot2Latitude(pub NinetyDegreeInt);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Dot2Longitude(pub OneEightyDegreeInt);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Dot2ThreeDLocation {
    pub latitude: Dot2Latitude,
    pub longitude: Dot2Longitude,
    pub elevation: Dot2Elevation,
}

impl Dot2ThreeDLocation {
    pub fn new(latitude: Dot2Latitude, longitude: Dot2Longitude, elevation: Dot2Elevation) -> Self {
        Self {
            latitude,
            longitude,
            elevation,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Dot2TwoDLocation {
    pub latitude: Dot2Latitude,
    pub longitude: Dot2Longitude,
}

impl Dot2TwoDLocation {
    pub fn new(latitude: Dot2Latitude, longitude: Dot2Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum DriveDirection {
    Forward = 0,
    Backward = 1,
    Unavailable = 2,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-32767..=32767"))]
pub struct DrivenLineOffsetLg(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-2047..=2047"))]
pub struct DrivenLineOffsetSm(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct DriverCharacteristics(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=13"))]
pub struct DrivingLaneStatus(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DtmHourMinutes {
    pub shm: HoursMinutes,
    pub ehm: HoursMinutes,
}

impl DtmHourMinutes {
    pub fn new(shm: HoursMinutes, ehm: HoursMinutes) -> Self {
        Self { shm, ehm }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DtmMonthDay {
    pub smd: MonthDay,
    pub emd: MonthDay,
}

impl DtmMonthDay {
    pub fn new(smd: MonthDay, emd: MonthDay) -> Self {
        Self { smd, emd }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct DtmYear {
    #[rasn(value("2000..=2127", extensible))]
    pub syr: u16,
    #[rasn(value("2000..=2127", extensible))]
    pub eyr: u16,
}

impl DtmYear {
    pub fn new(syr: u16, eyr: u16) -> Self {
        Self { syr, eyr }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Duration {
    Microseconds(Uint16),
    Milliseconds(Uint16),
    Seconds(Uint16),
    Minutes(Uint16),
    Hours(Uint16),
    SixtyHours(Uint16),
    Years(Uint16),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=2"))]
pub struct DynamicStatus(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct EDT(pub DTM);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EVChargingSpotNotificationPOIMessage {
    pub poi_header: ItsPOIHeader,
    pub evcsn_data: ItsEVCSNData,
}

impl EVChargingSpotNotificationPOIMessage {
    pub fn new(poi_header: ItsPOIHeader, evcsn_data: ItsEVCSNData) -> Self {
        Self {
            poi_header,
            evcsn_data,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EccP256CurvePointUncompressedP256 {
    #[rasn(size("32"))]
    pub x: OctetString,
    #[rasn(size("32"))]
    pub y: OctetString,
}

impl EccP256CurvePointUncompressedP256 {
    pub fn new(x: OctetString, y: OctetString) -> Self {
        Self { x, y }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EccP256CurvePoint {
    #[rasn(size("32"))]
    XOnly(OctetString),
    Fill(()),
    #[rasn(size("32"))]
    CompressedY0(OctetString),
    #[rasn(size("32"))]
    CompressedY1(OctetString),
    UncompressedP256(EccP256CurvePointUncompressedP256),
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EccP384CurvePointUncompressedP384 {
    #[rasn(size("48"))]
    pub x: OctetString,
    #[rasn(size("48"))]
    pub y: OctetString,
}

impl EccP384CurvePointUncompressedP384 {
    pub fn new(x: OctetString, y: OctetString) -> Self {
        Self { x, y }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EccP384CurvePoint {
    #[rasn(size("48"))]
    XOnly(OctetString),
    Fill(()),
    #[rasn(size("48"))]
    CompressedY0(OctetString),
    #[rasn(size("48"))]
    CompressedY1(OctetString),
    UncompressedP384(EccP384CurvePointUncompressedP384),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EcdsaP256Signature {
    pub r_sig: EccP256CurvePoint,
    #[rasn(size("32"))]
    pub s_sig: OctetString,
}

impl EcdsaP256Signature {
    pub fn new(r_sig: EccP256CurvePoint, s_sig: OctetString) -> Self {
        Self { r_sig, s_sig }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EcdsaP384Signature {
    pub r_sig: EccP384CurvePoint,
    #[rasn(size("48"))]
    pub s_sig: OctetString,
}

impl EcdsaP384Signature {
    pub fn new(r_sig: EccP384CurvePoint, s_sig: OctetString) -> Self {
        Self { r_sig, s_sig }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EciesP256EncryptedKey {
    pub v: EccP256CurvePoint,
    #[rasn(size("16"))]
    pub c: OctetString,
    #[rasn(size("16"))]
    pub t: OctetString,
}

impl EciesP256EncryptedKey {
    pub fn new(v: EccP256CurvePoint, c: OctetString, t: OctetString) -> Self {
        Self { v, c, t }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum EdcaIdentifier {
    Enum(EnumeratedEdcaIdentifier),
    Explicit(ExplicitEdcaIdentifier),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EdcaParameterRecord {
    #[rasn(value("0..=1"))]
    pub res: u8,
    #[rasn(value("0..=3"))]
    pub aci: u8,
    #[rasn(value("0..=1"))]
    pub acm: u8,
    #[rasn(value("0..=15"))]
    pub aifsn: u8,
    #[rasn(value("0..=15"))]
    pub ecw_max: u8,
    #[rasn(value("0..=15"))]
    pub ecw_min: u8,
    #[rasn(value("0..=65535"))]
    pub txop_limit: u16,
}

impl EdcaParameterRecord {
    pub fn new(
        res: u8,
        aci: u8,
        acm: u8,
        aifsn: u8,
        ecw_max: u8,
        ecw_min: u8,
        txop_limit: u16,
    ) -> Self {
        Self {
            res,
            aci,
            acm,
            aifsn,
            ecw_max,
            ecw_min,
            txop_limit,
        }
    }
}

///ChannelInfo extension elements

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EdcaParameterSet {
    pub acbe_record: EdcaParameterRecord,
    pub acbk_record: EdcaParameterRecord,
    pub acvi_record: EdcaParameterRecord,
    pub acvo_record: EdcaParameterRecord,
}

impl EdcaParameterSet {
    pub fn new(
        acbe_record: EdcaParameterRecord,
        acbk_record: EdcaParameterRecord,
        acvi_record: EdcaParameterRecord,
        acvo_record: EdcaParameterRecord,
    ) -> Self {
        Self {
            acbe_record,
            acbk_record,
            acvi_record,
            acvo_record,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ElevInt(pub Uint16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-4096..=61439"))]
pub struct Elevation(pub i32);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum ElevationConfidence {
    Unavailable = 0,
    Elev50000 = 1,
    Elev20000 = 2,
    Elev10000 = 3,
    Elev05000 = 4,
    Elev02000 = 5,
    Elev01000 = 6,
    Elev00500 = 7,
    Elev00200 = 8,
    Elev00100 = 9,
    Elev00050 = 10,
    Elev00020 = 11,
    Elev00010 = 12,
    Elev00005 = 13,
    Elev00002 = 14,
    Elev00001 = 15,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct EmbarkationStatus(pub bool);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EmergencyContainer {
    pub light_bar_siren_in_use: LightBarSirenInUse,
    pub incident_indication: Option<CauseCode>,
    pub emergency_priority: Option<EmergencyPriority>,
}

impl EmergencyContainer {
    pub fn new(
        light_bar_siren_in_use: LightBarSirenInUse,
        incident_indication: Option<CauseCode>,
        emergency_priority: Option<EmergencyPriority>,
    ) -> Self {
        Self {
            light_bar_siren_in_use,
            incident_indication,
            emergency_priority,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("2"))]
pub struct EmergencyPriority(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct EmergencyVehicleApproachingSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum EmissionType {
    Euro1 = 0,
    Euro2 = 1,
    Euro3 = 2,
    Euro4 = 3,
    Euro5 = 4,
    Euro6 = 5,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct EnabledLaneList(pub SequenceOf<LaneID>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EncryptionKey {
    Public(PublicEncryptionKey),
    Symmetric(SymmetricEncryptionKey),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("7"))]
pub struct EnergyStorageType(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct EngineCharacteristics(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum EnumeratedEdcaIdentifier {
    UsJ2945Bsm = 0,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EnvironmentalCharacteristics {
    pub euro_value: EuroValue,
    pub cop_value: CopValue,
}

impl EnvironmentalCharacteristics {
    pub fn new(euro_value: EuroValue, cop_value: CopValue) -> Self {
        Self {
            euro_value,
            cop_value,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum EuVehicleCategoryCode {
    EuVehicleCategoryL(EuVehicleCategoryL),
    EuVehicleCategoryM(EuVehicleCategoryM),
    EuVehicleCategoryN(EuVehicleCategoryN),
    EuVehicleCategoryO(EuVehicleCategoryO),
    EuVehilcleCategoryT(()),
    EuVehilcleCategoryG(()),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryL {
    L1 = 0,
    L2 = 1,
    L3 = 2,
    L4 = 3,
    L5 = 4,
    L6 = 5,
    L7 = 6,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryM {
    M1 = 0,
    M2 = 1,
    M3 = 2,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryN {
    N1 = 0,
    N2 = 1,
    N3 = 2,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuVehicleCategoryO {
    O1 = 0,
    O2 = 1,
    O3 = 2,
    O4 = 3,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum EuroValue {
    NoEntry = 0,
    Euro1 = 1,
    Euro2 = 2,
    Euro3 = 3,
    Euro4 = 4,
    Euro5 = 5,
    Euro6 = 6,
    ReservedForUse1 = 7,
    ReservedForUse2 = 8,
    ReservedForUse3 = 9,
    ReservedForUse4 = 10,
    ReservedForUse5 = 11,
    ReservedForUse6 = 12,
    ReservedForUse7 = 13,
    ReservedForUse8 = 14,
    Eev = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EvcsnPdu {
    pub header: ItsPduHeader,
    pub evcsn: EVChargingSpotNotificationPOIMessage,
}

impl EvcsnPdu {
    pub fn new(header: ItsPduHeader, evcsn: EVChargingSpotNotificationPOIMessage) -> Self {
        Self { header, evcsn }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=23"))]
pub struct EventHistory(pub SequenceOf<EventPoint>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct EventPoint {
    pub event_position: DeltaReferencePosition,
    pub event_delta_time: Option<PathDeltaTime>,
    pub information_quality: InformationQuality,
}

impl EventPoint {
    pub fn new(
        event_position: DeltaReferencePosition,
        event_delta_time: Option<PathDeltaTime>,
        information_quality: InformationQuality,
    ) -> Self {
        Self {
            event_position,
            event_delta_time,
            information_quality,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ExceptionalCondition {
    Unknown = 0,
    PublicTransportPriority = 1,
    EmergencyVehiclePriority = 2,
    TrainPriority = 3,
    BridgeOpen = 4,
    VehicleHeight = 5,
    Weather = 6,
    TrafficJam = 7,
    TunnelClosure = 8,
    MeteringActive = 9,
    TruckPriority = 10,
    BicyclePlatoonPriority = 11,
    VehiclePlatoonPriority = 12,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ExhaustEmissionValues {
    pub unit_type: UnitType,
    #[rasn(value("0..=32767"))]
    pub emission_c_o: u16,
    pub emission_h_c: Int2,
    pub emission_n_o_x: Int2,
    pub emission_h_c_n_o_x: Int2,
}

impl ExhaustEmissionValues {
    pub fn new(
        unit_type: UnitType,
        emission_c_o: u16,
        emission_h_c: Int2,
        emission_n_o_x: Int2,
        emission_h_c_n_o_x: Int2,
    ) -> Self {
        Self {
            unit_type,
            emission_c_o,
            emission_h_c,
            emission_n_o_x,
            emission_h_c_n_o_x,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ExplicitEdcaIdentifier {
    pub qos_info: Uint8,
    pub reserved: Uint8,
    #[rasn(size("4"))]
    pub set1: OctetString,
    #[rasn(size("4"))]
    pub set2: OctetString,
    #[rasn(size("4"))]
    pub set3: OctetString,
    #[rasn(size("4"))]
    pub set4: OctetString,
}

impl ExplicitEdcaIdentifier {
    pub fn new(
        qos_info: Uint8,
        reserved: Uint8,
        set1: OctetString,
        set2: OctetString,
        set3: OctetString,
        set4: OctetString,
    ) -> Self {
        Self {
            qos_info,
            reserved,
            set1,
            set2,
            set3,
            set4,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Ext1 {
    #[rasn(value("128..=16511"))]
    Content(u16),
    Extension(Ext2),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum Ext2 {
    #[rasn(value("16512..=2113663"))]
    Content(u32),
    Extension(Ext3),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("2113664..=270549119", extensible))]
pub struct Ext3(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ExtendedChannelInfo {
    pub med_id: MedType,
    pub value: Any,
}

impl ExtendedChannelInfo {
    pub fn new(med_id: MedType, value: Any) -> Self {
        Self { med_id, value }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ExtendedChannelInfos(pub SequenceOf<ExtendedChannelInfo>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Extension {
    pub extension_id: RefExt,
    pub value: Any,
}

impl Extension {
    pub fn new(extension_id: RefExt, value: Any) -> Self {
        Self {
            extension_id,
            value,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct ExteriorLights(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct FirstIntersection {
    #[rasn(value("1..=32"))]
    pub number_of_intersections: u8,
    pub partial_intersection: PartialIntersection,
}

impl FirstIntersection {
    pub fn new(number_of_intersections: u8, partial_intersection: PartialIntersection) -> Self {
        Self {
            number_of_intersections,
            partial_intersection,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct FirstSpat {
    #[rasn(value("1..=32"))]
    pub number_of_intersections: u8,
    pub partial_spat_intersection: PartialSpatIntersection,
}

impl FirstSpat {
    pub fn new(
        number_of_intersections: u8,
        partial_spat_intersection: PartialSpatIntersection,
    ) -> Self {
        Self {
            number_of_intersections,
            partial_spat_intersection,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct FreeSpaceAddendum {
    pub free_space_confidence: FreeSpaceConfidence,
    pub free_space_area: FreeSpaceArea,
    pub sensor_i_d_list: Option<SensorIdList>,
    #[rasn(default = "free_space_addendum_shadowing_applies_default")]
    pub shadowing_applies: ShadowingApplies,
}

impl FreeSpaceAddendum {
    pub fn new(
        free_space_confidence: FreeSpaceConfidence,
        free_space_area: FreeSpaceArea,
        sensor_i_d_list: Option<SensorIdList>,
        shadowing_applies: ShadowingApplies,
    ) -> Self {
        Self {
            free_space_confidence,
            free_space_area,
            sensor_i_d_list,
            shadowing_applies,
        }
    }
}

fn free_space_addendum_shadowing_applies_default() -> ShadowingApplies {
    ShadowingApplies(true).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=128", extensible))]
pub struct FreeSpaceAddendumContainer(pub SequenceOf<FreeSpaceAddendum>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum FreeSpaceArea {
    FreeSpacePolygon(AreaPolygon),
    FreeSpaceCircular(AreaCircular),
    FreeSpaceEllipse(AreaEllipse),
    FreeSpaceRectangle(AreaRectangle),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=101"))]
pub struct FreeSpaceConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=50"))]
pub struct FrontOverhang(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct FuelType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct FullPositionVector {
    pub utc_time: Option<DDateTime>,
    pub long: Longitude,
    pub lat: Latitude,
    pub elevation: Option<Elevation>,
    pub heading: Option<SAEHeading>,
    pub speed: Option<TransmissionAndSpeed>,
    pub pos_accuracy: Option<PositionalAccuracy>,
    pub time_confidence: Option<TimeConfidence>,
    pub pos_confidence: Option<PositionConfidenceSet>,
    pub speed_confidence: Option<SpeedandHeadingandThrottleConfidence>,
}

impl FullPositionVector {
    pub fn new(
        utc_time: Option<DDateTime>,
        long: Longitude,
        lat: Latitude,
        elevation: Option<Elevation>,
        heading: Option<SAEHeading>,
        speed: Option<TransmissionAndSpeed>,
        pos_accuracy: Option<PositionalAccuracy>,
        time_confidence: Option<TimeConfidence>,
        pos_confidence: Option<PositionConfidenceSet>,
        speed_confidence: Option<SpeedandHeadingandThrottleConfidence>,
    ) -> Self {
        Self {
            utc_time,
            long,
            lat,
            elevation,
            heading,
            speed,
            pos_accuracy,
            time_confidence,
            pos_confidence,
            speed_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct GNSSstatus(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct GatewayMacAddress(pub MACaddress);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct GeneralIviContainer(pub SequenceOf<GicPart>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct GenerationDeltaTime(pub u16);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct GenericLaneRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GenericLane {
    pub lane_i_d: LaneID,
    pub name: Option<DescriptiveName>,
    pub ingress_approach: Option<ApproachID>,
    pub egress_approach: Option<ApproachID>,
    pub lane_attributes: LaneAttributes,
    pub maneuvers: Option<AllowedManeuvers>,
    pub node_list: NodeListXY,
    pub connects_to: Option<ConnectsToList>,
    pub overlays: Option<OverlayLaneList>,
    pub regional: Option<GenericLaneRegional>,
}

impl GenericLane {
    pub fn new(
        lane_i_d: LaneID,
        name: Option<DescriptiveName>,
        ingress_approach: Option<ApproachID>,
        egress_approach: Option<ApproachID>,
        lane_attributes: LaneAttributes,
        maneuvers: Option<AllowedManeuvers>,
        node_list: NodeListXY,
        connects_to: Option<ConnectsToList>,
        overlays: Option<OverlayLaneList>,
        regional: Option<GenericLaneRegional>,
    ) -> Self {
        Self {
            lane_i_d,
            name,
            ingress_approach,
            egress_approach,
            lane_attributes,
            maneuvers,
            node_list,
            connects_to,
            overlays,
            regional,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct GeographicLocationContainerParts(pub SequenceOf<GlcPart>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GeographicLocationContainer {
    pub reference_position: ReferencePosition,
    pub reference_position_time: Option<TimestampIts>,
    pub reference_position_heading: Option<Heading>,
    pub reference_position_speed: Option<Speed>,
    pub parts: GeographicLocationContainerParts,
}

impl GeographicLocationContainer {
    pub fn new(
        reference_position: ReferencePosition,
        reference_position_time: Option<TimestampIts>,
        reference_position_heading: Option<Heading>,
        reference_position_speed: Option<Speed>,
        parts: GeographicLocationContainerParts,
    ) -> Self {
        Self {
            reference_position,
            reference_position_time,
            reference_position_heading,
            reference_position_speed,
            parts,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum GeographicRegion {
    CircularRegion(CircularRegion),
    RectangularRegion(SequenceOfRectangularRegion),
    PolygonalRegion(PolygonalRegion),
    IdentifiedRegion(SequenceOfIdentifiedRegion),
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct GicPartDetectionZoneIds(pub SequenceOf<Zid>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct GicPartRelevanceZoneIds(pub SequenceOf<Zid>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct GicPartDriverAwarenessZoneIds(pub SequenceOf<Zid>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct GicPartApplicableLanes(pub SequenceOf<LanePosition>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct GicPartVehicleCharacteristics(pub SequenceOf<CompleteVehicleCharacteristics>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct GicPartRoadSignCodes(pub SequenceOf<RSCode>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct GicPartExtraText(pub SequenceOf<Text>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GicPart {
    pub detection_zone_ids: Option<GicPartDetectionZoneIds>,
    pub its__rrid: Option<VarLengthNumber>,
    pub relevance_zone_ids: Option<GicPartRelevanceZoneIds>,
    pub direction: Option<Direction>,
    pub driver_awareness_zone_ids: Option<GicPartDriverAwarenessZoneIds>,
    #[rasn(value("0..=255"))]
    pub minimum_awareness_time: Option<u8>,
    pub applicable_lanes: Option<GicPartApplicableLanes>,
    pub ivi_type: IviType,
    pub ivi_purpose: Option<IviPurpose>,
    pub lane_status: Option<LaneStatus>,
    pub vehicle_characteristics: Option<GicPartVehicleCharacteristics>,
    pub driver_characteristics: Option<DriverCharacteristics>,
    #[rasn(value("1..=4", extensible))]
    pub layout_id: Option<u8>,
    #[rasn(value("1..=64", extensible))]
    pub pre_storedlayout_id: Option<u8>,
    pub road_sign_codes: GicPartRoadSignCodes,
    pub extra_text: Option<GicPartExtraText>,
}

impl GicPart {
    pub fn new(
        detection_zone_ids: Option<GicPartDetectionZoneIds>,
        its__rrid: Option<VarLengthNumber>,
        relevance_zone_ids: Option<GicPartRelevanceZoneIds>,
        direction: Option<Direction>,
        driver_awareness_zone_ids: Option<GicPartDriverAwarenessZoneIds>,
        minimum_awareness_time: Option<u8>,
        applicable_lanes: Option<GicPartApplicableLanes>,
        ivi_type: IviType,
        ivi_purpose: Option<IviPurpose>,
        lane_status: Option<LaneStatus>,
        vehicle_characteristics: Option<GicPartVehicleCharacteristics>,
        driver_characteristics: Option<DriverCharacteristics>,
        layout_id: Option<u8>,
        pre_storedlayout_id: Option<u8>,
        road_sign_codes: GicPartRoadSignCodes,
        extra_text: Option<GicPartExtraText>,
    ) -> Self {
        Self {
            detection_zone_ids,
            its__rrid,
            relevance_zone_ids,
            direction,
            driver_awareness_zone_ids,
            minimum_awareness_time,
            applicable_lanes,
            ivi_type,
            ivi_purpose,
            lane_status,
            vehicle_characteristics,
            driver_characteristics,
            layout_id,
            pre_storedlayout_id,
            road_sign_codes,
            extra_text,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct GlcPart {
    pub zone_id: Zid,
    pub lane_number: Option<LanePosition>,
    #[rasn(value("0..=255"))]
    pub zone_extension: Option<u8>,
    pub zone_heading: Option<HeadingValue>,
    pub zone: Option<Zone>,
}

impl GlcPart {
    pub fn new(
        zone_id: Zid,
        lane_number: Option<LanePosition>,
        zone_extension: Option<u8>,
        zone_heading: Option<HeadingValue>,
        zone: Option<Zone>,
    ) -> Self {
        Self {
            zone_id,
            lane_number,
            zone_extension,
            zone_heading,
            zone,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15", extensible))]
pub struct GoodsType(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct GroupLinkageValue {
    #[rasn(size("4"))]
    pub j_value: OctetString,
    #[rasn(size("9"))]
    pub value: OctetString,
}

impl GroupLinkageValue {
    pub fn new(j_value: OctetString, value: OctetString) -> Self {
        Self { j_value, value }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum HardShoulderStatus {
    AvailableForStopping = 0,
    Closed = 1,
    AvailableForDriving = 2,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum HashAlgorithm {
    Sha256 = 0,
    #[rasn(extension_addition)]
    Sha384 = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("10"))]
pub struct HashedId10(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("3"))]
pub struct HashedId3(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct HashedId8(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationAnimalOnTheRoadSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationDangerousCurveSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationObstacleOnTheRoadSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationSurfaceConditionSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Heading {
    pub heading_value: HeadingValue,
    pub heading_confidence: HeadingConfidence,
}

impl Heading {
    pub fn new(heading_value: HeadingValue, heading_confidence: HeadingConfidence) -> Self {
        Self {
            heading_value,
            heading_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct HeadingChangeIndication {
    pub direction: LeftOrRight,
    pub action_delta_time: ActionDeltaTime,
}

impl HeadingChangeIndication {
    pub fn new(direction: LeftOrRight, action_delta_time: ActionDeltaTime) -> Self {
        Self {
            direction,
            action_delta_time,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct HeadingConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3601"))]
pub struct HeadingValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=100"))]
pub struct HeightLonCarr(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum HighFrequencyContainer {
    BasicVehicleContainerHighFrequency(BasicVehicleContainerHighFrequency),
    RsuContainerHighFrequency(RSUContainerHighFrequency),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=100"))]
pub struct HitchPointOffset(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=255"))]
pub struct Hostname(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct HoursMinutes {
    #[rasn(value("0..=23"))]
    pub hours: u8,
    #[rasn(value("0..=59"))]
    pub mins: u8,
}

impl HoursMinutes {
    pub fn new(hours: u8, mins: u8) -> Self {
        Self { hours, mins }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HumanPresenceOnTheRoadSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct HumanProblemSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct IPv6Address(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum ISO14823Attribute {
    Dtm(DTM),
    Edt(EDT),
    Dfl(DFL),
    Ved(VED),
    Spe(SPE),
    Roi(ROI),
    Dbv(DBV),
    Ddd(DDD),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct ISO14823Attributes(pub SequenceOf<ISO14823Attribute>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ISO14823Code {
    pub pictogram_code: PictogramCode,
    pub attributes: Option<ISO14823Attributes>,
}

impl ISO14823Code {
    pub fn new(pictogram_code: PictogramCode, attributes: Option<ISO14823Attributes>) -> Self {
        Self {
            pictogram_code,
            attributes,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ITSRangingSAMAppData {
    #[rasn(value("0..=255"))]
    pub protocol_version: u8,
    pub ack_response_service: ACKResponseService,
    pub ground_altitude: Option<Altitude>,
    pub road_angles: Option<RoadAngles>,
}

impl ITSRangingSAMAppData {
    pub fn new(
        protocol_version: u8,
        ack_response_service: ACKResponseService,
        ground_altitude: Option<Altitude>,
        road_angles: Option<RoadAngles>,
    ) -> Self {
        Self {
            protocol_version,
            ack_response_service,
            ground_altitude,
            road_angles,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ITSaid(pub VarLengthNumber);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=1023"))]
pub struct IVILaneWidth(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct IVIM {
    pub header: ItsPduHeader,
    pub ivi: IviStructure,
}

impl IVIM {
    pub fn new(header: ItsPduHeader, ivi: IviStructure) -> Self {
        Self { header, ivi }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct IVIManagementContainerConnectedIviStructures(pub SequenceOf<IviIdentificationNumber>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IVIManagementContainer {
    pub service_provider_id: Provider,
    pub ivi_identification_number: IviIdentificationNumber,
    pub time_stamp: Option<TimestampIts>,
    pub valid_from: Option<TimestampIts>,
    pub valid_to: Option<TimestampIts>,
    pub connected_ivi_structures: Option<IVIManagementContainerConnectedIviStructures>,
    pub ivi_status: IviStatus,
}

impl IVIManagementContainer {
    pub fn new(
        service_provider_id: Provider,
        ivi_identification_number: IviIdentificationNumber,
        time_stamp: Option<TimestampIts>,
        valid_from: Option<TimestampIts>,
        valid_to: Option<TimestampIts>,
        connected_ivi_structures: Option<IVIManagementContainerConnectedIviStructures>,
        ivi_status: IviStatus,
    ) -> Self {
        Self {
            service_provider_id,
            ivi_identification_number,
            time_stamp,
            valid_from,
            valid_to,
            connected_ivi_structures,
            ivi_status,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct IValue(pub Uint16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IdentifiedRegion {
    CountryOnly(CountryOnly),
    CountryAndRegions(CountryAndRegions),
    CountryAndSubregions(CountryAndSubregions),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Identifier(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ImpactReductionContainer {
    pub height_lon_carr_left: HeightLonCarr,
    pub height_lon_carr_right: HeightLonCarr,
    pub pos_lon_carr_left: PosLonCarr,
    pub pos_lon_carr_right: PosLonCarr,
    pub position_of_pillars: PositionOfPillars,
    pub pos_cent_mass: PosCentMass,
    pub wheel_base_vehicle: WheelBaseVehicle,
    pub turning_radius: TurningRadius,
    pub pos_front_ax: PosFrontAx,
    pub position_of_occupants: PositionOfOccupants,
    pub vehicle_mass: VehicleMass,
    pub request_response_indication: RequestResponseIndication,
}

impl ImpactReductionContainer {
    pub fn new(
        height_lon_carr_left: HeightLonCarr,
        height_lon_carr_right: HeightLonCarr,
        pos_lon_carr_left: PosLonCarr,
        pos_lon_carr_right: PosLonCarr,
        position_of_pillars: PositionOfPillars,
        pos_cent_mass: PosCentMass,
        wheel_base_vehicle: WheelBaseVehicle,
        turning_radius: TurningRadius,
        pos_front_ax: PosFrontAx,
        position_of_occupants: PositionOfOccupants,
        vehicle_mass: VehicleMass,
        request_response_indication: RequestResponseIndication,
    ) -> Self {
        Self {
            height_lon_carr_left,
            height_lon_carr_right,
            pos_lon_carr_left,
            pos_lon_carr_right,
            position_of_pillars,
            pos_cent_mass,
            wheel_base_vehicle,
            turning_radius,
            pos_front_ax,
            position_of_occupants,
            vehicle_mass,
            request_response_indication,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum ImpactSection {
    Unavailable = 0,
    Rear = 1,
    Front = 2,
    SideLeftFront = 3,
    SideLeftBack = 4,
    SideRightFront = 5,
    SideRightBack = 6,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct InformationQuality(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Int1(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct Int2(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IntersectionAccessPoint {
    Lane(LaneID),
    Approach(ApproachID),
    Connection(LaneConnectionID),
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct IntersectionGeometryRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IntersectionGeometry {
    pub name: Option<DescriptiveName>,
    pub id: IntersectionReferenceID,
    pub revision: MsgCount,
    pub ref_point: Position3D,
    pub lane_width: Option<LaneWidth>,
    pub speed_limits: Option<SpeedLimitList>,
    pub lane_set: LaneList,
    pub preempt_priority_data: Option<PreemptPriorityList>,
    pub regional: Option<IntersectionGeometryRegional>,
}

impl IntersectionGeometry {
    pub fn new(
        name: Option<DescriptiveName>,
        id: IntersectionReferenceID,
        revision: MsgCount,
        ref_point: Position3D,
        lane_width: Option<LaneWidth>,
        speed_limits: Option<SpeedLimitList>,
        lane_set: LaneList,
        preempt_priority_data: Option<PreemptPriorityList>,
        regional: Option<IntersectionGeometryRegional>,
    ) -> Self {
        Self {
            name,
            id,
            revision,
            ref_point,
            lane_width,
            speed_limits,
            lane_set,
            preempt_priority_data,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32"))]
pub struct IntersectionGeometryList(pub SequenceOf<IntersectionGeometry>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct IntersectionID(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct IntersectionReferenceID {
    pub region: Option<RoadRegulatorID>,
    pub id: IntersectionID,
}

impl IntersectionReferenceID {
    pub fn new(region: Option<RoadRegulatorID>, id: IntersectionID) -> Self {
        Self { region, id }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct IntersectionStateRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IntersectionState {
    pub name: Option<DescriptiveName>,
    pub id: IntersectionReferenceID,
    pub revision: MsgCount,
    pub status: IntersectionStatusObject,
    pub moy: Option<MinuteOfTheYear>,
    pub time_stamp: Option<DSecond>,
    pub enabled_lanes: Option<EnabledLaneList>,
    pub states: MovementList,
    pub maneuver_assist_list: Option<ManeuverAssistList>,
    pub regional: Option<IntersectionStateRegional>,
}

impl IntersectionState {
    pub fn new(
        name: Option<DescriptiveName>,
        id: IntersectionReferenceID,
        revision: MsgCount,
        status: IntersectionStatusObject,
        moy: Option<MinuteOfTheYear>,
        time_stamp: Option<DSecond>,
        enabled_lanes: Option<EnabledLaneList>,
        states: MovementList,
        maneuver_assist_list: Option<ManeuverAssistList>,
        regional: Option<IntersectionStateRegional>,
    ) -> Self {
        Self {
            name,
            id,
            revision,
            status,
            moy,
            time_stamp,
            enabled_lanes,
            states,
            maneuver_assist_list,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct IntersectionStateAddGrpC {
    pub active_prioritizations: Option<PrioritizationResponseList>,
}

impl IntersectionStateAddGrpC {
    pub fn new(active_prioritizations: Option<PrioritizationResponseList>) -> Self {
        Self {
            active_prioritizations,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32"))]
pub struct IntersectionStateList(pub SequenceOf<IntersectionState>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct IntersectionStatusObject(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct IpV6Prefix(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct IpV6PrefixLength(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Iso3833VehicleType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=16383"))]
pub struct IssuerIdentifier(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=40"))]
pub struct ItineraryPath(pub SequenceOf<ReferencePosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ItsAidCtxRef {
    pub itsaid: ITSaid,
    pub ctx: CtxRef,
}

impl ItsAidCtxRef {
    pub fn new(itsaid: ITSaid, ctx: CtxRef) -> Self {
        Self { itsaid, ctx }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ItsChargingSpotDataElements {
    pub r_type: ChargingSpotType,
    pub ev_equipment_i_d: Option<Utf8String>,
    pub type_of_receptacle: TypeOfReceptacle,
    pub energy_availability: Utf8String,
    pub parking_places_data: Option<ParkingPlacesData>,
}

impl ItsChargingSpotDataElements {
    pub fn new(
        r_type: ChargingSpotType,
        ev_equipment_i_d: Option<Utf8String>,
        type_of_receptacle: TypeOfReceptacle,
        energy_availability: Utf8String,
        parking_places_data: Option<ParkingPlacesData>,
    ) -> Self {
        Self {
            r_type,
            ev_equipment_i_d,
            type_of_receptacle,
            energy_availability,
            parking_places_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct ItsChargingSpots(pub SequenceOf<ItsChargingSpotDataElements>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ItsChargingStationData {
    pub charging_station_i_d: StationID,
    #[rasn(size("1..=32"))]
    pub utility_distributor_id: Option<Utf8String>,
    #[rasn(size("1..=32"))]
    pub provider_i_d: Option<Utf8String>,
    pub charging_station_location: ReferencePosition,
    pub address: Option<Utf8String>,
    #[rasn(size("1..=16"))]
    pub phone_number: Option<NumericString>,
    #[rasn(size("1..=32"))]
    pub accessibility: Utf8String,
    pub digital_map: Option<DigitalMap>,
    pub opening_days_hours: Utf8String,
    pub pricing: Utf8String,
    pub booking_contact_info: Option<Utf8String>,
    pub payment: Option<Utf8String>,
    pub charging_spots_available: ItsChargingSpots,
}

impl ItsChargingStationData {
    pub fn new(
        charging_station_i_d: StationID,
        utility_distributor_id: Option<Utf8String>,
        provider_i_d: Option<Utf8String>,
        charging_station_location: ReferencePosition,
        address: Option<Utf8String>,
        phone_number: Option<NumericString>,
        accessibility: Utf8String,
        digital_map: Option<DigitalMap>,
        opening_days_hours: Utf8String,
        pricing: Utf8String,
        booking_contact_info: Option<Utf8String>,
        payment: Option<Utf8String>,
        charging_spots_available: ItsChargingSpots,
    ) -> Self {
        Self {
            charging_station_i_d,
            utility_distributor_id,
            provider_i_d,
            charging_station_location,
            address,
            phone_number,
            accessibility,
            digital_map,
            opening_days_hours,
            pricing,
            booking_contact_info,
            payment,
            charging_spots_available,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=256"))]
pub struct ItsEVCSNDataChargingStationsData(pub SequenceOf<ItsChargingStationData>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ItsEVCSNData {
    pub total_number_of_stations: NumberStations,
    pub charging_stations_data: ItsEVCSNDataChargingStationsData,
}

impl ItsEVCSNData {
    pub fn new(
        total_number_of_stations: NumberStations,
        charging_stations_data: ItsEVCSNDataChargingStationsData,
    ) -> Self {
        Self {
            total_number_of_stations,
            charging_stations_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ItsPOIHeader {
    pub poi_type: POIType,
    pub time_stamp: TimestampIts,
    pub relay_capable: bool,
}

impl ItsPOIHeader {
    pub fn new(poi_type: POIType, time_stamp: TimestampIts, relay_capable: bool) -> Self {
        Self {
            poi_type,
            time_stamp,
            relay_capable,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ItsPduHeader {
    #[rasn(value("0..=255"))]
    pub protocol_version: u8,
    #[rasn(value("0..=255"))]
    pub message_i_d: u8,
    pub station_i_d: StationID,
}

impl ItsPduHeader {
    pub fn new(protocol_version: u8, message_i_d: u8, station_i_d: StationID) -> Self {
        Self {
            protocol_version,
            message_i_d,
            station_i_d,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ItsStationPosition {
    pub station_i_d: StationID,
    pub lane_i_d: Option<LaneID>,
    pub node_x_y: Option<NodeOffsetPointXY>,
    pub time_reference: Option<TimeReference>,
}

impl ItsStationPosition {
    pub fn new(
        station_i_d: StationID,
        lane_i_d: Option<LaneID>,
        node_x_y: Option<NodeOffsetPointXY>,
        time_reference: Option<TimeReference>,
    ) -> Self {
        Self {
            station_i_d,
            lane_i_d,
            node_x_y,
            time_reference,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=5"))]
pub struct ItsStationPositionList(pub SequenceOf<ItsStationPosition>);

///Definition of Containers

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum IviContainer {
    Glc(GeographicLocationContainer),
    Giv(GeneralIviContainer),
    Rcc(RoadConfigurationContainer),
    Tc(TextContainer),
    Lac(LayoutContainer),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=32767", extensible))]
pub struct IviIdentificationNumber(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3"))]
pub struct IviPurpose(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct IviStatus(pub u8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct IviStructureOptional(pub SequenceOf<IviContainer>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct IviStructure {
    pub mandatory: IVIManagementContainer,
    pub optional: Option<IviStructureOptional>,
}

impl IviStructure {
    pub fn new(mandatory: IVIManagementContainer, optional: Option<IviStructureOptional>) -> Self {
        Self {
            mandatory,
            optional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct IviType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("-900000000..=900000000"))]
pub struct KnownLatitude(pub NinetyDegreeInt);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("-1799999999..=1800000000"))]
pub struct KnownLongitude(pub OneEightyDegreeInt);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct LMchannelBusyRatio(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("2"))]
pub struct LaId(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LaneAttributes {
    pub directional_use: LaneDirection,
    pub shared_with: LaneSharing,
    pub lane_type: LaneTypeAttributes,
    #[rasn(value("0.."))]
    pub regional: Option<RegionalExtension>,
}

impl LaneAttributes {
    pub fn new(
        directional_use: LaneDirection,
        shared_with: LaneSharing,
        lane_type: LaneTypeAttributes,
        regional: Option<RegionalExtension>,
    ) -> Self {
        Self {
            directional_use,
            shared_with,
            lane_type,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LaneAttributesBarrier(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LaneAttributesBike(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LaneAttributesCrosswalk(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LaneAttributesParking(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LaneAttributesSidewalk(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LaneAttributesStriping(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LaneAttributesTrackedVehicle(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8", extensible))]
pub struct LaneAttributesVehicle(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LaneAttributesAddGrpC {
    pub max_vehicle_height: Option<VehicleHeight>,
    pub max_vehicle_weight: Option<VehicleMass>,
}

impl LaneAttributesAddGrpC {
    pub fn new(
        max_vehicle_height: Option<VehicleHeight>,
        max_vehicle_weight: Option<VehicleMass>,
    ) -> Self {
        Self {
            max_vehicle_height,
            max_vehicle_weight,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct LaneConnectionID(pub u8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct LaneDataAttributeRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum LaneDataAttribute {
    PathEndPointAngle(DeltaAngle),
    LaneCrownPointCenter(RoadwayCrownAngle),
    LaneCrownPointLeft(RoadwayCrownAngle),
    LaneCrownPointRight(RoadwayCrownAngle),
    LaneAngle(MergeDivergeNodeAngle),
    SpeedLimits(SpeedLimitList),
    Regional(LaneDataAttributeRegional),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct LaneDataAttributeList(pub SequenceOf<LaneDataAttribute>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("2"))]
pub struct LaneDirection(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct LaneID(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LaneInformation {
    pub lane_number: LanePosition,
    pub direction: Direction,
    pub validity: Option<DTM>,
    pub lane_type: LaneType,
    pub lane_type_qualifier: Option<CompleteVehicleCharacteristics>,
    pub lane_status: LaneStatus,
    pub lane_width: Option<IVILaneWidth>,
}

impl LaneInformation {
    pub fn new(
        lane_number: LanePosition,
        direction: Direction,
        validity: Option<DTM>,
        lane_type: LaneType,
        lane_type_qualifier: Option<CompleteVehicleCharacteristics>,
        lane_status: LaneStatus,
        lane_width: Option<IVILaneWidth>,
    ) -> Self {
        Self {
            lane_number,
            direction,
            validity,
            lane_type,
            lane_type_qualifier,
            lane_status,
            lane_width,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=255"))]
pub struct LaneList(pub SequenceOf<GenericLane>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1..=14"))]
pub struct LanePosition(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("10"))]
pub struct LaneSharing(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7", extensible))]
pub struct LaneStatus(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=31"))]
pub struct LaneType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum LaneTypeAttributes {
    Vehicle(LaneAttributesVehicle),
    Crosswalk(LaneAttributesCrosswalk),
    BikeLane(LaneAttributesBike),
    Sidewalk(LaneAttributesSidewalk),
    Median(LaneAttributesBarrier),
    Striping(LaneAttributesStriping),
    TrackedVehicle(LaneAttributesTrackedVehicle),
    Parking(LaneAttributesParking),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=32767"))]
pub struct LaneWidth(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LateralAcceleration {
    pub lateral_acceleration_value: LateralAccelerationValue,
    pub lateral_acceleration_confidence: AccelerationConfidence,
}

impl LateralAcceleration {
    pub fn new(
        lateral_acceleration_value: LateralAccelerationValue,
        lateral_acceleration_confidence: AccelerationConfidence,
    ) -> Self {
        Self {
            lateral_acceleration_value,
            lateral_acceleration_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-160..=161"))]
pub struct LateralAccelerationValue(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-900000000..=900000001"))]
pub struct Latitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=100"))]
pub struct LayerID(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum LayerType {
    None = 0,
    MixedContent = 1,
    GeneralMapData = 2,
    IntersectionData = 3,
    CurveData = 4,
    RoadwaySectionData = 5,
    ParkingAreaData = 6,
    SharedLaneData = 7,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LayoutComponent {
    #[rasn(value("1..=8", extensible))]
    pub layout_component_id: u8,
    #[rasn(value("10..=73"))]
    pub height: u8,
    #[rasn(value("10..=265"))]
    pub width: u16,
    #[rasn(value("10..=265"))]
    pub x: u16,
    #[rasn(value("10..=73"))]
    pub y: u8,
    #[rasn(value("0..=1"))]
    pub text_scripting: u8,
}

impl LayoutComponent {
    pub fn new(
        layout_component_id: u8,
        height: u8,
        width: u16,
        x: u16,
        y: u8,
        text_scripting: u8,
    ) -> Self {
        Self {
            layout_component_id,
            height,
            width,
            x,
            y,
            text_scripting,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct LayoutContainerLayoutComponents(pub SequenceOf<LayoutComponent>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LayoutContainer {
    #[rasn(value("1..=4", extensible))]
    pub layout_id: u8,
    #[rasn(value("10..=73"))]
    pub height: Option<u8>,
    #[rasn(value("10..=265"))]
    pub width: Option<u16>,
    pub layout_components: LayoutContainerLayoutComponents,
}

impl LayoutContainer {
    pub fn new(
        layout_id: u8,
        height: Option<u8>,
        width: Option<u16>,
        layout_components: LayoutContainerLayoutComponents,
    ) -> Self {
        Self {
            layout_id,
            height,
            width,
            layout_components,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum LeftOrRight {
    Left = 0,
    Right = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("2"))]
pub struct LightBarSirenInUse(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("16"))]
pub struct LinkageSeed(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("9"))]
pub struct LinkageValue(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LoadType {
    pub goods_type: GoodsType,
    pub dangerous_goods_type: DangerousGoodsBasic,
    pub special_transport_type: SpecialTransportType,
}

impl LoadType {
    pub fn new(
        goods_type: GoodsType,
        dangerous_goods_type: DangerousGoodsBasic,
        special_transport_type: SpecialTransportType,
    ) -> Self {
        Self {
            goods_type,
            dangerous_goods_type,
            special_transport_type,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct LocationContainer {
    pub event_speed: Option<Speed>,
    pub event_position_heading: Option<Heading>,
    pub traces: Traces,
    pub road_type: Option<RoadType>,
}

impl LocationContainer {
    pub fn new(
        event_speed: Option<Speed>,
        event_position_heading: Option<Heading>,
        traces: Traces,
        road_type: Option<RoadType>,
    ) -> Self {
        Self {
            event_speed,
            event_position_heading,
            traces,
            road_type,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1800000000..=1800000001"))]
pub struct Longitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LongitudinalAcceleration {
    pub longitudinal_acceleration_value: LongitudinalAccelerationValue,
    pub longitudinal_acceleration_confidence: AccelerationConfidence,
}

impl LongitudinalAcceleration {
    pub fn new(
        longitudinal_acceleration_value: LongitudinalAccelerationValue,
        longitudinal_acceleration_confidence: AccelerationConfidence,
    ) -> Self {
        Self {
            longitudinal_acceleration_value,
            longitudinal_acceleration_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-160..=161"))]
pub struct LongitudinalAccelerationValue(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct LongitudinalLanePosition {
    pub longitudinal_lane_position_value: LongitudinalLanePositionValue,
    pub longitudinal_lane_position_confidence: LongitudinalLanePositionConfidence,
}

impl LongitudinalLanePosition {
    pub fn new(
        longitudinal_lane_position_value: LongitudinalLanePositionValue,
        longitudinal_lane_position_confidence: LongitudinalLanePositionConfidence,
    ) -> Self {
        Self {
            longitudinal_lane_position_value,
            longitudinal_lane_position_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=102"))]
pub struct LongitudinalLanePositionConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=32767"))]
pub struct LongitudinalLanePositionValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum LowFrequencyContainer {
    BasicVehicleContainerLowFrequency(BasicVehicleContainerLowFrequency),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("6"))]
pub struct MACaddress(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct MAPEM {
    pub header: ItsPduHeader,
    pub map: MapData,
}

impl MAPEM {
    pub fn new(header: ItsPduHeader, map: MapData) -> Self {
        Self { header, map }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MCDMApplicationContainer {}

impl MCDMApplicationContainer {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MCDMLocationContainer {
    pub event_position: ReferencePosition,
}

impl MCDMLocationContainer {
    pub fn new(event_position: ReferencePosition) -> Self {
        Self { event_position }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct MCDMManagementContainerMediaTypes(pub SequenceOf<MediaTypeOfMDUs>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct MCDMManagementContainerUrls(pub SequenceOf<URLOfMDUs>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MCDMManagementContainer {
    pub action_i_d: ActionID,
    pub request: Option<RequestResponseIndication>,
    pub ack: Option<AckNackIndication>,
    pub detection_time: Option<TimestampIts>,
    pub reference_time: TimestampIts,
    pub linked_denm: Option<ActionID>,
    pub validity_duration: Option<ValidityDuration>,
    pub station_type: Option<StationType>,
    #[rasn(
        value("0..=4294967295"),
        default = "m_c_d_m_management_container_number_of_m_d_us_default"
    )]
    pub number_of_m_d_us: u32,
    #[rasn(
        value("1..=4294967295"),
        default = "m_c_d_m_management_container_number_of_p_d_us_default"
    )]
    pub number_of_p_d_us: u32,
    #[rasn(
        value("1..=4294967295"),
        default = "m_c_d_m_management_container_pdu_sequence_number_default"
    )]
    pub pdu_sequence_number: u32,
    pub media_types: Option<MCDMManagementContainerMediaTypes>,
    pub urls: Option<MCDMManagementContainerUrls>,
    #[rasn(default = "m_c_d_m_management_container_real_time_default")]
    pub real_time: bool,
    #[rasn(value("0..=4294967295"))]
    pub size: Option<u32>,
}

impl MCDMManagementContainer {
    pub fn new(
        action_i_d: ActionID,
        request: Option<RequestResponseIndication>,
        ack: Option<AckNackIndication>,
        detection_time: Option<TimestampIts>,
        reference_time: TimestampIts,
        linked_denm: Option<ActionID>,
        validity_duration: Option<ValidityDuration>,
        station_type: Option<StationType>,
        number_of_m_d_us: u32,
        number_of_p_d_us: u32,
        pdu_sequence_number: u32,
        media_types: Option<MCDMManagementContainerMediaTypes>,
        urls: Option<MCDMManagementContainerUrls>,
        real_time: bool,
        size: Option<u32>,
    ) -> Self {
        Self {
            action_i_d,
            request,
            ack,
            detection_time,
            reference_time,
            linked_denm,
            validity_duration,
            station_type,
            number_of_m_d_us,
            number_of_p_d_us,
            pdu_sequence_number,
            media_types,
            urls,
            real_time,
            size,
        }
    }
}

fn m_c_d_m_management_container_number_of_m_d_us_default() -> u32 {
    1 // TODO: .into()
}

fn m_c_d_m_management_container_number_of_p_d_us_default() -> u32 {
    1
}

fn m_c_d_m_management_container_pdu_sequence_number_default() -> u32 {
    1
}

fn m_c_d_m_management_container_real_time_default() -> bool {
    false.into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=7"))]
pub struct MCDMMultimediaContainer(pub SequenceOf<MultimediaDataUnit>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MCDMSituationContainer {
    pub event_type: CauseCode,
    pub linked_cause: Option<CauseCode>,
    #[rasn(value("0..=100"))]
    pub authorized_percentage_loss: Option<u8>,
    pub information_quality: InformationQuality,
}

impl MCDMSituationContainer {
    pub fn new(
        event_type: CauseCode,
        linked_cause: Option<CauseCode>,
        authorized_percentage_loss: Option<u8>,
        information_quality: InformationQuality,
    ) -> Self {
        Self {
            event_type,
            linked_cause,
            authorized_percentage_loss,
            information_quality,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ManagementContainer {
    pub action_i_d: ActionID,
    pub detection_time: TimestampIts,
    pub reference_time: TimestampIts,
    pub termination: Option<Termination>,
    pub event_position: ReferencePosition,
    pub relevance_distance: Option<RelevanceDistance>,
    pub relevance_traffic_direction: Option<RelevanceTrafficDirection>,
    #[rasn(default = "management_container_validity_duration_default")]
    pub validity_duration: ValidityDuration,
    pub transmission_interval: Option<TransmissionInterval>,
    pub station_type: StationType,
}

impl ManagementContainer {
    pub fn new(
        action_i_d: ActionID,
        detection_time: TimestampIts,
        reference_time: TimestampIts,
        termination: Option<Termination>,
        event_position: ReferencePosition,
        relevance_distance: Option<RelevanceDistance>,
        relevance_traffic_direction: Option<RelevanceTrafficDirection>,
        validity_duration: ValidityDuration,
        transmission_interval: Option<TransmissionInterval>,
        station_type: StationType,
    ) -> Self {
        Self {
            action_i_d,
            detection_time,
            reference_time,
            termination,
            event_position,
            relevance_distance,
            relevance_traffic_direction,
            validity_duration,
            transmission_interval,
            station_type,
        }
    }
}

fn management_container_validity_duration_default() -> ValidityDuration {
    ValidityDuration(600).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct MandAppCtx(pub SequenceOf<ItsAidCtxRef>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct ManeuverAssistList(pub SequenceOf<ConnectionManeuverAssist>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct MapDataRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MapData {
    pub time_stamp: Option<MinuteOfTheYear>,
    pub msg_issue_revision: MsgCount,
    pub layer_type: Option<LayerType>,
    pub layer_i_d: Option<LayerID>,
    pub intersections: Option<IntersectionGeometryList>,
    pub road_segments: Option<RoadSegmentList>,
    pub data_parameters: Option<DataParameters>,
    pub restriction_list: Option<RestrictionClassList>,
    pub regional: Option<MapDataRegional>,
}

impl MapData {
    pub fn new(
        time_stamp: Option<MinuteOfTheYear>,
        msg_issue_revision: MsgCount,
        layer_type: Option<LayerType>,
        layer_i_d: Option<LayerID>,
        intersections: Option<IntersectionGeometryList>,
        road_segments: Option<RoadSegmentList>,
        data_parameters: Option<DataParameters>,
        restriction_list: Option<RestrictionClassList>,
        regional: Option<MapDataRegional>,
    ) -> Self {
        Self {
            time_stamp,
            msg_issue_revision,
            layer_type,
            layer_i_d,
            intersections,
            road_segments,
            data_parameters,
            restriction_list,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MapDataAddGrpC {
    pub signal_head_locations: Option<SignalHeadLocationList>,
}

impl MapDataAddGrpC {
    pub fn new(signal_head_locations: Option<SignalHeadLocationList>) -> Self {
        Self {
            signal_head_locations,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct MapPosition {
    pub intersection_id: IntersectionReferenceID,
    pub lane: LaneID,
}

impl MapPosition {
    pub fn new(intersection_id: IntersectionReferenceID, lane: LaneID) -> Self {
        Self {
            intersection_id,
            lane,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MatchedPosition {
    pub lane_i_d: Option<LaneID>,
    pub longitudinal_lane_position: Option<LongitudinalLanePosition>,
}

impl MatchedPosition {
    pub fn new(
        lane_i_d: Option<LaneID>,
        longitudinal_lane_position: Option<LongitudinalLanePosition>,
    ) -> Self {
        Self {
            lane_i_d,
            longitudinal_lane_position,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct McdmInfo {
    pub management: MCDMManagementContainer,
    pub situation: Option<MCDMSituationContainer>,
    pub location: Option<MCDMLocationContainer>,
    pub application: Option<MCDMApplicationContainer>,
    pub multimedia: Option<MCDMMultimediaContainer>,
}

impl McdmInfo {
    pub fn new(
        management: MCDMManagementContainer,
        situation: Option<MCDMSituationContainer>,
        location: Option<MCDMLocationContainer>,
        application: Option<MCDMApplicationContainer>,
        multimedia: Option<MCDMMultimediaContainer>,
    ) -> Self {
        Self {
            management,
            situation,
            location,
            application,
            multimedia,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct McdmPdu {
    pub header: ItsPduHeader,
    pub mcdm_info: McdmInfo,
}

impl McdmPdu {
    pub fn new(header: ItsPduHeader, mcdm_info: McdmInfo) -> Self {
        Self { header, mcdm_info }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct MedType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct MediaTypeOfMDUs {
    pub starting_m_d_u: Option<SequenceNumber>,
    pub ending_m_d_u: Option<SequenceNumber>,
    pub media_type: Ia5String,
}

impl MediaTypeOfMDUs {
    pub fn new(
        starting_m_d_u: Option<SequenceNumber>,
        ending_m_d_u: Option<SequenceNumber>,
        media_type: Ia5String,
    ) -> Self {
        Self {
            starting_m_d_u,
            ending_m_d_u,
            media_type,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-180..=180"))]
pub struct MergeDivergeNodeAngle(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=527040"))]
pub struct MinuteOfTheYear(pub u32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct MonthDay {
    #[rasn(value("1..=12"))]
    pub month: u8,
    #[rasn(value("1..=31"))]
    pub day: u8,
}

impl MonthDay {
    pub fn new(month: u8, day: u8) -> Self {
        Self { month, day }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MotorcylistSpecialContainer {
    pub vru_sub_profile_motorcyclist: VruSubProfileMotorcyclist,
    pub vru_size_class: VruSizeClass,
    pub roll_angle: Option<VruRollAngle>,
    pub vru_safe_distance: Option<SequenceOfVruSafeDistanceIndication>,
    pub path_history: Option<PathHistory>,
    pub path_prediction: Option<PathPrediction>,
    pub stability_change_indication: Option<StabilityChangeIndication>,
}

impl MotorcylistSpecialContainer {
    pub fn new(
        vru_sub_profile_motorcyclist: VruSubProfileMotorcyclist,
        vru_size_class: VruSizeClass,
        roll_angle: Option<VruRollAngle>,
        vru_safe_distance: Option<SequenceOfVruSafeDistanceIndication>,
        path_history: Option<PathHistory>,
        path_prediction: Option<PathPrediction>,
        stability_change_indication: Option<StabilityChangeIndication>,
    ) -> Self {
        Self {
            vru_sub_profile_motorcyclist,
            vru_size_class,
            roll_angle,
            vru_safe_distance,
            path_history,
            path_prediction,
            stability_change_indication,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct MovementEventRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MovementEvent {
    pub event_state: MovementPhaseState,
    pub timing: Option<TimeChangeDetails>,
    pub speeds: Option<AdvisorySpeedList>,
    pub regional: Option<MovementEventRegional>,
}

impl MovementEvent {
    pub fn new(
        event_state: MovementPhaseState,
        timing: Option<TimeChangeDetails>,
        speeds: Option<AdvisorySpeedList>,
        regional: Option<MovementEventRegional>,
    ) -> Self {
        Self {
            event_state,
            timing,
            speeds,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MovementEventAddGrpC {
    pub state_change_reason: Option<ExceptionalCondition>,
}

impl MovementEventAddGrpC {
    pub fn new(state_change_reason: Option<ExceptionalCondition>) -> Self {
        Self {
            state_change_reason,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct MovementEventList(pub SequenceOf<MovementEvent>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=255"))]
pub struct MovementList(pub SequenceOf<MovementState>);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum MovementPhaseState {
    Unavailable = 0,
    Dark = 1,
    StopThenProceed = 2,
    StopAndRemain = 3,
    PreMovement = 4,
    PermissiveMovementAllowed = 5,
    ProtectedMovementAllowed = 6,
    PermissiveClearance = 7,
    ProtectedClearance = 8,
    CautionConflictingTraffic = 9,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct MovementStateRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct MovementState {
    pub movement_name: Option<DescriptiveName>,
    pub signal_group: SignalGroupID,
    pub state_time_speed: MovementEventList,
    pub maneuver_assist_list: Option<ManeuverAssistList>,
    pub regional: Option<MovementStateRegional>,
}

impl MovementState {
    pub fn new(
        movement_name: Option<DescriptiveName>,
        signal_group: SignalGroupID,
        state_time_speed: MovementEventList,
        maneuver_assist_list: Option<ManeuverAssistList>,
        regional: Option<MovementStateRegional>,
    ) -> Self {
        Self {
            movement_name,
            signal_group,
            state_time_speed,
            maneuver_assist_list,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=127"))]
pub struct MsgCount(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum MultimediaDataUnit {
    MediaContentUTF8(Utf8String),
    MediaContentOctet(OctetString),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-900000000..=900000001"))]
pub struct NinetyDegreeInt(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct Node {
    pub id: Integer,
    pub lane: Option<LaneID>,
    pub connection_i_d: Option<LaneConnectionID>,
    pub intersection_i_d: Option<IntersectionID>,
}

impl Node {
    pub fn new(
        id: Integer,
        lane: Option<LaneID>,
        connection_i_d: Option<LaneConnectionID>,
        intersection_i_d: Option<IntersectionID>,
    ) -> Self {
        Self {
            id,
            lane,
            connection_i_d,
            intersection_i_d,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct NodeLLmD64b {
    pub lon: Longitude,
    pub lat: Latitude,
}

impl NodeLLmD64b {
    pub fn new(lon: Longitude, lat: Latitude) -> Self {
        Self { lon, lat }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct NodeXY20b {
    pub x: OffsetB10,
    pub y: OffsetB10,
}

impl NodeXY20b {
    pub fn new(x: OffsetB10, y: OffsetB10) -> Self {
        Self { x, y }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct NodeXY22b {
    pub x: OffsetB11,
    pub y: OffsetB11,
}

impl NodeXY22b {
    pub fn new(x: OffsetB11, y: OffsetB11) -> Self {
        Self { x, y }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct NodeXY24b {
    pub x: OffsetB12,
    pub y: OffsetB12,
}

impl NodeXY24b {
    pub fn new(x: OffsetB12, y: OffsetB12) -> Self {
        Self { x, y }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct NodeXY26b {
    pub x: OffsetB13,
    pub y: OffsetB13,
}

impl NodeXY26b {
    pub fn new(x: OffsetB13, y: OffsetB13) -> Self {
        Self { x, y }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct NodeXY28b {
    pub x: OffsetB14,
    pub y: OffsetB14,
}

impl NodeXY28b {
    pub fn new(x: OffsetB14, y: OffsetB14) -> Self {
        Self { x, y }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct NodeXY32b {
    pub x: OffsetB16,
    pub y: OffsetB16,
}

impl NodeXY32b {
    pub fn new(x: OffsetB16, y: OffsetB16) -> Self {
        Self { x, y }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct NodeAttributeSetAddGrpC {
    pub ptv_request: Option<PtvRequestType>,
    pub node_link: Option<NodeLink>,
    pub node: Option<Node>,
}

impl NodeAttributeSetAddGrpC {
    pub fn new(
        ptv_request: Option<PtvRequestType>,
        node_link: Option<NodeLink>,
        node: Option<Node>,
    ) -> Self {
        Self {
            ptv_request,
            node_link,
            node,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct NodeAttributeSetXYRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct NodeAttributeSetXY {
    pub local_node: Option<NodeAttributeXYList>,
    pub disabled: Option<SegmentAttributeXYList>,
    pub enabled: Option<SegmentAttributeXYList>,
    pub data: Option<LaneDataAttributeList>,
    pub d_width: Option<OffsetB10>,
    pub d_elevation: Option<OffsetB10>,
    pub regional: Option<NodeAttributeSetXYRegional>,
}

impl NodeAttributeSetXY {
    pub fn new(
        local_node: Option<NodeAttributeXYList>,
        disabled: Option<SegmentAttributeXYList>,
        enabled: Option<SegmentAttributeXYList>,
        data: Option<LaneDataAttributeList>,
        d_width: Option<OffsetB10>,
        d_elevation: Option<OffsetB10>,
        regional: Option<NodeAttributeSetXYRegional>,
    ) -> Self {
        Self {
            local_node,
            disabled,
            enabled,
            data,
            d_width,
            d_elevation,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum NodeAttributeXY {
    Reserved = 0,
    StopLine = 1,
    RoundedCapStyleA = 2,
    RoundedCapStyleB = 3,
    MergePoint = 4,
    DivergePoint = 5,
    DownstreamStopLine = 6,
    DownstreamStartNode = 7,
    ClosedToTraffic = 8,
    SafeIsland = 9,
    CurbPresentAtStepOff = 10,
    HydrantPresent = 11,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct NodeAttributeXYList(pub SequenceOf<NodeAttributeXY>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=5"))]
pub struct NodeLink(pub SequenceOf<Node>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum NodeListXY {
    Nodes(NodeSetXY),
    Computed(ComputedLane),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum NodeOffsetPointXY {
    NodeXY1(NodeXY20b),
    NodeXY2(NodeXY22b),
    NodeXY3(NodeXY24b),
    NodeXY4(NodeXY26b),
    NodeXY5(NodeXY28b),
    NodeXY6(NodeXY32b),
    NodeLatLon(NodeLLmD64b),
    #[rasn(value("0.."))]
    Regional(RegionalExtension),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum NodeOffsetPointZ {
    NodeZ1(OffsetB10),
    NodeZ2(OffsetB11),
    NodeZ3(OffsetB12),
    NodeZ4(OffsetB13),
    NodeZ5(OffsetB14),
    NodeZ6(OffsetB16),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("2..=63"))]
pub struct NodeSetXY(pub SequenceOf<NodeXY>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct NodeXY {
    pub delta: NodeOffsetPointXY,
    pub attributes: Option<NodeAttributeSetXY>,
}

impl NodeXY {
    pub fn new(delta: NodeOffsetPointXY, attributes: Option<NodeAttributeSetXY>) -> Self {
        Self { delta, attributes }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum NonIslandLanePosition {
    OffRoadLanePosition(OffRoadLanePosition),
    VehicularLanePosition(LanePosition),
    MapPosition(MapPosition),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct NullCtx(());

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=127"))]
pub struct NumberOfOccupants(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct NumberOfPerceivedObjects(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=256"))]
pub struct NumberStations(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=1500"))]
pub struct ObjectAge(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ObjectClass {
    pub confidence: ClassConfidence,
    pub class: ObjectClassChoice,
}

impl ObjectClass {
    pub fn new(confidence: ClassConfidence, class: ObjectClassChoice) -> Self {
        Self { confidence, class }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum ObjectClassChoice {
    Vehicle(VehicleSubclass),
    Person(PersonSubclass),
    Animal(AnimalSubclass),
    Other(OtherSubclass),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct ObjectClassDescription(pub SequenceOf<ObjectClass>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=101"))]
pub struct ObjectConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ObjectDimension {
    pub value: ObjectDimensionValue,
    pub confidence: ObjectDimensionConfidence,
}

impl ObjectDimension {
    pub fn new(value: ObjectDimensionValue, confidence: ObjectDimensionConfidence) -> Self {
        Self { value, confidence }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=102"))]
pub struct ObjectDimensionConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=1023"))]
pub struct ObjectDimensionValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ObjectDistanceWithConfidence {
    pub value: DistanceValue,
    pub confidence: DistanceConfidence,
}

impl ObjectDistanceWithConfidence {
    pub fn new(value: DistanceValue, confidence: DistanceConfidence) -> Self {
        Self { value, confidence }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=8"))]
pub struct ObjectRefPoint(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum OffRoadLanePosition {
    Unavailable = 0,
    Sidewalk = 1,
    ParkingLane = 2,
    BikeLane = 3,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-256..=255"))]
pub struct OffsetB09(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-512..=511"))]
pub struct OffsetB10(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1024..=1023"))]
pub struct OffsetB11(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-2048..=2047"))]
pub struct OffsetB12(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-4096..=4095"))]
pub struct OffsetB13(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-8192..=8191"))]
pub struct OffsetB14(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-32768..=32767"))]
pub struct OffsetB16(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct OffsetPoint {
    pub node_offset_pointxy: NodeOffsetPointXY,
    pub node_offset_point_z: Option<NodeOffsetPointZ>,
}

impl OffsetPoint {
    pub fn new(
        node_offset_pointxy: NodeOffsetPointXY,
        node_offset_point_z: Option<NodeOffsetPointZ>,
    ) -> Self {
        Self {
            node_offset_pointxy,
            node_offset_point_z,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum OffsetXaxis {
    Small(DrivenLineOffsetSm),
    Large(DrivenLineOffsetLg),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum OffsetYaxis {
    Small(DrivenLineOffsetSm),
    Large(DrivenLineOffsetLg),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1799999999..=1800000001"))]
pub struct OneEightyDegreeInt(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Opaque(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct OpeningDaysHours(pub Utf8String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct OperatingClass80211(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum OriginatingRSUContainer {
    IntersectionReferenceId(IntersectionReferenceID),
    RoadSegmentReferenceId(RoadSegmentReferenceID),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct OriginatingVehicleContainer {
    pub heading: Heading,
    pub speed: Speed,
    pub vehicle_orientation_angle: Option<WGS84Angle>,
    #[rasn(default = "originating_vehicle_container_drive_direction_default")]
    pub drive_direction: DriveDirection,
    pub longitudinal_acceleration: Option<LongitudinalAcceleration>,
    pub lateral_acceleration: Option<LateralAcceleration>,
    pub vertical_acceleration: Option<VerticalAcceleration>,
    pub yaw_rate: Option<YawRate>,
    pub pitch_angle: Option<CartesianAngle>,
    pub roll_angle: Option<CartesianAngle>,
    pub vehicle_length: Option<VehicleLength>,
    pub vehicle_width: Option<VehicleWidth>,
    pub vehicle_height: Option<VehicleHeight>,
    pub trailer_data_container: Option<TrailerDataContainer>,
}

impl OriginatingVehicleContainer {
    pub fn new(
        heading: Heading,
        speed: Speed,
        vehicle_orientation_angle: Option<WGS84Angle>,
        drive_direction: DriveDirection,
        longitudinal_acceleration: Option<LongitudinalAcceleration>,
        lateral_acceleration: Option<LateralAcceleration>,
        vertical_acceleration: Option<VerticalAcceleration>,
        yaw_rate: Option<YawRate>,
        pitch_angle: Option<CartesianAngle>,
        roll_angle: Option<CartesianAngle>,
        vehicle_length: Option<VehicleLength>,
        vehicle_width: Option<VehicleWidth>,
        vehicle_height: Option<VehicleHeight>,
        trailer_data_container: Option<TrailerDataContainer>,
    ) -> Self {
        Self {
            heading,
            speed,
            vehicle_orientation_angle,
            drive_direction,
            longitudinal_acceleration,
            lateral_acceleration,
            vertical_acceleration,
            yaw_rate,
            pitch_angle,
            roll_angle,
            vehicle_length,
            vehicle_width,
            vehicle_height,
            trailer_data_container,
        }
    }
}

fn originating_vehicle_container_drive_direction_default() -> DriveDirection {
    DriveDirection::Forward.into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct OtherSubclass {
    #[rasn(default = "other_subclass_r_type_default")]
    pub r_type: OtherSublassType,
    #[rasn(default = "other_subclass_confidence_default")]
    pub confidence: ClassConfidence,
}

impl OtherSubclass {
    pub fn new(r_type: OtherSublassType, confidence: ClassConfidence) -> Self {
        Self { r_type, confidence }
    }
}

fn other_subclass_r_type_default() -> OtherSublassType {
    OtherSublassType(0).into()
}

fn other_subclass_confidence_default() -> ClassConfidence {
    ClassConfidence(0).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct OtherSublassType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=5"))]
pub struct OverlayLaneList(pub SequenceOf<LaneID>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("4"))]
pub struct PMD(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct POIType(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct ParkingPlacesData(pub SequenceOf<SpotAvailability>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PartialIntersection {
    #[rasn(size("4"))]
    pub dummy_bitmap: BitString,
    pub name: Option<DescriptiveName>,
    pub id: IntersectionReferenceID,
}

impl PartialIntersection {
    pub fn new(
        dummy_bitmap: BitString,
        name: Option<DescriptiveName>,
        id: IntersectionReferenceID,
    ) -> Self {
        Self {
            dummy_bitmap,
            name,
            id,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PartialMapData {
    pub time_stamp: Option<MinuteOfTheYear>,
    pub msg_issue_revision: MsgCount,
    pub layer_type: Option<LayerType>,
    pub layer_i_d: Option<LayerID>,
    pub first_intersection: Option<FirstIntersection>,
    pub dummy1: Option<()>,
    pub dummy2: Option<()>,
    pub dummy3: Option<()>,
    pub dummy4: Option<()>,
}

impl PartialMapData {
    pub fn new(
        time_stamp: Option<MinuteOfTheYear>,
        msg_issue_revision: MsgCount,
        layer_type: Option<LayerType>,
        layer_i_d: Option<LayerID>,
        first_intersection: Option<FirstIntersection>,
        dummy1: Option<()>,
        dummy2: Option<()>,
        dummy3: Option<()>,
        dummy4: Option<()>,
    ) -> Self {
        Self {
            time_stamp,
            msg_issue_revision,
            layer_type,
            layer_i_d,
            first_intersection,
            dummy1,
            dummy2,
            dummy3,
            dummy4,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PartialMapem {
    pub header: ItsPduHeader,
    pub map: PartialMapData,
}

impl PartialMapem {
    pub fn new(header: ItsPduHeader, map: PartialMapData) -> Self {
        Self { header, map }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PartialSpat {
    pub time_stamp: Option<MinuteOfTheYear>,
    pub name: Option<DescriptiveName>,
    pub intersections: FirstSpat,
    pub dummy: Option<()>,
}

impl PartialSpat {
    pub fn new(
        time_stamp: Option<MinuteOfTheYear>,
        name: Option<DescriptiveName>,
        intersections: FirstSpat,
        dummy: Option<()>,
    ) -> Self {
        Self {
            time_stamp,
            name,
            intersections,
            dummy,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PartialSpatIntersection {
    #[rasn(size("5"))]
    pub dummy_bitmap: BitString,
    pub name: Option<DescriptiveName>,
    pub id: IntersectionReferenceID,
}

impl PartialSpatIntersection {
    pub fn new(
        dummy_bitmap: BitString,
        name: Option<DescriptiveName>,
        id: IntersectionReferenceID,
    ) -> Self {
        Self {
            dummy_bitmap,
            name,
            id,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PartialSpatem {
    pub header: ItsPduHeader,
    pub spat: PartialSpat,
}

impl PartialSpatem {
    pub fn new(header: ItsPduHeader, spat: PartialSpat) -> Self {
        Self { header, spat }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Particulate {
    pub unit_type: UnitType,
    #[rasn(value("0..=32767"))]
    pub value: u16,
}

impl Particulate {
    pub fn new(unit_type: UnitType, value: u16) -> Self {
        Self { unit_type, value }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PassengerCapacity {
    pub number_of_seats: Int1,
    pub number_of_standing_places: Int1,
}

impl PassengerCapacity {
    pub fn new(number_of_seats: Int1, number_of_standing_places: Int1) -> Self {
        Self {
            number_of_seats,
            number_of_standing_places,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=65535", extensible))]
pub struct PathDeltaTime(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=40"))]
pub struct PathHistory(pub SequenceOf<PathPoint>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PathPoint {
    pub path_position: DeltaReferencePosition,
    pub path_delta_time: Option<PathDeltaTime>,
}

impl PathPoint {
    pub fn new(
        path_position: DeltaReferencePosition,
        path_delta_time: Option<PathDeltaTime>,
    ) -> Self {
        Self {
            path_position,
            path_delta_time,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct PathPrediction(pub PathHistory);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct PedestrianBicycleDetect(pub bool);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PerceivedObject {
    pub object_i_d: Identifier,
    pub sensor_i_d_list: Option<SensorIdList>,
    pub time_of_measurement: TimeOfMeasurement,
    pub object_age: Option<ObjectAge>,
    #[rasn(default = "perceived_object_object_confidence_default")]
    pub object_confidence: ObjectConfidence,
    pub x_distance: ObjectDistanceWithConfidence,
    pub y_distance: ObjectDistanceWithConfidence,
    pub z_distance: Option<ObjectDistanceWithConfidence>,
    pub x_speed: SpeedExtended,
    pub y_speed: SpeedExtended,
    pub z_speed: Option<SpeedExtended>,
    pub x_acceleration: Option<LongitudinalAcceleration>,
    pub y_acceleration: Option<LateralAcceleration>,
    pub z_acceleration: Option<VerticalAcceleration>,
    pub yaw_angle: Option<CartesianAngle>,
    pub planar_object_dimension1: Option<ObjectDimension>,
    pub planar_object_dimension2: Option<ObjectDimension>,
    pub vertical_object_dimension: Option<ObjectDimension>,
    #[rasn(default = "perceived_object_object_ref_point_default")]
    pub object_ref_point: ObjectRefPoint,
    pub dynamic_status: Option<DynamicStatus>,
    pub classification: Option<ObjectClassDescription>,
    pub matched_position: Option<MatchedPosition>,
}

impl PerceivedObject {
    pub fn new(
        object_i_d: Identifier,
        sensor_i_d_list: Option<SensorIdList>,
        time_of_measurement: TimeOfMeasurement,
        object_age: Option<ObjectAge>,
        object_confidence: ObjectConfidence,
        x_distance: ObjectDistanceWithConfidence,
        y_distance: ObjectDistanceWithConfidence,
        z_distance: Option<ObjectDistanceWithConfidence>,
        x_speed: SpeedExtended,
        y_speed: SpeedExtended,
        z_speed: Option<SpeedExtended>,
        x_acceleration: Option<LongitudinalAcceleration>,
        y_acceleration: Option<LateralAcceleration>,
        z_acceleration: Option<VerticalAcceleration>,
        yaw_angle: Option<CartesianAngle>,
        planar_object_dimension1: Option<ObjectDimension>,
        planar_object_dimension2: Option<ObjectDimension>,
        vertical_object_dimension: Option<ObjectDimension>,
        object_ref_point: ObjectRefPoint,
        dynamic_status: Option<DynamicStatus>,
        classification: Option<ObjectClassDescription>,
        matched_position: Option<MatchedPosition>,
    ) -> Self {
        Self {
            object_i_d,
            sensor_i_d_list,
            time_of_measurement,
            object_age,
            object_confidence,
            x_distance,
            y_distance,
            z_distance,
            x_speed,
            y_speed,
            z_speed,
            x_acceleration,
            y_acceleration,
            z_acceleration,
            yaw_angle,
            planar_object_dimension1,
            planar_object_dimension2,
            vertical_object_dimension,
            object_ref_point,
            dynamic_status,
            classification,
            matched_position,
        }
    }
}

fn perceived_object_object_confidence_default() -> ObjectConfidence {
    ObjectConfidence(0).into()
}

fn perceived_object_object_ref_point_default() -> ObjectRefPoint {
    ObjectRefPoint(0).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=128", extensible))]
pub struct PerceivedObjectContainer(pub SequenceOf<PerceivedObject>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PerceivedObjectContainerSegmentInfo {
    pub total_msg_segments: SegmentCount,
    pub this_segment_num: SegmentCount,
}

impl PerceivedObjectContainerSegmentInfo {
    pub fn new(total_msg_segments: SegmentCount, this_segment_num: SegmentCount) -> Self {
        Self {
            total_msg_segments,
            this_segment_num,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct PerformanceClass(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PersonSubclass {
    #[rasn(default = "person_subclass_r_type_default")]
    pub r_type: PersonSubclassType,
    #[rasn(default = "person_subclass_confidence_default")]
    pub confidence: ClassConfidence,
}

impl PersonSubclass {
    pub fn new(r_type: PersonSubclassType, confidence: ClassConfidence) -> Self {
        Self { r_type, confidence }
    }
}

fn person_subclass_r_type_default() -> PersonSubclassType {
    PersonSubclassType(0).into()
}

fn person_subclass_confidence_default() -> ClassConfidence {
    ClassConfidence(0).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct PersonSubclassType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct PhoneNumber(pub NumericString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PictogramCategoryCode {
    #[rasn(value("1..=9"))]
    pub nature: u8,
    #[rasn(value("0..=99"))]
    pub serial_number: u8,
}

impl PictogramCategoryCode {
    pub fn new(nature: u8, serial_number: u8) -> Self {
        Self {
            nature,
            serial_number,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PictogramCode {
    #[rasn(size("2"))]
    pub country_code: Option<OctetString>,
    pub service_category_code: ServiceCategoryCode,
    pub pictogram_category_code: PictogramCategoryCode,
}

impl PictogramCode {
    pub fn new(
        country_code: Option<OctetString>,
        service_category_code: ServiceCategoryCode,
        pictogram_category_code: PictogramCategoryCode,
    ) -> Self {
        Self {
            country_code,
            service_category_code,
            pictogram_category_code,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("3..=16", extensible))]
pub struct PolyPointList(pub SequenceOf<OffsetPoint>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum PolygonalLine {
    DeltaPositions(DeltaPositions),
    DeltaPositionsWithAltitude(DeltaPositionsWithAltitude),
    AbsolutePositions(AbsolutePositions),
    AbsolutePositionsWithAltitude(AbsolutePositionsWithAltitude),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("3.."))]
pub struct PolygonalRegion(pub SequenceOf<Dot2TwoDLocation>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct PortNumber(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=63"))]
pub struct PosCentMass(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PosConfidenceEllipse {
    pub semi_major_confidence: SemiAxisLength,
    pub semi_minor_confidence: SemiAxisLength,
    pub semi_major_orientation: HeadingValue,
}

impl PosConfidenceEllipse {
    pub fn new(
        semi_major_confidence: SemiAxisLength,
        semi_minor_confidence: SemiAxisLength,
        semi_major_orientation: HeadingValue,
    ) -> Self {
        Self {
            semi_major_confidence,
            semi_minor_confidence,
            semi_major_orientation,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=20"))]
pub struct PosFrontAx(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct PosLonCarr(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=30"))]
pub struct PosPillar(pub u8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct Position3DRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct Position3D {
    pub lat: Latitude,
    pub long: Longitude,
    pub elevation: Option<Elevation>,
    pub regional: Option<Position3DRegional>,
}

impl Position3D {
    pub fn new(
        lat: Latitude,
        long: Longitude,
        elevation: Option<Elevation>,
        regional: Option<Position3DRegional>,
    ) -> Self {
        Self {
            lat,
            long,
            elevation,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct Position3DAddGrpC {
    pub altitude: Altitude,
}

impl Position3DAddGrpC {
    pub fn new(altitude: Altitude) -> Self {
        Self { altitude }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum PositionConfidence {
    Unavailable = 0,
    A500m = 1,
    A200m = 2,
    A100m = 3,
    A50m = 4,
    A20m = 5,
    A10m = 6,
    A5m = 7,
    A2m = 8,
    A1m = 9,
    A50cm = 10,
    A20cm = 11,
    A10cm = 12,
    A5cm = 13,
    A2cm = 14,
    A1cm = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PositionConfidenceSet {
    pub pos: PositionConfidence,
    pub elevation: ElevationConfidence,
}

impl PositionConfidenceSet {
    pub fn new(pos: PositionConfidence, elevation: ElevationConfidence) -> Self {
        Self { pos, elevation }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("20"))]
pub struct PositionOfOccupants(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=3", extensible))]
pub struct PositionOfPillars(pub SequenceOf<PosPillar>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PositionalAccuracy {
    pub semi_major: SemiMajorAxisAccuracy,
    pub semi_minor: SemiMinorAxisAccuracy,
    pub orientation: SemiMajorAxisOrientation,
}

impl PositionalAccuracy {
    pub fn new(
        semi_major: SemiMajorAxisAccuracy,
        semi_minor: SemiMinorAxisAccuracy,
        orientation: SemiMajorAxisOrientation,
    ) -> Self {
        Self {
            semi_major,
            semi_minor,
            orientation,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum PositioningSolutionType {
    NoPositioningSolution = 0,
    SGNSS = 1,
    DGNSS = 2,
    SGNSSplusDR = 3,
    DGNSSplusDR = 4,
    DR = 5,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct PostCrashSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PreCrashContainer {
    pub perceived_object: PerceivedObject,
    pub object_station_id: Option<StationID>,
    pub time_to_collision: Option<TransmissionInterval>,
    pub impact_section: Option<ImpactSection>,
    pub host_vehicle_orientation: WGS84Angle,
}

impl PreCrashContainer {
    pub fn new(
        perceived_object: PerceivedObject,
        object_station_id: Option<StationID>,
        time_to_collision: Option<TransmissionInterval>,
        impact_section: Option<ImpactSection>,
        host_vehicle_orientation: WGS84Angle,
    ) -> Self {
        Self {
            perceived_object,
            object_station_id,
            time_to_collision,
            impact_section,
            host_vehicle_orientation,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32"))]
pub struct PreemptPriorityList(pub SequenceOf<SignalControlZone>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct PrioritizationResponse {
    pub station_i_d: StationID,
    pub prior_state: PrioritizationResponseStatus,
    pub signal_group: SignalGroupID,
}

impl PrioritizationResponse {
    pub fn new(
        station_i_d: StationID,
        prior_state: PrioritizationResponseStatus,
        signal_group: SignalGroupID,
    ) -> Self {
        Self {
            station_i_d,
            prior_state,
            signal_group,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=10"))]
pub struct PrioritizationResponseList(pub SequenceOf<PrioritizationResponse>);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum PrioritizationResponseStatus {
    Unknown = 0,
    Requested = 1,
    Processing = 2,
    WatchOtherTraffic = 3,
    Granted = 4,
    Rejected = 5,
    MaxPresence = 6,
    ReserviceLocked = 7,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum PriorityRequestType {
    PriorityRequestTypeReserved = 0,
    PriorityRequest = 1,
    PriorityRequestUpdate = 2,
    PriorityCancellation = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct ProtectedCommunicationZone {
    pub protected_zone_type: ProtectedZoneType,
    pub expiry_time: Option<TimestampIts>,
    pub protected_zone_latitude: Latitude,
    pub protected_zone_longitude: Longitude,
    pub protected_zone_radius: Option<ProtectedZoneRadius>,
    pub protected_zone_i_d: Option<ProtectedZoneID>,
}

impl ProtectedCommunicationZone {
    pub fn new(
        protected_zone_type: ProtectedZoneType,
        expiry_time: Option<TimestampIts>,
        protected_zone_latitude: Latitude,
        protected_zone_longitude: Longitude,
        protected_zone_radius: Option<ProtectedZoneRadius>,
        protected_zone_i_d: Option<ProtectedZoneID>,
    ) -> Self {
        Self {
            protected_zone_type,
            expiry_time,
            protected_zone_latitude,
            protected_zone_longitude,
            protected_zone_radius,
            protected_zone_i_d,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct ProtectedCommunicationZonesRSU(pub SequenceOf<ProtectedCommunicationZone>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=134217727"))]
pub struct ProtectedZoneID(pub u32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=255", extensible))]
pub struct ProtectedZoneRadius(pub Integer);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ProtectedZoneType {
    PermanentCenDsrcTolling = 0,
    #[rasn(extension_addition)]
    TemporaryCenDsrcTolling = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ProtocolType(pub VarLengthNumber);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Provider {
    pub country_code: CountryCode,
    pub provider_identifier: IssuerIdentifier,
}

impl Provider {
    pub fn new(country_code: CountryCode, provider_identifier: IssuerIdentifier) -> Self {
        Self {
            country_code,
            provider_identifier,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ProviderMacAddress(pub MACaddress);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ProviderPermissions(pub SequenceOf<ChannelSpecificProviderPermission>);

///ServiceInfo extension elements

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ProviderServiceContext {
    #[rasn(size("3"))]
    pub fill_bit: BitString,
    #[rasn(size("0..=31"))]
    pub psc: OctetString,
}

impl ProviderServiceContext {
    pub fn new(fill_bit: BitString, psc: OctetString) -> Self {
        Self { fill_bit, psc }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0.."))]
pub struct Psid(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PsidSsp {
    pub psid: Psid,
    pub ssp: Option<ServiceSpecificPermissions>,
}

impl PsidSsp {
    pub fn new(psid: Psid, ssp: Option<ServiceSpecificPermissions>) -> Self {
        Self { psid, ssp }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PsidSspRange {
    pub psid: Psid,
    pub ssp_range: Option<SspRange>,
}

impl PsidSspRange {
    pub fn new(psid: Psid, ssp_range: Option<SspRange>) -> Self {
        Self { psid, ssp_range }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PtActivation {
    pub pt_activation_type: PtActivationType,
    pub pt_activation_data: PtActivationData,
}

impl PtActivation {
    pub fn new(pt_activation_type: PtActivationType, pt_activation_data: PtActivationData) -> Self {
        Self {
            pt_activation_type,
            pt_activation_data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=20"))]
pub struct PtActivationData(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct PtActivationType(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum PtvRequestType {
    PreRequest = 0,
    MainRequest = 1,
    DoorCloseRequest = 2,
    CancelRequest = 3,
    EmergencyRequest = 4,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PublicEncryptionKey {
    pub supported_symm_alg: SymmAlgorithm,
    pub public_key: BasePublicEncryptionKey,
}

impl PublicEncryptionKey {
    pub fn new(supported_symm_alg: SymmAlgorithm, public_key: BasePublicEncryptionKey) -> Self {
        Self {
            supported_symm_alg,
            public_key,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum PublicFacilitiesPictogram {
    PublicFacilities = 0,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct PublicTransportContainer {
    pub embarkation_status: EmbarkationStatus,
    pub pt_activation: Option<PtActivation>,
}

impl PublicTransportContainer {
    pub fn new(embarkation_status: EmbarkationStatus, pt_activation: Option<PtActivation>) -> Self {
        Self {
            embarkation_status,
            pt_activation,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum PublicVerificationKey {
    EcdsaNistP256(EccP256CurvePoint),
    EcdsaBrainpoolP256r1(EccP256CurvePoint),
    #[rasn(extension_addition)]
    EcdsaBrainpoolP384r1(EccP384CurvePoint),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=32"))]
pub struct ROI(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct RSCUnit(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RSCode {
    #[rasn(value("1..=4", extensible))]
    pub layout_component_id: Option<u8>,
    pub code: Code,
}

impl RSCode {
    pub fn new(layout_component_id: Option<u8>, code: Code) -> Self {
        Self {
            layout_component_id,
            code,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RSUContainerHighFrequency {
    pub protected_communication_zones_r_s_u: Option<ProtectedCommunicationZonesRSU>,
}

impl RSUContainerHighFrequency {
    pub fn new(
        protected_communication_zones_r_s_u: Option<ProtectedCommunicationZonesRSU>,
    ) -> Self {
        Self {
            protected_communication_zones_r_s_u,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum RTCMRevision {
    Unknown = 0,
    RtcmRev2 = 1,
    RtcmRev3 = 2,
    Reserved = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RTCMEM {
    pub header: ItsPduHeader,
    pub rtcmc: RTCMcorrections,
}

impl RTCMEM {
    pub fn new(header: ItsPduHeader, rtcmc: RTCMcorrections) -> Self {
        Self { header, rtcmc }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct RTCMcorrectionsRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RTCMcorrections {
    pub msg_cnt: MsgCount,
    pub rev: RTCMRevision,
    pub time_stamp: Option<MinuteOfTheYear>,
    pub anchor_point: Option<FullPositionVector>,
    pub rtcm_header: Option<RTCMheader>,
    pub msgs: RTCMmessageList,
    pub regional: Option<RTCMcorrectionsRegional>,
}

impl RTCMcorrections {
    pub fn new(
        msg_cnt: MsgCount,
        rev: RTCMRevision,
        time_stamp: Option<MinuteOfTheYear>,
        anchor_point: Option<FullPositionVector>,
        rtcm_header: Option<RTCMheader>,
        msgs: RTCMmessageList,
        regional: Option<RTCMcorrectionsRegional>,
    ) -> Self {
        Self {
            msg_cnt,
            rev,
            time_stamp,
            anchor_point,
            rtcm_header,
            msgs,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RTCMheader {
    pub status: GNSSstatus,
    pub offset_set: AntennaOffsetSet,
}

impl RTCMheader {
    pub fn new(status: GNSSstatus, offset_set: AntennaOffsetSet) -> Self {
        Self { status, offset_set }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=1023"))]
pub struct RTCMmessage(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=5"))]
pub struct RTCMmessageList(pub SequenceOf<RTCMmessage>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=10000"))]
pub struct Radius(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=10000"))]
pub struct Range(pub u16);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct RccPartZoneIds(pub SequenceOf<Zid>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct RccPartLaneConfiguration(pub SequenceOf<LaneInformation>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RccPart {
    pub zone_ids: RccPartZoneIds,
    pub road_type: RoadType,
    pub lane_configuration: RccPartLaneConfiguration,
}

impl RccPart {
    pub fn new(
        zone_ids: RccPartZoneIds,
        road_type: RoadType,
        lane_configuration: RccPartLaneConfiguration,
    ) -> Self {
        Self {
            zone_ids,
            road_type,
            lane_configuration,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RcpiThreshold(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=150"))]
pub struct RearOverhang(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RectangularRegion {
    pub north_west: Dot2TwoDLocation,
    pub south_east: Dot2TwoDLocation,
}

impl RectangularRegion {
    pub fn new(north_west: Dot2TwoDLocation, south_east: Dot2TwoDLocation) -> Self {
        Self {
            north_west,
            south_east,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RefExt(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RefPointId(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct ReferenceDenms(pub SequenceOf<ActionID>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ReferencePosition {
    pub latitude: Latitude,
    pub longitude: Longitude,
    pub position_confidence_ellipse: PosConfidenceEllipse,
    pub altitude: Altitude,
}

impl ReferencePosition {
    pub fn new(
        latitude: Latitude,
        longitude: Longitude,
        position_confidence_ellipse: PosConfidenceEllipse,
        altitude: Altitude,
    ) -> Self {
        Self {
            latitude,
            longitude,
            position_confidence_ellipse,
            altitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RegionAndSubregions {
    pub region: Uint8,
    pub subregions: SequenceOfUint16,
}

impl RegionAndSubregions {
    pub fn new(region: Uint8, subregions: SequenceOfUint16) -> Self {
        Self { region, subregions }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RegionId(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RegionalExtension {
    pub region_id: RegionId,
    pub reg_ext_value: Any,
}

impl RegionalExtension {
    pub fn new(region_id: RegionId, reg_ext_value: Any) -> Self {
        Self {
            region_id,
            reg_ext_value,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RegulatorySpeedLimit {
    pub r_type: SpeedLimitType,
    pub speed: Velocity,
}

impl RegulatorySpeedLimit {
    pub fn new(r_type: SpeedLimitType, speed: Velocity) -> Self {
        Self { r_type, speed }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum RejectedReason {
    Unknown = 0,
    ExceptionalCondition = 1,
    MaxWaitingTimeExceeded = 2,
    PtPriorityDisabled = 3,
    HigherPTPriorityGranted = 4,
    VehicleTrackingUnknown = 5,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RelevanceDistance {
    LessThan50m = 0,
    LessThan100m = 1,
    LessThan200m = 2,
    LessThan500m = 3,
    LessThan1000m = 4,
    LessThan5km = 5,
    LessThan10km = 6,
    Over10km = 7,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RelevanceTrafficDirection {
    AllTrafficDirections = 0,
    UpstreamTraffic = 1,
    DownstreamTraffic = 2,
    OppositeTraffic = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RepeatRate(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ReplyAddress(pub PortNumber);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RequestID(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RequestImportanceLevel {
    RequestImportanceLevelUnKnown = 0,
    RequestImportanceLevel1 = 1,
    RequestImportanceLevel2 = 2,
    RequestImportanceLevel3 = 3,
    RequestImportanceLevel4 = 4,
    RequestImportanceLevel5 = 5,
    RequestImportanceLevel6 = 6,
    RequestImportanceLevel7 = 7,
    RequestImportanceLevel8 = 8,
    RequestImportanceLevel9 = 9,
    RequestImportanceLevel10 = 10,
    RequestImportanceLevel11 = 11,
    RequestImportanceLevel12 = 12,
    RequestImportanceLevel13 = 13,
    RequestImportanceLevel14 = 14,
    RequestImportanceReserved = 15,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RequestResponseIndication {
    Request = 0,
    Response = 1,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RequestSubRole {
    RequestSubRoleUnKnown = 0,
    RequestSubRole1 = 1,
    RequestSubRole2 = 2,
    RequestSubRole3 = 3,
    RequestSubRole4 = 4,
    RequestSubRole5 = 5,
    RequestSubRole6 = 6,
    RequestSubRole7 = 7,
    RequestSubRole8 = 8,
    RequestSubRole9 = 9,
    RequestSubRole10 = 10,
    RequestSubRole11 = 11,
    RequestSubRole12 = 12,
    RequestSubRole13 = 13,
    RequestSubRole14 = 14,
    RequestSubRoleReserved = 15,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct RequesterDescriptionRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RequesterDescription {
    pub id: VehicleID,
    pub r_type: Option<RequesterType>,
    pub position: Option<RequesterPositionVector>,
    pub name: Option<DescriptiveName>,
    pub route_name: Option<DescriptiveName>,
    pub transit_status: Option<TransitVehicleStatus>,
    pub transit_occupancy: Option<TransitVehicleOccupancy>,
    pub transit_schedule: Option<DeltaTime>,
    pub regional: Option<RequesterDescriptionRegional>,
}

impl RequesterDescription {
    pub fn new(
        id: VehicleID,
        r_type: Option<RequesterType>,
        position: Option<RequesterPositionVector>,
        name: Option<DescriptiveName>,
        route_name: Option<DescriptiveName>,
        transit_status: Option<TransitVehicleStatus>,
        transit_occupancy: Option<TransitVehicleOccupancy>,
        transit_schedule: Option<DeltaTime>,
        regional: Option<RequesterDescriptionRegional>,
    ) -> Self {
        Self {
            id,
            r_type,
            position,
            name,
            route_name,
            transit_status,
            transit_occupancy,
            transit_schedule,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RequesterDescriptionAddGrpC {
    pub fuel: Option<FuelType>,
    pub battery_status: Option<BatteryStatus>,
}

impl RequesterDescriptionAddGrpC {
    pub fn new(fuel: Option<FuelType>, battery_status: Option<BatteryStatus>) -> Self {
        Self {
            fuel,
            battery_status,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RequesterPositionVector {
    pub position: Position3D,
    pub heading: Option<Angle>,
    pub speed: Option<TransmissionAndSpeed>,
}

impl RequesterPositionVector {
    pub fn new(
        position: Position3D,
        heading: Option<Angle>,
        speed: Option<TransmissionAndSpeed>,
    ) -> Self {
        Self {
            position,
            heading,
            speed,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RequesterType {
    pub role: BasicVehicleRole,
    pub subrole: Option<RequestSubRole>,
    pub request: Option<RequestImportanceLevel>,
    pub iso3883: Option<Iso3833VehicleType>,
    pub hpms_type: Option<VehicleType>,
    #[rasn(value("0.."))]
    pub regional: Option<RegionalExtension>,
}

impl RequesterType {
    pub fn new(
        role: BasicVehicleRole,
        subrole: Option<RequestSubRole>,
        request: Option<RequestImportanceLevel>,
        iso3883: Option<Iso3833VehicleType>,
        hpms_type: Option<VehicleType>,
        regional: Option<RegionalExtension>,
    ) -> Self {
        Self {
            role,
            subrole,
            request,
            iso3883,
            hpms_type,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RescueAndRecoveryWorkInProgressSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RescueContainer {
    pub light_bar_siren_in_use: LightBarSirenInUse,
}

impl RescueContainer {
    pub fn new(light_bar_siren_in_use: LightBarSirenInUse) -> Self {
        Self {
            light_bar_siren_in_use,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=3", extensible))]
pub struct RestrictedTypes(pub SequenceOf<StationType>);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum RestrictionAppliesTo {
    None = 0,
    EquippedTransit = 1,
    EquippedTaxis = 2,
    EquippedOther = 3,
    EmissionCompliant = 4,
    EquippedBicycle = 5,
    WeightCompliant = 6,
    HeightCompliant = 7,
    Pedestrians = 8,
    SlowMovingPersons = 9,
    WheelchairUsers = 10,
    VisualDisabilities = 11,
    AudioDisabilities = 12,
    OtherUnknownDisabilities = 13,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RestrictionClassAssignment {
    pub id: RestrictionClassID,
    pub users: RestrictionUserTypeList,
}

impl RestrictionClassAssignment {
    pub fn new(id: RestrictionClassID, users: RestrictionUserTypeList) -> Self {
        Self { id, users }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RestrictionClassID(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=254"))]
pub struct RestrictionClassList(pub SequenceOf<RestrictionClassAssignment>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct RestrictionUserTypeRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum RestrictionUserType {
    BasicType(RestrictionAppliesTo),
    Regional(RestrictionUserTypeRegional),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RestrictionUserTypeAddGrpC {
    pub emission: Option<EmissionType>,
    pub fuel: Option<FuelType>,
}

impl RestrictionUserTypeAddGrpC {
    pub fn new(emission: Option<EmissionType>, fuel: Option<FuelType>) -> Self {
        Self { emission, fuel }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16"))]
pub struct RestrictionUserTypeList(pub SequenceOf<RestrictionUserType>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct RoadAngles(pub SequenceOf<Heading>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct RoadConfigurationContainer(pub SequenceOf<RccPart>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=255"))]
pub struct RoadLaneSetList(pub SequenceOf<GenericLane>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct RoadRegulatorID(pub u16);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct RoadSegmentRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct RoadSegment {
    pub name: Option<DescriptiveName>,
    pub id: RoadSegmentReferenceID,
    pub revision: MsgCount,
    pub ref_point: Position3D,
    pub lane_width: Option<LaneWidth>,
    pub speed_limits: Option<SpeedLimitList>,
    pub road_lane_set: RoadLaneSetList,
    pub regional: Option<RoadSegmentRegional>,
}

impl RoadSegment {
    pub fn new(
        name: Option<DescriptiveName>,
        id: RoadSegmentReferenceID,
        revision: MsgCount,
        ref_point: Position3D,
        lane_width: Option<LaneWidth>,
        speed_limits: Option<SpeedLimitList>,
        road_lane_set: RoadLaneSetList,
        regional: Option<RoadSegmentRegional>,
    ) -> Self {
        Self {
            name,
            id,
            revision,
            ref_point,
            lane_width,
            speed_limits,
            road_lane_set,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct RoadSegmentID(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32"))]
pub struct RoadSegmentList(pub SequenceOf<RoadSegment>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoadSegmentReferenceID {
    pub region: Option<RoadRegulatorID>,
    pub id: RoadSegmentID,
}

impl RoadSegmentReferenceID {
    pub fn new(region: Option<RoadRegulatorID>, id: RoadSegmentID) -> Self {
        Self { region, id }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum RoadType {
    UrbanNoStructuralSeparationToOppositeLanes = 0,
    UrbanWithStructuralSeparationToOppositeLanes = 1,
    NonUrbanNoStructuralSeparationToOppositeLanes = 2,
    NonUrbanWithStructuralSeparationToOppositeLanes = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoadWorksContainerBasic {
    pub roadworks_sub_cause_code: Option<RoadworksSubCauseCode>,
    pub light_bar_siren_in_use: LightBarSirenInUse,
    pub closed_lanes: Option<ClosedLanes>,
}

impl RoadWorksContainerBasic {
    pub fn new(
        roadworks_sub_cause_code: Option<RoadworksSubCauseCode>,
        light_bar_siren_in_use: LightBarSirenInUse,
        closed_lanes: Option<ClosedLanes>,
    ) -> Self {
        Self {
            roadworks_sub_cause_code,
            light_bar_siren_in_use,
            closed_lanes,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoadWorksContainerExtended {
    pub light_bar_siren_in_use: Option<LightBarSirenInUse>,
    pub closed_lanes: Option<ClosedLanes>,
    pub restriction: Option<RestrictedTypes>,
    pub speed_limit: Option<SpeedLimit>,
    pub incident_indication: Option<CauseCode>,
    pub recommended_path: Option<ItineraryPath>,
    pub starting_point_speed_limit: Option<DeltaReferencePosition>,
    pub traffic_flow_rule: Option<TrafficRule>,
    pub reference_denms: Option<ReferenceDenms>,
}

impl RoadWorksContainerExtended {
    pub fn new(
        light_bar_siren_in_use: Option<LightBarSirenInUse>,
        closed_lanes: Option<ClosedLanes>,
        restriction: Option<RestrictedTypes>,
        speed_limit: Option<SpeedLimit>,
        incident_indication: Option<CauseCode>,
        recommended_path: Option<ItineraryPath>,
        starting_point_speed_limit: Option<DeltaReferencePosition>,
        traffic_flow_rule: Option<TrafficRule>,
        reference_denms: Option<ReferenceDenms>,
    ) -> Self {
        Self {
            light_bar_siren_in_use,
            closed_lanes,
            restriction,
            speed_limit,
            incident_indication,
            recommended_path,
            starting_point_speed_limit,
            traffic_flow_rule,
            reference_denms,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-128..=127"))]
pub struct RoadwayCrownAngle(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct RoadworksSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct RoutAdvertExt(pub Extension);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct RoutAdvertExts(pub SequenceOf<RoutAdvertExt>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct RouterLifetime(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct RoutingAdvertisement {
    pub lifetime: RouterLifetime,
    pub ip_prefix: IpV6Prefix,
    pub ip_prefix_length: IpV6PrefixLength,
    pub default_gateway: IPv6Address,
    pub primary_dns: IPv6Address,
    pub extensions: RoutAdvertExts,
}

impl RoutingAdvertisement {
    pub fn new(
        lifetime: RouterLifetime,
        ip_prefix: IpV6Prefix,
        ip_prefix_length: IpV6PrefixLength,
        default_gateway: IPv6Address,
        primary_dns: IPv6Address,
        extensions: RoutAdvertExts,
    ) -> Self {
        Self {
            lifetime,
            ip_prefix,
            ip_prefix_length,
            default_gateway,
            primary_dns,
            extensions,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct RsvAdvPrtVersion(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=28800"))]
pub struct SAEHeading(pub u16);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum SAEHeadingConfidence {
    Unavailable = 0,
    Prec10deg = 1,
    Prec05deg = 2,
    Prec01deg = 3,
    prec0_1deg = 4,
    Prec005deg = 5,
    Prec001deg = 6,
    Prec00125deg = 7,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum SAESpeedConfidence {
    Unavailable = 0,
    Prec100ms = 1,
    Prec10ms = 2,
    Prec5ms = 3,
    Prec1ms = 4,
    Prec01ms = 5,
    Prec005ms = 6,
    Prec001ms = 7,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum SAEThrottleConfidence {
    Unavailable = 0,
    Prec10percent = 1,
    Prec1percent = 2,
    Prec05percent = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-4096..=61439"))]
pub struct SAElevation(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SALatitude {
    #[rasn(size("1"))]
    pub fill_bit: BitString,
    #[rasn(value("-900000000..=900000001"))]
    pub lat: i32,
}

impl SALatitude {
    pub fn new(fill_bit: BitString, lat: i32) -> Self {
        Self { fill_bit, lat }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1800000000..=1800000001"))]
pub struct SALongitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SAMapplicationData(pub OctetString);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct SPATRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SPAT {
    pub time_stamp: Option<MinuteOfTheYear>,
    pub name: Option<DescriptiveName>,
    pub intersections: IntersectionStateList,
    pub regional: Option<SPATRegional>,
}

impl SPAT {
    pub fn new(
        time_stamp: Option<MinuteOfTheYear>,
        name: Option<DescriptiveName>,
        intersections: IntersectionStateList,
        regional: Option<SPATRegional>,
    ) -> Self {
        Self {
            time_stamp,
            name,
            intersections,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SPATEM {
    pub header: ItsPduHeader,
    pub spat: SPAT,
}

impl SPATEM {
    pub fn new(header: ItsPduHeader, spat: SPAT) -> Self {
        Self { header, spat }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SPE {
    #[rasn(value("0..=250"))]
    pub spm: Option<u8>,
    #[rasn(value("0..=250"))]
    pub mns: Option<u8>,
    pub unit: RSCUnit,
}

impl SPE {
    pub fn new(spm: Option<u8>, mns: Option<u8>, unit: RSCUnit) -> Self {
        Self { spm, mns, unit }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SREM {
    pub header: ItsPduHeader,
    pub srm: SignalRequestMessage,
}

impl SREM {
    pub fn new(header: ItsPduHeader, srm: SignalRequestMessage) -> Self {
        Self { header, srm }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SRMextension(pub Extension);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SRMextensions(pub SequenceOf<SRMextension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SSEM {
    pub header: ItsPduHeader,
    pub ssm: SignalStatusMessage,
}

impl SSEM {
    pub fn new(header: ItsPduHeader, ssm: SignalStatusMessage) -> Self {
        Self { header, ssm }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SafetyCarContainer {
    pub light_bar_siren_in_use: LightBarSirenInUse,
    pub incident_indication: Option<CauseCode>,
    pub traffic_rule: Option<TrafficRule>,
    pub speed_limit: Option<SpeedLimit>,
}

impl SafetyCarContainer {
    pub fn new(
        light_bar_siren_in_use: LightBarSirenInUse,
        incident_indication: Option<CauseCode>,
        traffic_rule: Option<TrafficRule>,
        speed_limit: Option<SpeedLimit>,
    ) -> Self {
        Self {
            light_bar_siren_in_use,
            incident_indication,
            traffic_rule,
            speed_limit,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Sam {
    pub version: RsvAdvPrtVersion,
    pub body: SamBody,
}

impl Sam {
    pub fn new(version: RsvAdvPrtVersion, body: SamBody) -> Self {
        Self { version, body }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SamBody {
    pub change_count: SrvAdvChangeCount,
    pub extensions: Option<SrvAdvMsgHeaderExts>,
    pub service_infos: Option<ServiceInfos>,
    pub channel_infos: Option<ChannelInfos>,
    pub routing_advertisement: Option<RoutingAdvertisement>,
}

impl SamBody {
    pub fn new(
        change_count: SrvAdvChangeCount,
        extensions: Option<SrvAdvMsgHeaderExts>,
        service_infos: Option<ServiceInfos>,
        channel_infos: Option<ChannelInfos>,
        routing_advertisement: Option<RoutingAdvertisement>,
    ) -> Self {
        Self {
            change_count,
            extensions,
            service_infos,
            channel_infos,
            routing_advertisement,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SamContext {
    pub itsaid_ctx_ref: ItsAidCtxRef,
    pub context: Any,
}

impl SamContext {
    pub fn new(itsaid_ctx_ref: ItsAidCtxRef, context: Any) -> Self {
        Self {
            itsaid_ctx_ref,
            context,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-2048..=2047"))]
pub struct ScaleB12(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SecondaryDns(pub IPv6Address);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Segment {
    pub line: PolygonalLine,
    pub lane_width: Option<IVILaneWidth>,
}

impl Segment {
    pub fn new(line: PolygonalLine, lane_width: Option<IVILaneWidth>) -> Self {
        Self { line, lane_width }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum SegmentAttributeXY {
    Reserved = 0,
    DoNotBlock = 1,
    WhiteLine = 2,
    MergingLaneLeft = 3,
    MergingLaneRight = 4,
    CurbOnLeft = 5,
    CurbOnRight = 6,
    LoadingzoneOnLeft = 7,
    LoadingzoneOnRight = 8,
    TurnOutPointOnLeft = 9,
    TurnOutPointOnRight = 10,
    AdjacentParkingOnLeft = 11,
    AdjacentParkingOnRight = 12,
    AdjacentBikeLaneOnLeft = 13,
    AdjacentBikeLaneOnRight = 14,
    SharedBikeLane = 15,
    BikeBoxInFront = 16,
    TransitStopOnLeft = 17,
    TransitStopOnRight = 18,
    TransitStopInLane = 19,
    SharedWithTrackedVehicle = 20,
    SafeIsland = 21,
    LowCurbsPresent = 22,
    RumbleStripPresent = 23,
    AudibleSignalingPresent = 24,
    AdaptiveTimingPresent = 25,
    RfSignalRequestPresent = 26,
    PartialCurbIntrusion = 27,
    TaperToLeft = 28,
    TaperToRight = 29,
    TaperToCenterLine = 30,
    ParallelParking = 31,
    HeadInParking = 32,
    FreeParking = 33,
    TimeRestrictionsOnParking = 34,
    CostToPark = 35,
    MidBlockCurbPresent = 36,
    UnEvenPavementPresent = 37,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct SegmentAttributeXYList(pub SequenceOf<SegmentAttributeXY>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct SegmentCount(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4095"))]
pub struct SemiAxisLength(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SemiMajorAxisAccuracy(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct SemiMajorAxisOrientation(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SemiMinorAxisAccuracy(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=10000"))]
pub struct SemiRangeLength(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-5000..=5000"))]
pub struct SensorHeight(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=128", extensible))]
pub struct SensorIdList(pub SequenceOf<Identifier>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SensorInformation {
    pub sensor_i_d: Identifier,
    pub r_type: SensorType,
    pub detection_area: DetectionArea,
    pub free_space_confidence: Option<FreeSpaceConfidence>,
}

impl SensorInformation {
    pub fn new(
        sensor_i_d: Identifier,
        r_type: SensorType,
        detection_area: DetectionArea,
        free_space_confidence: Option<FreeSpaceConfidence>,
    ) -> Self {
        Self {
            sensor_i_d,
            r_type,
            detection_area,
            free_space_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=128", extensible))]
pub struct SensorInformationContainer(pub SequenceOf<SensorInformation>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct SensorType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct SequenceNumber(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfEdcaIdentifier(pub SequenceOf<EdcaIdentifier>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfHashedId3(pub SequenceOf<HashedId3>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfIdentifiedRegion(pub SequenceOf<IdentifiedRegion>);

/// Anonymous SEQUENCE OF member

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct AnonymousSequenceOfOctetString(pub OctetString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfOctetString(pub SequenceOf<AnonymousSequenceOfOctetString>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfPsid(pub SequenceOf<Psid>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfPsidSsp(pub SequenceOf<PsidSsp>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfPsidSspRange(pub SequenceOf<PsidSspRange>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfRectangularRegion(pub SequenceOf<RectangularRegion>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfRegionAndSubregions(pub SequenceOf<RegionAndSubregions>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8"))]
pub struct SequenceOfTrajectoryInterceptionIndication(
    pub SequenceOf<TrajectoryInterceptionIndication>,
);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfUint16(pub SequenceOf<Uint16>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SequenceOfUint8(pub SequenceOf<Uint8>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("0..=8"))]
pub struct SequenceOfVruSafeDistanceIndication(pub SequenceOf<VruSafeDistanceIndication>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ServiceCategoryCode {
    TrafficSignPictogram(TrafficSignPictogram),
    PublicFacilitiesPictogram(PublicFacilitiesPictogram),
    AmbientOrRoadConditionPictogram(AmbientOrRoadConditionPictogram),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ServiceInfo {
    pub service_i_d: ITSaid,
    pub channel_index: ChannelIndex,
    pub ch_options: ChannelOptions,
}

impl ServiceInfo {
    pub fn new(
        service_i_d: ITSaid,
        channel_index: ChannelIndex,
        ch_options: ChannelOptions,
    ) -> Self {
        Self {
            service_i_d,
            channel_index,
            ch_options,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ServiceInfoExt(pub Extension);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ServiceInfoExts(pub SequenceOf<ServiceInfoExt>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ServiceInfos(pub SequenceOf<ServiceInfo>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct ServicePort(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum ServiceSpecificPermissions {
    Opaque(OctetString),
    #[rasn(extension_addition)]
    BitmapSsp(BitmapSsp),
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct ShadowingApplies(pub bool);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalControlZone {
    #[rasn(value("0.."))]
    pub zone: RegionalExtension,
}

impl SignalControlZone {
    pub fn new(zone: RegionalExtension) -> Self {
        Self { zone }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SignalGroupID(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalHeadLocation {
    pub node_x_y: NodeOffsetPointXY,
    pub node_z: DeltaAltitude,
    pub signal_group_i_d: SignalGroupID,
}

impl SignalHeadLocation {
    pub fn new(
        node_x_y: NodeOffsetPointXY,
        node_z: DeltaAltitude,
        signal_group_i_d: SignalGroupID,
    ) -> Self {
        Self {
            node_x_y,
            node_z,
            signal_group_i_d,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=64"))]
pub struct SignalHeadLocationList(pub SequenceOf<SignalHeadLocation>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct SignalRequestRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalRequest {
    pub id: IntersectionReferenceID,
    pub request_i_d: RequestID,
    pub request_type: PriorityRequestType,
    pub in_bound_lane: IntersectionAccessPoint,
    pub out_bound_lane: Option<IntersectionAccessPoint>,
    pub regional: Option<SignalRequestRegional>,
}

impl SignalRequest {
    pub fn new(
        id: IntersectionReferenceID,
        request_i_d: RequestID,
        request_type: PriorityRequestType,
        in_bound_lane: IntersectionAccessPoint,
        out_bound_lane: Option<IntersectionAccessPoint>,
        regional: Option<SignalRequestRegional>,
    ) -> Self {
        Self {
            id,
            request_i_d,
            request_type,
            in_bound_lane,
            out_bound_lane,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32"))]
pub struct SignalRequestList(pub SequenceOf<SignalRequestPackage>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct SignalRequestMessageRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalRequestMessage {
    pub time_stamp: Option<MinuteOfTheYear>,
    pub second: DSecond,
    pub sequence_number: Option<MsgCount>,
    pub requests: Option<SignalRequestList>,
    pub requester: RequesterDescription,
    pub regional: Option<SignalRequestMessageRegional>,
}

impl SignalRequestMessage {
    pub fn new(
        time_stamp: Option<MinuteOfTheYear>,
        second: DSecond,
        sequence_number: Option<MsgCount>,
        requests: Option<SignalRequestList>,
        requester: RequesterDescription,
        regional: Option<SignalRequestMessageRegional>,
    ) -> Self {
        Self {
            time_stamp,
            second,
            sequence_number,
            requests,
            requester,
            regional,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct SignalRequestPackageRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalRequestPackage {
    pub request: SignalRequest,
    pub minute: Option<MinuteOfTheYear>,
    pub second: Option<DSecond>,
    pub duration: Option<DSecond>,
    pub regional: Option<SignalRequestPackageRegional>,
}

impl SignalRequestPackage {
    pub fn new(
        request: SignalRequest,
        minute: Option<MinuteOfTheYear>,
        second: Option<DSecond>,
        duration: Option<DSecond>,
        regional: Option<SignalRequestPackageRegional>,
    ) -> Self {
        Self {
            request,
            minute,
            second,
            duration,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalRequesterInfo {
    pub id: VehicleID,
    pub request: RequestID,
    pub sequence_number: MsgCount,
    pub role: Option<BasicVehicleRole>,
    pub type_data: Option<RequesterType>,
}

impl SignalRequesterInfo {
    pub fn new(
        id: VehicleID,
        request: RequestID,
        sequence_number: MsgCount,
        role: Option<BasicVehicleRole>,
        type_data: Option<RequesterType>,
    ) -> Self {
        Self {
            id,
            request,
            sequence_number,
            role,
            type_data,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct SignalStatusRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalStatus {
    pub sequence_number: MsgCount,
    pub id: IntersectionReferenceID,
    pub sig_status: SignalStatusPackageList,
    pub regional: Option<SignalStatusRegional>,
}

impl SignalStatus {
    pub fn new(
        sequence_number: MsgCount,
        id: IntersectionReferenceID,
        sig_status: SignalStatusPackageList,
        regional: Option<SignalStatusRegional>,
    ) -> Self {
        Self {
            sequence_number,
            id,
            sig_status,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32"))]
pub struct SignalStatusList(pub SequenceOf<SignalStatus>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct SignalStatusMessageRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalStatusMessage {
    pub time_stamp: Option<MinuteOfTheYear>,
    pub second: DSecond,
    pub sequence_number: Option<MsgCount>,
    pub status: SignalStatusList,
    pub regional: Option<SignalStatusMessageRegional>,
}

impl SignalStatusMessage {
    pub fn new(
        time_stamp: Option<MinuteOfTheYear>,
        second: DSecond,
        sequence_number: Option<MsgCount>,
        status: SignalStatusList,
        regional: Option<SignalStatusMessageRegional>,
    ) -> Self {
        Self {
            time_stamp,
            second,
            sequence_number,
            status,
            regional,
        }
    }
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct SignalStatusPackageRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalStatusPackage {
    pub requester: Option<SignalRequesterInfo>,
    pub inbound_on: IntersectionAccessPoint,
    pub outbound_on: Option<IntersectionAccessPoint>,
    pub minute: Option<MinuteOfTheYear>,
    pub second: Option<DSecond>,
    pub duration: Option<DSecond>,
    pub status: PrioritizationResponseStatus,
    pub regional: Option<SignalStatusPackageRegional>,
}

impl SignalStatusPackage {
    pub fn new(
        requester: Option<SignalRequesterInfo>,
        inbound_on: IntersectionAccessPoint,
        outbound_on: Option<IntersectionAccessPoint>,
        minute: Option<MinuteOfTheYear>,
        second: Option<DSecond>,
        duration: Option<DSecond>,
        status: PrioritizationResponseStatus,
        regional: Option<SignalStatusPackageRegional>,
    ) -> Self {
        Self {
            requester,
            inbound_on,
            outbound_on,
            minute,
            second,
            duration,
            status,
            regional,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SignalStatusPackageAddGrpC {
    pub synch_to_schedule: Option<DeltaTime>,
    pub rejected_reason: Option<RejectedReason>,
}

impl SignalStatusPackageAddGrpC {
    pub fn new(
        synch_to_schedule: Option<DeltaTime>,
        rejected_reason: Option<RejectedReason>,
    ) -> Self {
        Self {
            synch_to_schedule,
            rejected_reason,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=32"))]
pub struct SignalStatusPackageList(pub SequenceOf<SignalStatusPackage>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SignalViolationSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Signature {
    EcdsaNistP256Signature(EcdsaP256Signature),
    EcdsaBrainpoolP256r1Signature(EcdsaP256Signature),
    #[rasn(extension_addition)]
    EcdsaBrainpoolP384r1Signature(EcdsaP384Signature),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct SituationContainer {
    pub information_quality: InformationQuality,
    pub event_type: CauseCode,
    pub linked_cause: Option<CauseCode>,
    pub event_history: Option<EventHistory>,
}

impl SituationContainer {
    pub fn new(
        information_quality: InformationQuality,
        event_type: CauseCode,
        linked_cause: Option<CauseCode>,
        event_history: Option<EventHistory>,
    ) -> Self {
        Self {
            information_quality,
            event_type,
            linked_cause,
            event_history,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SlowVehicleSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SoundLevel {
    pub soundstationary: Int1,
    pub sounddriveby: Int1,
}

impl SoundLevel {
    pub fn new(soundstationary: Int1, sounddriveby: Int1) -> Self {
        Self {
            soundstationary,
            sounddriveby,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SpecialTransportContainer {
    pub special_transport_type: SpecialTransportType,
    pub light_bar_siren_in_use: LightBarSirenInUse,
}

impl SpecialTransportContainer {
    pub fn new(
        special_transport_type: SpecialTransportType,
        light_bar_siren_in_use: LightBarSirenInUse,
    ) -> Self {
        Self {
            special_transport_type,
            light_bar_siren_in_use,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("4"))]
pub struct SpecialTransportType(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SpecialVehicleContainer {
    PublicTransportContainer(PublicTransportContainer),
    SpecialTransportContainer(SpecialTransportContainer),
    DangerousGoodsContainer(DangerousGoodsContainer),
    RoadWorksContainerBasic(RoadWorksContainerBasic),
    RescueContainer(RescueContainer),
    EmergencyContainer(EmergencyContainer),
    SafetyCarContainer(SafetyCarContainer),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Speed {
    pub speed_value: SpeedValue,
    pub speed_confidence: SpeedConfidence,
}

impl Speed {
    pub fn new(speed_value: SpeedValue, speed_confidence: SpeedConfidence) -> Self {
        Self {
            speed_value,
            speed_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=500"))]
pub struct SpeedAdvice(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct SpeedConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SpeedExtended {
    pub value: SpeedValueExtended,
    pub confidence: SpeedConfidence,
}

impl SpeedExtended {
    pub fn new(value: SpeedValueExtended, confidence: SpeedConfidence) -> Self {
        Self { value, confidence }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct SpeedLimit(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=9"))]
pub struct SpeedLimitList(pub SequenceOf<RegulatorySpeedLimit>);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum SpeedLimitType {
    Unknown = 0,
    MaxSpeedInSchoolZone = 1,
    MaxSpeedInSchoolZoneWhenChildrenArePresent = 2,
    MaxSpeedInConstructionZone = 3,
    VehicleMinSpeed = 4,
    VehicleMaxSpeed = 5,
    VehicleNightMaxSpeed = 6,
    TruckMinSpeed = 7,
    TruckMaxSpeed = 8,
    TruckNightMaxSpeed = 9,
    VehiclesWithTrailersMinSpeed = 10,
    VehiclesWithTrailersMaxSpeed = 11,
    VehiclesWithTrailersNightMaxSpeed = 12,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=16383"))]
pub struct SpeedValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-16383..=16383"))]
pub struct SpeedValueExtended(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SpeedandHeadingandThrottleConfidence {
    pub heading: SAEHeadingConfidence,
    pub speed: SAESpeedConfidence,
    pub throttle: SAEThrottleConfidence,
}

impl SpeedandHeadingandThrottleConfidence {
    pub fn new(
        heading: SAEHeadingConfidence,
        speed: SAESpeedConfidence,
        throttle: SAEThrottleConfidence,
    ) -> Self {
        Self {
            heading,
            speed,
            throttle,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SpotAvailability {
    #[rasn(value("0..=1400"))]
    pub max_waiting_time_minutes: u16,
    pub blocking: bool,
}

impl SpotAvailability {
    pub fn new(max_waiting_time_minutes: u16, blocking: bool) -> Self {
        Self {
            max_waiting_time_minutes,
            blocking,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Srm {
    pub header: RsvAdvPrtVersion,
    pub body: SrmBody,
}

impl Srm {
    pub fn new(header: RsvAdvPrtVersion, body: SrmBody) -> Self {
        Self { header, body }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SrmBody {
    pub extensions: Option<SRMextensions>,
    pub prv_channels_rq: Option<SrmPrivateChannelsRq>,
    pub contexts: Option<SrmContexts>,
    pub prv_channels_cf: Option<SrmPrvChAllocConf>,
}

impl SrmBody {
    pub fn new(
        extensions: Option<SRMextensions>,
        prv_channels_rq: Option<SrmPrivateChannelsRq>,
        contexts: Option<SrmContexts>,
        prv_channels_cf: Option<SrmPrvChAllocConf>,
    ) -> Self {
        Self {
            extensions,
            prv_channels_rq,
            contexts,
            prv_channels_cf,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SrmContext {
    pub context: SamContext,
    pub client_port: PortNumber,
}

impl SrmContext {
    pub fn new(context: SamContext, client_port: PortNumber) -> Self {
        Self {
            context,
            client_port,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SrmContexts(pub SequenceOf<SrmContext>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SrmPrivateChannelsRq {
    pub port_dyn_sam: PortNumber,
    pub alloc_reqs: SrmPrvChAllocReq,
}

impl SrmPrivateChannelsRq {
    pub fn new(port_dyn_sam: PortNumber, alloc_reqs: SrmPrvChAllocReq) -> Self {
        Self {
            port_dyn_sam,
            alloc_reqs,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SrmPrvChAllocConf(pub SequenceOf<ITSaid>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SrmPrvChAllocReq(pub SequenceOf<ITSaid>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SrvAdvChangeCount {
    pub sa_i_d: SrvAdvID,
    pub content_count: SrvAdvContentCount,
}

impl SrvAdvChangeCount {
    pub fn new(sa_i_d: SrvAdvID, content_count: SrvAdvContentCount) -> Self {
        Self {
            sa_i_d,
            content_count,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct SrvAdvContentCount(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct SrvAdvID(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SrvAdvMsgHeaderExt(pub Extension);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SrvAdvMsgHeaderExts(pub SequenceOf<SrvAdvMsgHeaderExt>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SspRange {
    Opaque(SequenceOfOctetString),
    All(()),
    #[rasn(extension_addition)]
    BitmapSspRange(BitmapSspRange),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct StabilityChangeIndication {
    pub loss_probability: StabilityLossProbability,
    pub action_delta_time: ActionDeltaTime,
}

impl StabilityChangeIndication {
    pub fn new(
        loss_probability: StabilityLossProbability,
        action_delta_time: ActionDeltaTime,
    ) -> Self {
        Self {
            loss_probability,
            action_delta_time,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=63"))]
pub struct StabilityLossProbability(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum StationDataContainer {
    OriginatingVehicleContainer(OriginatingVehicleContainer),
    OriginatingRSUContainer(OriginatingRSUContainer),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct StationID(pub u32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct StationType(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum StationarySince {
    LessThan1Minute = 0,
    LessThan2Minutes = 1,
    LessThan15Minutes = 2,
    EqualOrGreater15Minutes = 3,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct StationaryVehicleContainer {
    pub stationary_since: Option<StationarySince>,
    pub stationary_cause: Option<CauseCode>,
    pub carrying_dangerous_goods: Option<DangerousGoodsExtended>,
    pub number_of_occupants: Option<NumberOfOccupants>,
    pub vehicle_identification: Option<VehicleIdentification>,
    pub energy_storage_type: Option<EnergyStorageType>,
}

impl StationaryVehicleContainer {
    pub fn new(
        stationary_since: Option<StationarySince>,
        stationary_cause: Option<CauseCode>,
        carrying_dangerous_goods: Option<DangerousGoodsExtended>,
        number_of_occupants: Option<NumberOfOccupants>,
        vehicle_identification: Option<VehicleIdentification>,
        energy_storage_type: Option<EnergyStorageType>,
    ) -> Self {
        Self {
            stationary_since,
            stationary_cause,
            carrying_dangerous_goods,
            number_of_occupants,
            vehicle_identification,
            energy_storage_type,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct StationaryVehicleSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct SteeringWheelAngle {
    pub steering_wheel_angle_value: SteeringWheelAngleValue,
    pub steering_wheel_angle_confidence: SteeringWheelAngleConfidence,
}

impl SteeringWheelAngle {
    pub fn new(
        steering_wheel_angle_value: SteeringWheelAngleValue,
        steering_wheel_angle_confidence: SteeringWheelAngleConfidence,
    ) -> Self {
        Self {
            steering_wheel_angle_value,
            steering_wheel_angle_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct SteeringWheelAngleConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-511..=512"))]
pub struct SteeringWheelAngleValue(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct SubCauseCodeType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1"))]
pub struct SubjectAssurance(pub OctetString);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum SymmAlgorithm {
    Aes128Ccm = 0,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum SymmetricEncryptionKey {
    #[rasn(size("16"))]
    Aes128Ccm(OctetString),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SystemService(pub SequenceOf<SystemServiceAndContext>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct SystemServiceAndContext(pub SamContext);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-128..=127"))]
pub struct TXpower80211(pub i8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct TcPartDetectionZoneIds(pub SequenceOf<Zid>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct TcPartRelevanceZoneIds(pub SequenceOf<Zid>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct TcPartDriverAwarenessZoneIds(pub SequenceOf<Zid>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct TcPartApplicableLanes(pub SequenceOf<LanePosition>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TcPartText(pub SequenceOf<Text>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct TcPart {
    pub detection_zone_ids: Option<TcPartDetectionZoneIds>,
    pub relevance_zone_ids: TcPartRelevanceZoneIds,
    pub direction: Option<Direction>,
    pub driver_awareness_zone_ids: Option<TcPartDriverAwarenessZoneIds>,
    #[rasn(value("0..=255"))]
    pub minimum_awareness_time: Option<u8>,
    pub applicable_lanes: Option<TcPartApplicableLanes>,
    #[rasn(value("1..=4", extensible))]
    pub layout_id: Option<u8>,
    #[rasn(value("1..=64", extensible))]
    pub pre_storedlayout_id: Option<u8>,
    pub text: Option<TcPartText>,
    pub data: OctetString,
}

impl TcPart {
    pub fn new(
        detection_zone_ids: Option<TcPartDetectionZoneIds>,
        relevance_zone_ids: TcPartRelevanceZoneIds,
        direction: Option<Direction>,
        driver_awareness_zone_ids: Option<TcPartDriverAwarenessZoneIds>,
        minimum_awareness_time: Option<u8>,
        applicable_lanes: Option<TcPartApplicableLanes>,
        layout_id: Option<u8>,
        pre_storedlayout_id: Option<u8>,
        text: Option<TcPartText>,
        data: OctetString,
    ) -> Self {
        Self {
            detection_zone_ids,
            relevance_zone_ids,
            direction,
            driver_awareness_zone_ids,
            minimum_awareness_time,
            applicable_lanes,
            layout_id,
            pre_storedlayout_id,
            text,
            data,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-60..=67"))]
pub struct Temperature(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("4"))]
pub struct TemporaryID(pub OctetString);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum Termination {
    IsCancellation = 0,
    IsNegation = 1,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4"))]
pub struct TestDataRegional(pub SequenceOf<RegionalExtension>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct TestData {
    pub regional: Option<TestDataRegional>,
}

impl TestData {
    pub fn new(regional: Option<TestDataRegional>) -> Self {
        Self { regional }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Text {
    #[rasn(value("1..=4", extensible))]
    pub layout_component_id: Option<u8>,
    #[rasn(size("10"))]
    pub language: BitString,
    pub text_content: Utf8String,
}

impl Text {
    pub fn new(
        layout_component_id: Option<u8>,
        language: BitString,
        text_content: Utf8String,
    ) -> Self {
        Self {
            layout_component_id,
            language,
            text_content,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=16", extensible))]
pub struct TextContainer(pub SequenceOf<TcPart>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ThreeDLocation {
    pub latitude: SALatitude,
    pub longitude: SALongitude,
    pub elevation: SAElevation,
}

impl ThreeDLocation {
    pub fn new(latitude: SALatitude, longitude: SALongitude, elevation: SAElevation) -> Self {
        Self {
            latitude,
            longitude,
            elevation,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Time32(pub Uint32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct Time64(pub Uint64);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TimeChangeDetails {
    pub start_time: Option<TimeMark>,
    pub min_end_time: TimeMark,
    pub max_end_time: Option<TimeMark>,
    pub likely_time: Option<TimeMark>,
    pub confidence: Option<TimeIntervalConfidence>,
    pub next_time: Option<TimeMark>,
}

impl TimeChangeDetails {
    pub fn new(
        start_time: Option<TimeMark>,
        min_end_time: TimeMark,
        max_end_time: Option<TimeMark>,
        likely_time: Option<TimeMark>,
        confidence: Option<TimeIntervalConfidence>,
        next_time: Option<TimeMark>,
    ) -> Self {
        Self {
            start_time,
            min_end_time,
            max_end_time,
            likely_time,
            confidence,
            next_time,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum TimeConfidence {
    Unavailable = 0,
    Time100000 = 1,
    Time050000 = 2,
    Time020000 = 3,
    Time010000 = 4,
    Time002000 = 5,
    Time001000 = 6,
    Time000500 = 7,
    Time000200 = 8,
    Time000100 = 9,
    Time000050 = 10,
    Time000020 = 11,
    Time000010 = 12,
    Time000005 = 13,
    Time000002 = 14,
    Time000001 = 15,
    Time0000005 = 16,
    Time0000002 = 17,
    Time0000001 = 18,
    Time00000005 = 19,
    Time00000002 = 20,
    Time00000001 = 21,
    Time000000005 = 22,
    Time000000002 = 23,
    Time000000001 = 24,
    Time0000000005 = 25,
    Time0000000002 = 26,
    Time0000000001 = 27,
    Time00000000005 = 28,
    Time00000000002 = 29,
    Time00000000001 = 30,
    Time000000000005 = 31,
    Time000000000002 = 32,
    Time000000000001 = 33,
    Time0000000000005 = 34,
    Time0000000000002 = 35,
    Time0000000000001 = 36,
    Time00000000000005 = 37,
    Time00000000000002 = 38,
    Time00000000000001 = 39,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=15"))]
pub struct TimeIntervalConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=36001"))]
pub struct TimeMark(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1500..=1500"))]
pub struct TimeOfMeasurement(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=60000"))]
pub struct TimeReference(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4398046511103"))]
pub struct TimestampIts(pub u64);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=7"))]
pub struct Traces(pub SequenceOf<PathHistory>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TractorCharacteristicsEqualTo(pub SequenceOf<VehicleCharacteristicsFixValues>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TractorCharacteristicsNotEqualTo(pub SequenceOf<VehicleCharacteristicsFixValues>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TractorCharacteristicsRanges(pub SequenceOf<VehicleCharacteristicsRanges>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TractorCharacteristics {
    pub equal_to: Option<TractorCharacteristicsEqualTo>,
    pub not_equal_to: Option<TractorCharacteristicsNotEqualTo>,
    pub ranges: Option<TractorCharacteristicsRanges>,
}

impl TractorCharacteristics {
    pub fn new(
        equal_to: Option<TractorCharacteristicsEqualTo>,
        not_equal_to: Option<TractorCharacteristicsNotEqualTo>,
        ranges: Option<TractorCharacteristicsRanges>,
    ) -> Self {
        Self {
            equal_to,
            not_equal_to,
            ranges,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct TrafficConditionSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TrafficIslandPosition {
    pub one_side: NonIslandLanePosition,
    pub other_side: NonIslandLanePosition,
}

impl TrafficIslandPosition {
    pub fn new(one_side: NonIslandLanePosition, other_side: NonIslandLanePosition) -> Self {
        Self {
            one_side,
            other_side,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum TrafficRule {
    NoPassing = 0,
    NoPassingForTrucks = 1,
    PassToRight = 2,
    PassToLeft = 3,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum TrafficSignPictogram {
    DangerWarning = 0,
    Regulatory = 1,
    Informative = 2,
}

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TrailerCharacteristicsEqualTo(pub SequenceOf<VehicleCharacteristicsFixValues>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TrailerCharacteristicsNotEqualTo(pub SequenceOf<VehicleCharacteristicsFixValues>);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=4", extensible))]
pub struct TrailerCharacteristicsRanges(pub SequenceOf<VehicleCharacteristicsRanges>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TrailerCharacteristics {
    pub equal_to: Option<TrailerCharacteristicsEqualTo>,
    pub not_equal_to: Option<TrailerCharacteristicsNotEqualTo>,
    pub ranges: Option<TrailerCharacteristicsRanges>,
}

impl TrailerCharacteristics {
    pub fn new(
        equal_to: Option<TrailerCharacteristicsEqualTo>,
        not_equal_to: Option<TrailerCharacteristicsNotEqualTo>,
        ranges: Option<TrailerCharacteristicsRanges>,
    ) -> Self {
        Self {
            equal_to,
            not_equal_to,
            ranges,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct TrailerData {
    pub ref_point_id: RefPointId,
    pub hitch_point_offset: HitchPointOffset,
    pub front_overhang: FrontOverhang,
    pub rear_overhang: RearOverhang,
    pub trailer_width: Option<VehicleWidth>,
    pub hitch_angle: Option<CartesianAngle>,
}

impl TrailerData {
    pub fn new(
        ref_point_id: RefPointId,
        hitch_point_offset: HitchPointOffset,
        front_overhang: FrontOverhang,
        rear_overhang: RearOverhang,
        trailer_width: Option<VehicleWidth>,
        hitch_angle: Option<CartesianAngle>,
    ) -> Self {
        Self {
            ref_point_id,
            hitch_point_offset,
            front_overhang,
            rear_overhang,
            trailer_width,
            hitch_angle,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=2"))]
pub struct TrailerDataContainer(pub SequenceOf<TrailerData>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct TrainCharacteristics(pub TractorCharacteristics);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TrajectoryInterceptionIndication {
    pub subject_station: Option<StationID>,
    pub trajectory_interception_indication: bool,
}

impl TrajectoryInterceptionIndication {
    pub fn new(
        subject_station: Option<StationID>,
        trajectory_interception_indication: bool,
    ) -> Self {
        Self {
            subject_station,
            trajectory_interception_indication,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum TransitVehicleOccupancy {
    OccupancyUnknown = 0,
    OccupancyEmpty = 1,
    OccupancyVeryLow = 2,
    OccupancyLow = 3,
    OccupancyMed = 4,
    OccupancyHigh = 5,
    OccupancyNearlyFull = 6,
    OccupancyFull = 7,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct TransitVehicleStatus(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TransmissionAndSpeed {
    pub transmisson: TransmissionState,
    pub speed: Velocity,
}

impl TransmissionAndSpeed {
    pub fn new(transmisson: TransmissionState, speed: Velocity) -> Self {
        Self { transmisson, speed }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=10000"))]
pub struct TransmissionInterval(pub u16);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum TransmissionState {
    Neutral = 0,
    Park = 1,
    ForwardGears = 2,
    ReverseGears = 3,
    Reserved1 = 4,
    Reserved2 = 5,
    Reserved3 = 6,
    Unavailable = 7,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct TurningRadius(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct TwoDLocation {
    pub latitude: SALatitude,
    pub longitude: SALongitude,
}

impl TwoDLocation {
    pub fn new(latitude: SALatitude, longitude: SALongitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct TypeOfReceptacle(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct URLOfMDUs {
    pub starting_m_d_u: Option<SequenceNumber>,
    pub ending_m_d_u: Option<SequenceNumber>,
    pub url: Ia5String,
}

impl URLOfMDUs {
    pub fn new(
        starting_m_d_u: Option<SequenceNumber>,
        ending_m_d_u: Option<SequenceNumber>,
        url: Ia5String,
    ) -> Self {
        Self {
            starting_m_d_u,
            ending_m_d_u,
            url,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=65535"))]
pub struct Uint16(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct Uint3(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct Uint32(pub u32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=18446744073709551615"))]
pub struct Uint64(pub u64);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct Uint8(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum UnitType {
    MgKm = 0,
    MgKWh = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("900000001"))]
pub struct UnknownLatitude(pub NinetyDegreeInt);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, value("1800000001"))]
pub struct UnknownLongitude(pub OneEightyDegreeInt);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VAM {
    pub header: ItsPduHeader,
    pub vam: VruAwareness,
}

impl VAM {
    pub fn new(header: ItsPduHeader, vam: VruAwareness) -> Self {
        Self { header, vam }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("6"))]
pub struct VDS(pub Ia5String);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VED {
    pub hei: Option<Distance>,
    pub wid: Option<Distance>,
    pub vln: Option<Distance>,
    pub wei: Option<Weight>,
}

impl VED {
    pub fn new(
        hei: Option<Distance>,
        wid: Option<Distance>,
        vln: Option<Distance>,
        wei: Option<Weight>,
    ) -> Self {
        Self { hei, wid, vln, wei }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=86400"))]
pub struct ValidityDuration(pub u32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct ValidityPeriod {
    pub start: Time32,
    pub duration: Duration,
}

impl ValidityPeriod {
    pub fn new(start: Time32, duration: Duration) -> Self {
        Self { start, duration }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VamParameters {
    pub basic_container: BasicContainer,
    pub vru_high_frequency_container: Option<VruHighFrequencyContainer>,
    pub vru_low_frequency_container: Option<VruLowFrequencyContainer>,
    pub vru_cluster_information_container: Option<VruClusterInformationContainer>,
    pub vru_cluster_operation_container: Option<VruClusterOperationContainer>,
    pub vru_motion_prediction_container: Option<VruMotionPredictionContainer>,
}

impl VamParameters {
    pub fn new(
        basic_container: BasicContainer,
        vru_high_frequency_container: Option<VruHighFrequencyContainer>,
        vru_low_frequency_container: Option<VruLowFrequencyContainer>,
        vru_cluster_information_container: Option<VruClusterInformationContainer>,
        vru_cluster_operation_container: Option<VruClusterOperationContainer>,
        vru_motion_prediction_container: Option<VruMotionPredictionContainer>,
    ) -> Self {
        Self {
            basic_container,
            vru_high_frequency_container,
            vru_low_frequency_container,
            vru_cluster_information_container,
            vru_cluster_operation_container,
            vru_motion_prediction_container,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum VarLengthNumber {
    #[rasn(value("0..=127"))]
    Content(u8),
    Extension(Ext1),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct VcClass(pub u8);

/// Inner type

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=8", extensible))]
pub struct VcCodeValidity(pub SequenceOf<DTM>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VcCode {
    pub road_sign_class: VcClass,
    #[rasn(value("1..=64"))]
    pub road_sign_code: u8,
    pub vc_option: VcOption,
    pub validity: Option<VcCodeValidity>,
    #[rasn(value("0..=65535"))]
    pub value: Option<u16>,
    pub unit: Option<RSCUnit>,
}

impl VcCode {
    pub fn new(
        road_sign_class: VcClass,
        road_sign_code: u8,
        vc_option: VcOption,
        validity: Option<VcCodeValidity>,
        value: Option<u16>,
        unit: Option<RSCUnit>,
    ) -> Self {
        Self {
            road_sign_class,
            road_sign_code,
            vc_option,
            validity,
            value,
            unit,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=7"))]
pub struct VcOption(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct VehicleBreakdownSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VehicleCharacteristicsFixValues {
    SimpleVehicleType(StationType),
    EuVehicleCategoryCode(EuVehicleCategoryCode),
    Iso3833VehicleType(Iso3833VehicleType),
    EuroAndCo2value(EnvironmentalCharacteristics),
    EngineCharacteristics(EngineCharacteristics),
    LoadType(LoadType),
    Usage(VehicleRole),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VehicleCharacteristicsRangeLimits {
    #[rasn(value("0..=7"))]
    NumberOfAxles(u8),
    VehicleDimensions(VehicleDimensions),
    VehicleWeightLimits(VehicleWeightLimits),
    AxleWeightLimits(AxleWeightLimits),
    PassengerCapacity(PassengerCapacity),
    ExhaustEmissionValues(ExhaustEmissionValues),
    DieselEmissionValues(DieselEmissionValues),
    SoundLevel(SoundLevel),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleCharacteristicsRanges {
    pub comparison_operator: ComparisonOperator,
    pub limits: VehicleCharacteristicsRangeLimits,
}

impl VehicleCharacteristicsRanges {
    pub fn new(
        comparison_operator: ComparisonOperator,
        limits: VehicleCharacteristicsRangeLimits,
    ) -> Self {
        Self {
            comparison_operator,
            limits,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleDimensions {
    pub vehicle_length_overall: Int1,
    pub vehicle_heigth_overall: Int1,
    pub vehicle_width_overall: Int1,
}

impl VehicleDimensions {
    pub fn new(
        vehicle_length_overall: Int1,
        vehicle_heigth_overall: Int1,
        vehicle_width_overall: Int1,
    ) -> Self {
        Self {
            vehicle_length_overall,
            vehicle_heigth_overall,
            vehicle_width_overall,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=127"))]
pub struct VehicleHeight(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
pub enum VehicleID {
    EntityID(TemporaryID),
    StationID(StationID),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VehicleIdentification {
    pub w_m_inumber: Option<WMInumber>,
    pub v_d_s: Option<VDS>,
}

impl VehicleIdentification {
    pub fn new(w_m_inumber: Option<WMInumber>, v_d_s: Option<VDS>) -> Self {
        Self { w_m_inumber, v_d_s }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleLength {
    pub vehicle_length_value: VehicleLengthValue,
    pub vehicle_length_confidence_indication: VehicleLengthConfidenceIndication,
}

impl VehicleLength {
    pub fn new(
        vehicle_length_value: VehicleLengthValue,
        vehicle_length_confidence_indication: VehicleLengthConfidenceIndication,
    ) -> Self {
        Self {
            vehicle_length_value,
            vehicle_length_confidence_indication,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VehicleLengthConfidenceIndication {
    NoTrailerPresent = 0,
    TrailerPresentWithKnownLength = 1,
    TrailerPresentWithUnknownLength = 2,
    TrailerPresenceIsUnknown = 3,
    Unavailable = 4,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=1023"))]
pub struct VehicleLengthValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=1024"))]
pub struct VehicleMass(pub u16);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VehicleRole {
    Default = 0,
    PublicTransport = 1,
    SpecialTransport = 2,
    DangerousGoods = 3,
    RoadWork = 4,
    Rescue = 5,
    Emergency = 6,
    SafetyCar = 7,
    Agriculture = 8,
    Commercial = 9,
    Military = 10,
    RoadOperator = 11,
    Taxi = 12,
    Reserved1 = 13,
    Reserved2 = 14,
    Reserved3 = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VehicleSensor {
    #[rasn(default = "vehicle_sensor_ref_point_id_default")]
    pub ref_point_id: RefPointId,
    pub x_sensor_offset: XSensorOffset,
    pub y_sensor_offset: YSensorOffset,
    pub z_sensor_offset: Option<ZSensorOffset>,
    pub vehicle_sensor_property_list: VehicleSensorPropertyList,
}

impl VehicleSensor {
    pub fn new(
        ref_point_id: RefPointId,
        x_sensor_offset: XSensorOffset,
        y_sensor_offset: YSensorOffset,
        z_sensor_offset: Option<ZSensorOffset>,
        vehicle_sensor_property_list: VehicleSensorPropertyList,
    ) -> Self {
        Self {
            ref_point_id,
            x_sensor_offset,
            y_sensor_offset,
            z_sensor_offset,
            vehicle_sensor_property_list,
        }
    }
}

fn vehicle_sensor_ref_point_id_default() -> RefPointId {
    RefPointId(0).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VehicleSensorProperties {
    pub range: Range,
    pub horizontal_opening_angle_start: CartesianAngleValue,
    pub horizontal_opening_angle_end: CartesianAngleValue,
    pub vertical_opening_angle_start: Option<CartesianAngleValue>,
    pub vertical_opening_angle_end: Option<CartesianAngleValue>,
}

impl VehicleSensorProperties {
    pub fn new(
        range: Range,
        horizontal_opening_angle_start: CartesianAngleValue,
        horizontal_opening_angle_end: CartesianAngleValue,
        vertical_opening_angle_start: Option<CartesianAngleValue>,
        vertical_opening_angle_end: Option<CartesianAngleValue>,
    ) -> Self {
        Self {
            range,
            horizontal_opening_angle_start,
            horizontal_opening_angle_end,
            vertical_opening_angle_start,
            vertical_opening_angle_end,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=10"))]
pub struct VehicleSensorPropertyList(pub SequenceOf<VehicleSensorProperties>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleSubclass {
    #[rasn(default = "vehicle_subclass_r_type_default")]
    pub r_type: VehicleSubclassType,
    #[rasn(default = "vehicle_subclass_confidence_default")]
    pub confidence: ClassConfidence,
}

impl VehicleSubclass {
    pub fn new(r_type: VehicleSubclassType, confidence: ClassConfidence) -> Self {
        Self { r_type, confidence }
    }
}

fn vehicle_subclass_r_type_default() -> VehicleSubclassType {
    VehicleSubclassType(0).into()
}

fn vehicle_subclass_confidence_default() -> ClassConfidence {
    ClassConfidence(0).into()
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct VehicleSubclassType(pub u8);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum VehicleType {
    None = 0,
    Unknown = 1,
    Special = 2,
    Moto = 3,
    Car = 4,
    CarOther = 5,
    Bus = 6,
    AxleCnt2 = 7,
    AxleCnt3 = 8,
    AxleCnt4 = 9,
    AxleCnt4Trailer = 10,
    AxleCnt5Trailer = 11,
    AxleCnt6Trailer = 12,
    AxleCnt5MultiTrailer = 13,
    AxleCnt6MultiTrailer = 14,
    AxleCnt7MultiTrailer = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VehicleWeightLimits {
    pub vehicle_max_laden_weight: Int2,
    pub vehicle_train_maximum_weight: Int2,
    pub vehicle_weight_unladen: Int2,
}

impl VehicleWeightLimits {
    pub fn new(
        vehicle_max_laden_weight: Int2,
        vehicle_train_maximum_weight: Int2,
        vehicle_weight_unladen: Int2,
    ) -> Self {
        Self {
            vehicle_max_laden_weight,
            vehicle_train_maximum_weight,
            vehicle_weight_unladen,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=62"))]
pub struct VehicleWidth(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=8191"))]
pub struct Velocity(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VerticalAcceleration {
    pub vertical_acceleration_value: VerticalAccelerationValue,
    pub vertical_acceleration_confidence: AccelerationConfidence,
}

impl VerticalAcceleration {
    pub fn new(
        vertical_acceleration_value: VerticalAccelerationValue,
        vertical_acceleration_confidence: AccelerationConfidence,
    ) -> Self {
        Self {
            vertical_acceleration_value,
            vertical_acceleration_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-160..=161"))]
pub struct VerticalAccelerationValue(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VruAwareness {
    pub generation_delta_time: GenerationDeltaTime,
    pub vam_parameters: VamParameters,
}

impl VruAwareness {
    pub fn new(generation_delta_time: GenerationDeltaTime, vam_parameters: VamParameters) -> Self {
        Self {
            generation_delta_time,
            vam_parameters,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VruClusterInformationContainer {
    pub cluster_id: ClusterId,
    pub cluster_bounding_box_shape: ClusterBoundingBoxShape,
    #[rasn(value("0..=255"))]
    pub cluster_cardinality_size: u8,
    pub cluster_profiles: ClusterProfiles,
}

impl VruClusterInformationContainer {
    pub fn new(
        cluster_id: ClusterId,
        cluster_bounding_box_shape: ClusterBoundingBoxShape,
        cluster_cardinality_size: u8,
        cluster_profiles: ClusterProfiles,
    ) -> Self {
        Self {
            cluster_id,
            cluster_bounding_box_shape,
            cluster_cardinality_size,
            cluster_profiles,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=255"))]
pub struct VruClusterOpTimestamp(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VruClusterOperationContainer {
    pub cluster_join_info: Option<ClusterJoinInfo>,
    pub cluster_leave_info: Option<ClusterLeaveInfo>,
    pub cluster_breakup_info: Option<ClusterBreakupInfo>,
    pub cluster_id_change_info: Option<VruClusterOpTimestamp>,
}

impl VruClusterOperationContainer {
    pub fn new(
        cluster_join_info: Option<ClusterJoinInfo>,
        cluster_leave_info: Option<ClusterLeaveInfo>,
        cluster_breakup_info: Option<ClusterBreakupInfo>,
        cluster_id_change_info: Option<VruClusterOpTimestamp>,
    ) -> Self {
        Self {
            cluster_join_info,
            cluster_leave_info,
            cluster_breakup_info,
            cluster_id_change_info,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruDeviceUsage {
    Unavailable = 0,
    Other = 1,
    Idle = 2,
    ListeningToAudio = 3,
    Typing = 4,
    Calling = 5,
    PlayingGames = 6,
    Reading = 7,
    Viewing = 8,
    Max = 255,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruEnvironment {
    Unavailable = 0,
    IntersectionCrossing = 1,
    ZebraCrossing = 2,
    Sidewalk = 3,
    OnVehicleRoad = 4,
    ProtectedGeographicArea = 5,
    Max = 255,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct VruExteriorLights {
    pub vru_specific: VruSpecificExteriorLights,
    pub generic: ExteriorLights,
}

impl VruExteriorLights {
    pub fn new(vru_specific: VruSpecificExteriorLights, generic: ExteriorLights) -> Self {
        Self {
            vru_specific,
            generic,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VruHighFrequencyContainer {
    pub heading: Heading,
    pub speed: Speed,
    pub longitudinal_acceleration: LongitudinalAcceleration,
    pub curvature: Option<Curvature>,
    pub curvature_calculation_mode: Option<CurvatureCalculationMode>,
    pub yaw_rate: Option<YawRate>,
    pub lateral_acceleration: Option<LateralAcceleration>,
    pub vertical_acceleration: Option<VerticalAcceleration>,
    pub vru_lane_position: Option<VruLanePosition>,
    pub environment: Option<VruEnvironment>,
    pub movement_control: Option<VruMovementControl>,
    pub orientation: Option<VruOrientation>,
    pub roll_angle: Option<VruRollAngle>,
    pub device_usage: Option<VruDeviceUsage>,
}

impl VruHighFrequencyContainer {
    pub fn new(
        heading: Heading,
        speed: Speed,
        longitudinal_acceleration: LongitudinalAcceleration,
        curvature: Option<Curvature>,
        curvature_calculation_mode: Option<CurvatureCalculationMode>,
        yaw_rate: Option<YawRate>,
        lateral_acceleration: Option<LateralAcceleration>,
        vertical_acceleration: Option<VerticalAcceleration>,
        vru_lane_position: Option<VruLanePosition>,
        environment: Option<VruEnvironment>,
        movement_control: Option<VruMovementControl>,
        orientation: Option<VruOrientation>,
        roll_angle: Option<VruRollAngle>,
        device_usage: Option<VruDeviceUsage>,
    ) -> Self {
        Self {
            heading,
            speed,
            longitudinal_acceleration,
            curvature,
            curvature_calculation_mode,
            yaw_rate,
            lateral_acceleration,
            vertical_acceleration,
            vru_lane_position,
            environment,
            movement_control,
            orientation,
            roll_angle,
            device_usage,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VruLanePosition {
    OffRoadLanePosition(OffRoadLanePosition),
    VehicularLanePosition(LanePosition),
    TrafficIslandPosition(TrafficIslandPosition),
    MapPosition(MapPosition),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VruLowFrequencyContainer {
    pub profile_and_subprofile: Option<VruProfileAndSubprofile>,
    pub exterior_lights: Option<VruExteriorLights>,
    pub size_class: Option<VruSizeClass>,
}

impl VruLowFrequencyContainer {
    pub fn new(
        profile_and_subprofile: Option<VruProfileAndSubprofile>,
        exterior_lights: Option<VruExteriorLights>,
        size_class: Option<VruSizeClass>,
    ) -> Self {
        Self {
            profile_and_subprofile,
            exterior_lights,
            size_class,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VruMotionPredictionContainer {
    pub path_history: Option<PathHistory>,
    pub path_prediction: Option<PathPrediction>,
    pub safe_distance: Option<SequenceOfVruSafeDistanceIndication>,
    pub trajectory_change_indication: Option<SequenceOfTrajectoryInterceptionIndication>,
    pub acceleration_change_indication: Option<AccelerationChangeIndication>,
    pub heading_change_indication: Option<HeadingChangeIndication>,
    pub stability_change_indication: Option<StabilityChangeIndication>,
}

impl VruMotionPredictionContainer {
    pub fn new(
        path_history: Option<PathHistory>,
        path_prediction: Option<PathPrediction>,
        safe_distance: Option<SequenceOfVruSafeDistanceIndication>,
        trajectory_change_indication: Option<SequenceOfTrajectoryInterceptionIndication>,
        acceleration_change_indication: Option<AccelerationChangeIndication>,
        heading_change_indication: Option<HeadingChangeIndication>,
        stability_change_indication: Option<StabilityChangeIndication>,
    ) -> Self {
        Self {
            path_history,
            path_prediction,
            safe_distance,
            trajectory_change_indication,
            acceleration_change_indication,
            heading_change_indication,
            stability_change_indication,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruMovementControl {
    Unavailable = 0,
    Braking = 1,
    HardBraking = 2,
    StopPedaling = 3,
    NoReaction = 4,
    Max = 255,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct VruOrientation(pub Heading);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruProfile {
    Unavailable = 0,
    Pedestrian = 1,
    Cyclist = 2,
    Motorcyclist = 3,
    Animal = 4,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum VruProfileAndSubprofile {
    Pedestrian(VruSubProfilePedestrian),
    Bicyclist(VruSubProfileBicyclist),
    Motorcylist(VruSubProfileMotorcyclist),
    Animal(VruSubProfileAnimal),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct VruRollAngle(pub SteeringWheelAngle);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct VruSafeDistanceIndication {
    pub subject_station: Option<StationID>,
    pub station_safe_distance_indication: bool,
    pub time_to_collision: Option<ActionDeltaTime>,
}

impl VruSafeDistanceIndication {
    pub fn new(
        subject_station: Option<StationID>,
        station_safe_distance_indication: bool,
        time_to_collision: Option<ActionDeltaTime>,
    ) -> Self {
        Self {
            subject_station,
            station_safe_distance_indication,
            time_to_collision,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruSizeClass {
    Unavailable = 0,
    Low = 1,
    Medium = 2,
    High = 3,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("8"))]
pub struct VruSpecificExteriorLights(pub BitString);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruSubProfileAnimal {
    Unavailable = 0,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruSubProfileBicyclist {
    Unavailable = 0,
    Bicyclist = 1,
    WheelchairUser = 2,
    HorseAndRider = 3,
    Rollerskater = 4,
    EScooter = 5,
    PersonalTransporter = 6,
    Pedelec = 7,
    SpeedPedelec = 8,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruSubProfileMotorcyclist {
    Unavailable = 0,
    Moped = 1,
    Motorcycle = 2,
    MotorcycleAndSidecarRight = 3,
    MotorcycleAndSidecarLeft = 4,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum VruSubProfilePedestrian {
    Unavailable = 0,
    OrdinaryPedestrian = 1,
    RoadWorker = 2,
    FirstResponder = 3,
    Max = 15,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct WGS84Angle {
    pub value: WGS84AngleValue,
    pub confidence: AngleConfidence,
}

impl WGS84Angle {
    pub fn new(value: WGS84AngleValue, confidence: AngleConfidence) -> Self {
        Self { value, confidence }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=3601"))]
pub struct WGS84AngleValue(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate, size("1..=3"))]
pub struct WMInumber(pub Ia5String);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
pub struct WaitOnStopline(pub bool);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct Weight {
    #[rasn(value("1..=16384"))]
    pub value: u16,
    pub unit: RSCUnit,
}

impl Weight {
    pub fn new(value: u16, unit: RSCUnit) -> Self {
        Self { value, unit }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=127"))]
pub struct WheelBaseVehicle(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct WrongWayDrivingSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct WsaChInfoDataRate {
    #[rasn(size("1"))]
    pub adaptable: BitString,
    #[rasn(value("0..=127"))]
    pub data_rate: u8,
}

impl WsaChInfoDataRate {
    pub fn new(adaptable: BitString, data_rate: u8) -> Self {
        Self {
            adaptable,
            data_rate,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct WsaCountThreshold(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=255"))]
pub struct WsaCountThresholdInterval(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
#[non_exhaustive]
pub struct WsaSsp {
    pub version: Uint8,
    pub advertiser_permissions: Option<AdvertiserPermissions>,
    pub provider_permissions: Option<ProviderPermissions>,
}

impl WsaSsp {
    pub fn new(
        version: Uint8,
        advertiser_permissions: Option<AdvertiserPermissions>,
        provider_permissions: Option<ProviderPermissions>,
    ) -> Self {
        Self {
            version,
            advertiser_permissions,
            provider_permissions,
        }
    }
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-5000..=0"))]
pub struct XSensorOffset(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-1000..=1000"))]
pub struct YSensorOffset(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(automatic_tags)]
pub struct YawRate {
    pub yaw_rate_value: YawRateValue,
    pub yaw_rate_confidence: YawRateConfidence,
}

impl YawRate {
    pub fn new(yaw_rate_value: YawRateValue, yaw_rate_confidence: YawRateConfidence) -> Self {
        Self {
            yaw_rate_value,
            yaw_rate_confidence,
        }
    }
}

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]

pub enum YawRateConfidence {
    DegSec00001 = 0,
    DegSec00005 = 1,
    DegSec00010 = 2,
    DegSec00100 = 3,
    DegSec00500 = 4,
    DegSec01000 = 5,
    DegSec10000 = 6,
    OutOfRange = 7,
    Unavailable = 8,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("-32766..=32767"))]
pub struct YawRateValue(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=1000"))]
pub struct ZSensorOffset(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("1..=32", extensible))]
pub struct Zid(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum Zone {
    Segment(Segment),
    Area(PolygonalLine),
    ComputedSegment(ComputedSegment),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate, value("0..=10000"))]
pub struct ZoneLength(pub u16);

pub const ADD_GRP_A: RegionId = RegionId(1);

pub const ADD_GRP_B: RegionId = RegionId(2);

pub const ADD_GRP_C: RegionId = RegionId(3);

pub const C_2DLOCATION: RefExt = RefExt(5);

pub const C_3DLOCATION: RefExt = RefExt(6);

pub const C__CHANNEL_ACCESS: RefExt = RefExt(21);

pub const C__CHANNEL_LOAD: RefExt = RefExt(23);

pub const C__CHANNEL_NUMBER80211: RefExt = RefExt(15);

pub const C__COUNTRY_STRING: RefExt = RefExt(18);

pub const C__DATA_RATE80211: RefExt = RefExt(16);

pub const C__E_D_C_APARAMETER_SET: RefExt = RefExt(12);

pub const C__EXTENDED_CHANNEL_INFOS: RefExt = RefExt(84);

pub const C__GATEWAY_M_A_CADDRESS: RefExt = RefExt(14);

pub const C__I_PV6ADDRESS: RefExt = RefExt(9);

pub const C__L_MCHANNEL_BUSY_RATIO: RefExt = RefExt(82);

pub const C__L_MPACKET_I_D: RefExt = RefExt(83);

pub const C__L_MRX_CIP: RefExt = RefExt(81);

pub const C__L_MTX_CIP: RefExt = RefExt(80);

pub const C__PROTOCOL_TYPE: RefExt = RefExt(24);

pub const C__PROVIDER_M_A_CADDRESS: RefExt = RefExt(11);

pub const C__PROVIDER_SERV_CONTEXT: RefExt = RefExt(8);

pub const C__R_C_P_ITHRESHOLD: RefExt = RefExt(19);

pub const C__REPEAT_RATE: RefExt = RefExt(17);

pub const C__RESERVED: RefExt = RefExt(0);

pub const C__S_A_MAPPLICATION_DATA: RefExt = RefExt(85);

pub const C__SECONDARY_D_N_S: RefExt = RefExt(13);

pub const C__TX_POWER_USED80211: RefExt = RefExt(4);

pub const C__W_S_ACOUNT_THRES_INT: RefExt = RefExt(22);

pub const C__W_S_ACOUNT_THRESHOLD: RefExt = RefExt(20);

pub const C_ADVERTISER_I_D: RefExt = RefExt(7);

pub const C_CTX_REF_MAND_APP: CtxRef = CtxRef(1);

pub const C_CTX_REF_NULL: CtxRef = CtxRef(0);

pub const C_SERVICE_PORT: RefExt = RefExt(10);

pub const CH_INFO_TYPE_15628: MedType = MedType(128);

pub const CH_INFO_TYPE_2G: MedType = MedType(2);

pub const CH_INFO_TYPE_3G: MedType = MedType(3);

pub const CH_INFO_TYPE_6LOW_PAN: MedType = MedType(11);

pub const CH_INFO_TYPE_80216E: MedType = MedType(7);

pub const CH_INFO_TYPE_80220: MedType = MedType(9);

pub const CH_INFO_TYPE__C_A_N: MedType = MedType(254);

pub const CH_INFO_TYPE__ETHERNET: MedType = MedType(255);

pub const CH_INFO_TYPE__H_C__S_D_M_A: MedType = MedType(8);

pub const CH_INFO_TYPE__I_R: MedType = MedType(4);

pub const CH_INFO_TYPE__L_T_E: MedType = MedType(10);

pub const CH_INFO_TYPE__M5: MedType = MedType(5);

pub const CH_INFO_TYPE__M_M: MedType = MedType(6);

pub const CH_INFO_TYPE_ANY: MedType = MedType(1);

pub const CH_INFO_TYPE_UNKNOWN: MedType = MedType(0);

pub const DEFAULT_VALIDITY: u16 = 600;

pub const DIESEL: FuelType = FuelType(3);

pub const ELECTRIC: FuelType = FuelType(4);

pub const ETHANOL: FuelType = FuelType(2);

pub const GASOLINE: FuelType = FuelType(1);

pub const HYBRID: FuelType = FuelType(5);

pub const HYDROGEN: FuelType = FuelType(6);

pub const MAP_DATA: DSRCmsgID = DSRCmsgID(18);

pub const NAT_GAS_COMP: FuelType = FuelType(8);

pub const NAT_GAS_LIQUID: FuelType = FuelType(7);

pub const NO_REGION: RegionId = RegionId(0);

pub const PROPANE: FuelType = FuelType(9);

pub const RTCM_CORRECTIONS: DSRCmsgID = DSRCmsgID(28);

pub const SIGNAL_PHASE_AND_TIMING_MESSAGE: DSRCmsgID = DSRCmsgID(19);

pub const SIGNAL_REQUEST_MESSAGE: DSRCmsgID = DSRCmsgID(29);

pub const SIGNAL_STATUS_MESSAGE: DSRCmsgID = DSRCmsgID(30);

pub const UNKNOWN_FUEL: FuelType = FuelType(0);
