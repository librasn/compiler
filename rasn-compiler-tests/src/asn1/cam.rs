#![no_std]

extern crate alloc;

use rasn::prelude::*;
use serde::{Deserialize, Serialize};
// =====================================================
// CAM-PDU-Descriptions
// { itu-t(0) identified-organization(4) etsi(0) itsDomain(5) wg1(1) en(302637) cam(2) version(2) }
// =====================================================

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

///	The root data frame for cooperative awareness messages

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=65535"))]
pub struct GenerationDeltaTime(pub u16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum HighFrequencyContainer {
    BasicVehicleContainerHighFrequency(BasicVehicleContainerHighFrequency),
    RsuContainerHighFrequency(RSUContainerHighFrequency),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(choice, automatic_tags)]
#[non_exhaustive]
pub enum LowFrequencyContainer {
    BasicVehicleContainerLowFrequency(BasicVehicleContainerLowFrequency),
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

// =====================================================
// ITS-Container
// { itu-t(0) identified-organization(4) etsi(0) itsDomain(5) wg1(1) ts(102894) cdd(2) version(2) }
// =====================================================

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=102"))]
pub struct AccelerationConfidence(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("7"))]
pub struct AccelerationControl(pub BitString);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct AccidentSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionAdhesionSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionExtremeWeatherConditionSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionPrecipitationSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct AdverseWeatherConditionVisibilitySubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-100000..=800001"))]
pub struct AltitudeValue(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct CauseCodeType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate)]
pub struct CenDsrcTollingZoneID(pub ProtectedZoneID);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct CollisionRiskSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum CurvatureCalculationMode {
    YawRateUsed = 0,
    YawRateNotUsed = 1,
    Unavailable = 2,
}

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-1023..=1023"))]
pub struct CurvatureValue(pub i16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct DangerousEndOfQueueSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct DangerousSituationSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-12700..=12800"))]
pub struct DeltaAltitude(pub i16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-131071..=131072"))]
pub struct DeltaLatitude(pub i32);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-131071..=131072"))]
pub struct DeltaLongitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=256"))]
pub struct DigitalMap(pub SequenceOf<ReferencePosition>);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]

pub enum DriveDirection {
    Forward = 0,
    Backward = 1,
    Unavailable = 2,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=13"))]
pub struct DrivingLaneStatus(pub BitString);

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate)]
pub struct EmbarkationStatus(pub bool);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("2"))]
pub struct EmergencyPriority(pub BitString);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct EmergencyVehicleApproachingSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("7"))]
pub struct EnergyStorageType(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=23"))]
pub struct EventHistory(pub SequenceOf<EventPoint>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("8"))]
pub struct ExteriorLights(pub BitString);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]

pub enum HardShoulderStatus {
    AvailableForStopping = 0,
    Closed = 1,
    AvailableForDriving = 2,
}

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationAnimalOnTheRoadSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationDangerousCurveSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationObstacleOnTheRoadSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct HazardousLocationSurfaceConditionSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=127"))]
pub struct HeadingConfidence(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=3601"))]
pub struct HeadingValue(pub u16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=100"))]
pub struct HeightLonCarr(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct HumanPresenceOnTheRoadSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct HumanProblemSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=7"))]
pub struct InformationQuality(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=40"))]
pub struct ItineraryPath(pub SequenceOf<ReferencePosition>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-1..=14"))]
pub struct LanePosition(pub i8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-160..=161"))]
pub struct LateralAccelerationValue(pub i16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-900000000..=900000001"))]
pub struct Latitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("2"))]
pub struct LightBarSirenInUse(pub BitString);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-1800000000..=1800000001"))]
pub struct Longitude(pub i32);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-160..=161"))]
pub struct LongitudinalAccelerationValue(pub i16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=127"))]
pub struct NumberOfOccupants(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate)]
pub struct OpeningDaysHours(pub Utf8String);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=65535", extensible))]
pub struct PathDeltaTime(pub Integer);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("0..=40"))]
pub struct PathHistory(pub SequenceOf<PathPoint>);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=7"))]
pub struct PerformanceClass(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=16"))]
pub struct PhoneNumber(pub NumericString);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=63"))]
pub struct PosCentMass(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=20"))]
pub struct PosFrontAx(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=127"))]
pub struct PosLonCarr(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=30"))]
pub struct PosPillar(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("20"))]
pub struct PositionOfOccupants(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=3", extensible))]
pub struct PositionOfPillars(pub SequenceOf<PosPillar>);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct PostCrashSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=16"))]
pub struct ProtectedCommunicationZonesRSU(pub SequenceOf<ProtectedCommunicationZone>);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=134217727"))]
pub struct ProtectedZoneID(pub u32);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=255", extensible))]
pub struct ProtectedZoneRadius(pub Integer);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum ProtectedZoneType {
    PermanentCenDsrcTolling = 0,
    #[rasn(extension_addition)]
    TemporaryCenDsrcTolling = 1,
}

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=20"))]
pub struct PtActivationData(pub OctetString);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct PtActivationType(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]

pub enum RelevanceTrafficDirection {
    AllTrafficDirections = 0,
    UpstreamTraffic = 1,
    DownstreamTraffic = 2,
    OppositeTraffic = 3,
}

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]

pub enum RequestResponseIndication {
    Request = 0,
    Response = 1,
}

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct RescueAndRecoveryWorkInProgressSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=3", extensible))]
pub struct RestrictedTypes(pub SequenceOf<StationType>);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]

pub enum RoadType {
    UrbanNoStructuralSeparationToOppositeLanes = 0,
    UrbanWithStructuralSeparationToOppositeLanes = 1,
    NonUrbanNoStructuralSeparationToOppositeLanes = 2,
    NonUrbanWithStructuralSeparationToOppositeLanes = 3,
}

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct RoadworksSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=4095"))]
pub struct SemiAxisLength(pub u16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=65535"))]
pub struct SequenceNumber(pub u16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct SignalViolationSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct SlowVehicleSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("4"))]
pub struct SpecialTransportType(pub BitString);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=127"))]
pub struct SpeedConfidence(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=255"))]
pub struct SpeedLimit(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=16383"))]
pub struct SpeedValue(pub u16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=4294967295"))]
pub struct StationID(pub u32);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct StationType(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]

pub enum StationarySince {
    LessThan1Minute = 0,
    LessThan2Minutes = 1,
    LessThan15Minutes = 2,
    EqualOrGreater15Minutes = 3,
}

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct StationaryVehicleSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=127"))]
pub struct SteeringWheelAngleConfidence(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-511..=512"))]
pub struct SteeringWheelAngleValue(pub i16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct SubCauseCodeType(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-60..=67"))]
pub struct Temperature(pub i8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=4398046511103"))]
pub struct TimestampIts(pub u64);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=7"))]
pub struct Traces(pub SequenceOf<PathHistory>);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct TrafficConditionSubCauseCode(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]
#[non_exhaustive]
pub enum TrafficRule {
    NoPassing = 0,
    NoPassingForTrucks = 1,
    PassToRight = 2,
    PassToLeft = 3,
}

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=10000"))]
pub struct TransmissionInterval(pub u16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=255"))]
pub struct TurningRadius(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("6"))]
pub struct VDS(pub Ia5String);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=86400"))]
pub struct ValidityDuration(pub u32);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct VehicleBreakdownSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(enumerated)]

pub enum VehicleLengthConfidenceIndication {
    NoTrailerPresent = 0,
    TrailerPresentWithKnownLength = 1,
    TrailerPresentWithUnknownLength = 2,
    TrailerPresenceIsUnknown = 3,
    Unavailable = 4,
}

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=1023"))]
pub struct VehicleLengthValue(pub u16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=1024"))]
pub struct VehicleMass(pub u16);

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=62"))]
pub struct VehicleWidth(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-160..=161"))]
pub struct VerticalAccelerationValue(pub i16);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
#[rasn(delegate, size("1..=3"))]
pub struct WMInumber(pub Ia5String);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("1..=127"))]
pub struct WheelBaseVehicle(pub u8);

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("0..=255"))]
pub struct WrongWayDrivingSubCauseCode(pub u8);

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, Serialize, Deserialize)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Copy,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
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

#[derive(
    AsnType,
    Debug,
    Clone,
    Decode,
    Encode,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[rasn(delegate, value("-32766..=32767"))]
pub struct YawRateValue(pub i16);
