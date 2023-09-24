#![no_std]

        extern crate alloc;
        
        use rasn::prelude::*;
            // =====================================================
            // DENM-PDU-Description
            // { itu-t(0) identified-organization(4) etsi(0) itsDomain(5) wg1(1) denmPduRelease2(103831) major-version-2(2) minor-version-2(2) }
            // =====================================================
            
        
        
        /// Inner type 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct AlacarteContainerExtGroupRoadConfiguration {
            pub road_configuration: Option<RoadConfigurationContainer>,
                    pub precrash: Option<PreCrashContainer>,
                    
        }

        impl AlacarteContainerExtGroupRoadConfiguration {
        pub fn new(
            road_configuration: Option<RoadConfigurationContainer>,
	precrash: Option<PreCrashContainer>,
        ) -> Self {
            Self {
                road_configuration,
	precrash,
            }
        }
    }

        
        ///* 
 /// * This type represents the À La Carte Container.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field lanePosition: the optional lane position of the event.
 /// *
 /// * @field impactReduction: optional vehicle data for collision mitigation.
 /// *
 /// * @field externalTemperature: optional the ambient temperature at the event position.
 /// *
 /// * @field roadWorks: optional information of the roadwork zone.
 /// *
 /// * @field positioningSolution: optionally indicates the technical solution being used by the originating ITS-S to estimate the event position.
 /// *
 /// * @field stationaryVehicle: optional information about a stationary vehicle.
 /// *
 /// * @field roadConfiguration: optional information about the configuration of the road.
 /// *
 /// * @field precrash: the optional information about perceived objects that represent hazards and/or could be subject of collisions. 
 /// *
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct AlacarteContainer {
            pub lane_position: Option<LanePosition>,
                    pub impact_reduction: Option<ImpactReductionContainer>,
                    pub external_temperature: Option<Temperature>,
                    pub road_works: Option<RoadWorksContainerExtended>,
                    pub positioning_solution: Option<PositioningSolutionType>,
                    pub stationary_vehicle: Option<StationaryVehicleContainer>,
                    #[rasn(extension_addition_group)]
        pub ext_group_road_configuration: AlacarteContainerExtGroupRoadConfiguration,
                    
        }

        impl AlacarteContainer {
        pub fn new(
            lane_position: Option<LanePosition>,
	impact_reduction: Option<ImpactReductionContainer>,
	external_temperature: Option<Temperature>,
	road_works: Option<RoadWorksContainerExtended>,
	positioning_solution: Option<PositioningSolutionType>,
	stationary_vehicle: Option<StationaryVehicleContainer>,
	ext_group_road_configuration: AlacarteContainerExtGroupRoadConfiguration,
        ) -> Self {
            Self {
                lane_position,
	impact_reduction,
	external_temperature,
	road_works,
	positioning_solution,
	stationary_vehicle,
	ext_group_road_configuration,
            }
        }
    }

        

        
        ///* 
 /// * This type represents the DENM PDU.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field header: the header of the DENM PDU.
 /// *
 /// * @field denm: the payload of the DENM PDU.
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct DENM {
            #[rasn(value("0.."))]
        pub header: ItsPduHeader,
                    pub denm: DenmPayload,
                    
        }

        impl DENM {
        pub fn new(
            header: ItsPduHeader,
	denm: DenmPayload,
        ) -> Self {
            Self {
                header,
	denm,
            }
        }
    }

        

        
        ///* 
 /// * This type represents the DENM payload.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field management: the Management Container.
 /// *
 /// * @field situation: the optional Situation Container.
 /// *
 /// * @field location: the optional Location Container.
 /// *
 /// * @field alacarte: the optional À La Carte Container .
 /// *
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct DenmPayload {
            pub management: ManagementContainer,
                    pub situation: Option<SituationContainer>,
                    pub location: Option<LocationContainer>,
                    pub alacarte: Option<AlacarteContainer>,
                    
        }

        impl DenmPayload {
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

        

        
        ///* 
 /// * This type contains detailed information about the vehicle in which the originating ITS-S is mounted, for mitigating the consequences of a collision.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field heightLonCarrLeft: the height of the left longitudinal carrier of the vehicle from base to top.
 /// *
 /// * @field heightLonCarrRight: the height of the right longitudinal carrier of the vehicle from base to top.
 /// *
 /// * @field posLonCarrLeft: the position of the left longitudinal carrier of vehicle.
 /// *
 /// * @field posLonCarrRight: the position of the right longitudinal carrier of vehicle.
 /// *
 /// * @field positionOfPillars: information about the vertical support of the vehicle in which the originating ITS-S is mounted. It shall be included for passenger vehicles only.
 /// *
 /// * @field posCentMass: the position of the centre of mass of the vehicle.
 /// *
 /// * @field wheelBaseVehicle: the wheel base of the vehicle.
 /// *
 /// * @field turningRadius: the turning radius of the vehicle.
 /// *
 /// * @field posFrontAx: the position of the front axle of the vehicle.
 /// *
 /// * @field positionOfOccupants: indicates whether an in-vehicle seat is occupied at the moment of generation of the message.
 /// *
 /// * @field vehicleMass: the mass of the unloaded vehicle
 /// *
 /// * @field requestResponseIndication: indicates whether the originating ITS-S transmitting the impactReduction component is requesting the receiving ITS-S to provide also its impactReduction component.
 /// *
 ///

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

        

        
        
        /// Inner type 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct LocationContainerExtGroupLanePositions {
            pub lane_positions: Option<GeneralizedLanePositions>,
                    pub occupied_lanes: Option<OccupiedLanesWithConfidence>,
                    pub linked_ivims: Option<IvimReferences>,
                    pub linked_mapems: Option<MapReferences>,
                    pub detection_zones_to_specified_event_point: Option<TracesExtended>,
                    pub predicted_paths: Option<PathPredictedList>,
                    
        }

        impl LocationContainerExtGroupLanePositions {
        pub fn new(
            lane_positions: Option<GeneralizedLanePositions>,
	occupied_lanes: Option<OccupiedLanesWithConfidence>,
	linked_ivims: Option<IvimReferences>,
	linked_mapems: Option<MapReferences>,
	detection_zones_to_specified_event_point: Option<TracesExtended>,
	predicted_paths: Option<PathPredictedList>,
        ) -> Self {
            Self {
                lane_positions,
	occupied_lanes,
	linked_ivims,
	linked_mapems,
	detection_zones_to_specified_event_point,
	predicted_paths,
            }
        }
    }

        
        ///* 
 /// * This type represents the Location Container.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field eventSpeed: optional speed of a detected dynamic event and the confidence of the speed information. 
 /// *
 /// * @field eventPositionHeading: the optional heading of a dynamic event and the confidence of the heading information.
 /// *
 /// * @field detectionZonesToEventPosition: the detection zone information approaching the event position.
 /// *
 /// * @field roadType: the optional road type information at the event position. 
 /// *
 /// * @field lanePositions: the optional lane(s) where the event is located, at the position indicated by the component eventPosition of the Management container.
 /// *
 /// * @field occupiedLanes: the optional lane(s) that are fully or partially occupied by the event, at the position indicated by the component eventPosition of the Management container.
 /// *
 /// * @field linkedIvims: the optional list of DF IvimReference, pointing to IVIMs that are semantically connected because providing information 
 /// * applying to the road segment(s) covered by the components detectionZonesToEventPosition, detectionZonesToSpecifiedEventPoint and the SituationContainer component eventZone.
 /// *
 /// * @field linkedMapem: the optional list of DF Mapreference, pointing to MAPEMs that are semantically connected because providing information 
 /// * applying to the road segment(s) covered by the component detectionZonesToEventPosition, detectionZonesToSpecifiedEventPoint and the SituationContainer component eventZone.
 /// *
 /// * @field detectionZonesToSpecifiedEventPoint: the optional detection zone information approaching towards a specified event point. 
 /// *
 /// * @field predictedPaths: the optional list of future paths or trajectories that the event may move along or zones that the event may occupy. 
 /// *
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct LocationContainer {
            pub event_speed: Option<Speed>,
                    pub event_position_heading: Option<Wgs84Angle>,
                    pub detection_zones_to_event_position: Traces,
                    pub road_type: Option<RoadType>,
                    #[rasn(extension_addition_group)]
        pub ext_group_lane_positions: LocationContainerExtGroupLanePositions,
                    
        }

        impl LocationContainer {
        pub fn new(
            event_speed: Option<Speed>,
	event_position_heading: Option<Wgs84Angle>,
	detection_zones_to_event_position: Traces,
	road_type: Option<RoadType>,
	ext_group_lane_positions: LocationContainerExtGroupLanePositions,
        ) -> Self {
            Self {
                event_speed,
	event_position_heading,
	detection_zones_to_event_position,
	road_type,
	ext_group_lane_positions,
            }
        }
    }

        

        
        ///* 
 /// * This type represents the Management Container.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field actionId: the identifier of the DENM.
 /// *
 /// * @field detectionTime: the time at which the event is detected.
 /// *
 /// * @field referenceTime: the time at which a new DENM, an update DENM or a cancellation DENM is generated
 /// *
 /// * @field termination: the optional termination type of the DENM.
 /// *
 /// * @field eventPosition: the geographical position used in the definition of the awareness area / relevance zone, see clause 6.1.3.
 /// *
 /// * @field awarenessDistance: the optional radius of the circular awareness area, with centre at the event position or at any of the event history points as defined in clause 6.1.3.1.
 /// *
 /// * @field awarenessTrafficDirection: the optional traffic direction along which the receiving ITS-S may encounter the event, as defined in clause 6.1.3.1.
 /// *
 /// * @field validityDuration: the validity duration of a DENM. This component represents a time offset in the unit of second since detectionTime.
 /// *
 /// * @field transmissionInterval: the optional time interval for DENM transmission as defined by the originating ITS-S. If the component is not present in DENM, a default value defaultValidity is assumed.
 /// *
 /// * @field stationType: the station type information of the originating ITS-S.
 /// *
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ManagementContainer {
            pub action_id: ActionId,
                    pub detection_time: TimestampIts,
                    pub reference_time: TimestampIts,
                    pub termination: Option<Termination>,
                    pub event_position: ReferencePosition,
                    pub awareness_distance: Option<StandardLength3b>,
                    pub awareness_traffic_direction: Option<TrafficDirection>,
                    #[rasn(default = "management_container_validity_duration_default")]
        pub validity_duration: DeltaTimeSecond,
                    pub transmission_interval: Option<DeltaTimeMilliSecondPositive>,
                    pub station_type: StationType,
                    
        }

        impl ManagementContainer {
        pub fn new(
            action_id: ActionId,
	detection_time: TimestampIts,
	reference_time: TimestampIts,
	termination: Option<Termination>,
	event_position: ReferencePosition,
	awareness_distance: Option<StandardLength3b>,
	awareness_traffic_direction: Option<TrafficDirection>,
	validity_duration: DeltaTimeSecond,
	transmission_interval: Option<DeltaTimeMilliSecondPositive>,
	station_type: StationType,
        ) -> Self {
            Self {
                action_id,
	detection_time,
	reference_time,
	termination,
	event_position,
	awareness_distance,
	awareness_traffic_direction,
	validity_duration,
	transmission_interval,
	station_type,
            }
        }
    }

        fn management_container_validity_duration_default() -> DeltaTimeSecond {
                DeltaTimeSecond(600).into()
            }
            
            

        
        ///* 
 /// * This type contains detailed information about an object with which a vehicle and/or the traffic is likely to collide.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field perceivedPreCrashObject: information about a perceived object in the East, North, Up reference frame.
 /// *
 /// * @field objectStationId: the optional station Id of the object for which the information is provided.
 /// *
 /// * @field timeToCollision: the optional estimated time to collision of a vehicle with the object. 
 /// *
 /// * @field impactSection: indication of the objects section where the impact will most likely occur. 
 /// * When the target object is likely to be a vehicle, then this component should be present, otherwise it should not be provided.  
 /// *
 /// * @field estimatedBrakingDistance: the optional estimated distance in which the vehicle would need to come to a complete hold, if no obstruction was in the way. 
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct PreCrashContainer {
            pub perceived_pre_crash_object: PerceivedObject,
                    pub object_station_id: Option<StationId>,
                    pub time_to_collision: Option<DeltaTimeMilliSecondPositive>,
                    pub impact_section: Option<ObjectFace>,
                    pub estimated_braking_distance: Option<StandardLength12b>,
                    
        }

        impl PreCrashContainer {
        pub fn new(
            perceived_pre_crash_object: PerceivedObject,
	object_station_id: Option<StationId>,
	time_to_collision: Option<DeltaTimeMilliSecondPositive>,
	impact_section: Option<ObjectFace>,
	estimated_braking_distance: Option<StandardLength12b>,
        ) -> Self {
            Self {
                perceived_pre_crash_object,
	object_station_id,
	time_to_collision,
	impact_section,
	estimated_braking_distance,
            }
        }
    }

        

        
        ///* 
 /// * This type contains detailed information about the configuration of road section(s) that are geographically related to the event.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field roadConfigurationConfidence: information about the source of the road configuration and the confidence in the information.
 /// *
 /// * @field roadConfigurationSectionList: a list of road configuration information per applicable road section. 
 ///*
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RoadConfigurationContainer {
            pub road_configuration_confidence: MetaInformation,
                    pub road_configuration_section_list: RoadConfigurationSectionList,
                    
        }

        impl RoadConfigurationContainer {
        pub fn new(
            road_configuration_confidence: MetaInformation,
	road_configuration_section_list: RoadConfigurationSectionList,
        ) -> Self {
            Self {
                road_configuration_confidence,
	road_configuration_section_list,
            }
        }
    }

        

        
        ///* 
 /// * This type contains detailed information of a roadwork zone and specific access conditions.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field lightBarSirenInUse: optionally indicates whether a roadwork vehicle has switched on the light bar or siren. It is used when the roadwork involves a specific roadwork vehicle
 /// *
 /// * @field closedLanes: optionally indicates whether the roadwork has caused the closure of one or several driving lanes. 
 /// * Optionally, it may indicate whether a hard shoulder lane is closed to traffic or can be used for specific usage (e.g. for stopping).
 /// *
 /// * @field restriction: the optional type(s) of vehicles that are restricted to access the road work zone. More than one vehicle types may be provided by this component if the restriction apply to multiple vehicle types. 
 /// *
 /// * @field speedLimit: the optional speed limitation applied to the roadwork zone.
 /// *
 /// * @field incidentIndication: the optional incident related to the roadworks to provide additional information of the roadworks zone.
 /// *
 /// * @field recommendedPath: the optional recommended itinerary in order to contour the roadworks zone.
 /// * If present, a recommended path shall be a list of path points in the order from the starting point closest to the roadworks zone to the end point of the recommended path. 
 /// *
 /// * @field startingPointSpeedLimit: the optional effective starting position of a speed limit being applied to the roadwork zone, w.r.t. the component eventPosition on the Management Container.
 /// * This component shall be present if the speed limit is applied at a certain distance prior to the roadwork zone starting position.
 /// *
 /// * @field trafficFlowRule: optionally indicates the side of the road to which the traffic should flow around a roadwork.
 /// *
 /// * @field referenceDenms: an optional sequence of actionIds for different DENMs that describe the same event. If it is available, it indicates the actionIds of all other DENMs describing this event.
 /// *
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RoadWorksContainerExtended {
            pub light_bar_siren_in_use: Option<LightBarSirenInUse>,
                    pub closed_lanes: Option<ClosedLanes>,
                    pub restriction: Option<RestrictedTypes>,
                    pub speed_limit: Option<SpeedLimit>,
                    pub incident_indication: Option<CauseCodeV2>,
                    pub recommended_path: Option<ItineraryPath>,
                    pub starting_point_speed_limit: Option<DeltaReferencePosition>,
                    pub traffic_flow_rule: Option<TrafficRule>,
                    pub reference_denms: Option<ActionIdList>,
                    
        }

        impl RoadWorksContainerExtended {
        pub fn new(
            light_bar_siren_in_use: Option<LightBarSirenInUse>,
	closed_lanes: Option<ClosedLanes>,
	restriction: Option<RestrictedTypes>,
	speed_limit: Option<SpeedLimit>,
	incident_indication: Option<CauseCodeV2>,
	recommended_path: Option<ItineraryPath>,
	starting_point_speed_limit: Option<DeltaReferencePosition>,
	traffic_flow_rule: Option<TrafficRule>,
	reference_denms: Option<ActionIdList>,
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

        

        
        
        /// Inner type 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct SituationContainerExtGroupLinkedDenms {
            pub linked_denms: Option<ActionIdList>,
                    pub event_end: Option<Position1d>,
                    
        }

        impl SituationContainerExtGroupLinkedDenms {
        pub fn new(
            linked_denms: Option<ActionIdList>,
	event_end: Option<Position1d>,
        ) -> Self {
            Self {
                linked_denms,
	event_end,
            }
        }
    }

        
        ///* 
 /// * This type represents the situation container.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field informationQuality: the quality level of the information provided by the ITS-S application of the originating ITS-S. 
 /// * It indicates the probability of the detected event being truly existent at the event position.
 /// *
 /// * @field eventType: the event type, including direct cause and sub cause.
 /// *
 /// * @field linkedCause: the optional type of a linked event co-existing at the same time and the same place (same event zone), including direct cause and sub cause of the linked event, 
 /// * for which no other DENM is sent out.
 /// *
 /// * @field eventZone: an optional list of EventPoint, using the position indicated in the component eventPosition of the Management container as the reference position for the first EventPoint.
 /// *
 /// * @field linkedDenms: the optional list of DF ActionId, pointing to DENMs that are semantically connected because applying to consecutive event zones at the same time.
 /// *
 /// * @field eventEnd: the end position of the event along the road that is affected by the event w.r.t. the component eventPosition of the Management container. 
 /// * This end position is represented by the length of the event along the road. 
 /// *
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SituationContainer {
            pub information_quality: InformationQuality,
                    pub event_type: CauseCodeV2,
                    pub linked_cause: Option<CauseCodeV2>,
                    pub event_zone: Option<EventZone>,
                    #[rasn(extension_addition_group)]
        pub ext_group_linked_denms: SituationContainerExtGroupLinkedDenms,
                    
        }

        impl SituationContainer {
        pub fn new(
            information_quality: InformationQuality,
	event_type: CauseCodeV2,
	linked_cause: Option<CauseCodeV2>,
	event_zone: Option<EventZone>,
	ext_group_linked_denms: SituationContainerExtGroupLinkedDenms,
        ) -> Self {
            Self {
                information_quality,
	event_type,
	linked_cause,
	event_zone,
	ext_group_linked_denms,
            }
        }
    }

        

        
        ///* 
 /// * This type contains detailed information about a stationary vehicle.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field stationarySince: the optional time duration of the stationary vehicle being stationary.
 /// *
 /// * @field stationaryCause: optional additional information to describe causes of the stationary vehicle event such as human problem.
 /// *
 /// * @field carryingDangerousGoods: optional information on the type of dangerous goods, the required emergency action and other information.
 /// *
 /// * @field numberOfOccupants: the optional estimated number of occupants involved in the stationary vehicle event.
 /// *
 /// * @field vehicleIdentification: the optional identification of the stationary vehicle.
 /// *
 /// * @field energyStorageType: the optional vehicle energy storage type information of the stationary vehicle, such as electric, diesel, etc.
 /// *
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct StationaryVehicleContainer {
            pub stationary_since: Option<StationarySince>,
                    pub stationary_cause: Option<CauseCodeV2>,
                    pub carrying_dangerous_goods: Option<DangerousGoodsExtended>,
                    pub number_of_occupants: Option<NumberOfOccupants>,
                    pub vehicle_identification: Option<VehicleIdentification>,
                    pub energy_storage_type: Option<EnergyStorageType>,
                    
        }

        impl StationaryVehicleContainer {
        pub fn new(
            stationary_since: Option<StationarySince>,
	stationary_cause: Option<CauseCodeV2>,
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

        

///*
 ///  * This indicates the termination type of generated DENM, i.e. if it is a cancellation DENM or a negation DENM
 ///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum Termination {
     IsCancellation = 0,
                 IsNegation = 1,
                
}

///* 
 /// * This type specifies the default value for DENM validity duration used for DENM protocol operation.
 ///

pub const DEFAULT_VALIDITY: u16 = 600;


            // =====================================================
            // ETSI-ITS-CDD
            // { itu-t(0) identified-organization(4) etsi(0) itsDomain(5) wg1(1) 102894 cdd(2) major-version-4(4) minor-version-1(1) }
            // =====================================================
            
///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 /// Specification of CDD Data Frames:
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///*
 /// * This DF represents an acceleration vector with associated confidence value.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field polarAcceleration: the representation of the acceleration vector in a polar or cylindrical coordinate system. 
 /// * 
 /// * @field cartesianAcceleration: the representation of the acceleration vector in a cartesian coordinate system.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum Acceleration3dWithConfidence {
   PolarAcceleration(AccelerationPolarWithZ),
     CartesianAcceleration(AccelerationCartesian),
    
}


        
        ///*
 /// * This DF represents a acceleration vector in a cartesian coordinate system.
 /// 
 /// * It shall include the following components: 
 /// * 
 /// * @field xAcceleration: the x component of the acceleration vector with the associated confidence value.
 /// * 
 /// * @field yAcceleration: the y component of the acceleration vector with the associated confidence value.
 /// *
 /// * @field zAcceleration: the optional z component of the acceleration vector with the associated confidence value.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct AccelerationCartesian {
            pub x_acceleration: AccelerationComponent,
                    pub y_acceleration: AccelerationComponent,
                    pub z_acceleration: Option<AccelerationComponent>,
                    
        }

        impl AccelerationCartesian {
        pub fn new(
            x_acceleration: AccelerationComponent,
	y_acceleration: AccelerationComponent,
	z_acceleration: Option<AccelerationComponent>,
        ) -> Self {
            Self {
                x_acceleration,
	y_acceleration,
	z_acceleration,
            }
        }
    }

        

///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 /// Specification of CDD Data Elements: 
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///
 ///* 
 /// * This DE indicates a change of acceleration.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `accelerate` - if the magnitude of the horizontal velocity vector increases.
 /// * - 1 - `decelerate` - if the magnitude of the horizontal velocity vector decreases.
 /// *
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum AccelerationChange {
     Accelerate = 0,
                 Decelerate = 1,
                
}


        
        ///* 
 /// * This DF represents information associated to changes in acceleration. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field accelOrDecel: the indication of an acceleration change.
 /// *
 /// * @field actionDeltaTime: the period over which the acceleration change action is performed.
 /// *
 /// * @category: Kinematic Information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct AccelerationChangeIndication {
            pub accel_or_decel: AccelerationChange,
                    pub action_delta_time: DeltaTimeTenthOfSecond,
                    
        }

        impl AccelerationChangeIndication {
        pub fn new(
            accel_or_decel: AccelerationChange,
	action_delta_time: DeltaTimeTenthOfSecond,
        ) -> Self {
            Self {
                accel_or_decel,
	action_delta_time,
            }
        }
    }

        

        
        ///* 
 /// * This DF represents an acceleration component along with a confidence value.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field value: the value of the acceleration component which can be estimated as the mean of the current distribution.
 /// *
 /// * @field confidence: the confidence value associated to the provided value.
 /// *
 /// * @category: Kinematic Information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct AccelerationComponent {
            pub value: AccelerationValue,
                    pub confidence: AccelerationConfidence,
                    
        }

        impl AccelerationComponent {
        pub fn new(
            value: AccelerationValue,
	confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

///*
 /// * This DE indicates the acceleration confidence value which represents the estimated absolute accuracy of an acceleration value with a default confidence level of 95 %. 
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 101`) if the confidence value is equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
 /// * - `101` if the confidence value is out of range i.e. greater than 10 m/s^2,
 /// * - `102` if the confidence value is unavailable.
 /// *
 /// * The value 0 shall not be used.
 /// *
 /// * @note: The fact that an acceleration value is received with confidence value set to `unavailable(102)` can be caused by several reasons, such as:
 /// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
 /// * - the sensor cannot calculate the accuracy due to lack of variables, or
 /// * - there has been a vehicle bus (e.g. CAN bus) error.
 /// * In all 3 cases above, the acceleration value may be valid and used by the application.
 /// * 
 /// * @note: If an acceleration value is received and its confidence value is set to `outOfRange(101)`, it means that the value is not valid and therefore cannot be trusted. Such value is not useful for the application.
 /// *
 /// * @unit 0,1 m/s^2
 /// * @category: Kinematic information
 /// * @revision: Description revised in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=102"))]
        pub struct AccelerationConfidence(pub u8);


///*
 /// * This DE indicates the current controlling mechanism for longitudinal movement of the vehicle.
 /// * The data may be provided via the in-vehicle network. It indicates whether a specific in-vehicle
 /// * acceleration control system is engaged or not. Currently, this DE includes the information of the
 /// * vehicle brake pedal, gas pedal, emergency brake system, collision warning system, adaptive cruise
 /// * control system, cruise control system and speed limiter system.
 /// *
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * - 0 - `brakePedalEngaged`      - Driver is stepping on the brake pedal,
 /// * - 1 - `gasPedalEngaged`        - Driver is stepping on the gas pedal,
 /// * - 2 - `emergencyBrakeEngaged`  - emergency brake system is engaged,
 /// * - 3 - `collisionWarningEngaged`- collision warning system is engaged,
 /// * - 4 - `accEngaged`             - ACC is engaged,
 /// * - 5 - `cruiseControlEngaged`   - cruise control is engaged,
 /// * - 6 - `speedLimiterEngaged`    - speed limiter is engaged.
 /// *
 /// * Otherwise (for example when the corresponding system is not available due to non equipped system
 /// * or information is unavailable), the corresponding bit shall be set to 0.
 /// *
 /// * @note: The system engagement condition is OEM specific and therefore out of scope of the present document.
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("7..=7"))]
        pub struct AccelerationControl(pub BitString);


        
        ///*
 /// * This DF represents the magnitude of the acceleration vector and associated confidence value.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field accelerationMagnitudeValue: the magnitude of the acceleration vector.
 /// * 
 /// * @field accelerationConfidence: the confidence value of the magnitude value.
 /// *
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct AccelerationMagnitude {
            pub acceleration_magnitude_value: AccelerationMagnitudeValue,
                    pub acceleration_confidence: AccelerationConfidence,
                    
        }

        impl AccelerationMagnitude {
        pub fn new(
            acceleration_magnitude_value: AccelerationMagnitudeValue,
	acceleration_confidence: AccelerationConfidence,
        ) -> Self {
            Self {
                acceleration_magnitude_value,
	acceleration_confidence,
            }
        }
    }

        

///* 
 /// * This DE represents the magnitude of the acceleration vector in a defined coordinate system.
 /// *
 /// * The value shall be set to:
 /// * - `0` to indicate no acceleration,
 /// * - `n` (`n > 0` and `n < 160`) to indicate acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
 /// * - `160` for acceleration values greater than 15,9 m/s^2,
 /// * - `161` when the data is unavailable.
 /// *
 /// * @unit 0,1 m/s^2
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=161"))]
        pub struct AccelerationMagnitudeValue(pub u8);


        
        ///*
 /// * This DF represents an acceleration vector in a polar or cylindrical coordinate system.
 /// 
 /// * It shall include the following components: 
 /// * 
 /// * @field accelerationMagnitude: magnitude of the acceleration vector projected onto the reference plane, with the associated confidence value.
 /// * 
 /// * @field accelerationDirection: polar angle of the acceleration vector projected onto the reference plane, with the associated confidence value.
 /// *
 /// * @field zAcceleration: the optional z component of the acceleration vector along the reference axis of the cylindrical coordinate system, with the associated confidence value.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct AccelerationPolarWithZ {
            pub acceleration_magnitude: AccelerationMagnitude,
                    pub acceleration_direction: CartesianAngle,
                    pub z_acceleration: Option<AccelerationComponent>,
                    
        }

        impl AccelerationPolarWithZ {
        pub fn new(
            acceleration_magnitude: AccelerationMagnitude,
	acceleration_direction: CartesianAngle,
	z_acceleration: Option<AccelerationComponent>,
        ) -> Self {
            Self {
                acceleration_magnitude,
	acceleration_direction,
	z_acceleration,
            }
        }
    }

        

///* 
 /// * This DE represents the value of an acceleration component in a defined coordinate system.
 /// *
 /// * The value shall be set to:
 /// * - `-160` for acceleration values equal to or less than -16 m/s^2,
 /// * - `n` (`n > -160` and `n <= 0`) to indicate negative acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
 /// * - `n` (`n > 0` and `n < 160`) to indicate positive acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
 /// * - `160` for acceleration values greater than 15,9 m/s^2,
 /// * - `161` when the data is unavailable.
 /// *
 /// * @note: the formula for values > -160 and <160 results in rounding up to the next value. Zero acceleration is indicated using n=0.
 /// * @unit 0,1 m/s^2
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-160..=161"))]
        pub struct AccelerationValue(pub i16);


///*
 /// * This DE indicates an access technology.
 /// *
 /// * The value shall be set to:
 /// * - `0`: in case of any access technology class,
 /// * - `1`: in case of ITS-G5 access technology class,
 /// * - `2`: in case of LTE-V2X access technology class,
 /// * - `3`: in case of NR-V2X access technology class.
 /// * 
 /// * @category: Communication information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
                #[non_exhaustive]
pub enum AccessTechnologyClass {
     Any = 0,
                 Itsg5Class = 1,
                 Ltev2xClass = 2,
                 Nrv2xClass = 3,
                
}


///*
 /// * This DE represents the value of the sub cause code of the @ref CauseCode `accident`.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `unavailable`                        - in case the information on the sub cause of the accident is unavailable,
 /// * - 1 - `multiVehicleAccident`               - in case more than two vehicles are involved in accident,
 /// * - 2 - `heavyAccident`                      - in case the airbag of the vehicle involved in the accident is triggered, 
 /// *                                              the accident requires important rescue and/or recovery work,
 /// * - 3 - `accidentInvolvingLorry`             - in case the accident involves a lorry,
 /// * - 4 - `accidentInvolvingBus`               - in case the accident involves a bus,
 /// * - 5 - `accidentInvolvingHazardousMaterials`- in case the accident involves hazardous material,
 /// * - 6 - `accidentOnOppositeLane`             - in case the accident happens on opposite lanes,
 /// * - 7 - `unsecuredAccident`                  - in case the accident is not secured,
 /// * - 8 - `assistanceRequested`                - in case rescue and assistance are requested,
 /// * - 9-255                                    - reserved for future usage. 
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct AccidentSubCauseCode(pub u8);


        
        ///*
 /// * This DF represents an identifier used to describe a protocol action taken by an ITS-S.
 /// * 
 /// * It shall include the following components: 
 /// *
 /// * @field originatingStationId: Id of the ITS-S that takes the action. 
 /// * 
 /// * @field sequenceNumber: a sequence number. 
 /// * 
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref ActionId instead. 
 /// * @category: Communication information
 /// * @revision: V1.3.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ActionID {
            pub originating_station_id: StationID,
                    pub sequence_number: SequenceNumber,
                    
        }

        impl ActionID {
        pub fn new(
            originating_station_id: StationID,
	sequence_number: SequenceNumber,
        ) -> Self {
            Self {
                originating_station_id,
	sequence_number,
            }
        }
    }

        

        
        ///*
 /// * This DF represents an identifier used to describe a protocol action taken by an ITS-S.
 /// * 
 /// * It shall include the following components: 
 /// *
 /// * @field originatingStationId: Id of the ITS-S that takes the action. 
 /// * 
 /// * @field sequenceNumber: a sequence number. 
 /// * 
 /// * @category: Communication information
 /// * @revision: Created in V2.1.1 based on @ref ActionID.
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ActionId {
            pub originating_station_id: StationId,
                    pub sequence_number: SequenceNumber,
                    
        }

        impl ActionId {
        pub fn new(
            originating_station_id: StationId,
	sequence_number: SequenceNumber,
        ) -> Self {
            Self {
                originating_station_id,
	sequence_number,
            }
        }
    }

        

        
///*
 /// * This DF shall contain a list of @ref ActionId. 
 ///
 /// * @category: Communication Information
 /// * @revision: Created in V2.1.1 based on ReferenceDenms from DENM Release 1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct ActionIdList(pub SequenceOf<ActionId>);


///*
 /// * This DE represents the value of the sub cause code of the @ref CauseCode `adverseWeatherCondition-Adhesion`. 
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`     - in case information on the cause of the low road adhesion is unavailable,
 /// * - 1 - `heavyFrostOnRoad`- in case the low road adhesion is due to heavy frost on the road,
 /// * - 2 - `fuelOnRoad`      - in case the low road adhesion is due to fuel on the road,
 /// * - 3 - `mudOnRoad`       - in case the low road adhesion is due to mud on the road,
 /// * - 4 - `snowOnRoad`      - in case the low road adhesion is due to snow on the road,
 /// * - 5 - `iceOnRoad`       - in case the low road adhesion is due to ice on the road,
 /// * - 6 - `blackIceOnRoad`  - in case the low road adhesion is due to black ice on the road,
 /// * - 7 - `oilOnRoad`       - in case the low road adhesion is due to oil on the road,
 /// * - 8 - `looseChippings`  - in case the low road adhesion is due to loose gravel or stone fragments detached from a road surface or from a hazard,
 /// * - 9 - `instantBlackIce` - in case the low road adhesion is due to instant black ice on the road surface,
 /// * - 10 - `roadsSalted`    - when the low road adhesion is due to salted road,
 /// * - 11-255                - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct AdverseWeatherConditionAdhesionSubCauseCode(pub u8);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-ExtremeWeatherCondition`.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `unavailable` - in case information on the type of extreme weather condition is unavailable,
 /// * - 1 - `strongWinds` - in case the type of extreme weather condition is strong wind,
 /// * - 2 - `damagingHail`- in case the type of extreme weather condition is damaging hail,
 /// * - 3 - `hurricane`   - in case the type of extreme weather condition is hurricane,
 /// * - 4 - `thunderstorm`- in case the type of extreme weather condition is thunderstorm,
 /// * - 5 - `tornado`     - in case the type of extreme weather condition is tornado,
 /// * - 6 - `blizzard`    - in case the type of extreme weather condition is blizzard.
 /// * - 7-255             - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct AdverseWeatherConditionExtremeWeatherConditionSubCauseCode(pub u8);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-Precipitation`. 
 /// *
 /// * The value shall be set to:
 /// * - 0 - `unavailable`   - in case information on the type of precipitation is unavailable,
 /// * - 1 - `heavyRain`     - in case the type of precipitation is heavy rain,
 /// * - 2 - `heavySnowfall` - in case the type of precipitation is heavy snow fall,
 /// * - 3 - `softHail`      - in case the type of precipitation is soft hail.
 /// * - 4-255               - are reserved for future usage
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct AdverseWeatherConditionPrecipitationSubCauseCode(pub u8);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `adverseWeatherCondition-Visibility`.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `unavailable`    - in case information on the cause of low visibility is unavailable,
 /// * - 1 - `fog`            - in case the cause of low visibility is fog,
 /// * - 2 - `smoke`          - in case the cause of low visibility is smoke,
 /// * - 3 - `heavySnowfall`  - in case the cause of low visibility is heavy snow fall,
 /// * - 4 - `heavyRain`      - in case the cause of low visibility is heavy rain,
 /// * - 5 - `heavyHail`      - in case the cause of low visibility is heavy hail,
 /// * - 6 - `lowSunGlare`    - in case the cause of low visibility is sun glare,
 /// * - 7 - `sandstorms`     - in case the cause of low visibility is sand storm,
 /// * - 8 - `swarmsOfInsects`- in case the cause of low visibility is swarm of insects.
 /// * - 9-255                - are reserved for future usage
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct AdverseWeatherConditionVisibilitySubCauseCode(pub u8);


///*
 /// * This DE represents the air humidity in tenths of percent.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 1001`) indicates that the applicable value is equal to or less than n x 0,1 percent and greater than (n-1) x 0,1 percent.
 /// * - `1001` indicates that the air humidity is unavailable.
 /// *
 /// * @category: Basic information
 /// * @unit: 0,1 % 
 /// * @revision: created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=1001"))]
        pub struct AirHumidity(pub u16);


        
        ///*
 /// * This DF provides the altitude and confidence level of an altitude information in a WGS84 coordinate system.
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field altitudeValue: altitude of a geographical point.
 /// *
 /// * @field altitudeConfidence: confidence level of the altitudeValue.
 /// *
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref AltitudeWithConfidence instead. 
 /// * @category: GeoReference information
 /// * @revision: Description revised in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Altitude {
            pub altitude_value: AltitudeValue,
                    pub altitude_confidence: AltitudeConfidence,
                    
        }

        impl Altitude {
        pub fn new(
            altitude_value: AltitudeValue,
	altitude_confidence: AltitudeConfidence,
        ) -> Self {
            Self {
                altitude_value,
	altitude_confidence,
            }
        }
    }

        

///*
 /// * This DE indicates the altitude confidence value which represents the estimated absolute accuracy of an altitude value of a geographical point with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to: 
 /// *   - 0  - `alt-000-01`   - if the confidence value is equal to or less than 0,01 metre,
 /// *   - 1  - `alt-000-02`   - if the confidence value is equal to or less than 0,02 metre and greater than 0,01 metre,
 /// *   - 2  - `alt-000-05`   - if the confidence value is equal to or less than 0,05 metre and greater than 0,02 metre,            
 /// *   - 3  - `alt-000-10`   - if the confidence value is equal to or less than 0,1 metre and greater than 0,05 metre,            
 /// *   - 4  - `alt-000-20`   - if the confidence value is equal to or less than 0,2 metre and greater than 0,1 metre,            
 /// *   - 5  - `alt-000-50`   - if the confidence value is equal to or less than 0,5 metre and greater than 0,2 metre,             
 /// *   - 6  - `alt-001-00`   - if the confidence value is equal to or less than 1 metre and greater than 0,5 metre,             
 /// *   - 7  - `alt-002-00`   - if the confidence value is equal to or less than 2 metres and greater than 1 metre,             
 /// *   - 8  - `alt-005-00`   - if the confidence value is equal to or less than 5 metres and greater than 2 metres,              
 /// *   - 9  - `alt-010-00`   - if the confidence value is equal to or less than 10 metres and greater than 5 metres,             
 /// *   - 10 - `alt-020-00`   - if the confidence value is equal to or less than 20 metres and greater than 10 metres,            
 /// *   - 11 - `alt-050-00`   - if the confidence value is equal to or less than 50 metres and greater than 20 metres,            
 /// *   - 12 - `alt-100-00`   - if the confidence value is equal to or less than 100 metres and greater than 50 metres,           
 /// *   - 13 - `alt-200-00`   - if the confidence value is equal to or less than 200 metres and greater than 100 metres,           
 /// *   - 14 - `outOfRange`   - if the confidence value is out of range, i.e. greater than 200 metres,
 /// *   - 15 - `unavailable`  - if the confidence value is unavailable.       
 /// *
 /// * @note: The fact that an altitude value is received with confidence value set to `unavailable(15)` can be caused
 /// * by several reasons, such as:
 /// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
 /// * - the sensor cannot calculate the accuracy due to lack of variables, or
 /// * - there has been a vehicle bus (e.g. CAN bus) error.
 /// * In all 3 cases above, the altitude value may be valid and used by the application.
 /// * 
 /// * @note: If an altitude value is received and its confidence value is set to `outOfRange(14)`, it means that the  
 /// * altitude value is not valid and therefore cannot be trusted. Such value is not useful for the application.             
 /// *
 /// * @category: GeoReference information
 /// * @revision: Description revised in V2.1.1
 /// 

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


///*
 /// * This DE represents the altitude value in a WGS84 coordinate system.
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to: 
 /// * - `-100 000` if the altitude is equal to or less than -1 000 m,
 /// * - `n` (`n > -100 000` and `n < 800 000`) if the altitude is equal to or less than n  x 0,01 metre and greater than (n-1) x 0,01 metre,
 /// * - `800 000` if the altitude  greater than 7 999,99 m,
 /// * - `800 001` if the information is not available.
 /// *
 /// * @note: the range of this DE does not use the full binary encoding range, but all reasonable values are covered. In order to cover all possible altitude ranges a larger encoding would be necessary.
 /// * @unit: 0,01 metre
 /// * @category: GeoReference information
 /// * @revision: Description revised in V2.1.1 (definition of 800 000 has slightly changed) 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-100000..=800001"))]
        pub struct AltitudeValue(pub i32);


///* 
 /// * This DE indicates the angle confidence value which represents the estimated absolute accuracy of an angle value with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to: 
 /// * - `n` (`n > 0` and `n < 126`)  if the accuracy is equal to or less than n * 0,1 degrees and greater than (n-1) x * 0,1 degrees,
 /// * - `126` if the  accuracy is out of range, i.e. greater than 12,5 degrees,
 /// * - `127` if the accuracy information is not available.
 /// *
 /// * @unit: 0,1 degrees
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=127"))]
        pub struct AngleConfidence(pub u8);


///* 
 /// * This DE indicates the angular acceleration confidence value which represents the estimated accuracy of an angular acceleration value with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// * For correlation computation, maximum interval levels shall be assumed.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `degSecSquared-01` - if the accuracy is equal to or less than 1 degree/second^2,
 /// * - 1 - `degSecSquared-02` - if the accuracy is equal to or less than 2 degrees/second^2 and greater than 1 degree/second^2,
 /// * - 2 - `degSecSquared-05` - if the accuracy is equal to or less than 5 degrees/second^2 and greater than 1 degree/second^2,
 /// * - 3 - `degSecSquared-10` - if the accuracy is equal to or less than 10 degrees/second^2 and greater than 5 degrees/second^2,
 /// * - 4 - `degSecSquared-20` - if the accuracy is equal to or less than 20 degrees/second^2 and greater than 10 degrees/second^2,
 /// * - 5 - `degSecSquared-50` - if the accuracy is equal to or less than 50 degrees/second^2 and greater than 20 degrees/second^2,
 /// * - 6 - `outOfRange`       - if the accuracy is out of range, i.e. greater than 50 degrees/second^2,
 /// * - 7 - `unavailable`      - if the accuracy information is unavailable.
 /// *
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum AngularAccelerationConfidence {
     DegSecSquared01 = 0,
                 DegSecSquared02 = 1,
                 DegSecSquared05 = 2,
                 DegSecSquared10 = 3,
                 DegSecSquared20 = 4,
                 DegSecSquared50 = 5,
                 OutOfRange = 6,
                 Unavailable = 7,
                
}


///* 
 /// * This DE indicates the angular speed confidence value which represents the estimated absolute accuracy of an angular speed value with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// * For correlation computation, maximum interval levels can be assumed.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `degSec-01`   - if the accuracy is equal to or less than 1 degree/second,
 /// * - 1 - `degSec-02`   - if the accuracy is equal to or less than 2 degrees/second and greater than 1 degree/second,
 /// * - 2 - `degSec-05`   - if the accuracy is equal to or less than 5 degrees/second and greater than 2 degrees/second,
 /// * - 3 - `degSec-10`   - if the accuracy is equal to or less than 10 degrees/second and greater than 5 degrees/second,
 /// * - 4 - `degSec-20`   - if the accuracy is equal to or less than 20 degrees/second and greater than 10 degrees/second,
 /// * - 5 - `degSec-50`   - if the accuracy is equal to or less than 50 degrees/second and greater than 20 degrees/second,
 /// * - 6 - `outOfRange`  - if the accuracy is out of range, i.e. greater than 50 degrees/second,
 /// * - 7 - `unavailable` - if the accuracy information is unavailable.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum AngularSpeedConfidence {
     DegSec01 = 0,
                 DegSec02 = 1,
                 DegSec05 = 2,
                 DegSec10 = 3,
                 DegSec20 = 4,
                 DegSec50 = 5,
                 OutOfRange = 6,
                 Unavailable = 7,
                
}


///*
 /// * This DE indicates the number of axles of a passing train.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 2` and `n < 1001`) indicates that the train has n x axles,
 /// * - `1001`indicates that the number of axles is out of range,
 /// * - `1002` the information is unavailable.
 /// *
 /// * 
 /// * @unit: Number of axles
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("2..=1002"))]
        pub struct AxlesCount(pub u16);


///*
 /// * This DE represents the measured uncompensated atmospheric pressure.
 /// * 
 /// * The value shall be set to:
 /// * - `2999` indicates that the applicable value is less than 29990 Pa,
 /// * - `n` (`n > 2999` and `n <= 12000`) indicates that the applicable value is equal to or less than n x 10 Pa and greater than (n-1) x 10 Pa, 
 /// * - `12001` indicates that the values is greater than 120000 Pa,
 /// * - `12002` indicates that the information is not available.
 /// *
 /// * @category: Basic information
 /// * @unit: 10 Pascal
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("2999..=12002"))]
        pub struct BarometricPressure(pub u16);


        
        ///* 
 /// * This DE represents a general container for usage in various types of messages.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field stationType: the type of technical context in which the ITS-S that has generated the message is integrated in.
 /// *
 /// * @field referencePosition: the reference position of the station that has generated the message that contains the basic container.
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct BasicContainer {
            pub station_type: TrafficParticipantType,
                    pub reference_position: ReferencePositionWithConfidence,
                    
        }

        impl BasicContainer {
        pub fn new(
            station_type: TrafficParticipantType,
	reference_position: ReferencePositionWithConfidence,
        ) -> Self {
            Self {
                station_type,
	reference_position,
            }
        }
    }

        

        
///* 
 /// * This DF provides information about the configuration of a road section in terms of lanes using a list of @ref LanePositionAndType .
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct BasicLaneConfiguration(pub SequenceOf<BasicLaneInformation>);


        
        ///* 
 /// * This DF provides basic information about a single lane of a road segment.
 /// * It includes the following components: 
 /// * 
 /// * @field laneNumber: the number associated to the lane that provides a transversal identification. 
 /// * 
 /// * @field direction: the direction of traffic flow allowed on the lane. 
 /// * 
 /// * @field laneWidth: the optional width of the lane.
 /// *
 /// * @field connectingLane: the number of the connecting lane in the next road section, i.e. the number of the lane which the vehicle will use when travelling from one section to the next,
 /// * if it does not actively change lanes. If this component is absent, the lane name number remains the same in the next section.
 /// *
 /// * @field connectingRoadSection: the identifier of the next road section in direction of traffic, that is connecting to the current road section. 
 /// * If this component is absent, the connecting road section is the one following the instance where this DF is placed in the @ref RoadConfigurationSectionList.
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct BasicLaneInformation {
            pub lane_number: LanePosition,
                    pub direction: Direction,
                    pub lane_width: Option<LaneWidth>,
                    pub connecting_lane: Option<LanePosition>,
                    pub connecting_road_section: Option<RoadSectionId>,
                    
        }

        impl BasicLaneInformation {
        pub fn new(
            lane_number: LanePosition,
	direction: Direction,
	lane_width: Option<LaneWidth>,
	connecting_lane: Option<LanePosition>,
	connecting_road_section: Option<RoadSectionId>,
        ) -> Self {
            Self {
                lane_number,
	direction,
	lane_width,
	connecting_lane,
	connecting_road_section,
            }
        }
    }

        

///*
 /// * This DE indicates the cardinal number of bogies of a train.
 /// *
 /// * The value shall be set to: 
 /// * - `n` (`n > 1` and `n < 100`) indicates that the train has n x bogies,
 /// * - `100`indicates that the number of bogies is out of range, 
 /// * - `101` the information is unavailable.
 /// *
 /// * @unit: Number of bogies
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("2..=101"))]
        pub struct BogiesCount(pub u8);


///*
 /// * The DE represents a cardinal number that counts the size of a set. 
 /// * 
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct CardinalNumber1B(pub u8);


///*
 /// * The DE represents a cardinal number that counts the size of a set. 
 /// * 
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=8"))]
        pub struct CardinalNumber3b(pub u8);


        
        ///* 
 /// * This DF represents a general Data Frame to describe an angle component along with a confidence value in a cartesian coordinate system.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field value: The angle value which can be estimated as the mean of the current distribution.
 /// *
 /// * @field confidence: The confidence value associated to the provided value.
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CartesianAngle {
            pub value: CartesianAngleValue,
                    pub confidence: AngleConfidence,
                    
        }

        impl CartesianAngle {
        pub fn new(
            value: CartesianAngleValue,
	confidence: AngleConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

///* 
 /// * This DE represents an angle value described in a local Cartesian coordinate system, per default counted positive in
 /// * a right-hand local coordinate system from the abscissa.
 /// *
 /// * The value shall be set to: 
 /// * - `n` (`n >= 0` and `n < 3600`) if the angle is equal to or less than n x 0,1 degrees, and greater than (n-1) x 0,1 degrees,
 /// * - `3601` if the information is not available.
 /// *
 /// * The value 3600 shall not be used. 
 /// * 
 /// * @unit 0,1 degrees
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1, description and value for 3601 corrected in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3601"))]
        pub struct CartesianAngleValue(pub u16);


        
        ///* 
 /// * This DF represents a general Data Frame to describe an angular acceleration component along with a confidence value in a cartesian coordinate system.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field value: The angular acceleration component value.
 /// *
 /// * @field confidence: The confidence value associated to the provided value.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1 
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CartesianAngularAccelerationComponent {
            pub value: CartesianAngularAccelerationComponentValue,
                    pub confidence: AngularAccelerationConfidence,
                    
        }

        impl CartesianAngularAccelerationComponent {
        pub fn new(
            value: CartesianAngularAccelerationComponentValue,
	confidence: AngularAccelerationConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

///*
 /// * This DE represents an angular acceleration value described in a local Cartesian coordinate system, per default counted positive in
 /// * a right-hand local coordinate system from the abscissa.
 /// *
 ///  * The value shall be set to: 
 /// * - `-255` if the acceleration is equal to or less than -255 degrees/s^2,
 /// * - `n` (`n > -255` and `n < 255`) if the acceleration is equal to or less than n x 1 degree/s^2,
 ///      and greater than `(n-1)` x 0,01 degree/s^2,
 /// * - `255` if the acceleration is greater than 254 degrees/s^2,
 /// * - `256` if the information is unavailable.
 /// *
 /// * @unit:  degree/s^2 (degrees per second squared)
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-255..=256"))]
        pub struct CartesianAngularAccelerationComponentValue(pub i16);


        
        ///* 
 /// * This DF represents an angular velocity component along with a confidence value in a cartesian coordinate system.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field value: The angular velocity component.
 /// *
 /// * @field confidence: The confidence value associated to the provided value.
 /// *
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CartesianAngularVelocityComponent {
            pub value: CartesianAngularVelocityComponentValue,
                    pub confidence: AngularSpeedConfidence,
                    
        }

        impl CartesianAngularVelocityComponent {
        pub fn new(
            value: CartesianAngularVelocityComponentValue,
	confidence: AngularSpeedConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

///*
 /// * This DE represents an angular velocity component described in a local Cartesian coordinate system, per default counted positive in
 /// * a right-hand local coordinate system from the abscissa.
 /// *
 /// * The value shall be set to: 
 /// * - `-255` if the velocity is equal to or less than -255 degrees/s,
 /// * - `n` (`n > -255` and `n < 255`) if the velocity is equal to or less than n x 1 degree/s, and greater than (n-1) x 1 degree/s,
 /// * - `255` if the velocity is greater than 254 degrees/s,
 /// * - `256` if the information is unavailable.
 /// *
 /// * @unit: degree/s
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-255..=256"))]
        pub struct CartesianAngularVelocityComponentValue(pub i16);


///*
 /// * This DF represents the value of a cartesian coordinate with a range of -327,68 metres to +327,66 metres.
 /// *
 /// * The value shall be set to:
 /// * - `-32 768` if the longitudinal offset is out of range, i.e. less than or equal to -327,68 metres,
 /// * - `n` (`n > -32 768` and `n < 32 767`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
 /// * - `32 767` if the longitudinal offset is out of range, i.e. greater than + 327,66 metres.
 /// *
 /// * @unit 0,01 m
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-32768..=32767"))]
        pub struct CartesianCoordinate(pub i16);


///*
 /// * This DF represents the value of a cartesian coordinate with a range of -1 310,72 metres to +1 310,70 metres.
 /// *
 /// * The value shall be set to:
 /// * - `-131072` if the longitudinal offset is out of range, i.e. less than or equal to -1 310,72 metres,
 /// * - `n` (`n > 131 072` and `n < 131 071`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
 /// * - `131 071` if the longitudinal offset is out of range, i.e. greater than + 1 310,70 metres.
 /// *  
 /// * @unit 0,01 m
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-131072..=131071"))]
        pub struct CartesianCoordinateLarge(pub i32);


///*
 /// * This DF represents the value of a cartesian coordinate with a range of -30,94 metres to +10,00 metres.
 /// *
 /// * The value shall be set to:
 /// * - `3094` if the longitudinal offset is out of range, i.e. less than or equal to -30,94 metres,
 /// * - `n` (`n > -3 094` and `n < 1 001`) if the longitudinal offset information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
 /// * - `1001` if the longitudinal offset is out of range, i.e. greater than 10 metres.
 /// *
 /// * @unit 0,01 m
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-3094..=1001"))]
        pub struct CartesianCoordinateSmall(pub i16);


        
        ///*
 /// * This DF represents a coordinate along with a confidence value in a cartesian reference system.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field value: the coordinate value, which can be estimated as the mean of the current distribution.
 /// * 
 /// * @field confidence: the coordinate confidence value associated to the provided value.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CartesianCoordinateWithConfidence {
            pub value: CartesianCoordinateLarge,
                    pub confidence: CoordinateConfidence,
                    
        }

        impl CartesianCoordinateWithConfidence {
        pub fn new(
            value: CartesianCoordinateLarge,
	confidence: CoordinateConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

        
        ///* 
 /// * This DF represents a  position in a two- or three-dimensional cartesian coordinate system.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field xCoordinate: the X coordinate value.
 /// *
 /// * @field yCoordinate: the Y coordinate value.
 /// *
 /// * @field zCoordinate: the optional Z coordinate value.
 /// * 
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CartesianPosition3d {
            pub x_coordinate: CartesianCoordinate,
                    pub y_coordinate: CartesianCoordinate,
                    pub z_coordinate: Option<CartesianCoordinate>,
                    
        }

        impl CartesianPosition3d {
        pub fn new(
            x_coordinate: CartesianCoordinate,
	y_coordinate: CartesianCoordinate,
	z_coordinate: Option<CartesianCoordinate>,
        ) -> Self {
            Self {
                x_coordinate,
	y_coordinate,
	z_coordinate,
            }
        }
    }

        

        
        ///* 
 /// * This DF represents a  position in a two- or three-dimensional cartesian coordinate system with an associated confidence level for each coordinate.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field xCoordinate: the X coordinate value with the associated confidence level.
 /// *
 /// * @field yCoordinate: the Y coordinate value with the associated confidence level.
 /// *
 /// * @field zCoordinate: the optional Z coordinate value with the associated confidence level.
 /// * 
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CartesianPosition3dWithConfidence {
            pub x_coordinate: CartesianCoordinateWithConfidence,
                    pub y_coordinate: CartesianCoordinateWithConfidence,
                    pub z_coordinate: Option<CartesianCoordinateWithConfidence>,
                    
        }

        impl CartesianPosition3dWithConfidence {
        pub fn new(
            x_coordinate: CartesianCoordinateWithConfidence,
	y_coordinate: CartesianCoordinateWithConfidence,
	z_coordinate: Option<CartesianCoordinateWithConfidence>,
        ) -> Self {
            Self {
                x_coordinate,
	y_coordinate,
	z_coordinate,
            }
        }
    }

        

        
        ///*
 /// * This DF is a representation of the cause code value of a traffic event. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field causeCode: the main cause of a detected event. 
 /// *
 /// * @field subCauseCode: the subordinate cause of a detected event. 
 /// *
 /// * The semantics of the entire DF are completely defined by the component causeCode. The interpretation of the subCauseCode may 
 /// * provide additional information that is not strictly necessary to understand the causeCode itself, and is therefore optional.
 /// *
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref CauseCodeV2 instead. 
 /// *
 /// * @category: Traffic information
 /// * @revision: Editorial update in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct CauseCode {
            pub cause_code: CauseCodeType,
                    pub sub_cause_code: SubCauseCodeType,
                    
        }

        impl CauseCode {
        pub fn new(
            cause_code: CauseCodeType,
	sub_cause_code: SubCauseCodeType,
        ) -> Self {
            Self {
                cause_code,
	sub_cause_code,
            }
        }
    }

        

///*
 /// * This DF is a representation of the cause code value and associated sub cause code value of a traffic event. 
 /// *
 /// * The following options are available:
 /// * - 0                                                        - reserved for future use,
 /// * - 1  - `trafficCondition1`                                 - in case the type of event is an abnormal traffic condition,
 /// * - 2  - `accident2`                                         - in case the type of event is a road accident,
 /// * - 3  - `roadworks3`                                        - in case the type of event is roadwork,
 /// * - 4                                                        - reserved for future usage,
 /// * - 5  - `impassability5`                                    - in case the  type of event is unmanaged road blocking, referring to any
 /// *                                                              blocking of a road, partial or total, which has not been adequately secured and signposted,
 /// * - 6  - `adverseWeatherCondition-Adhesion6`                 - in case the  type of event is low adhesion,
 /// * - 7  - `aquaplaning7`                                      - danger of aquaplaning on the road,
 /// * - 8                                                        - reserved for future usage,
 /// * - 9  - `hazardousLocation-SurfaceCondition9`               - in case the type of event is abnormal road surface condition,
 /// * - 10 - `hazardousLocation-ObstacleOnTheRoad10`             - in case the type of event is obstacle on the road,
 /// * - 11 - `hazardousLocation-AnimalOnTheRoad11`               - in case the type of event is animal on the road,
 /// * - 12 - `humanPresenceOnTheRoad`                            - in case the type of event is presence of human vulnerable road user on the road,
 /// * - 13                                                       - reserved for future usage,
 /// * - 14 - `wrongWayDriving14`                                 - in case the type of the event is vehicle driving in wrong way,
 /// * - 15 - `rescueAndRecoveryWorkInProgress15`                 - in case the type of event is rescue and recovery work for accident or for a road hazard in progress,
 /// * - 16                                                       - reserved for future usage,
 /// * - 17 - `adverseWeatherCondition-ExtremeWeatherCondition17` - in case the type of event is extreme weather condition,
 /// * - 18 - `adverseWeatherCondition-Visibility18`              - in case the type of event is low visibility,
 /// * - 19 - `adverseWeatherCondition-Precipitation19`           - in case the type of event is precipitation,
 /// * - 20 - `violence20`                                        - in case the the type of event is human violence on or near the road,
 /// * - 21-25                                                    - reserved for future usage,
 /// * - 26 - `slowVehicle26`                                     - in case the type of event is slow vehicle driving on the road,
 /// * - 27 - `dangerousEndOfQueue27`                             - in case the type of event is dangerous end of vehicle queue,
 /// * - 28 - `publicTransportVehicleApproaching                  - in case the type of event is a public transport vehicle approaching, with a priority defined by applicable traffic regulations,
 /// * - 29-90                                                    - are reserved for future usage,
 /// * - 91 - `vehicleBreakdown91`                                - in case the type of event is break down vehicle on the road,
 /// * - 92 - `postCrash92`                                       - in case the type of event is a detected crash,
 /// * - 93 - `humanProblem93`                                    - in case the type of event is human health problem in vehicles involved in traffic,
 /// * - 94 - `stationaryVehicle94`                               - in case the type of event is stationary vehicle,
 /// * - 95 - `emergencyVehicleApproaching95`                     - in case the type of event is an approaching vehicle operating on a mission for which the 
 ///                                                                applicable traffic regulations provide it with defined priority rights in traffic. 
 /// * - 96 - `hazardousLocation-DangerousCurve96`                - in case the type of event is dangerous curve,
 /// * - 97 - `collisionRisk97`                                   - in case the type of event is a collision risk,
 /// * - 98 - `signalViolation98`                                 - in case the type of event is signal violation,
 /// * - 99 - `dangerousSituation99`                              - in case the type of event is dangerous situation in which autonomous safety system in vehicle 
 /// *                                                              is activated,
 /// * - 100 - `railwayLevelCrossing100`                          - in case the type of event is a railway level crossing. 
 /// * - 101-255                                                  - are reserved for future usage.
 /// *
 /// * @note: this DF is defined for use as part of CauseCodeV2. It is recommended to use CauseCodeV2.
 /// * @category: Traffic information
 /// * @revision: Created in V2.1.1, the type of impassability5 changed to ImpassabilitySubCauseCode in V2.2.1, value 28 added in V2.2.1, definition of value 12 and 95 changed in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum CauseCodeChoice {
   Reserved0(SubCauseCodeType),
     TrafficCondition1(TrafficConditionSubCauseCode),
     Accident2(AccidentSubCauseCode),
     Roadworks3(RoadworksSubCauseCode),
     Reserved4(SubCauseCodeType),
     Impassability5(ImpassabilitySubCauseCode),
     AdverseWeatherConditionAdhesion6(AdverseWeatherConditionAdhesionSubCauseCode),
     Aquaplaning7(SubCauseCodeType),
     Reserved8(SubCauseCodeType),
     HazardousLocationSurfaceCondition9(HazardousLocationSurfaceConditionSubCauseCode),
     HazardousLocationObstacleOnTheRoad10(HazardousLocationObstacleOnTheRoadSubCauseCode),
     HazardousLocationAnimalOnTheRoad11(HazardousLocationAnimalOnTheRoadSubCauseCode),
     HumanPresenceOnTheRoad12(HumanPresenceOnTheRoadSubCauseCode),
     Reserved13(SubCauseCodeType),
     WrongWayDriving14(WrongWayDrivingSubCauseCode),
     RescueAndRecoveryWorkInProgress15(RescueAndRecoveryWorkInProgressSubCauseCode),
     Reserved16(SubCauseCodeType),
     AdverseWeatherConditionExtremeWeatherCondition17(AdverseWeatherConditionExtremeWeatherConditionSubCauseCode),
     AdverseWeatherConditionVisibility18(AdverseWeatherConditionVisibilitySubCauseCode),
     AdverseWeatherConditionPrecipitation19(AdverseWeatherConditionPrecipitationSubCauseCode),
     Violence20(SubCauseCodeType),
     Reserved21(SubCauseCodeType),
     Reserved22(SubCauseCodeType),
     Reserved23(SubCauseCodeType),
     Reserved24(SubCauseCodeType),
     Reserved25(SubCauseCodeType),
     SlowVehicle26(SlowVehicleSubCauseCode),
     DangerousEndOfQueue27(DangerousEndOfQueueSubCauseCode),
     PublicTransportVehicleApproaching28(SubCauseCodeType),
     Reserved29(SubCauseCodeType),
     Reserved30(SubCauseCodeType),
     Reserved31(SubCauseCodeType),
     Reserved32(SubCauseCodeType),
     Reserved33(SubCauseCodeType),
     Reserved34(SubCauseCodeType),
     Reserved35(SubCauseCodeType),
     Reserved36(SubCauseCodeType),
     Reserved37(SubCauseCodeType),
     Reserved38(SubCauseCodeType),
     Reserved39(SubCauseCodeType),
     Reserved40(SubCauseCodeType),
     Reserved41(SubCauseCodeType),
     Reserved42(SubCauseCodeType),
     Reserved43(SubCauseCodeType),
     Reserved44(SubCauseCodeType),
     Reserved45(SubCauseCodeType),
     Reserved46(SubCauseCodeType),
     Reserved47(SubCauseCodeType),
     Reserved48(SubCauseCodeType),
     Reserved49(SubCauseCodeType),
     Reserved50(SubCauseCodeType),
     Reserved51(SubCauseCodeType),
     Reserved52(SubCauseCodeType),
     Reserved53(SubCauseCodeType),
     Reserved54(SubCauseCodeType),
     Reserved55(SubCauseCodeType),
     Reserved56(SubCauseCodeType),
     Reserved57(SubCauseCodeType),
     Reserved58(SubCauseCodeType),
     Reserved59(SubCauseCodeType),
     Reserved60(SubCauseCodeType),
     Reserved61(SubCauseCodeType),
     Reserved62(SubCauseCodeType),
     Reserved63(SubCauseCodeType),
     Reserved64(SubCauseCodeType),
     Reserved65(SubCauseCodeType),
     Reserved66(SubCauseCodeType),
     Reserved67(SubCauseCodeType),
     Reserved68(SubCauseCodeType),
     Reserved69(SubCauseCodeType),
     Reserved70(SubCauseCodeType),
     Reserved71(SubCauseCodeType),
     Reserved72(SubCauseCodeType),
     Reserved73(SubCauseCodeType),
     Reserved74(SubCauseCodeType),
     Reserved75(SubCauseCodeType),
     Reserved76(SubCauseCodeType),
     Reserved77(SubCauseCodeType),
     Reserved78(SubCauseCodeType),
     Reserved79(SubCauseCodeType),
     Reserved80(SubCauseCodeType),
     Reserved81(SubCauseCodeType),
     Reserved82(SubCauseCodeType),
     Reserved83(SubCauseCodeType),
     Reserved84(SubCauseCodeType),
     Reserved85(SubCauseCodeType),
     Reserved86(SubCauseCodeType),
     Reserved87(SubCauseCodeType),
     Reserved88(SubCauseCodeType),
     Reserved89(SubCauseCodeType),
     Reserved90(SubCauseCodeType),
     VehicleBreakdown91(VehicleBreakdownSubCauseCode),
     PostCrash92(PostCrashSubCauseCode),
     HumanProblem93(HumanProblemSubCauseCode),
     StationaryVehicle94(StationaryVehicleSubCauseCode),
     EmergencyVehicleApproaching95(EmergencyVehicleApproachingSubCauseCode),
     HazardousLocationDangerousCurve96(HazardousLocationDangerousCurveSubCauseCode),
     CollisionRisk97(CollisionRiskSubCauseCode),
     SignalViolation98(SignalViolationSubCauseCode),
     DangerousSituation99(DangerousSituationSubCauseCode),
     RailwayLevelCrossing100(RailwayLevelCrossingSubCauseCode),
     Reserved101(SubCauseCodeType),
     Reserved102(SubCauseCodeType),
     Reserved103(SubCauseCodeType),
     Reserved104(SubCauseCodeType),
     Reserved105(SubCauseCodeType),
     Reserved106(SubCauseCodeType),
     Reserved107(SubCauseCodeType),
     Reserved108(SubCauseCodeType),
     Reserved109(SubCauseCodeType),
     Reserved110(SubCauseCodeType),
     Reserved111(SubCauseCodeType),
     Reserved112(SubCauseCodeType),
     Reserved113(SubCauseCodeType),
     Reserved114(SubCauseCodeType),
     Reserved115(SubCauseCodeType),
     Reserved116(SubCauseCodeType),
     Reserved117(SubCauseCodeType),
     Reserved118(SubCauseCodeType),
     Reserved119(SubCauseCodeType),
     Reserved120(SubCauseCodeType),
     Reserved121(SubCauseCodeType),
     Reserved122(SubCauseCodeType),
     Reserved123(SubCauseCodeType),
     Reserved124(SubCauseCodeType),
     Reserved125(SubCauseCodeType),
     Reserved126(SubCauseCodeType),
     Reserved127(SubCauseCodeType),
     Reserved128(SubCauseCodeType),
    
}


///*
 /// *The DE represents the value of the cause code of an event. 
 /// * 
 /// * The value shall be set to:
 /// * - 0                                                     - reserved for future use,
 /// * - 1  - `trafficCondition`                               - in case the type of event is an abnormal traffic condition,
 /// * - 2  - `accident`                                       - in case the type of event is a road accident,
 /// * - 3  - `roadworks`                                      - in case the type of event is roadwork,
 /// * - 4                                                     - reserved for future usage,
 /// * - 5  - `impassability`                                  - in case the  type of event is unmanaged road blocking, referring to any
 /// *                                                           blocking of a road, partial or total, which has not been adequately
 /// *                                                           secured and signposted,
 /// * - 6  - `adverseWeatherCondition-Adhesion`               - in case the  type of event is low adhesion,
 /// * - 7  - `aquaplaning`                                    - danger of aquaplaning on the road,
 /// * - 8                                                     - reserved for future usage,
 /// * - 9  - `hazardousLocation-SurfaceCondition`             - in case the type of event is abnormal road surface condition,
 /// * - 10 - `hazardousLocation-ObstacleOnTheRoad`            - in case the type of event is obstacle on the road,
 /// * - 11 - `hazardousLocation-AnimalOnTheRoad`              - in case the type of event is animal on the road,
 /// * - 12 - `humanPresenceOnTheRoad`                         - in case the type of event is presence of human vulnerable road user on the road,
 /// * - 13                                                    - reserved for future usage,
 /// * - 14 - `wrongWayDriving`                                - in case the type of the event is vehicle driving in wrong way,
 /// * - 15 - `rescueAndRecoveryWorkInProgress`                - in case the type of event is rescue and recovery work for accident or for a road hazard in progress,
 /// * - 16                                                    - reserved for future usage,
 /// * - 17 - `adverseWeatherCondition-ExtremeWeatherCondition`- in case the type of event is extreme weather condition,
 /// * - 18 - `adverseWeatherCondition-Visibility`             - in case the type of event is low visibility,
 /// * - 19 - `adverseWeatherCondition-Precipitation`          - in case the type of event is precipitation,
 /// * - 20 - `violence`                                       - in case the the type of event is human violence on or near the road,
 /// * - 21-25                                                 - reserved for future usage,
 /// * - 26 - `slowVehicle`                                    - in case the type of event is slow vehicle driving on the road,
 /// * - 27 - `dangerousEndOfQueue`                            - in case the type of event is dangerous end of vehicle queue,
 /// * - 28 - `publicTransportVehicleApproaching               - in case the type of event is a public transport vehicle approaching, with a priority defined by applicable traffic regulations,
 /// * - 29-90                                                 - are reserved for future usage,
 /// * - 91 - `vehicleBreakdown`                               - in case the type of event is break down vehicle on the road,
 /// * - 92 - `postCrash`                                      - in case the type of event is a detected crash,
 /// * - 93 - `humanProblem`                                   - in case the type of event is human health problem in vehicles involved in traffic,
 /// * - 94 - `stationaryVehicle`                              - in case the type of event is stationary vehicle,
 /// * - 95 - `emergencyVehicleApproaching`                    - in case the type of event is an approaching vehicle operating on a mission for which the applicable 
 ///                                                             traffic regulations provide it with defined priority rights in traffic. 
 /// * - 96 - `hazardousLocation-DangerousCurve`               - in case the type of event is dangerous curve,
 /// * - 97 - `collisionRisk`                                  - in case the type of event is a collision risk,
 /// * - 98 - `signalViolation`                                - in case the type of event is signal violation,
 /// * - 99 - `dangerousSituation`                             - in case the type of event is dangerous situation in which autonomous safety system in vehicle 
 /// *                                                             is activated,
 /// * - 100 - `railwayLevelCrossing`                          - in case the type of event is a railway level crossing. 
 /// * - 101-255                                               - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1, value 28 added in V2.2.1, definition of values 12 and 95 changed on V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct CauseCodeType(pub u8);


        
        ///*
 /// * This DF is an alternative representation of the cause code value of a traffic event. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field ccAndScc: the main cause of a detected event. Each entry is of a different type and represents the sub cause code.
 /// *
 /// * The semantics of the entire DF are completely defined by the component causeCode. The interpretation of the subCauseCode may 
 /// * provide additional information that is not strictly necessary to understand the causeCode itself, and is therefore optional.
 /// * 
 /// * @category: Traffic information
 /// * @revision: Created in V2.1.1, 
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct CauseCodeV2 {
            pub cc_and_scc: CauseCodeChoice,
                    
        }

        impl CauseCodeV2 {
        pub fn new(
            cc_and_scc: CauseCodeChoice,
        ) -> Self {
            Self {
                cc_and_scc,
            }
        }
    }

        

        
        ///*
 /// * The DF describes the position of a CEN DSRC road side equipment.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field protectedZoneLatitude: the latitude of the CEN DSRC road side equipment.
 /// * 
 /// * @field protectedZoneLongitude: the latitude of the CEN DSRC road side equipment. 
 /// * 
 /// * @field cenDsrcTollingZoneID: the optional ID of the CEN DSRC road side equipment.
 /// * 
 /// * @category: Infrastructure information, Communication information
 /// * @revision: revised in V2.1.1 (cenDsrcTollingZoneId is directly of type ProtectedZoneId)
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct CenDsrcTollingZone {
            pub protected_zone_latitude: Latitude,
                    pub protected_zone_longitude: Longitude,
                    pub cen_dsrc_tolling_zone_id: Option<ProtectedZoneId>,
                    
        }

        impl CenDsrcTollingZone {
        pub fn new(
            protected_zone_latitude: Latitude,
	protected_zone_longitude: Longitude,
	cen_dsrc_tolling_zone_id: Option<ProtectedZoneId>,
        ) -> Self {
            Self {
                protected_zone_latitude,
	protected_zone_longitude,
	cen_dsrc_tolling_zone_id,
            }
        }
    }

        

///*
 /// * This DE represents the ID of a CEN DSRC tolling zone. 
 /// * 
 /// * @category: Communication information
 /// * @revision: V1.3.1
 /// * @note: this DE is deprecated and shall not be used anymore.  
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct CenDsrcTollingZoneID(pub ProtectedZoneId);


        
        ///* 
 /// * 
 /// * This DF represents the shape of a circular area or a right cylinder that is centred on the shape's reference point. 
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field shapeReferencePoint: optional reference point that represents the centre of the circle, relative to an externally specified reference position. 
 /// * If this component is absent, the externally specified reference position represents the shape's reference point. 
 /// *
 /// * @field radius: the radius of the circular area.
 /// *
 /// * @field height: the optional height, present if the shape is a right cylinder extending in the positive z-axis. 
 /// *
 /// * 
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CircularShape {
            pub shape_reference_point: Option<CartesianPosition3d>,
                    pub radius: StandardLength12b,
                    pub height: Option<StandardLength12b>,
                    
        }

        impl CircularShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
	radius: StandardLength12b,
	height: Option<StandardLength12b>,
        ) -> Self {
            Self {
                shape_reference_point,
	radius,
	height,
            }
        }
    }

        

        
        ///*
 /// * This DF indicates the opening/closure status of the lanes of a carriageway.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field innerhardShoulderStatus: this information is optional and shall be included if an inner hard shoulder is present and the information is known.
 /// * It indicates the open/closing status of inner hard shoulder lanes. 
 /// * 
 /// * @field outerhardShoulderStatus: this information is optional and shall be included if an outer hard shoulder is present and the information is known.
 /// * It indicates the open/closing status of outer hard shoulder lanes. 
 /// * 
 /// * @field drivingLaneStatus: this information is optional and shall be included if the information is known.
 /// * It indicates the open/closing status of driving lanes. 
 /// * For carriageways with more than 13 driving lanes, the drivingLaneStatus component shall not be present.
 /// * 
 /// * @category: Infrastructure information, Road topology information
 /// * @revision: Description revised in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ClosedLanes {
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

        

        
        ///*
 /// * This DF provides information about the breakup of a cluster.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field clusterBreakupReason: indicates the reason for breakup.
 /// * 
 /// * @field breakupTime: indicates the time of breakup. 
 /// *
 /// * @category: Cluster Information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ClusterBreakupInfo {
            pub cluster_breakup_reason: ClusterBreakupReason,
                    pub breakup_time: DeltaTimeQuarterSecond,
                    
        }

        impl ClusterBreakupInfo {
        pub fn new(
            cluster_breakup_reason: ClusterBreakupReason,
	breakup_time: DeltaTimeQuarterSecond,
        ) -> Self {
            Self {
                cluster_breakup_reason,
	breakup_time,
            }
        }
    }

        

///*
 /// * This DE indicates the reason why a cluster leader intends to break up the cluster.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `notProvided`                          - if the information is not provided,
 /// * - 1 - `clusteringPurposeCompleted`           - if the cluster purpose has been completed,
 /// * - 2 - `leaderMovedOutOfClusterBoundingBox`   - if the leader moved out of the cluster's bounding box,
 /// * - 3 - `joiningAnotherCluster`                - if the cluster leader is about to join another cluster,
 /// * - 4 - `enteringLowRiskAreaBasedOnMaps`       - if the cluster is entering an area idenrified as low risk based on the use of maps,
 /// * - 5 - `receptionOfCpmContainingCluster`      - if the leader received a Collective Perception Message containing information about the same cluster. 
 /// * - 6 to 15                                    - are reserved for future use.                                    
 /// *
 /// * @category: Cluster information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct ClusterBreakupReason(pub u8);


        
        ///*
 /// * This DF provides information about the joining of a cluster.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field clusterId: indicates the identifier of the cluster.
 /// * 
 /// * @field joinTime: indicates the time of joining. 
 /// *
 /// * @category: Cluster Information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ClusterJoinInfo {
            pub cluster_id: Identifier1B,
                    pub join_time: DeltaTimeQuarterSecond,
                    
        }

        impl ClusterJoinInfo {
        pub fn new(
            cluster_id: Identifier1B,
	join_time: DeltaTimeQuarterSecond,
        ) -> Self {
            Self {
                cluster_id,
	join_time,
            }
        }
    }

        

        
        ///*
 /// * The DF provides information about the leaving of a cluster.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field clusterId: indicates the cluster.
 /// * 
 /// * @field clusterLeaveReason: indicates the reason for leaving. 
 /// *
 /// * @category: Cluster Information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ClusterLeaveInfo {
            pub cluster_id: Identifier1B,
                    pub cluster_leave_reason: ClusterLeaveReason,
                    
        }

        impl ClusterLeaveInfo {
        pub fn new(
            cluster_id: Identifier1B,
	cluster_leave_reason: ClusterLeaveReason,
        ) -> Self {
            Self {
                cluster_id,
	cluster_leave_reason,
            }
        }
    }

        

///*
 /// * This DE indicates the reason why a cluster participant is leaving the cluster.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `notProvided `                 - if the information is not provided,
 /// * - 1 - `clusterLeaderLost`            - if the cluster leader cannot be found anymore,   
 /// * - 2 - `clusterDisbandedByLeader`     - if the cluster has been disbanded by the leader,
 /// * - 3 - `outOfClusterBoundingBox`      - if the participants moved out of the cluster's bounding box,
 /// * - 4 - `outOfClusterSpeedRange`       - if the cluster speed moved out of a defined range, 
 /// * - 5 - `joiningAnotherCluster`        - if the participant is joining another cluster,
 /// * - 6 - `cancelledJoin`                - if the participant is cancelling a joining procedure,
 /// * - 7 - `failedJoin`                   - if the participant failed to join the cluster,
 /// * - 8 - `safetyCondition`              - if a safety condition applies.
 /// * - 9 to 15                            - are reserved for future use                             
 /// *
 /// * @category: Cluster information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct ClusterLeaveReason(pub u8);


///*
 /// * This DE represents the sub cause codes of the @ref CauseCode `collisionRisk`.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`              - in case information on the type of collision risk is unavailable,
 /// * - 1 - `longitudinalCollisionRisk`- in case the type of detected collision risk is longitudinal collision risk, 
 /// *                                       e.g. forward collision or face to face collision,
 /// * - 2 - `crossingCollisionRisk`    - in case the type of detected collision risk is crossing collision risk,
 /// * - 3 - `lateralCollisionRisk`     - in case the type of detected collision risk is lateral collision risk,
 /// * - 4 - `vulnerableRoadUser`       - in case the type of detected collision risk involves vulnerable road users
 /// *                                       e.g. pedestrians or bicycles.
 /// * - 5-255                          - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct CollisionRiskSubCauseCode(pub u8);


///* 
 /// * This DE represents a confidence level in percentage.
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 101`) : for the confidence level in %,
 /// * - `101`                   : in case the confidence level is not available.
 /// *
 /// * @unit Percent 
 /// * @category: Basic information 
 /// * @revision: Created in V2.1.1 
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=101"))]
        pub struct ConfidenceLevel(pub u8);


///* 
 /// * This DE indicates the coordinate confidence value which represents the estimated absolute accuracy of a position coordinate with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to: 
 /// * - `n` (`n > 0` and `n < 4095`) if the confidence value is is equal to or less than n x 0,01 metre, and greater than (n-1) x 0,01 metre,
 /// * - `4095` if the confidence value is greater than 40,94 metres,
 /// * - `4096` if the confidence value is not available.
 /// *
 /// * @unit 0,01 m
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=4096"))]
        pub struct CoordinateConfidence(pub u16);


///* 
 /// * This DE represents the Bravais-Pearson correlation value for each cell of a lower triangular correlation matrix.
 /// *
 /// * The value shall be set to: 
 /// * - `-100` in case of full negative correlation,
 /// * - `n` (`n > -100` and `n < 0`) if the correlation is negative and equal to n x 100,
 /// * - `0` in case of no correlation,
 /// * - `n` (`n > 0` and `n < 100`) if the correlation is positive and equal to n x 100,
 /// * - `100` in case of full positive correlation,
 /// * - `101` in case the correlation information is unavailable. 
 /// *
 /// * @unit: the value is scaled by 100
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-100..=101"))]
        pub struct CorrelationCellValue(pub i8);


        
///*
 /// * This DF represents a column of a lower triangular positive semi-definite matrix and consists of a list of correlation cell values ordered by rows.
 /// * Given a matrix "A" of size n x n, the number of columns to be included in the lower triangular matrix is k=n-1.
 /// * Each column "i" of the lower triangular matrix then contains k-(i-1) values (ordered by rows from 1 to n-1), where "i" refers to the column number count
 /// * starting at 1 from the left.
 /// *
 /// * @category: Sensing Information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=13", extensible))]
        pub struct CorrelationColumn(pub SequenceOf<CorrelationCellValue>);


///* 
 /// * This DE represents an ISO 3166-1 [25] country code encoded using ITA-2 encoding.
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.2.1 based on ISO 14816 [23]
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("10..=10"))]
        pub struct CountryCode(pub BitString);


        
        ///*
 /// * This DF represents the curvature of the vehicle trajectory and the associated confidence value.
 /// * The curvature detected by a vehicle represents the curvature of actual vehicle trajectory.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field curvatureValue: Detected curvature of the vehicle trajectory.
 /// * 
 /// * @field curvatureConfidence: along with a confidence value of the curvature value with a predefined confidence level. 
 /// * 
 /// * @category: Vehicle information
 /// * @revision: Description revised in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Curvature {
            pub curvature_value: CurvatureValue,
                    pub curvature_confidence: CurvatureConfidence,
                    
        }

        impl Curvature {
        pub fn new(
            curvature_value: CurvatureValue,
	curvature_confidence: CurvatureConfidence,
        ) -> Self {
            Self {
                curvature_value,
	curvature_confidence,
            }
        }
    }

        

///*
 /// * The DE describes whether the yaw rate is used to calculate the curvature for a curvature value.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `yawRateUsed`    - if the yaw rate is used,
 /// * - 1 - `yawRateNotUsed` - if the yaw rate is not used,
 /// * - 2 - `unavailable`    - if the information of curvature calculation mode is unknown.
 /// *
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
                #[non_exhaustive]
pub enum CurvatureCalculationMode {
     YawRateUsed = 0,
                 YawRateNotUsed = 1,
                 Unavailable = 2,
                
}


///*
 /// * This DE indicates the acceleration confidence value which represents the estimated absolute accuracy range of a curvature value with a confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `onePerMeter-0-00002` - if the confidence value is less than or equal to 0,00002 m-1,
 /// * - 1 - `onePerMeter-0-0001`  - if the confidence value is less than or equal to 0,0001 m-1 and greater than 0,00002 m-1,
 /// * - 2 - `onePerMeter-0-0005`  - if the confidence value is less than or equal to 0,0005 m-1 and greater than 0,0001 m-1,
 /// * - 3 - `onePerMeter-0-002`   - if the confidence value is less than or equal to 0,002 m-1 and greater than 0,0005 m-1,
 /// * - 4 - `nePerMeter-0-01`     - if the confidence value is less than or equal to 0,01 m-1 and greater than 0,002 m-1,
 /// * - 5 - `nePerMeter-0-1`      - if the confidence value is less than or equal to 0,1 m-1  and greater than 0,01 m-1,
 /// * - 6 - `outOfRange`          - if the confidence value is out of range, i.e. greater than 0,1 m-1,
 /// * - 7 - `unavailable`         - if the confidence value is not available.
 /// * 
 /// * @note:	The fact that a curvature value is received with confidence value set to `unavailable(7)` can be caused by
 /// * several reasons, such as:
 /// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
 /// * - the sensor cannot calculate the accuracy due to lack of variables, or
 /// * - there has been a vehicle bus (e.g. CAN bus) error.
 /// * In all 3 cases above, the curvature value may be valid and used by the application.
 /// * 
 /// * @note: If a curvature value is received and its confidence value is set to `outOfRange(6)`, it means that the curvature value is not valid 
 /// * and therefore cannot be trusted. Such value is not useful for the application.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: Description revised in V2.1.1
 ///

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


///*
 /// * This DE describes vehicle turning curve with the following information:
 /// * ```
 /// *     Value = 1 / Radius * 10000
 /// * ```
 /// * wherein radius is the vehicle turning curve radius in metres. 
 /// * 
 /// * Positive values indicate a turning curve to the left hand side of the driver.
 /// * It corresponds to the vehicle coordinate system as defined in ISO 8855 [21].
 /// *
 /// * The value shall be set to:
 /// * - `-1023` for  values smaller than -1023,
 /// * - `n` (`n > -1023` and `n < 0`) for negative values equal to or less than `n`, and greater than `(n-1)`,
 /// * - `0` when the vehicle is moving straight,
 /// * - `n` (`n > 0` and `n < 1022`) for positive values equal to or less than `n`, and greater than `(n-1)`,
 /// * - `1022`, for values  greater than 1021,
 /// * - `1023`, if the information is not available.
 /// * 
 /// * @note: The present DE is limited to vehicle types as defined in ISO 8855 [21].
 /// * 
 /// * @unit: 1 over 10 000 metres
 /// * @category: Vehicle information
 /// * @revision: description revised in V2.1.1 (the definition of value 1022 has changed slightly)
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-1023..=1023"))]
        pub struct CurvatureValue(pub i16);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `dangerousEndOfQueue`. 
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`     - in case information on the type of dangerous queue is unavailable,
 /// * - 1 - `suddenEndOfQueue`- in case a sudden end of queue is detected, e.g. due to accident or obstacle,
 /// * - 2 - `queueOverHill`   - in case the dangerous end of queue is detected on the road hill,
 /// * - 3 - `queueAroundBend` - in case the dangerous end of queue is detected around the road bend,
 /// * - 4 - `queueInTunnel`   - in case queue is detected in tunnel,
 /// * - 5-255                 - reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct DangerousEndOfQueueSubCauseCode(pub u8);


///*
 /// * This DE indicates the type of the dangerous goods being carried by a heavy vehicle.
 /// * The value is assigned according to `class` and `division` definitions of dangerous goods as specified in part II,
 /// * chapter 2.1.1.1 of European Agreement concerning the International Carriage of Dangerous Goods by Road [3].
 /// * 
 /// * 
 /// * @category Vehicle information
 /// * @revision: V1.3.1
 /// 

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


        
        ///*
 /// * This DF provides a description of dangerous goods being carried by a heavy vehicle.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field dangerousGoodsType: Type of dangerous goods.
 /// * 
 /// * @field unNumber: a 4-digit number that identifies the substance of the dangerous goods as specified in
 /// * United Nations Recommendations on the Transport of Dangerous Goods - Model Regulations [4],
 /// * 
 /// * @field elevatedTemperature: whether the carried dangerous goods are transported at high temperature.
 /// * If yes, the value shall be set to TRUE,
 /// * 
 /// * @field tunnelsRestricted: whether the heavy vehicle carrying dangerous goods is restricted to enter tunnels.
 /// * If yes, the value shall be set to TRUE,
 /// * 
 /// * @field limitedQuantity: whether the carried dangerous goods are packed with limited quantity.
 /// * If yes, the value shall be set to TRUE,
 /// * 
 /// * @field emergencyActionCode: physical signage placard at the vehicle that carries information on how an emergency
 /// * service should deal with an incident. This component is optional; it shall be present if the information is available.
 /// * 
 /// * @field phoneNumber: contact phone number of assistance service in case of incident or accident.
 /// * This component is optional, it shall be present if the information is available.
 /// * 
 /// * @field companyName: name of company that manages the transportation of the dangerous goods.
 /// * This component is optional; it shall be present if the information is available.
 /// * 
 /// * @category Vehicle information
 /// * @revision: V1.3.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct DangerousGoodsExtended {
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

        

///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `dangerousSituation` 
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`                      - in case information on the type of dangerous situation is unavailable,
 /// * - 1 - `emergencyElectronicBrakeEngaged`  - in case emergency electronic brake is engaged,
 /// * - 2 - `preCrashSystemEngaged`            - in case pre-crash system is engaged,
 /// * - 3 - `espEngaged`                       - in case Electronic Stability Program (ESP) system is engaged,
 /// * - 4 - `absEngaged`                       - in case Anti-lock Braking System (ABS) is engaged,
 /// * - 5 - `aebEngaged`                       - in case Autonomous Emergency Braking (AEB) system is engaged,
 /// * - 6 - `brakeWarningEngaged`              - in case brake warning is engaged,
 /// * - 7 - `collisionRiskWarningEngaged`      - in case collision risk warning is engaged,
 /// * - 8-255                                  - reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct DangerousSituationSubCauseCode(pub u8);


///*
 /// * This DE represents an offset altitude with regards to a defined altitude value.
 /// * It may be used to describe a geographical point with regards to a specific reference geographical position.
 /// *
 /// * The value shall be set to:
 /// * - `-12 700` for values equal to or lower than -127 metres,
 /// * - `n` (`n > -12 700` and `n <= 0`) for altitude offset n x 0,01 metre below the reference position,
 /// * - `0` for no altitudinal offset,
 /// * - `n` (`n > 0` and `n < 12799`) for altitude offset n x 0,01 metre above the reference position,
 /// * - `12 799` for values equal to or greater than 127,99 metres,
 /// * - `12 800` when the information is unavailable.
 /// *
 /// * @unit: 0,01 metre
 /// * @category: GeoReference information
 /// * @revision: editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-12700..=12800"))]
        pub struct DeltaAltitude(pub i16);


///*
 /// * This DE represents an offset latitude with regards to a defined latitude value.
 /// * It may be used to describe a geographical point with regards to a specific reference geographical position.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n >= -131 071` and `n < 0`) for offset n x 10^-7 degree towards the south from the reference position,
 /// * - `0` for no latitudinal offset,
 /// * - `n` (`n > 0` and `n < 131 072`) for offset n x 10^-7 degree towards the north from the reference position,
 /// * - `131 072` when the information is unavailable.
 /// *
 /// * @unit: 10^-7 degree
 /// * @category: GeoReference information
 /// * @revision: editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-131071..=131072"))]
        pub struct DeltaLatitude(pub i32);


///*
 /// * This DE represents an offset longitude with regards to a defined longitude value.
 /// * It may be used to describe a geographical point with regards to a specific reference geographical position.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n >= -131 071` and `n < 0`) for offset n x 10^-7 degree towards the west from the reference position,
 /// * - `0` for no longitudinal offset,
 /// * - `n` (`n > 0` and `n < 131 072`) for offset n x 10^-7 degree towards the east from the reference position, 
 /// * - `131 072` when the information is unavailable.
 /// *
 /// * @unit: 10^-7 degree
 /// * @category: GeoReference information
 /// * @revision: editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-131071..=131072"))]
        pub struct DeltaLongitude(pub i32);


        
        ///*
 /// * This DF defines a geographical point position as a 3 dimensional offset position to a geographical reference point.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field deltaLatitude: A delta latitude offset with regards to the latitude value of the reference position.
 /// *
 /// * @field deltaLongitude: A delta longitude offset with regards to the longitude value of the reference position.
 /// *
 /// * @field deltaAltitude: A delta altitude offset with regards to the altitude value of the reference position.
 /// *
 /// * @category: GeoReference information
 /// * @revision:  V1.3.1
 /// 

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

        

///*
 /// * This DE represents a difference in time with respect to a reference time.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 10001`) to indicate a time value equal to or less than n x 0,001 s, and greater than (n-1) x 0,001 s,
 /// *
 /// * Example: a time interval between two consecutive message transmissions.
 /// * 
 /// * @unit: 0,001 s
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1 from the DE TransmissionInterval in [2]
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=10000"))]
        pub struct DeltaTimeMilliSecondPositive(pub u16);


///* 
 /// * This DE represents a signed difference in time with respect to a reference time.
 /// *
 /// * The value shall be set to:
 /// * - `-2048` for time values equal to or less than -2,048 s,
 /// * - `n` (`n > -2048` and `n < 2047`) to indicate a time value equal to or less than n x 0,001 s, and greater than (n-1) x 0,001 s,
 /// * - `2047` for time values greater than 2,046 s
 /// *
 /// * @unit: 0,001 s
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-2048..=2047"))]
        pub struct DeltaTimeMilliSecondSigned(pub i16);


///* 
 /// * This DE represents a difference in time with respect to a reference time.
 /// * It can be interpreted as the first 8 bits of a GenerationDeltaTime. To convert it to a @ref GenerationDeltaTime, 
 /// * multiply by 256 (i.e. append a `00` byte)
 /// *
 /// * @unit: 256 * 0,001 s 
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=255"))]
        pub struct DeltaTimeQuarterSecond(pub u8);


///* 
 /// * This DE represents a  difference in time with respect to a reference time.
 /// *
 /// * The value shall be set to:
 /// * - `-0` for a difference in time of 0 seconds. 
 /// * - `n` (`n > 0` and `n <= 86400`) to indicate a time value equal to or less than n x 1 s, and greater than (n-1) x 1 s,
 /// *
 /// * @unit: 1 s
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1 from ValidityDuration
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=86400"))]
        pub struct DeltaTimeSecond(pub u32);


///*
 /// * This DE represents a difference in time with respect to a reference time.
 /// *
 /// * The value shall be set to:
 /// * - `-0` for a difference in time of 0 seconds. 
 /// * - `n` (`n > 0` and `n <= 128`) to indicate a time value equal to or less than n x 10 s, and greater than (n-1) x 10 s,
 /// *
 /// * @unit: 10 s
 /// * @category: Basic information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=127"))]
        pub struct DeltaTimeTenSeconds(pub u8);


///* 
 /// * This DE represents a difference in time with respect to a reference time.
 /// *
 /// * The value shall be set to:
 /// * - `0` for a difference in time of 0 seconds. 
 /// * - `n` (`n > 0` and `n < 128`) to indicate a time value equal to or less than n x 0,1 s, and greater than (n-1) x 0,1 s,
 /// *
 /// * @unit: 0,1 s
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=127"))]
        pub struct DeltaTimeTenthOfSecond(pub u8);


        
///*
 /// * This DF represents a portion of digital map. It shall contain a list of waypoints @ref ReferencePosition.
 /// * 
 /// * @category: GeoReference information
 /// * @revision:  V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=256"))]
        pub struct DigitalMap(pub SequenceOf<ReferencePosition>);


///*
 /// * This DE indicates a direction with respect to a defined reference direction.
 /// * Example: a reference direction may be implicitly defined by the definition of a geographical zone.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `sameDirection`     - to indicate the same direction as the reference direction,
 /// * - 1 - `oppositeDirection` - to indicate opposite direction as the reference direction,
 /// * - 2 - `bothDirections`    - to indicate both directions, i.e. the same and the opposite direction,
 /// * - 3 - `unavailable`       - to indicate that the information is unavailable.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3"))]
        pub struct Direction(pub u8);


///*
 /// * This DE indicates in which direction something is moving.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `forward`     - to indicate it is moving forward,
 /// * - 1 - `backwards`   - to indicate it is moving backwards,
 /// * - 2 - `unavailable` - to indicate that the information is unavailable.
 /// *
 /// * @category: Kinematic information
 /// * @revision: editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum DriveDirection {
     Forward = 0,
                 Backward = 1,
                 Unavailable = 2,
                
}


///*
 /// * This DE indicates whether a driving lane is open to traffic.
 /// * 
 /// * A lane is counted from inside border of the road excluding the hard shoulder. The size of the bit string shall
 /// * correspond to the total number of the driving lanes in the carriageway.
 /// * 
 /// * The numbering is matched to @ref LanePosition.
 /// * The bit `0` is used to indicate the innermost lane, bit `1` is used to indicate the second lane from inside border.
 /// * 
 /// * If a lane is closed to traffic, the corresponding bit shall be set to `1`. Otherwise, it shall be set to `0`.
 /// * 
 /// * @note: hard shoulder status is not provided by this DE but in @ref HardShoulderStatus.
 /// * 
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=13"))]
        pub struct DrivingLaneStatus(pub BitString);


        
        ///* 
 /// * 
 /// * This DF represents the shape of an elliptical area or right elliptical cylinder that is centred 
 /// * on the shape's reference point defined outside of the context of this DF and oriented w.r.t. a  
 /// * cartesian coordinate system defined outside of the context of this DF. 
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field shapeReferencePoint: optional reference point which represents the centre of the ellipse, 
 /// * relative to an externally specified reference position. If this component is absent, the 
 /// * externally specified reference position represents the shape's reference point. 
 /// *
 /// * @field semiMajorAxisLength: half length of the major axis of the ellipse located in the X-Y Plane.
 /// * 
 /// * @field semiMinorAxisLength: half length of the minor axis of the ellipse located in the X-Y Plane.
 /// *
 /// * @field orientation: the optional orientation of the major axis of the ellipse, measured with 
 /// * positive values turning around the z-axis using the right-hand rule, starting from the X-axis. 
 /// * 
 /// * @field height: the optional height, present if the shape is a right elliptical cylinder extending 
 /// * in the positive Z-axis.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1, the type of the field orientation changed and the description revised in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct EllipticalShape {
            pub shape_reference_point: Option<CartesianPosition3d>,
                    pub semi_major_axis_length: StandardLength12b,
                    pub semi_minor_axis_length: StandardLength12b,
                    pub orientation: Option<CartesianAngleValue>,
                    pub height: Option<StandardLength12b>,
                    
        }

        impl EllipticalShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
	semi_major_axis_length: StandardLength12b,
	semi_minor_axis_length: StandardLength12b,
	orientation: Option<CartesianAngleValue>,
	height: Option<StandardLength12b>,
        ) -> Self {
            Self {
                shape_reference_point,
	semi_major_axis_length,
	semi_minor_axis_length,
	orientation,
	height,
            }
        }
    }

        

///*
 /// * This DE indicates whether a vehicle (e.g. public transport vehicle, truck) is under the embarkation process.
 /// * If that is the case, the value is *TRUE*, otherwise *FALSE*.
 /// *
 /// * @category: Vehicle information
 /// * @revision: editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct EmbarkationStatus(pub bool);


///*
 /// * This DE indicates the right of priority requested or assumed by an operating emergency vehicle.
 /// * The right-of-priority bit shall be set to `1` if the corresponding right is requested.
 /// * 
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * - 0 - `requestForRightOfWay`                  - when the vehicle is requesting/assuming the right of way,
 /// * - 1 - `requestForFreeCrossingAtATrafficLight` - when the vehicle is requesting/assuming the right to pass at a (red) traffic light.
 /// *
 /// * @category: Traffic information
 /// * @revision: description revised in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("2..=2"))]
        pub struct EmergencyPriority(pub BitString);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode "emergencyVehicleApproaching". 
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`                   - in case further detailed information on the emergency vehicle approaching event 
 /// *                                         is unavailable,
 /// * - 1 - `emergencyVehicleApproaching`   - in case an operating emergency vehicle is approaching,
 /// * - 2 - `prioritizedVehicleApproaching` - in case a prioritized vehicle is approaching,
 /// * - 3-255                               - reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct EmergencyVehicleApproachingSubCauseCode(pub u8);


///*
 /// * This DE indicated the type of energy being used and stored in vehicle.
 /// *
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * - 0 - `hydrogenStorage`       - when hydrogen is being used and stored in vehicle,
 /// * - 1 - `electricEnergyStorage` - when electric energy is being used and stored in vehicle,
 /// * - 2 - `liquidPropaneGas`      - when liquid Propane Gas (LPG) is being used and stored in vehicle,   
 /// * - 3 - `compressedNaturalGas ` - when compressedNaturalGas (CNG) is being used and stored in vehicle,
 /// * - 4 - `diesel`                - when diesel is being used and stored in vehicle,
 /// * - 5 - `gasoline`              - when gasoline is being used and stored in vehicle,
 /// * - 6 - `ammonia`               - when ammonia is being used and stored in vehicle.
 /// *
 /// * - Otherwise, the corresponding bit shall be set to `0`.
 /// *
 /// * @category: Vehicle information
 /// * @revision: editorial revision in V2.1.1 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("7..=7"))]
        pub struct EnergyStorageType(pub BitString);


///* 
 /// * 
 /// * This DF represents a vehicle category according to the UNECE/TRANS/WP.29/78/Rev.4 [16].
 /// * The following options are available:
 /// * 
 /// * @field euVehicleCategoryL: indicates a vehicle in the L category.
 /// *
 /// * @field euVehicleCategoryM: indicates a vehicle in the M category.
 /// *
 /// * @field euVehicleCategoryN: indicates a vehicle in the N category.
 /// *
 /// * @field euVehicleCategoryO: indicates a vehicle in the O category.
 /// *
 /// * @field euVehicleCategoryT: indicates a vehicle in the T category.
 /// *
 /// * @field euVehicleCategoryG: indicates a vehicle in the G category.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum EuVehicleCategoryCode {
   EuVehicleCategoryL(EuVehicleCategoryL),
     EuVehicleCategoryM(EuVehicleCategoryM),
     EuVehicleCategoryN(EuVehicleCategoryN),
     EuVehicleCategoryO(EuVehicleCategoryO),
     EuVehicleCategoryT(()),
     EuVehicleCategoryG(()),
    
}


///*
 /// * This DE represents one of the specific categories in the L category: L1, L2, L3, L4, L5, L6, or L7 according to UNECE/TRANS/WP.29/78/Rev.4 [16].
 /// *
 /// *
 /// * @category: Vehicle information
 /// * @revision: V2.1.1
 /// 

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


///*
 /// * This DE represents one of the specific categories in the M category: M1, M2, or M3 according to UNECE/TRANS/WP.29/78/Rev.4 [16].
 /// *
 /// *
 /// * @category: Vehicle information
 /// * @revision: V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum EuVehicleCategoryM {
     M1 = 0,
                 M2 = 1,
                 M3 = 2,
                
}


///*
 /// * This DE represents one of the specific categories in the N category: N1, N2, or N3 according to UNECE/TRANS/WP.29/78/Rev.4 [16].
 /// *
 /// *
 /// * @category: Vehicle information
 /// * @revision: V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum EuVehicleCategoryN {
     N1 = 0,
                 N2 = 1,
                 N3 = 2,
                
}


///*
 /// * This DE represents one of the specific categories in the O category: O1, O2, O3 or O4 according to UNECE/TRANS/WP.29/78/Rev.4 [16].
 /// *
 /// *
 /// * @category: Vehicle information
 /// * @revision: V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum EuVehicleCategoryO {
     O1 = 0,
                 O2 = 1,
                 O3 = 2,
                 O4 = 3,
                
}


        
        ///* 
 /// * This DF represents the Euler angles which describe the orientation of an object bounding box in a Cartesian coordinate system with an associated confidence level for each angle.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field zAngle: z-angle of object bounding box at the time of measurement, with the associated confidence.
 /// * The angle is measured with positive values considering the object orientation turning around the z-axis using the right-hand rule, starting from the x-axis. 
 /// * This extrinsic rotation shall be applied around the centre point of the object bounding box before all other rotations.
 /// *
 /// * @field yAngle: optional y-angle of object bounding box at the time of measurement, with the associated confidence.
 /// * The angle is measured with positive values considering the object orientation turning around the y-axis using the right-hand rule, starting from the z-axis. 
 /// * This extrinsic rotation shall be applied around the centre point of the object bounding box after the rotation by zAngle and before the rotation by xAngle.
 /// *
 /// * @field xAngle: optional x-angle of object bounding box at the time of measurement, with the associated confidence.
 /// * The angle is measured with positive values considering the object orientation turning around the x-axis using the right-hand rule, starting from the z-axis. 
 /// * This extrinsic rotation shall be applied around the centre point of the object bounding box after all other rotations.
 /// * 
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct EulerAnglesWithConfidence {
            pub z_angle: CartesianAngle,
                    pub y_angle: Option<CartesianAngle>,
                    pub x_angle: Option<CartesianAngle>,
                    
        }

        impl EulerAnglesWithConfidence {
        pub fn new(
            z_angle: CartesianAngle,
	y_angle: Option<CartesianAngle>,
	x_angle: Option<CartesianAngle>,
        ) -> Self {
            Self {
                z_angle,
	y_angle,
	x_angle,
            }
        }
    }

        

        
///*
 /// * The DF shall contain a list of @ref EventPoint.  
 /// *
 /// * The eventPosition of each @ref EventPoint is defined with respect to the previous @ref EventPoint in the list. 
 /// * Except for the first @ref EventPoint which is defined with respect to a position outside of the context of this DF.
 /// *
 /// * @category: GeoReference information, Traffic information
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref EventZone instead. 
 /// * @revision: Generalized the semantics in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=23"))]
        pub struct EventHistory(pub SequenceOf<EventPoint>);


        
        ///*
 /// * This DF provides information related to an event at a defined position.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field eventPosition: offset position of a detected event point to a defined position. 
 /// * 
 /// * @field eventDeltaTime: optional time travelled by the detecting ITS-S since the previous detected event point.
 /// * 
 /// * @field informationQuality: Information quality of the detection for this event point.
 /// * 
 /// * @category: GeoReference information, Traffic information
 /// * @revision: generalized the semantics in V2.1.1
 /// 

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

        

///*
 /// * The DF shall contain a list of @ref EventPoint, where all @ref EventPoint either contain the COMPONENT eventDeltaTime 
 /// * or do not contain the COMPONENT eventDeltaTime.  
 /// *
 /// * The eventPosition of each @ref EventPoint is defined with respect to the previous @ref EventPoint in the list. 
 /// * Except for the first @ref EventPoint which is defined with respect to a position outside of the context of this DF.
 /// *
 /// * @category: GeoReference information, Traffic information
 /// * @revision: created in V2.1.1 based on EventHistory
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct EventZone(pub EventHistory);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum Ext1 {
  #[rasn(value("128..=16511"),tag(context, 0))]
         Content(u16),
    #[rasn(tag(context, 1))]
         Extension(Ext2),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum Ext2 {
  #[rasn(value("16512..=2113663"),tag(context, 0))]
         Content(u32),
    #[rasn(tag(context, 1))]
         Extension(Ext3),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("2113664..=270549119", extensible))]
        pub struct Ext3(pub Integer);


///*
 /// * This DE describes the status of the exterior light switches of a vehicle incl. VRU vehicles.
 /// *
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * - 0 - `lowBeamHeadlightsOn`    - when the low beam head light switch is on,
 /// * - 1 - `highBeamHeadlightsOn`   - when the high beam head light switch is on,
 /// * - 2 - `leftTurnSignalOn`       - when the left turnSignal switch is on,
 /// * - 3 - `rightTurnSignalOn`      - when the right turn signal switch is on,
 /// * - 4 - `daytimeRunningLightsOn` - when the daytime running light switch is on,
 /// * - 5 - `reverseLightOn`         - when the reverse light switch is on,
 /// * - 6 - `fogLightOn`             - when the tail fog light switch is on,
 /// * - 7 - `parkingLightsOn`        - when the parking light switch is on.
 /// * 
 /// * @note: The value of each bit indicates the state of the switch, which commands the corresponding light.
 /// * The bit corresponding to a specific light is set to `1`, when the corresponding switch is turned on,
 /// * either manually by the driver or automatically by a vehicle system. The bit value does not indicate
 /// * if the corresponding lamps are alight or not.
 /// * 
 /// * If a vehicle is not equipped with a certain light or if the light switch status information is not available,
 /// * the corresponding bit shall be set to `0`.
 /// * 
 /// * As the bit value indicates only the state of the switch, the turn signal and hazard signal bit values shall not
 /// * alternate with the blinking interval.
 /// * 
 /// * For hazard indicator, the `leftTurnSignalOn (2)` and `rightTurnSignalOn (3)` shall be both set to `1`.
 /// * 
 /// * @category Vehicle information
 /// * @revision: Description revised in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("8..=8"))]
        pub struct ExteriorLights(pub BitString);


        
        ///*
 /// * This DF represents the top-level DF to represent a lane position. A lane position is a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway.
 /// *
 /// * @note: This DF is the most general way to represent a lane position: it provides a complete set of information regarding a transversal (dimensionless) position on the carriageway at a specific 
 /// * reference position, i.e. it provides different options and synonyms to represent the lane at which the reference position (the point) is located. A confidence is used to describe the probability 
 /// * that the object is located in the provided lane. The dimension of the object or extension of an area are not considered: See @ref OccupiedLanesWithConfidence for describing the occupation of lanes, 
 /// * where the dimensions of an object or the extension of an area is considered.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field lanePositionBased: lane position information for a defined reference position.
 /// * 
 /// * @field mapBased: optional lane position information described in the context of a MAPEM as specified in ETSI TS 103 301 [15]. 
 /// * If present, it shall describe the same reference position using the lane identification in the MAPEM. This component can be used only if a MAPEM is available for the reference position 
 /// * (e.g. on an intersection): In this case it is used as a synonym to the mandatory component lanePositionBased. 
 /// * 
 /// * @field confidence: confidence information for expressing the probability that the object is located at the indicated lane.  
 /// * If the value of the component lanePositionBased is generated directly from the absolute reference position and reference topology information, 
 /// * no sensor shall be indicated in the component usedDetectionInformation of the @ref MetaInformation.
 /// *
 /// * @category: Road Topology information
 /// * @revision: newly created in V2.2.1. The previous DF GeneralizedLanePosition is now renamed to @ref LanePositionOptions. 
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct GeneralizedLanePosition {
            pub lane_position_based: LanePositionOptions,
                    pub map_based: Option<MapPosition>,
                    pub confidence: MetaInformation,
                    
        }

        impl GeneralizedLanePosition {
        pub fn new(
            lane_position_based: LanePositionOptions,
	map_based: Option<MapPosition>,
	confidence: MetaInformation,
        ) -> Self {
            Self {
                lane_position_based,
	map_based,
	confidence,
            }
        }
    }

        

        
///*
 /// * This DF represents transversal position information w.r.t the road, at an externally defined reference position. It shall contain a set of up to `4` @ref GeneralizedLanePosition.
 /// * Multiple entries can be used to describe several lane positions with the associated confidence, in cases where the reference position cannot be mapped to a single lane.
 /// *
 /// * @category: Road Topology information
 /// * @revision: Created in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct GeneralizedLanePositions(pub SequenceOf<GeneralizedLanePosition>);


///*
 /// * This DE represents a timestamp based on TimestampIts modulo 65 536.
 /// * This means that generationDeltaTime = TimestampIts mod 65 536.
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1 based on ETSI TS 103 900 [1]
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct GenerationDeltaTime(pub u16);


        
        ///* 
 /// * This DF indicates a geographical position.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field latitude: the latitude of the geographical position.
 /// *
 /// * @field longitude: the longitude of the geographical position.
 /// *
 /// * @field altitude: the altitude of the geographical position with default value unavailable.
 /// *
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct GeoPosition {
            pub latitude: Latitude,
                    pub longitude: Longitude,
                    #[rasn(default = "geo_position_altitude_default")]
        pub altitude: AltitudeValue,
                    
        }

        impl GeoPosition {
        pub fn new(
            latitude: Latitude,
	longitude: Longitude,
	altitude: AltitudeValue,
        ) -> Self {
            Self {
                latitude,
	longitude,
	altitude,
            }
        }
    }

        fn geo_position_altitude_default() -> AltitudeValue {
                AltitudeValue::Unavailable.into()
            }
            
            

///*
 /// * This DE indicates the current status of a hard shoulder: whether it is available for special usage
 /// * (e.g. for stopping or for driving) or closed for all vehicles.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `availableForStopping` - if the hard shoulder is available for stopping in e.g. emergency situations,
 /// * - 1 - `closed`               - if the hard shoulder is closed and cannot be occupied in any case,
 /// * - 2 - `availableForDriving`  - if the hard shoulder is available for regular driving.
 /// *
 /// * @category: Traffic information
 /// * @revision: Description revised in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum HardShoulderStatus {
     AvailableForStopping = 0,
                 Closed = 1,
                 AvailableForDriving = 2,
                
}


///*
 /// * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-AnimalOnTheRoad`.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`          - in case further detailed information on the animal(s) on the road is unavailable,
 /// * - 1 - `wildAnimals`          - in case wild animals of unknown size are present on the road,
 /// * - 2 - `herdOfAnimals`        - in case a herd of animals is present on the road,
 /// * - 3 - `smallAnimals`         - in case small size animals of unknown type are present on the road,
 /// * - 4 - `largeAnimals`         - in case large size animals of unknown type are present on the road,
 /// * - 5 - `wildAnimalsSmall`     - in case small size wild animal(s) are present on the road,
 /// * - 6 - `wildAnimalsLarge`     - in case large size wild animal(s) are present on the road,
 /// * - 7 - `domesticAnimals`      - in case domestic animal(s) of unknown size are detected on the road,
 /// * - 8 - `domesticAnimalsSmall` - in case small size domestic animal(s) are present on the road,
 /// * - 9 - `domesticAnimalsLarge` - in case large size domestic animal(s) are present on the road.
 /// * - 10-255                     - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1, named values 5 to 9 added in V2.2.1 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct HazardousLocationAnimalOnTheRoadSubCauseCode(pub u8);


///*
 /// * This DE represents the sub cause code of the @ref CauseCode  `hazardousLocation-DangerousCurve`.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`                                        - in case further detailed information on the dangerous curve is unavailable,
 /// * - 1 - `dangerousLeftTurnCurve`                             - in case the dangerous curve is a left turn curve,
 /// * - 2 - `dangerousRightTurnCurve`                            - in case the dangerous curve is a right turn curve,
 /// * - 3 - `multipleCurvesStartingWithUnknownTurningDirection`  - in case of multiple curves for which the starting curve turning direction is not known,
 /// * - 4 - `multipleCurvesStartingWithLeftTurn`                 - in case of multiple curves starting with a left turn curve,
 /// * - 5 - `multipleCurvesStartingWithRightTurn`                - in case of multiple curves starting with a right turn curve.
 /// * - 6-255                                                    - are reserved for future usage.
 /// * 
 /// * The definition of whether a curve is dangerous may vary according to region and according to vehicle types/mass
 /// * and vehicle speed driving on the curve. This definition is out of scope of the present document.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct HazardousLocationDangerousCurveSubCauseCode(pub u8);


///*
 /// * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-ObstacleOnTheRoad`. 
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`    - in case further detailed information on the detected obstacle is unavailable,
 /// * - 1 - `shedLoad`       - in case detected obstacle is large amount of obstacles (shedload),
 /// * - 2 - `partsOfVehicles`- in case detected obstacles are parts of vehicles,
 /// * - 3 - `partsOfTyres`   - in case the detected obstacles are parts of tyres,
 /// * - 4 - `bigObjects`     - in case the detected obstacles are big objects,
 /// * - 5 - `fallenTrees`    - in case the detected obstacles are fallen trees,
 /// * - 6 - `hubCaps`        - in case the detected obstacles are hub caps,
 /// * - 7 - `waitingVehicles`- in case the detected obstacles are waiting vehicles.
 /// * - 8-255                - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct HazardousLocationObstacleOnTheRoadSubCauseCode(pub u8);


///*
 /// * This DE represents the value of the sub cause code of the @ref CauseCode `hazardousLocation-SurfaceCondition`. 
 /// * 
 ///The value shall be set to:
 /// * - 0  - `unavailable`     - in case further detailed information on the road surface condition is unavailable,
 /// * - 1  - `rockfalls`       - in case rock falls are detected on the road surface,
 /// * - 2  - `earthquakeDamage`- in case the road surface is damaged by earthquake,
 /// * - 3  - `sewerCollapse`   - in case of sewer collapse on the road surface,
 /// * - 4  - `subsidence`      - in case road surface is damaged by subsidence,
 /// * - 5  - `snowDrifts`      - in case road surface is damaged due to snow drift,
 /// * - 6  - `stormDamage`     - in case road surface is damaged by strong storm,
 /// * - 7  - `burstPipe`       - in case road surface is damaged due to pipe burst,
 /// * - 8  - `volcanoEruption` - in case road surface is damaged due to volcano eruption,
 /// * - 9  - `fallingIce`      - in case road surface damage is due to falling ice,
 /// * - 10 - `fire`            - in case there is fire on or near to the road surface.
 /// * - 11-255                 - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct HazardousLocationSurfaceConditionSubCauseCode(pub u8);


        
        ///*
 /// * This DF represents the Heading in a WGS84 co-ordinates system.
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field headingValue: the heading value.
 /// * 
 /// * @field headingConfidence: the confidence value of the heading value with a predefined confidence level.
 /// * 
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84Angle instead. 
 /// * @category: Kinematic Information
 /// * @revision: Description revised in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Heading {
            pub heading_value: HeadingValue,
                    pub heading_confidence: HeadingConfidence,
                    
        }

        impl Heading {
        pub fn new(
            heading_value: HeadingValue,
	heading_confidence: HeadingConfidence,
        ) -> Self {
            Self {
                heading_value,
	heading_confidence,
            }
        }
    }

        

        
        ///*
 /// * This DF  provides  information  associated to heading  change indicators  such as  a  change  of  direction.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field direction: the direction of heading change value.
 /// * 
 /// * @field actionDeltaTime: the period over which a direction change action is performed. 
 /// * 
 /// * @category: Kinematic Information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct HeadingChangeIndication {
            pub direction: TurningDirection,
                    pub action_delta_time: DeltaTimeTenthOfSecond,
                    
        }

        impl HeadingChangeIndication {
        pub fn new(
            direction: TurningDirection,
	action_delta_time: DeltaTimeTenthOfSecond,
        ) -> Self {
            Self {
                direction,
	action_delta_time,
            }
        }
    }

        

///*
 /// * This DE indicates the heading confidence value which represents the estimated absolute accuracy of a heading value with a confidence level of 95 %.
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n x 0,1 degree and more than (n-1) x 0,1 degree,
 /// * - `126` if the confidence value is out of range, i.e. greater than 12,5 degrees,
 /// * - `127` if the confidence value information is not available.
 /// * 
 /// * @note:	The fact that a value is received with confidence value set to `unavailable(127)` can be caused by several reasons,
 /// * such as:
 /// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
 /// * - the sensor cannot calculate the accuracy due to lack of variables, or
 /// * - there has been a vehicle bus (e.g. CAN bus) error.
 /// * In all 3 cases above, the heading value may be valid and used by the application.
 /// *
 /// * @note: If a heading value is received and its confidence value is set to `outOfRange(126)`, it means that the 
 /// * heading value is not valid and therefore cannot be trusted. Such value is not useful for the application.
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84AngleConfidence instead. 
 /// * 
 /// * @unit: 0,1 degree
 /// * @category: GeoReference information
 /// * @revision: Description revised in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=127"))]
        pub struct HeadingConfidence(pub u8);


///*
 /// * This DE represents the orientation of the horizontal velocity vector with regards to the WGS84 north.
 /// * When the information is not available, the DE shall be set to 3 601. The value 3600 shall not be used.
 /// *
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref Wgs84AngleValue instead. 
 /// *
 /// * Unit: 0,1 degree
 /// * Categories: GeoReference information
 /// * @revision: Description revised in V2.1.1 (usage of value 3600 specified) 
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3601"))]
        pub struct HeadingValue(pub u16);


///*
 /// * This DE represents the height of the left or right longitude carrier of vehicle from base to top (left or right carrier seen from vehicle
 /// * rear to front). 
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n >= 1` and `n < 99`) if the height information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
 /// * - `99` if the height is out of range, i.e. equal to or greater than 0,98 m,
 /// * - `100` if the height information is not available.
 /// *
 /// * @unit 0,01 metre
 /// * @category Vehicle information
 /// * @revision: Description revised in V2.1.1 (the definition of 99 has changed slightly) 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=100"))]
        pub struct HeightLonCarr(pub u8);


///*
 /// * This DE represents the value of the sub cause code of the @ref CauseCode `humanPresenceOnTheRoad`.
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`                    - in case further detailed information abou the human presence on the road is unavailable,
 /// * - 1 - `childrenOnRoadway`              - in case children are present on the road,
 /// * - 2 - `cyclistOnRoadway`               - in case cyclist(s) are present on the road,
 /// * - 3 - `motorcyclistOnRoadway`          - in case motorcyclist(s) are present on the road,
 /// * - 4 - `pedestrian`                     - in case pedestrian(s) of any type are present on the road,
 /// * - 5 - `ordinary-pedestrian`            - in case pedestrian(s) to which no more-specific profile applies are present on the road,
 /// * - 6 - `road-worker`                    - in case pedestrian(s) with the role of a road worker applies are present on the road,
 /// * - 7 - `first-responder`                - in case pedestrian(s) with the role of a first responder applies are present on the road,  
 /// * - 8 - `lightVruVehicle                 - in case light vru vehicle(s) of any type are present on the road,
 /// * - 9 - `bicyclist `                     - in case cycle(s) and their bicyclist(s) are present on the road,
 /// * - 10 - `wheelchair-user`               - in case wheelchair(s) and their user(s) are present on the road,
 /// * - 11 - `horse-and-rider`               - in case horse(s) and rider(s) are present on the road,
 /// * - 12 - `rollerskater`                  - in case rolleskater(s) and skater(s) are present on the road,
 /// * - 13 - `e-scooter`                     - in case e-scooter(s) and rider(s) are present on the road,
 /// * - 14 - `personal-transporter`          - in case personal-transporter(s) and rider(s) are present on the road,
 /// * - 15 - `pedelec`                       - in case pedelec(s) and rider(s) are present on the road,
 /// * - 16 - `speed-pedelec`                 - in case speed-pedelec(s) and rider(s) are present on the road,
 /// * - 17 - `ptw`                           - in case powered-two-wheeler(s) of any type are present on the road,
 /// * - 18 - `moped`                         - in case moped(s) and rider(s) are present on the road,
 /// * - 19 - `motorcycle`                    - in case motorcycle(s) and rider(s) are present on the road,
 /// * - 20 - `motorcycle-and-sidecar-right`  - in case motorcycle(s) with sidecar(s) on the right and rider are present on the road,
 /// * - 21 - `motorcycle-and-sidecar-left`   - in case motorcycle(s) with sidecar(s) on the left and rider are present on the road.
 /// * - 22-255                               - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: editorial revision in V2.1.1, named values 4-21 added in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct HumanPresenceOnTheRoadSubCauseCode(pub u8);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode "humanProblem".
 /// * 
 /// * The value shall be set to:
 /// * - 0 - `unavailable`    - in case further detailed information on human health problem is unavailable,
 /// * - 1 - `glycemiaProblem`- in case human problem is due to glycaemia problem,
 /// * - 2 - `heartProblem`   - in case human problem is due to heart problem.
 /// * - 3-255                - reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct HumanProblemSubCauseCode(pub u8);


///* 
 /// * This DE is a general identifier.
 /// * 
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct Identifier1B(pub u8);


///* 
 /// * This DE is a general identifier.
 /// * 
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct Identifier2B(pub u16);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `impassability`
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`              - in case further detailed information about the unmanaged road blockage is unavailable,
 /// * - 1 `flooding          `       - in case the road is affected by flooding,
 /// * - 2 `dangerOfAvalanches`       - in case the road is at risk of being affected or blocked by avalanches,
 /// * - 3 `blastingOfAvalanches`     - in case there is an active blasting of avalanches on or near the road,
 /// * - 4 `landslips`                - in case the road is affected by landslips,
 /// * - 5 `chemicalSpillage`         - in case the road is affected by chemical spillage,
 /// * - 6 `winterClosure`            - in case the road is impassable due to a winter closure.
 /// * - 7-255                        - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct ImpassabilitySubCauseCode(pub u8);


///*
 /// * This DE represents the quality level of provided information.
 /// * 
 /// * The value shall be set to:
 /// * - `0` if the information is unavailable,
 /// * - `1` if the quality level is lowest,
 /// * - `n` (`n > 1` and `n < 7`) if the quality level is n, 
 /// * - `7` if the quality level is highest.
 /// *
 /// * @note: Definition of quality level is out of scope of the present document.
 /// * @category: Basic information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=7"))]
        pub struct InformationQuality(pub u8);


        
        ///*
 /// * This DF represents a frequency channel 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field centreFrequency: the centre frequency of the channel in 10^(exp+2) Hz (where exp is exponent)
 /// * 
 /// * @field channelWidth: width of the channel in 10^exp Hz (where exp is exponent)
 /// *
 /// * @field exponent: exponent of the power of 10 used in the calculation of the components above.
 /// *
 /// * @category: Communication information
 /// * @revision: created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct InterferenceManagementChannel {
            #[rasn(value("1..=99999"))]
        pub centre_frequency: u32,
                    #[rasn(value("0..=9999"))]
        pub channel_width: u16,
                    #[rasn(value("0..=15"))]
        pub exponent: u8,
                    
        }

        impl InterferenceManagementChannel {
        pub fn new(
            centre_frequency: u32,
	channel_width: u16,
	exponent: u8,
        ) -> Self {
            Self {
                centre_frequency,
	channel_width,
	exponent,
            }
        }
    }

        

        
///*
 /// * This DF shall contain a list of up to 16 definitions containing interference management information, per affected frequency channels.
 /// *  
 /// * @category: Communication information.
 /// * @revision: created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct InterferenceManagementInfo(pub SequenceOf<InterferenceManagementInfoPerChannel>);


        
        ///*
 /// * This DF contains interference management information for one affected frequency channel.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field interferenceManagementChannel: frequency channel for which the zone should be applied interference management 
 /// *
 /// * @field interferenceManagementZoneType: type of the interference management zone. 
 /// *
 /// * @field interferenceManagementMitigationType: optional type of the mitigation to be used in the interference management zone. 
 /// * In the case where no mitigation should be applied by the ITS-S, this is indicated by the field interferenceManagementMitigationType being absent.
 /// *
 /// * @field expiryTime: optional time at which the validity of the interference management communication zone will expire. 
 /// * This component is present when the interference management is temporarily valid
 /// *
 /// * @category: Communication information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct InterferenceManagementInfoPerChannel {
            pub interference_management_channel: InterferenceManagementChannel,
                    pub interference_management_zone_type: InterferenceManagementZoneType,
                    pub interference_management_mitigation_type: Option<MitigationForTechnologies>,
                    pub expiry_time: Option<TimestampIts>,
                    
        }

        impl InterferenceManagementInfoPerChannel {
        pub fn new(
            interference_management_channel: InterferenceManagementChannel,
	interference_management_zone_type: InterferenceManagementZoneType,
	interference_management_mitigation_type: Option<MitigationForTechnologies>,
	expiry_time: Option<TimestampIts>,
        ) -> Self {
            Self {
                interference_management_channel,
	interference_management_zone_type,
	interference_management_mitigation_type,
	expiry_time,
            }
        }
    }

        

        
        ///*
 /// * 
 /// * This DF represents a zone  inside which the ITS communication should be restricted in order to manage interference.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field zoneDefinition: contains the geographical definition of the zone.
 /// *
 /// * @field managementInfo: contains interference management information applicable in the zone defined in the component zoneDefinition.
 /// *
 /// * @category: Communication information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct InterferenceManagementZone {
            pub zone_definition: InterferenceManagementZoneDefinition,
                    pub management_info: InterferenceManagementInfo,
                    
        }

        impl InterferenceManagementZone {
        pub fn new(
            zone_definition: InterferenceManagementZoneDefinition,
	management_info: InterferenceManagementInfo,
        ) -> Self {
            Self {
                zone_definition,
	management_info,
            }
        }
    }

        

        
        ///*
 /// * This DF represents the geographical definition of the zone where band sharing occurs. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field interferenceManagementZoneLatitude: Latitude of the centre point of the interference management zone.
 /// *
 /// * @field interferenceManagementZoneLongitude: Longitude of the centre point of the interference management zone.
 /// *
 /// * @field interferenceManagementZoneId: optional identification of the interference management zone. 
 /// *
 /// * @field interferenceManagementZoneShape: shape of the interference management zone placed at the centre point. 
 /// *
 /// * @category: Communication information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct InterferenceManagementZoneDefinition {
            pub interference_management_zone_latitude: Latitude,
                    pub interference_management_zone_longitude: Longitude,
                    pub interference_management_zone_id: Option<ProtectedZoneId>,
                    #[rasn(value("0.."))]
        pub interference_management_zone_shape: Option<Shape>,
                    
        }

        impl InterferenceManagementZoneDefinition {
        pub fn new(
            interference_management_zone_latitude: Latitude,
	interference_management_zone_longitude: Longitude,
	interference_management_zone_id: Option<ProtectedZoneId>,
	interference_management_zone_shape: Option<Shape>,
        ) -> Self {
            Self {
                interference_management_zone_latitude,
	interference_management_zone_longitude,
	interference_management_zone_id,
	interference_management_zone_shape,
            }
        }
    }

        

///* 
 /// * This DE defines the type of an interference management zone, so that an ITS-S can 
 /// * assert the actions to do while passing by such zone (e.g. reduce the transmit power in case of a DSRC tolling station).
 /// * It is an extension of the type @ref ProtectedZoneType.
 /// *
 /// * The value shall be set to:
 /// * - 0 - `permanentCenDsrcTolling` - as specified in ETSI TS 102 792 [14],
 /// * - 1 - `temporaryCenDsrcTolling` - as specified in ETSI TS 102 792 [14],
 /// * - 2 - `unavailable`             - default value. Set to 2 for backwards compatibility with DSRC tolling,
 /// * - 3 - `urbanRail`               - as specified in ETSI TS 103 724 [13], clause 7,
 /// * - 4 - `satelliteStation`        - as specified in ETSI TS 103 724 [13], clause 7,
 /// * - 5 - `fixedLinks`              - as specified in ETSI TS 103 724 [13], clause 7.
 /// *
 /// * @category: Communication information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
                #[non_exhaustive]
pub enum InterferenceManagementZoneType {
     PermanentCenDsrcTolling = 0,
                 TemporaryCenDsrcTolling = 1,
                 Unavailable = 2,
                 UrbanRail = 3,
                 SatelliteStation = 4,
                 FixedLinks = 5,
                
}


        
///*
 /// * This DF shall contain a list of up to 16 interference  management zones.  
 /// *
 /// * **EXAMPLE**: An interference management communication zone may be defined around a CEN DSRC road side equipment or an urban rail operational area.
 /// * 
 /// * @category: Communication information
 /// * @revision: created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16"))]
        pub struct InterferenceManagementZones(pub SequenceOf<InterferenceManagementZone>);


        
        ///*
 /// * This DF represents a unique id for an intersection, in accordance with ETSI TS 103 301 [15].
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field region: the optional identifier of the entity that is responsible for the region in which the intersection is placed.
 /// * It is the duty of that entity to guarantee that the @ref Id is unique within the region.
 /// *
 /// * @field id: the identifier of the intersection
 /// *
 /// * @note: when the component region is present, the IntersectionReferenceId is guaranteed to be globally unique.
 /// * @category: Road topology information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct IntersectionReferenceId {
            pub region: Option<Identifier2B>,
                    pub id: Identifier2B,
                    
        }

        impl IntersectionReferenceId {
        pub fn new(
            region: Option<Identifier2B>,
	id: Identifier2B,
        ) -> Self {
            Self {
                region,
	id,
            }
        }
    }

        

///*
 /// * This DE represents the vehicle type according to ISO 3833 [22].
 /// * A "term No" refers to the number of the corresponding term and its definition in ISO 3833.
 /// *
 /// * The value shall be set to:
 /// * - 0	- `passengerCar`              - term No 3.1.1
 /// * - 1	- `saloon`                    - term No 3.1.1.1 (sedan)
 /// * - 2	- `convertibleSaloon`         - term No 3.1.1.2
 /// * - 3	- `pullmanSaloon`             - term No 3.1.1.3
 /// * - 4	- `stationWagon`              - term No 3.1.1.4
 /// * - 5	- `truckStationWagon`         - term No 3.1.1.4.1
 /// * - 6	- `coupe`                     - term No 3.1.1.5 (coupe)
 /// * - 7	- `convertible`               - term No 3.1.1.6 (open tourer, roadstar, spider)
 /// * - 8	- `multipurposePassengerCar`  - term No 3.1.1.7
 /// * - 9	- `forwardControlPassengerCar`- term No 3.1.1.8
 /// * - 10	- `specialPassengerCar`       - term No 3.1.1.9
 /// * - 11	- `bus`                       - term No 3.1.2
 /// * - 12	- `minibus`                   - term No 3.1.2.1
 /// * - 13	- `urbanBus`                  - term No 3.1.2.2
 /// * - 14	- `interurbanCoach`           - term No 3.1.2.3
 /// * - 15	- `longDistanceCoach`         - term No 3.1.2.4
 /// * - 16	- `articulatedBus`            - term No 3.1.2.5
 /// * - 17	- `trolleyBus	`             - term No 3.1.2.6
 /// * - 18	- `specialBus`                - term No 3.1.2.7
 /// * - 19	- `commercialVehicle`         - term No 3.1.3
 /// * - 20	- `specialCommercialVehicle`  - term No 3.1.3.1
 /// * - 21	- `specialVehicle`            - term No 3.1.4
 /// * - 22	- `trailingTowingVehicle`     - term No 3.1.5 (draw-bar tractor)
 /// * - 23	- `semiTrailerTowingVehicle`  - term No 3.1.6 (fifth wheel tractor)
 /// * - 24	- `trailer`                   - term No 3.2.1
 /// * - 25	- `busTrailer`                - term No 3.2.1.1
 /// * - 26	- `generalPurposeTrailer`     - term No 3.2.1.2
 /// * - 27	- `caravan`                   - term No 3.2.1.3
 /// * - 28	- `specialTrailer`            - term No 3.2.1.4
 /// * - 29	- `semiTrailer`               - term No 3.2.2
 /// * - 30	- `busSemiTrailer`            - term No 3.2.2.1
 /// * - 31	- `generalPurposeSemiTrailer` - term No 3.2.2.2
 /// * - 32	- `specialSemiTrailer`        - term No 3.2.2.3
 /// * - 33	- `roadTrain`                 - term No 3.3.1
 /// * - 34	- `passengerRoadTrain`        - term No 3.3.2
 /// * - 35	- `articulatedRoadTrain`      - term No 3.3.3
 /// * - 36	- `doubleRoadTrain`           - term No 3.3.4
 /// * - 37	- `compositeRoadTrain`        - term No 3.3.5
 /// * - 38	- `specialRoadTrain`          - term No 3.3.6
 /// * - 39	- `moped`                     - term No 3.4
 /// * - 40	- `motorCycle`                - term No 3.5
 /// * - 41-255                           - reserved for future use
 /// * 
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct Iso3833VehicleType(pub u8);


///* 
 /// * This DE represent the identifier of an organization according to the applicable registry.
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.2.1 based on ISO 14816 [23]
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=16383"))]
        pub struct IssuerIdentifier(pub u16);


        
///*
 /// * This DF shall contain  a list of waypoints @ref ReferencePosition.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=40"))]
        pub struct ItineraryPath(pub SequenceOf<ReferencePosition>);


        
        ///*
 /// * This DF represents a common message header for application and facilities layer messages.
 /// * It is included at the beginning of an ITS message as the message header.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field protocolVersion: version of the ITS message.
 /// *
 /// * @field messageId: type of the ITS message.
 /// *
 /// * @field stationId: the identifier of the ITS-S that generated the ITS message.
 /// *
 /// * @category: Communication information
 /// * @revision:  update in V2.1.1: messageID and stationID changed to messageId and stationId; messageId is of type MessageId.
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ItsPduHeader {
            pub protocol_version: OrdinalNumber1B,
                    pub message_id: MessageId,
                    pub station_id: StationId,
                    
        }

        impl ItsPduHeader {
        pub fn new(
            protocol_version: OrdinalNumber1B,
	message_id: MessageId,
	station_id: StationId,
        ) -> Self {
            Self {
                protocol_version,
	message_id,
	station_id,
            }
        }
    }

        

///* 
 /// * This DE represents the identifier of the IVIM.
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.2.1 based on ETSI TS 103 301 [15]
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=32767", extensible))]
        pub struct IviIdentificationNumber(pub Integer);


        
        ///*
 /// * This DF provides the reference to the information contained in a IVIM according to ETSI TS 103 301 [15]. 
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field serviceProviderId: identifier of the organization that provided the IVIM.
 /// *
 /// * @field iviIdentificationNumber: identifier of the IVIM, as assigned by the organization identified in serviceProviderId.
 /// *
 /// * @category: Communication information
 /// * @revision: Created in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct IvimReference {
            pub service_provider_id: Provider,
                    pub ivi_identification_number: IviIdentificationNumber,
                    
        }

        impl IvimReference {
        pub fn new(
            service_provider_id: Provider,
	ivi_identification_number: IviIdentificationNumber,
        ) -> Self {
            Self {
                service_provider_id,
	ivi_identification_number,
            }
        }
    }

        

        
///*
 /// * This DF shall contain a list of @ref IvimReference.
 /// *
 /// * @category: Communication information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct IvimReferences(pub SequenceOf<IvimReference>);


///*
 /// * This DE indicates a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway. 
 /// *
 /// * For right-hand traffic roads, the value shall be set to:
 /// * - `-1` if the position is off, i.e. besides the road,
 /// * - `0` if the position is on the inner hard shoulder, i.e. the hard should adjacent to the leftmost lane,
 /// * - `n` (`n > 0` and `n < 14`), if the position is on the n-th driving lane counted from the leftmost lane to the rightmost lane of a specific traffic direction,
 /// * - `14` if the position is on the outer hard shoulder, i.e. the hard should adjacent to rightmost lane (if present).
 /// *
 /// * For left-hand traffic roads, the value shall be set to:
 /// * - `-1` if the position is off, i.e. besides the road,
 /// * - `0` if the position is on the inner hard shoulder, i.e. the hard should adjacent to the rightmost lane,
 /// * - `n` (`n > 0` and `n < 14`), if the position is on the n-th driving lane counted from the rightmost lane to the leftmost lane of a specific traffic direction,
 /// * - `14` if the position is on the outer hard shoulder, i.e. the hard should adjacent to leftmost lane (if present).
 /// *
 /// *  @note: in practice this means that the position is counted from "inside" to "outside" no matter which traffic practice is used.
 /// *
 /// * If the carriageway allows only traffic in one direction (e.g. in case of dual or multiple carriageway roads), the position is counted from the physical border of the carriageway. 
 /// * If the carriageway allows traffic in both directions and there is no physical delimitation between traffic directions (e.g. on a single carrriageway road), 
 /// * the position is counted from the legal (i.e. optical) separation between traffic directions (horizontal marking). 
 /// *
 /// * If not indicated otherwise (by lane markings or traffic signs), the legal separation on carriageways allowing traffic on both directions is identified as follows:
 /// * - If the total number of lanes N is even, the lanes are divided evenly between the traffic directions starting from the outside of the carriageway on both sides and the 
 /// *   imaginary separation between traffic directions is on the border between the even number of lanes N/2.
 /// * - If the total number of lanes N is odd, the lanes are divided evenly between traffic direction starting from the outside of the carriageway on both sides. 
 /// *   The remaining middle lane is assigned to both traffic directions as innermost lane.
 /// *
 /// * @category: Road topology information
 /// * @revision: Description of the legal separation of carriageways added in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-1..=14"))]
        pub struct LanePosition(pub i8);


        
        ///*
 /// * This DF indicates a transversal position in resolution of lanes and other associated details.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field transversalPosition: the transversal position.
 /// * 
 /// * @field laneType: the type of the lane identified in the component transversalPosition. By default set to `traffic`.
 /// *
 /// * @field direction: the traffic direction for the lane position relative to a defined reference direction. By default set to `sameDirection`, i.e. following the reference direction.
 /// *
 /// * @category Road topology information
 /// * @revision: direction added in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct LanePositionAndType {
            pub transversal_position: LanePosition,
                    #[rasn(default = "lane_position_and_type_lane_type_default")]
        pub lane_type: LaneType,
                    #[rasn(default = "lane_position_and_type_direction_default")]
        pub direction: Direction,
                    
        }

        impl LanePositionAndType {
        pub fn new(
            transversal_position: LanePosition,
	lane_type: LaneType,
	direction: Direction,
        ) -> Self {
            Self {
                transversal_position,
	lane_type,
	direction,
            }
        }
    }

        fn lane_position_and_type_lane_type_default() -> LaneType {
                LaneType::Traffic.into()
            }
            
            fn lane_position_and_type_direction_default() -> Direction {
                Direction::SameDirection.into()
            }
            
            

///*
 /// * This DF represents a set of options to describe a lane position and is the second level DF to represent a lane position. The top-level DFs are @ref GeneralizedLanePosition or @ref OccupiedLanesWithConfidence. 
 /// * A lane position is a transversal position on the carriageway at a specific longitudinal position, in resolution of lanes of the carriageway.
 /// *
 /// * The following options are available:
 /// *
 /// * @field simplelanePosition: a single lane position without any additional context information.
 /// *
 /// * @field simpleLaneType: a lane type, to be used when the lane position is unknown but the type of lane is known. This can be used in scenarios where a certain confidence about the used lane type is given 
 /// * but no or limited knowledge about the absolute lane number is available. For example, a cyclist on a cycle-lane or vehicles on a specific lane that is unique for the part of the road (e.g. a bus lane).
 /// * 
 /// * @field detailedlanePosition: a single lane position with additional lane details.
 /// * 
 /// * @field lanePositionWithLateralDetails: a single lane position with additional details and the lateral position within the lane.
 /// *
 /// * @field trafficIslandPosition: a position on a traffic island, i.e. between two lanes. 
 /// *
 /// * @category: Road Topology information
 /// * @revision: Created in V2.2.1 from the DF GeneralizedLanePosition of V2.1.1. 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum LanePositionOptions {
   SimplelanePosition(LanePosition),
     SimpleLaneType(LaneType),
     DetailedlanePosition(LanePositionAndType),
     LanePositionWithLateralDetails(LanePositionWithLateralDetails),
     TrafficIslandPosition(TrafficIslandPosition),
    
}


        
        ///*
 /// * This DF is a third-level DF that represents a lane position and is an extended version of @ref LanePositionAndType that adds the distances to the left and right lane border.
 /// *
 /// * It shall additionally include the following components: 
 /// *
 /// * @field distanceToLeftBorder: the distance of the transversal position to the left lane border. The real value shall be rounded to the next lower encoding-value.
 /// *
 /// * @field distanceToRightBorder: the distance of the transversal position to the right lane border. The real value shall be rounded to the next lower encoding-value.
 /// * 
 /// * @category: Road Topology information
 /// * @revision: Created in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct LanePositionWithLateralDetails {
            pub transversal_position: LanePosition,
                    #[rasn(default = "lane_position_with_lateral_details_lane_type_default")]
        pub lane_type: LaneType,
                    #[rasn(default = "lane_position_with_lateral_details_direction_default")]
        pub direction: Direction,
                    pub distance_to_left_border: StandardLength9b,
                    pub distance_to_right_border: StandardLength9b,
                    
        }

        impl LanePositionWithLateralDetails {
        pub fn new(
            transversal_position: LanePosition,
	lane_type: LaneType,
	direction: Direction,
	distance_to_left_border: StandardLength9b,
	distance_to_right_border: StandardLength9b,
        ) -> Self {
            Self {
                transversal_position,
	lane_type,
	direction,
	distance_to_left_border,
	distance_to_right_border,
            }
        }
    }

        fn lane_position_with_lateral_details_lane_type_default() -> LaneType {
                LaneType::Traffic.into()
            }
            
            fn lane_position_with_lateral_details_direction_default() -> Direction {
                Direction::SameDirection.into()
            }
            
            

///*
 /// * This DE represents the type of a lane. 
 /// * 
 /// * The value shall be set to:
 /// * - 0	- `traffic`            - Lane dedicated to the movement of vehicles,
 /// * - 1	- `through`            - Lane dedicated to the movement of vehicles travelling ahead and not turning,
 /// * - 2	- `reversible`         - Lane where the direction of traffic can be changed to match the peak flow,
 /// * - 3	- `acceleration`	   - Lane that allows vehicles entering a road to accelerate to the speed of through traffic before merging with it,
 /// * - 4	- `deceleration`       - Lane that allows vehicles exiting a road to decelerate before leaving it,
 /// * - 5	- `leftHandTurning`    - Lane reserved for slowing down and making a left turn, so as not to disrupt traffic,
 /// * - 6	- `rightHandTurning`   - Lane reserved for slowing down and making a right turn so as not to disrupt traffic,
 /// * - 7	- `dedicatedVehicle`   - Lane dedicated to movement of motor vehicles with specific characteristics, such as heavy goods vehicles, etc., 
 /// * - 8	- `bus`                - Lane dedicated to movement of buses providing public transport,
 /// * - 9	- `taxi`               - Lane dedicated to movement of taxis,
 /// * - 10	- `hov`                - Carpooling lane or high occupancy vehicle lane,
 /// * - 11	- `hot`                - High occupancy vehicle lanes that is allowed to be used without meeting the occupancy criteria by paying a toll,
 /// * - 12	- `pedestrian`         - Lanes dedicated to pedestrians such as pedestrian sidewalk paths,
 /// * - 13	- `cycleLane`	       - Lane dedicated to exclusive or preferred use by bicycles,
 /// * - 14	- `median`             - Lane not dedicated to movement of vehicles but representing a median / central reservation  such as the central median, 
 ///                                 separating the two directional carriageways of the highway,
 /// * - 15	- `striping`	       - Lane not dedicated to movement of vehicles but covered with roadway markings,
 /// * - 16	- `trackedVehicle`     - Lane dedicated to movement of trains, trams and trolleys,
 /// * - 17	- `parking`            - Lanes dedicated to vehicles parking, stopping and loading lanes,
 /// * - 18	- `emergency`          - Lane dedicated to vehicles in breakdown or to emergency vehicles also called hard shoulder,
 /// * - 19	- `verge`              - Lane representing the verge, i.e. a narrow strip of grass or plants and sometimes also trees located between 
 ///                                 the road surface edge and the boundary of a road,
 /// * - 20	`minimumRiskManoeuvre` - Lane dedicated to automated vehicles making a minimum risk manoeuvre,
 /// * - 21	`separatedCycleLane`   - Lane dedicated to exclusive or preferred use by bicycles that is phyisically separated from the vehicle-traffic lanes, e.g. by a verge.
 /// * - values 22 to 30             reserved for future use. 
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.1.1, named value 21 added in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=31"))]
        pub struct LaneType(pub u8);


///*
 /// * This DE represents the width of a lane measured at a defined position.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 1022`) if the lane width information is equal to or less than n x 0,01 metre and more than (n-1) x 0,01 metre,
 /// * - `1022` if the lane width is out of range, i.e. greater than 10,21 m,
 /// * - `1023` if the lane width information is not available.
 /// *
 /// * The value 0 shall not be used.
 /// *
 /// * @unit: 0,01 metre
 /// * @category: Road topology information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=1023"))]
        pub struct LaneWidth(pub u16);


        
        ///*
 /// * This DF indicates the vehicle acceleration at lateral direction and the confidence value of the lateral acceleration.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field lateralAccelerationValue: lateral acceleration value at a point in time.
 /// * 
 /// * @field lateralAccelerationConfidence: confidence value of the lateral acceleration value.
 /// *
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead.
 /// * @category Vehicle information
 /// * @revision: Description revised in V2.1.1
 /// 

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

        

///*
 /// * This DE represents the vehicle acceleration at lateral direction in the centre of the mass of the empty vehicle.
 /// * It corresponds to the vehicle coordinate system as specified in ISO 8855 [21].
 /// *
 /// * The value shall be set to:
 /// * - `-160` for acceleration values equal to or less than -16 m/s^2,
 /// * - `n` (`n > -160` and `n <= 0`) to indicate that the vehicle is accelerating towards the right side with regards to the vehicle orientation 
 /// *                            with acceleration equal to or less than n x 0,1 m/s^2 and greater than (n-1) x 0,1 m/s^2,
 /// * - `n` (`n > 0` and `n < 160`) to indicate that the vehicle is accelerating towards the left hand side with regards to the vehicle orientation 
 ///						     with acceleration equal to or less than n x 0,1 m/s^2 and greater than (n-1) x 0,1 m/s^2,
 /// * - `160` for acceleration values greater than 15,9 m/s^2,
 /// * - `161` when the data is unavailable.
 /// *
 /// * @note: the empty load vehicle is defined in ISO 1176 [8], clause 4.6.
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationValue instead.
 /// *  
 /// * @unit: 0,1 m/s^2
 /// * @category: Vehicle information
 /// * @revision: Description updated in V2.1.1 (the meaning of 160 has changed slightly). 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-160..=161"))]
        pub struct LateralAccelerationValue(pub i16);


///*
 /// * This DE represents the absolute geographical latitude in a WGS84 coordinate system, providing a range of 90 degrees in north or
 /// * in south hemisphere.
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n >= -900 000 000` and `n < 0`) x 10^-7 degree, i.e. negative values for latitudes south of the Equator,
 /// * - `0` is used for the latitude of the equator,
 /// * - `n` (`n > 0` and `n < 900 000 001`) x 10^-7 degree, i.e. positive values for latitudes north of the Equator,
 /// * - `900 000 001` when the information is unavailable.
 /// *
 /// * @unit: 10^-7 degree
 /// * @category: GeoReference information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-900000000..=900000001"))]
        pub struct Latitude(pub i32);


///*
 /// * This DE indicates the status of light bar and any sort of audible alarm system besides the horn.
 /// * This includes various common sirens as well as backup up beepers and other slow speed manoeuvring alerts.
 /// *
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * - 0 - `lightBarActivated`      - when the light bar is activated,
 /// * - 1 - `sirenActivated`         - when the siren is activated.
 /// *
 /// * Otherwise, it shall be set to 0.
 /// *
 /// * @category Vehicle information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("2..=2"))]
        pub struct LightBarSirenInUse(pub BitString);


///*
 /// * This DE represents the absolute geographical longitude in a WGS84 coordinate system, providing a range of 180 degrees
 /// * to the east or to the west of the prime meridian.
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > -1 800 000 000` and `n < 0`) x 10^-7 degree, i.e. negative values for longitudes to the west,
 /// * - `0` to indicate the prime meridian,
 /// * - `n` (`n > 0` and `n < 1 800 000 001`) x 10^-7 degree, i.e. positive values for longitudes to the east,
 /// * - `1 800 000 001` when the information is unavailable.
 /// *
 /// * The value -1 800 000 000 shall not be used. 
 /// * 
 /// * @unit: 10^-7 degree
 /// * @category: GeoReference information
 /// * @revision: Description revised in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-1800000000..=1800000001"))]
        pub struct Longitude(pub i32);


        
        ///*
 /// * This DF indicates the vehicle acceleration at longitudinal direction and the confidence value of the longitudinal acceleration.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field longitudinalAccelerationValue: longitudinal acceleration value at a point in time.
 ///
 /// * @field longitudinalAccelerationConfidence: confidence value of the longitudinal acceleration value.
 /// *
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead. 
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

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

        

///*
 /// * This DE represents the vehicle acceleration at longitudinal direction in the centre of the mass of the empty vehicle.
 /// * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11.
 /// *
 /// * The value shall be set to:
 /// * - `-160` for acceleration values equal to or less than -16 m/s^2,
 /// * - `n` (`n > -160` and `n <= 0`) to indicate that the vehicle is braking with acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2
 /// * - `n` (`n > 0` and `n < 160`) to indicate that the vehicle is accelerating with acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
 /// * - `160` for acceleration values greater than 15,9 m/s^2,
 /// * - `161` when the data is unavailable. 
 /// * 
 /// * This acceleration is along the tangent plane of the road surface and does not include gravity components.
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationValue instead.
 /// * 
 /// * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
 /// * @unit: 0,1 m/s^2
 /// * @category: Vehicle information
 /// * @revision: description revised in V2.1.1 (the meaning of 160 has changed slightly). T
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-160..=161"))]
        pub struct LongitudinalAccelerationValue(pub i16);


        
        ///* 
 /// * This DF represents the estimated position along the longitudinal extension of a carriageway or lane. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field  longitudinalLanePositionValue: the mean value of the longitudinal position along the carriageway or lane w.r.t. an externally defined start position.
 /// *
 /// * @field  longitudinalLanePositionConfidence: The confidence value associated to the value.
 /// *
 /// * @category: Road topology information
 /// * @revision: created in V2.1.1, description revised in V2.2.1
 /// 

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

        

///* 
 /// * This DE indicates the longitudinal lane position confidence value which represents the estimated accuracy of longitudinal lane position measurement with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 1 022`) if the  confidence value is equal to or less than n x 0,1 m, and more than (n-1) x 0,1 m,
 /// * - `1 022` if the confidence value is out of range i.e. greater than 102,1 m,
 /// * - `1 023` if the confidence value is unavailable.
 /// *
 /// * @unit 0,1 metre
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=1023"))]
        pub struct LongitudinalLanePositionConfidence(pub u16);


///* 
 /// * This DE represents the longitudinal offset of a map-matched position along a matched lane, beginning from the lane's starting point.
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n >= 0` and `n < 32766`) if the longitudinal offset information is equal to or less than n x 0,1 metre and more than (n-1) x 0,1 metre,
 /// * - `32 766` if the longitudinal offset is out of range, i.e. greater than 3276,5 m,
 /// * - `32 767` if the longitudinal offset information is not available. 
 /// *
 /// * @unit 0,1 metre
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=32767"))]
        pub struct LongitudinalLanePositionValue(pub u16);


        
///* 
 /// * This DF shall contain a list of a lower triangular positive semi-definite matrices.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct LowerTriangularPositiveSemidefiniteMatrices(pub SequenceOf<LowerTriangularPositiveSemidefiniteMatrix>);


        
        ///* 
 /// * This DF represents a lower triangular positive semi-definite matrix. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field componentsIncludedIntheMatrix: the indication of which components of a @ref PerceivedObject are included in the matrix. 
 /// * This component also implicitly indicates the number n of included components which defines the size (n x n) of the full correlation matrix "A".
 /// *
 /// * @field matrix: the list of cells of the lower triangular positive semi-definite matrix ordered by columns and by rows. 
 /// *
 /// * The number of columns to be included "k" is equal to the number of included components "n" indicated by componentsIncludedIntheMatrix minus 1: k = n-1.
 /// * These components shall be included in the order or their appearance in componentsIncludedIntheMatrix.
 /// * Each column "i" of the lowerTriangularCorrelationMatrixColumns contains k-(i-1) values.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct LowerTriangularPositiveSemidefiniteMatrix {
            pub components_included_inthe_matrix: MatrixIncludedComponents,
                    pub matrix: LowerTriangularPositiveSemidefiniteMatrixColumns,
                    
        }

        impl LowerTriangularPositiveSemidefiniteMatrix {
        pub fn new(
            components_included_inthe_matrix: MatrixIncludedComponents,
	matrix: LowerTriangularPositiveSemidefiniteMatrixColumns,
        ) -> Self {
            Self {
                components_included_inthe_matrix,
	matrix,
            }
        }
    }

        

        
///* 
 /// * This DF represents the columns of a lower triangular positive semi-definite matrix, each column not including the main diagonal cell of the matrix.
 /// * Given a matrix "A" of size n x n, the number of @ref CorrelationColumn to be included in the lower triangular matrix is k=n-1.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1, extension indicator added in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=13", extensible))]
        pub struct LowerTriangularPositiveSemidefiniteMatrixColumns(pub SequenceOf<CorrelationColumn>);


        
        ///*
 /// * This DF indicates a position on a topology description transmitted in a MAPEM according to ETSI TS 103 301 [15].
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field mapReference: optionally identifies the MAPEM containing the topology information.
 /// * It is absent if the MAPEM topology is known from the context.
 /// * 
 /// * @field laneId: optionally identifies the lane in the road segment or intersection topology on which the position is located.
 /// *
 /// * @field connectionId: optionally identifies the connection inside the conflict area of an intersection, i.e. it identifies a trajectory for travelling through the
 /// * conflict area of an intersection which connects e.g an ingress with an egress lane.
 /// *
 /// * @field longitudinalLanePosition: optionally indicates the longitudinal offset of the map-matched position of the object along the lane or connection measured from the start of the lane/connection, along the lane.
 /// * 
 /// * @category: Road topology information
 /// * @revision: Created in V2.1.1, definition of longitudinalLanePosition amended in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MapPosition {
            pub map_reference: Option<MapReference>,
                    pub lane_id: Option<Identifier1B>,
                    pub connection_id: Option<Identifier1B>,
                    pub longitudinal_lane_position: Option<LongitudinalLanePosition>,
                    
        }

        impl MapPosition {
        pub fn new(
            map_reference: Option<MapReference>,
	lane_id: Option<Identifier1B>,
	connection_id: Option<Identifier1B>,
	longitudinal_lane_position: Option<LongitudinalLanePosition>,
        ) -> Self {
            Self {
                map_reference,
	lane_id,
	connection_id,
	longitudinal_lane_position,
            }
        }
    }

        

///*
 /// * This DF provides the reference to the information contained in a MAPEM according to ETSI TS 103 301 [15]. 
 /// *
 /// * The following options are provided:
 /// * 
 /// * @field roadsegment: option that identifies the description of a road segment contained in a MAPEM.
 /// * 
 /// * @field intersection: option that identifies the description of an intersection contained in a MAPEM.
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum MapReference {
   Roadsegment(RoadSegmentReferenceId),
     Intersection(IntersectionReferenceId),
    
}


        
///*
 /// * This DF shall contain a list of @ref MapReference.
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct MapReferences(pub SequenceOf<MapReference>);


        
///* 
 /// * This DF provides information about the configuration of a road section in terms of MAPEM lanes or connections using a list of @ref MapemExtractedElementReference. 
 ///
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct MapemConfiguration(pub SequenceOf<MapemElementReference>);


        
///* 
 /// * This DF provides references to MAPEM connections using a list of @ref Identifier1B.
 /// * Note: connections are  allowed “maneuvers” (e.g. an ingress / egress relation) on an intersection.
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct MapemConnectionList(pub SequenceOf<Identifier1B>);


        
        ///* 
 /// * This DF provides references to an element described in a MAPEM according to ETSI TS 103 301 [i.15], such as a lane or connection at a specific intersection or road segment. 
 /// * 
 /// * It shall include the following components: 
 /// * 
 /// * @field mapReference: the optional reference to a MAPEM that describes the intersection or road segment. It is absent if the MAPEM topology is known from the context.
 /// * 
 /// * @field laneIds: the optional list of the identifiers of the lanes to be referenced. 
 /// * 
 /// * @field connectionIds: the optional list of the identifiers of the connections to be referenced. 
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MapemElementReference {
            pub map_reference: Option<MapReference>,
                    pub lane_ids: Option<MapemLaneList>,
                    pub connection_ids: Option<MapemConnectionList>,
                    
        }

        impl MapemElementReference {
        pub fn new(
            map_reference: Option<MapReference>,
	lane_ids: Option<MapemLaneList>,
	connection_ids: Option<MapemConnectionList>,
        ) -> Self {
            Self {
                map_reference,
	lane_ids,
	connection_ids,
            }
        }
    }

        

        
///* 
 /// * This DF provides references to MAPEM lanes using a list of @ref Identifier1B.
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in 2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct MapemLaneList(pub SequenceOf<Identifier1B>);


///*
 /// * This DE indicates the components of an @ref PerceivedObject that are included in the @ref LowerTriangularPositiveSemidefiniteMatrix.
 /// *
 /// * The corresponding bit shall be set to 1 if the component is included:
 /// * - 0 - `xCoordinate`                   - when the component xCoordinate of the component @ref CartesianPosition3dWithConfidence is included,
 /// * - 1 - `yCoordinate`                   - when the component yCoordinate of the component @ref CartesianPosition3dWithConfidence is included,   
 /// * - 2 - `zCoordinate`                   - when the component zCoordinate of the component @ref CartesianPosition3dWithConfidence is included, 
 /// * - 3 - `xVelocityOrVelocityMagnitude`  - when the component xVelocity of the component @ref VelocityCartesian or the component VelocityMagnitude of the component @ref VelocityPolarWithZ is included,   
 /// * - 4 - `yVelocityOrVelocityDirection`  - when the component yVelocity of the component @ref VelocityCartesian or the component VelocityDirection of the component @ref VelocityPolarWithZ is included,   
 /// * - 5 - `zVelocity`                     - when the component zVelocity of the component @ref VelocityCartesian or of the component @ref VelocityPolarWithZ is included,
 /// * - 6 - `xAccelOrAccelMagnitude`        - when the component xAcceleration of the component @ref AccelerationCartesian or the component AccelerationMagnitude of the component @ref AccelerationPolarWithZ is included,  
 /// * - 7 - `yAccelOrAccelDirection`        - when the component yAcceleration of the component @ref AccelerationCartesian or the component AccelerationDirection of the component @ref AccelerationPolarWithZ is included,   
 /// * - 8 - `zAcceleration`                 - when the component zAcceleration of the component @ref AccelerationCartesian or of the component @ref AccelerationPolarWithZ is included,
 /// * - 9 - `zAngle`                        - when the component zAngle is included,
 /// * - 10 - `yAngle`                       - when the component yAngle is included,   
 /// * - 11 - `xAngle`                       - when the component xAngle is included,   
 /// * - 12 - `zAngularVelocity`             - when the component zAngularVelocity is included.   
 /// *
 /// * Otherwise, it shall be set to 0.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("13..=13", extensible))]
        pub struct MatrixIncludedComponents(pub BitString);


///* 
 /// * This DE represents the type of facility layer message.
 /// *
 /// *  The value shall be set to:
 /// *	- 1  - `denm`              - for Decentralized Environmental Notification Message (DENM) as specified in ETSI EN 302 637-3 [2],
 /// *  - 2  - `cam`               - for Cooperative Awareness Message (CAM) as specified in ETSI EN 302 637-2 [1],
 /// *  - 3  - `poi`               - for Point of Interest message as specified in ETSI TS 101 556-1 [9],
 /// *  - 4  - `spatem`            - for Signal Phase And Timing Extended Message (SPATEM) as specified in ETSI TS 103 301 [15],
 /// *  - 5  - `mapem`             - for MAP Extended Message (MAPEM) as specified in ETSI TS 103 301 [15],
 /// *  - 6  - `ivim`              - for in Vehicle Information Message (IVIM) as specified in ETSI TS 103 301 [15],
 /// *  - 7  - `ev-rsr`            - for Electric vehicle recharging spot reservation message, as defined in ETSI TS 101 556-3 [11],
 /// *  - 8  - `tistpgtransaction` - for messages for Tyre Information System (TIS) and Tyre Pressure Gauge (TPG) interoperability, as specified in ETSI TS 101 556-2 [10],
 /// *  - 9  - `srem`              - for Signal Request Extended Message as specified in ETSI TS 103 301 [15],
 /// *  - 10 - `ssem`              - for Signal request Status Extended Message as specified in ETSI TS 103 301 [15],
 /// *  - 11 - `evcsn`             - for Electrical Vehicle Charging Spot Notification message as specified in ETSI TS 101 556-1 [9],
 /// *  - 12 - `saem`              - for Services Announcement Extended Message as specified in ETSI EN 302 890-1 [17],
 /// *  - 13 - `rtcmem`            - for Radio Technical Commission for Maritime Services Extended Message (RTCMEM) as specified in ETSI TS 103 301 [15],
 /// *  - 14 - `cpm`               - reserved for Collective Perception Message (CPM), 
 /// *  - 15 - `imzm`              - for Interference Management Zone Message (IMZM) as specified in ETSI TS 103 724 [13],
 /// *  - 16 - `vam`               - for Vulnerable Road User Awareness Message as specified in ETSI TS 130 300-3 [12], 
 /// *  - 17 - `dsm`               - reserved for Diagnosis, logging and Status Message,
 /// *  - 18 - `pcim`              - reserved for Parking Control Infrastructure Message,
 /// *  - 19 - `pcvm`              - reserved for Parking Control Vehicle Message,
 /// *  - 20 - `mcm`               - reserved for Manoeuvre Coordination Message,
 /// *  - 21 - `pam`               - reserved for Parking Availability Message,
 /// *  - 22-255                   - reserved for future usage.
 /// *
 /// * @category: Communication information
 /// * @revision: Created in V2.1.1 from @ref ItsPduHeader.
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct MessageId(pub u8);


        
        ///*
 /// * This DE indicates a message rate.
 /// *
 /// * @field mantissa: indicates the mantissa.
 /// *
 /// * @field exponent: indicates the exponent.
 /// *
 /// * The specified message rate is: mantissa*(10^exponent) 
 /// *
 /// * @unit: Hz
 /// * @category: Communication information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct MessageRateHz {
            #[rasn(value("1..=100"))]
        pub mantissa: u8,
                    #[rasn(value("-5..=2"))]
        pub exponent: i8,
                    
        }

        impl MessageRateHz {
        pub fn new(
            mantissa: u8,
	exponent: i8,
        ) -> Self {
            Self {
                mantissa,
	exponent,
            }
        }
    }

        

        
        ///*
 /// * This DF provides information about a message with respect to the segmentation process on facility layer at the sender.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field totalMsgNo: indicates the total number of messages that have been assembled on the transmitter side to encode the information 
 /// * during the same messsage generation process.
 /// *
 /// * @field thisMsgNo: indicates the position of the message within of the total set of messages generated during the same message generation process.
 /// *
 /// * @category: Communication information
 /// * @revision: Created in V2.1.1, description revised in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct MessageSegmentationInfo {
            pub total_msg_no: CardinalNumber3b,
                    pub this_msg_no: OrdinalNumber3b,
                    
        }

        impl MessageSegmentationInfo {
        pub fn new(
            total_msg_no: CardinalNumber3b,
	this_msg_no: OrdinalNumber3b,
        ) -> Self {
            Self {
                total_msg_no,
	this_msg_no,
            }
        }
    }

        

        
        ///* 
 /// * This DF provides information about the source of and confidence in information.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field usedDetectionInformation: the type of sensor(s) that is used to provide the detection information.
 /// * 
 /// * @field usedStoredInformation: the type of source of the stored information 
 /// *
 /// * @field confidenceValue: an optional confidence value associated to the information. 
 /// * 
 /// * @category: Basic information
 /// * @revision: Created in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MetaInformation {
            pub used_detection_information: SensorTypes,
                    pub used_stored_information: StoredInformationType,
                    pub confidence_value: Option<ConfidenceLevel>,
                    
        }

        impl MetaInformation {
        pub fn new(
            used_detection_information: SensorTypes,
	used_stored_information: StoredInformationType,
	confidence_value: Option<ConfidenceLevel>,
        ) -> Self {
            Self {
                used_detection_information,
	used_stored_information,
	confidence_value,
            }
        }
    }

        

        
///*
 /// * This DF shall contain a list of @ref MitigationPerTechnologyClass.
 /// *
 /// * @category: Communication information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8"))]
        pub struct MitigationForTechnologies(pub SequenceOf<MitigationPerTechnologyClass>);


        
        ///*
 /// * This DF represents a set of mitigation parameters for a specific technology, as specified in ETSI TS 103 724 [24], clause 7.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field accessTechnologyClass:  channel access technology to which this mitigation is intended to be applied.
 /// *
 /// * @field lowDutyCycle: duty cycle limit.
 /// * @unit: 0,01 % steps
 /// *
 /// * @field powerReduction: the delta value of power to be reduced.
 /// * @unit: dB
 /// *
 /// * @field dmcToffLimit: idle time limit as defined in ETSI TS 103 175 [19].
 /// * @unit: ms
 /// *
 /// * @field dmcTonLimit: Transmission duration limit, as defined in ETSI EN 302 571 [20].
 /// * @unit: ms
 /// *
 /// * @note: All parameters are optional, as they may not apply to some of the technologies or
 /// * interference management zone types. Specification details are in ETSI TS 103 724 [24], clause 7. 
 /// *
 /// * @category: Communication information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MitigationPerTechnologyClass {
            pub access_technology_class: AccessTechnologyClass,
                    #[rasn(value("0..=10000"))]
        pub low_duty_cycle: Option<u16>,
                    #[rasn(value("0..=30"))]
        pub power_reduction: Option<u8>,
                    #[rasn(value("0..=1200"))]
        pub dmc_toff_limit: Option<u16>,
                    #[rasn(value("0..=20"))]
        pub dmc_ton_limit: Option<u8>,
                    
        }

        impl MitigationPerTechnologyClass {
        pub fn new(
            access_technology_class: AccessTechnologyClass,
	low_duty_cycle: Option<u16>,
	power_reduction: Option<u8>,
	dmc_toff_limit: Option<u16>,
	dmc_ton_limit: Option<u8>,
        ) -> Self {
            Self {
                access_technology_class,
	low_duty_cycle,
	power_reduction,
	dmc_toff_limit,
	dmc_ton_limit,
            }
        }
    }

        

///*
 /// * This DE represents the number of occupants in a vehicle.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n >= 0` and `n < 126`) for the number n of occupants,
 /// * - `126` for values equal to or higher than 125,
 /// * - `127` if information is not available.
 /// *
 /// * @unit: 1 person
 /// * @category: Vehicle information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=127"))]
        pub struct NumberOfOccupants(pub u8);


///* 
 /// * This DF indicates both the class and associated subclass that best describes an object.
 /// *
 /// * The following options are available:
 /// *
 /// * @field vehicleSubClass: the object is a road vehicle and the specific subclass is specified.
 /// *
 /// * @field vruSubClass: the object is a VRU and the specific subclass is specified.
 /// *
 /// * @field groupSubClass: the object is a VRU group or cluster and the cluster information is specified.
 /// *
 /// * @field otherSubClass: the object is of a different type than the above and the specific subclass is specified.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum ObjectClass {
  #[rasn(value("0..=14"))]
         VehicleSubClass(TrafficParticipantType),
     VruSubClass(VruProfileAndSubprofile),
    #[rasn(value("0.."))]
         GroupSubClass(VruClusterInformation),
     OtherSubClass(OtherSubClass),
    
}


        
///* 
 /// * This DF shall contain a list of object classes.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8"))]
        pub struct ObjectClassDescription(pub SequenceOf<ObjectClassWithConfidence>);


        
        ///* 
 /// * This DF represents the classification of a detected object together with a confidence level.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field objectClass: the class of the object.
 /// *
 /// * @field Confidence: the associated confidence level.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ObjectClassWithConfidence {
            pub object_class: ObjectClass,
                    pub confidence: ConfidenceLevel,
                    
        }

        impl ObjectClassWithConfidence {
        pub fn new(
            object_class: ObjectClass,
	confidence: ConfidenceLevel,
        ) -> Self {
            Self {
                object_class,
	confidence,
            }
        }
    }

        

        
        ///* 
 /// * This DF represents a dimension of an object together with a confidence value.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field value: the object dimension value which can be estimated as the mean of the current distribution.
 /// *
 /// * @field confidence: the associated confidence value.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ObjectDimension {
            pub value: ObjectDimensionValue,
                    pub confidence: ObjectDimensionConfidence,
                    
        }

        impl ObjectDimension {
        pub fn new(
            value: ObjectDimensionValue,
	confidence: ObjectDimensionConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

///* 
 /// * This DE indicates the object dimension confidence value which represents the estimated absolute accuracy of an object dimension value with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 31`) if the confidence value is equal to or less than n x 0,1 metre, and more than (n-1) x 0,1 metre,
 /// * - `31` if the confidence value is out of range i.e. greater than 3,0 m,
 /// * - `32` if the confidence value is unavailable.
 /// *
 /// * @unit 0,1 m
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1 
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=32"))]
        pub struct ObjectDimensionConfidence(pub u8);


///* 
 /// * This DE represents a single dimension of an object.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 255`) if the  accuracy is equal to or less than n x 0,1 m, and more than (n-1) x 0,1 m,
 /// * - `255` if the accuracy is out of range i.e. greater than 25,4 m,
 /// * - `256` if the data is unavailable.
 /// *
 /// * @unit 0,1 m
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1 
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=256"))]
        pub struct ObjectDimensionValue(pub u16);


///*
 /// * This DE indicates the face or part of a face of a solid object.
 /// *
 /// * The object is modelled  as a rectangular prism that has a length that is greater than its width, with the faces of the object being defined as:
 /// * - front: the face defined by the prism's width and height, and which is the first face in direction of longitudinal movement of the object,
 /// * - back: the face defined by the prism's width and height, and which is the last face in direction of longitudinal movement of the object,
 /// * - side: the faces defined by the prism's length and height with "left" and "right" defined by looking at the front face and "front" and "back" defined w.r.t to the front and back faces. 
 /// *
 /// * Note: It is permissible to derive the required object dimensions and orientation from models to provide a best guess.
 /// * 
 /// * @category: Basic information
 /// * @revision: V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum ObjectFace {
     Front = 0,
                 SideLeftFront = 1,
                 SideLeftBack = 2,
                 SideRightFront = 3,
                 SideRightBack = 4,
                 Back = 5,
                
}


///* 
 /// * This DE represents a single-value indication about the overall information quality of a perceived object.
 /// * 
 /// * The value shall be set to:  
 /// * - `0`                        : if there is no confidence in detected object, e.g. for "ghost"-objects or if confidence could not be computed,
 /// * - `n` (`n > 0` and `n < 15`) : for the applicable confidence value,
 /// * - `15`                       : if there is full confidence in the detected Object.
 /// * 
 /// * @unit n/a
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1 
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct ObjectPerceptionQuality(pub u8);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct OccupiedLanesWithConfidenceLanePositionBased(pub SequenceOf<LanePositionOptions>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct OccupiedLanesWithConfidenceMapBased(pub SequenceOf<MapPosition>);

        ///*
 /// * This DF represents a set of lanes which are partially or fully occupied by an object or event at an externally defined reference position. 
 /// *
 /// * @note: In contrast to @ref GeneralizedLanePosition, the dimension of the object or event area (width and length) is taken into account to determine the occupancy, 
 /// * i.e. this DF describes the lanes which are blocked by an object or event and not the position of the object / event itself. A confidence is used to describe the 
 /// * probability that exactly all the provided lanes are occupied. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field lanePositionBased: a set of up to `4` lanes that are partially or fully occupied by an object or event, ordered by increasing value of @ref LanePosition. 
 /// * Lanes that are partially occupied can be described using the component lanePositionWithLateralDetails of @ref  Options, with the following constraints: 
 /// * The distance to lane borders which are covered by the object / event shall be set to 0. Only the distances to the leftmost and/or rightmost border which are not covered by 
 /// * the object / event shall be provided with values > 0. Those values shall be added to the respective instances of @ref LanePositionOptions, i.e. the first entry shall contain the component distanceToLeftBorder > 0 , 
 /// * and/or the last entry shall contain the component distanceToRightBorder > 0; the respective other components of these entries shall be set to 0.
 /// * 
 /// * @field mapBased: optional lane information described in the context of a MAPEM as specified in ETSI TS 103 301 [15]. 
 /// * If present, it shall describe the same lane(s) as listed in the component lanePositionBased, but using the lane identification of the MAPEM. This component can be used only if a 
 /// * MAPEM is available for the reference position (e.g. on an intersection): In this case it is used as a synonym to the mandatory component lanePositionBased. 
 /// *
 /// * @field confidence: mandatory confidence information for expressing the probability that all the provided lanes are occupied. It also provides information on how the lane 
 /// * information were generated. If none of the sensors were used, the lane information is assumed to be derived directly from the absolute reference position and the related dimension.
 /// *
 /// * @category: Road Topology information
 /// * @revision: Created in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct OccupiedLanesWithConfidence {
            pub lane_position_based: OccupiedLanesWithConfidenceLanePositionBased,
                    pub map_based: Option<OccupiedLanesWithConfidenceMapBased>,
                    pub confidence: MetaInformation,
                    
        }

        impl OccupiedLanesWithConfidence {
        pub fn new(
            lane_position_based: OccupiedLanesWithConfidenceLanePositionBased,
	map_based: Option<OccupiedLanesWithConfidenceMapBased>,
	confidence: MetaInformation,
        ) -> Self {
            Self {
                lane_position_based,
	map_based,
	confidence,
            }
        }
    }

        

///*
 /// * This DE represents a time period to describe the opening days and hours of a Point of Interest.
 /// * (for example local commerce).
 /// *
 /// * @category: Basic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct OpeningDaysHours(pub Utf8String);


///*
 /// * The DE represents an ordinal number that indicates the position of an element in a set. 
 /// * 
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct OrdinalNumber1B(pub u8);


///*
 /// * The DE represents an ordinal number that indicates the position of an element in a set. 
 /// * 
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=8"))]
        pub struct OrdinalNumber3b(pub u8);


///* 
 /// * This DE indicates the subclass of a detected object for @ref ObjectClass "otherSubclass".
 /// *
 /// * The value shall be set to:
 /// * - `0` - unknown          - if the subclass is unknown.
 /// * - `1` - singleObject     - if the object is a single object.
 /// * - `2` - multipleObjects  - if the object is a group of multiple objects.
 /// * - `3` - bulkMaterial     - if the object is a bulk material.
 /// *
 /// * @category: Sensing information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct OtherSubClass(pub u8);


        
///*
 /// * This DF represents a path with a set of path points.
 /// * It shall contain up to `40` @ref PathPoint. 
 /// * 
 /// * The first PathPoint presents an offset delta position with regards to an external reference position.
 /// * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint. 
 /// *
 /// * @category: GeoReference information, Vehicle information
 /// * @revision: created in V2.1.1 based on PathHistory
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("0..=40"))]
        pub struct Path(pub SequenceOf<PathPoint>);


///*
 /// * This DE represents the recorded or estimated travel time between a position and a predefined reference position. 
 /// *
 /// * @unit 0,01 second
 /// * @category: Basic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=65535", extensible))]
        pub struct PathDeltaTime(pub Integer);


///*
 /// * This DF represents estimated/predicted travel time between a position and a predefined reference position. 
 /// *
 /// * the following options are available:
 /// * 
 /// * @field deltaTimeHighPrecision: delta time with precision of 0,1 s.
 /// *
 /// * @field deltaTimeBigRange: delta time with precision of 10 s.
 /// *
 /// * @category: Basic information
 /// * @revision: V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum PathDeltaTimeChoice {
   DeltaTimeHighPrecision(DeltaTimeTenthOfSecond),
     DeltaTimeBigRange(DeltaTimeTenSeconds),
    
}


        
        ///* 
 /// * This DF represents a path towards a specific point specified in the @ref EventZone.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field pointOfEventZone: the ordinal number of the point within the DF EventZone, i.e. within the list of EventPoints.
 /// *
 /// * @field path: the associated path towards the point specified in pointOfEventZone.
 /// * The first PathPoint presents an offset delta position with regards to the position of that pointOfEventZone.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Created in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PathExtended {
            #[rasn(value("1..=23"))]
        pub point_of_event_zone: u8,
                    pub path: Path,
                    
        }

        impl PathExtended {
        pub fn new(
            point_of_event_zone: u8,
	path: Path,
        ) -> Self {
            Self {
                point_of_event_zone,
	path,
            }
        }
    }

        

        
///*
 /// * This DF represents a path history with a set of path points.
 /// * It shall contain up to `40` @ref PathPoint. 
 /// * 
 /// * The first PathPoint presents an offset delta position with regards to an external reference position.
 /// * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint. 
 /// *
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref Path instead.
 /// * @category: GeoReference information, Vehicle information
 /// * @revision: semantics updated in V2.1.1, size corrected to 0..40 in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("0..=40"))]
        pub struct PathHistory(pub SequenceOf<PathPoint>);


///* 
 /// * This DE indicates an ordinal number that represents the position of a component in the list of @ref Traces or @ref TracesExtended. 
 /// *
 /// * The value shall be set to:
 /// * - `0` - noPath  - if no path is identified
 /// * - `1..7`        - for instances 1..7 of @ref Traces 
 /// * - `8..14`       - for instances 1..7 of @ref TracesExtended. 
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=14"))]
        pub struct PathId(pub u8);


        
        ///*
 /// * This DF defines an offset waypoint position within a path.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field pathPosition: The waypoint position defined as an offset position with regards to a pre-defined reference position. 
 /// *
 /// * @field pathDeltaTime: The optional travel time separated from a waypoint to the predefined reference position.
 /// *
 /// * @category GeoReference information
 /// * @revision: semantics updated in V2.1.1
 /// 

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

        

        
        ///*
 /// * This DF defines a predicted offset position that can be used within a predicted path or trajectory, together with optional data to describe a path zone shape.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field deltaLatitude: the offset latitude with regards to a pre-defined reference position. 
 /// *
 /// * @field deltaLongitude: the offset longitude with regards to a pre-defined reference position. 
 /// * 
 /// * @field horizontalPositionConfidence: the optional confidence value associated to the horizontal geographical position.
 /// *
 /// * @field deltaAltitude: the optional offset altitude with regards to a pre-defined reference position, with default value unavailable. 
 /// *
 /// * @field altitudeConfidence: the optional confidence value associated to the altitude value of the geographical position, with default value unavailable.
 /// * 
 /// * @field pathDeltaTime: the optional travel time separated from the waypoint to the predefined reference position.
 ///
 /// * @field symmetricAreaOffset: the optional symmetric offset applied to the position in order to generate a shape.
 /// * if the component asymmetricAreaOffset is absent, the symmetricAreaOffset is applied to both sides of the position (left and right)
 /// * If the component asymmetricAreaOffset is present, the symmetricAreaOffset is applied to the enclosed side of a curve. 
 /// * If the curvature of the path is 0 the offset is applied to the left handside of the path. 
 /// * To determine how the lateral offset is to be applied to the path, the following steps are executed: 
 /// * - For each nth pathPointPredicted (ppp_n), vectors v_(n-) and v_(n+) from ppp_n to ppp_(n-1) and from ppp_n to ppp_(n+1) are created. 
 /// * - v^_(n-) and v^_(n+), the normalized vectors of vectors v_(n-) and v_(n+) are created.  
 /// * - v^_o the normalized sum of the vectors v^_(n-) and v^_(n+) is calculated. 
 /// * The symmetrical border offset spans a vector in the direction of both v^_o and -v^_o, if the asymmetrical border offset is ABSENT.
 /// * The symmetrical border offset spans a vector in the direction of v^_o if the asymmetrical border offset is PRESENT.
 /// * The ends of the border offsets vectors of ppp_n for each side are connected to the border offsets vectors of the endings of ppp_(n-1) and ppp_(n+1) to create the indicated zone. 
 /// * If @ref PathPredicted has only one element, the encoded area represents a circle with the asymmetricAreaOffset as the defining radius.
 /// *  
 /// * @field asymmetricAreaOffset: the optional asymmetric offset applied to the outer curve of the path in order to generate a shape. 
 /// * The asymmetrical border offset spans a vector in the direction of -v^_o
 /// *
 /// * @category GeoReference information
 /// * @revision: Created in V2.1.1, type of pathDeltaTime changed and optionality added, fields symmetricAreaOffset and asymmetricAreaOffset added in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct PathPointPredicted {
            pub delta_latitude: DeltaLatitude,
                    pub delta_longitude: DeltaLongitude,
                    pub horizontal_position_confidence: Option<PosConfidenceEllipse>,
                    #[rasn(default = "path_point_predicted_delta_altitude_default")]
        pub delta_altitude: DeltaAltitude,
                    #[rasn(default = "path_point_predicted_altitude_confidence_default")]
        pub altitude_confidence: AltitudeConfidence,
                    pub path_delta_time: PathDeltaTimeChoice,
                    pub symmetric_area_offset: Option<StandardLength9b>,
                    pub asymmetric_area_offset: Option<StandardLength9b>,
                    
        }

        impl PathPointPredicted {
        pub fn new(
            delta_latitude: DeltaLatitude,
	delta_longitude: DeltaLongitude,
	horizontal_position_confidence: Option<PosConfidenceEllipse>,
	delta_altitude: DeltaAltitude,
	altitude_confidence: AltitudeConfidence,
	path_delta_time: PathDeltaTimeChoice,
	symmetric_area_offset: Option<StandardLength9b>,
	asymmetric_area_offset: Option<StandardLength9b>,
        ) -> Self {
            Self {
                delta_latitude,
	delta_longitude,
	horizontal_position_confidence,
	delta_altitude,
	altitude_confidence,
	path_delta_time,
	symmetric_area_offset,
	asymmetric_area_offset,
            }
        }
    }

        fn path_point_predicted_delta_altitude_default() -> DeltaAltitude {
                DeltaAltitude::Unavailable.into()
            }
            
            fn path_point_predicted_altitude_confidence_default() -> AltitudeConfidence {
                AltitudeConfidence::Unavailable.into()
            }
            
            

        
///*
 /// * This DF represents a predicted path or trajectory with a set of predicted points and optional information to generate a shape which is estimated to contain the real path. 
 /// * It shall contain up to `16` @ref PathPoint. 
 /// * 
 /// * The first PathPoint presents an offset delta position with regards to an external reference position.
 /// * Each other PathPoint presents an offset delta position and optionally an offset travel time with regards to the previous PathPoint. 
 /// *
 /// * @category: GeoReference information
 /// * @revision: created in V2.1.1 , size constraint changed to SIZE(1..16) in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct PathPredicted(pub SequenceOf<PathPointPredicted>);


        
        ///* 
 /// * This DF represents a predicted path, predicted trajectory or predicted path zone together with usage information and a prediction confidence.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field pathPredicted: the predicted path (pathDeltaTime ABSENT) or trajectory (pathDeltaTime PRESENT) and/or the path zone (symmetricAreaOffset PRESENT).
 /// *
 /// * @field usageIndication: an indication of how the predicted path will be used. 
 /// *
 /// * @field confidenceLevel: the confidence that the path/trajectory in pathPredicted will occur as predicted
 /// *
 /// * @category: GeoReference information
 /// * @revision: created in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct PathPredicted2 {
            pub path_predicted: PathPredicted,
                    pub usage_indication: UsageIndication,
                    pub confidence_level: ConfidenceLevel,
                    
        }

        impl PathPredicted2 {
        pub fn new(
            path_predicted: PathPredicted,
	usage_indication: UsageIndication,
	confidence_level: ConfidenceLevel,
        ) -> Self {
            Self {
                path_predicted,
	usage_indication,
	confidence_level,
            }
        }
    }

        

        
///*
 /// * This DF represents one or more predicted paths, or trajectories or path zones (zones that include all possible paths/trajectories within its boundaries) using @ref PathPredicted2.
 /// * It shall contain up to `16` @ref PathPredicted2. 
 /// * 
 /// * @category: GeoReference information
 /// * @revision: V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct PathPredictedList(pub SequenceOf<PathPredicted2>);


        
///* 
 /// * This DF represents a list of references to the components of a @ref Traces or @ref TracesExtended DF using the @ref PathId. 
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=14"))]
        pub struct PathReferences(pub SequenceOf<PathId>);


        
        ///* 
 /// * This DF contains information about a perceived object including its kinematic state and attitude vector in a pre-defined coordinate system and with respect to a reference time.
 /// * 
 /// * It shall include the following components: 
 /// *
 /// * @field objectId: optional identifier assigned to a detected object.
 /// *
 /// * @field measurementDeltaTime: the time difference from a reference time to the time of the  measurement of the object. 
 /// * Negative values indicate that the provided object state refers to a point in time before the reference time.
 /// *
 /// * @field position: the position of the geometric centre of the object's bounding box within the pre-defined coordinate system.
 /// *
 /// * @field velocity: the velocity vector of the object within the pre-defined coordinate system.
 /// *
 /// * @field acceleration: the acceleration vector of the object within the pre-defined coordinate system.
 /// *
 /// * @field angles: optional Euler angles of the object bounding box at the time of measurement. 
 /// * 
 /// * @field zAngularVelocity: optional angular velocity of the object around the z-axis at the time of measurement.
 /// * The angular velocity is measured with positive values considering the object orientation turning around the z-axis using the right-hand rule.
 /// *
 /// * @field lowerTriangularCorrelationMatrices: optional set of lower triangular correlation matrices for selected components of the provided kinematic state and attitude vector.
 /// *
 /// * @field objectDimensionZ: optional z-dimension of object bounding box. 
 /// * This dimension shall be measured along the direction of the z-axis after all the rotations have been applied. 
 /// *
 /// * @field objectDimensionY: optional y-dimension of the object bounding box. 
 /// * This dimension shall be measured along the direction of the y-axis after all the rotations have been applied. 
 /// *
 /// * @field objectDimensionX: optional x-dimension of object bounding box.
 /// * This dimension shall be measured along the direction of the x-axis after all the rotations have been applied.
 /// * 
 /// * @field objectAge: optional age of the detected and described object, i.e. the difference in time between the moment 
 /// * it has been first detected and the reference time of the message. Value `1500` indicates that the object has been observed for more than 1.5s.
 /// *
 /// * @field objectPerceptionQuality: optional confidence associated to the object. 
 /// *
 /// * @field sensorIdList: optional list of sensor-IDs which provided the measurement data. 
 /// *
 /// * @field classification: optional classification of the described object
 /// *
 /// * @field matchedPosition: optional map-matched position of an object.
 /// *
 /// * @category Sensing information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct PerceivedObject {
            pub object_id: Option<Identifier2B>,
                    pub measurement_delta_time: DeltaTimeMilliSecondSigned,
                    pub position: CartesianPosition3dWithConfidence,
                    pub velocity: Option<Velocity3dWithConfidence>,
                    pub acceleration: Option<Acceleration3dWithConfidence>,
                    pub angles: Option<EulerAnglesWithConfidence>,
                    pub z_angular_velocity: Option<CartesianAngularVelocityComponent>,
                    pub lower_triangular_correlation_matrices: Option<LowerTriangularPositiveSemidefiniteMatrices>,
                    pub object_dimension_z: Option<ObjectDimension>,
                    pub object_dimension_y: Option<ObjectDimension>,
                    pub object_dimension_x: Option<ObjectDimension>,
                    #[rasn(value("0..=2047"))]
        pub object_age: Option<DeltaTimeMilliSecondSigned>,
                    pub object_perception_quality: Option<ObjectPerceptionQuality>,
                    pub sensor_id_list: Option<SequenceOfIdentifier1B>,
                    pub classification: Option<ObjectClassDescription>,
                    pub map_position: Option<MapPosition>,
                    
        }

        impl PerceivedObject {
        pub fn new(
            object_id: Option<Identifier2B>,
	measurement_delta_time: DeltaTimeMilliSecondSigned,
	position: CartesianPosition3dWithConfidence,
	velocity: Option<Velocity3dWithConfidence>,
	acceleration: Option<Acceleration3dWithConfidence>,
	angles: Option<EulerAnglesWithConfidence>,
	z_angular_velocity: Option<CartesianAngularVelocityComponent>,
	lower_triangular_correlation_matrices: Option<LowerTriangularPositiveSemidefiniteMatrices>,
	object_dimension_z: Option<ObjectDimension>,
	object_dimension_y: Option<ObjectDimension>,
	object_dimension_x: Option<ObjectDimension>,
	object_age: Option<DeltaTimeMilliSecondSigned>,
	object_perception_quality: Option<ObjectPerceptionQuality>,
	sensor_id_list: Option<SequenceOfIdentifier1B>,
	classification: Option<ObjectClassDescription>,
	map_position: Option<MapPosition>,
        ) -> Self {
            Self {
                object_id,
	measurement_delta_time,
	position,
	velocity,
	acceleration,
	angles,
	z_angular_velocity,
	lower_triangular_correlation_matrices,
	object_dimension_z,
	object_dimension_y,
	object_dimension_x,
	object_age,
	object_perception_quality,
	sensor_id_list,
	classification,
	map_position,
            }
        }
    }

        

///*
 /// * This DE denotes the ability of an ITS-S to provide up-to-date information.
 /// * A performance class value is used to describe age of data. The exact values are out of scope of the present document.
 /// * 
 /// *  The value shall be set to:
 /// * - `0` if  the performance class is unknown,
 /// * - `1` for performance class A as defined in ETSI TS 101 539-1 [5],
 /// * - `2` for performance class B as defined in ETSI TS 101 539-1 [5],
 /// * -  3-7 reserved for future use.
 /// *
 /// * @category: Vehicle information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=7"))]
        pub struct PerformanceClass(pub u8);


///*
 /// * This DE represents a telephone number
 /// * 
 /// * @category: Basic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16"))]
        pub struct PhoneNumber(pub NumericString);


        
        ///* 
 /// * This DF represents the shape of a polygonal area or of a right prism.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field shapeReferencePoint: the optional reference point used for the definition of the shape, relative to an externally specified reference position. 
 /// * If this component is absent, the externally specified reference position represents the shape's reference point. 
 /// *
 /// * @field polygon: the polygonal area represented by a list of minimum `3` to maximum `16` @ref CartesianPosition3d.
 /// * All nodes of the polygon shall be considered relative to the shape's reference point.
 /// *
 /// * @field height: the optional height, present if the shape is a right prism extending in the positive z-axis.
 /// * 
 /// * @category GeoReference information
 /// * @revision: created in V2.1.1
 /// *
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PolygonalShape {
            pub shape_reference_point: Option<CartesianPosition3d>,
                    #[rasn(size("3..=16", extensible))]
        pub polygon: SequenceOfCartesianPosition3d,
                    pub height: Option<StandardLength12b>,
                    
        }

        impl PolygonalShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
	polygon: SequenceOfCartesianPosition3d,
	height: Option<StandardLength12b>,
        ) -> Self {
            Self {
                shape_reference_point,
	polygon,
	height,
            }
        }
    }

        

///*
 /// * This DE indicates the perpendicular distance from the centre of mass of an empty load vehicle to the front line of
 /// * the vehicle bounding box of the empty load vehicle.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 62`) for any aplicable value n between 0,1 metre and 6,2 metres, 
 /// * - `62` for values equal to or higher than 6.1 metres,
 /// * - `63`  if the information is unavailable.
 /// * 
 /// * @note:	The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
 /// *
 /// * @unit 0,1 metre 
 /// * @category Vehicle information
 /// * @revision: description revised in V2.1.1 (the meaning of 62 has changed slightly) 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=63"))]
        pub struct PosCentMass(pub u8);


        
        ///*
 /// * This DF indicates the horizontal position confidence ellipse which represents the estimated accuracy with a 
 /// * confidence level of 95  %. The centre of the ellipse shape corresponds to the reference
 /// * position point for which the position accuracy is evaluated.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field semiMajorConfidence: half of length of the major axis, i.e. distance between the centre point
 /// * and major axis point of the position accuracy ellipse. 
 /// *
 /// * @field semiMinorConfidence: half of length of the minor axis, i.e. distance between the centre point
 /// * and minor axis point of the position accuracy ellipse. 
 /// *
 /// * @field semiMajorOrientation: orientation direction of the ellipse major axis of the position accuracy
 /// * ellipse with regards to the WGS84 north. 
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// *
 /// *
 /// * @category GeoReference information
 /// * @revision: V1.3.1
 /// 

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

        

///*
 /// * This DE indicates the perpendicular distance between the vehicle front line of the bounding box and the front wheel axle in 0,1 metre.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 19`) for any aplicable value between 0,1 metre and 1,9 metres,
 /// * - `19` for values equal to or higher than 1.8 metres,
 /// * - `20` if the information is unavailable.
 /// *
 /// * @category: Vehicle information
 /// * @unit 0,1 metre
 /// * @revision: description revised in V2.1.1 (the meaning of 19 has changed slightly) 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=20"))]
        pub struct PosFrontAx(pub u8);


///*
 /// * This DE represents the distance from the centre of vehicle front bumper to the right or left longitudinal carrier of vehicle.
 /// * The left/right carrier refers to the left/right as seen from a passenger sitting in the vehicle.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 126`) for any aplicable value between 0,01 metre and 1,26 metres, 
 /// * - `126` for values equal to or higher than 1.25 metres,
 /// * - `127` if the information is unavailable.
 /// *
 /// * @unit 0,01 metre 
 /// * @category Vehicle information
 /// * @revision: description revised in V2.1.1 (the meaning of 126 has changed slightly) 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=127"))]
        pub struct PosLonCarr(pub u8);


///*
 /// * This DE represents the perpendicular inter-distance of neighbouring pillar axis of vehicle starting from the
 /// * middle point of the front line of the vehicle bounding box.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 29`) for any aplicable value between 0,1 metre and 2,9 metres, 
 /// * - `29` for values equal to or greater than 2.8 metres,
 /// * - `30` if the information is unavailable.
 /// * 
 /// * @unit 0,1 metre 
 /// * @category Vehicle information
 /// * @revision: description revised in V2.1.1 (the meaning of 29 has changed slightly) 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=30"))]
        pub struct PosPillar(pub u8);


///* 
 /// * This DE represents a position along a single dimension such as the middle of a road or lane, measured as an offset from an externally defined starting point, 
 /// * in direction of an externally defined reference direction.
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n >= -8190` and `n < 0`) if the position is equal to or less than n x 1 metre and more than (n-1) x 1 metre, in opposite direction of the reference direction,
 /// * - `0` if the position is at the starting point,
 /// * - `n` (`n > 0` and `n < 8190`) if the position is equal to or less than n x 1 metre and more than (n-1) x 1 metre, in the same direction as the reference direction,
 /// * - `8 190` if the position is out of range, i.e. equal to or greater than 8 189 m,
 /// * - `8 191` if the position information is not available. 
 /// *
 /// * @unit 1 metre
 /// * @category: GeoReference information
 /// * @revision: Created in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-8190..=8191"))]
        pub struct Position1d(pub i16);


        
        ///*
 /// * This DF indicates the horizontal position confidence ellipse which represents the estimated accuracy with a 
 /// * confidence level of 95 %. The centre of the ellipse shape corresponds to the reference
 /// * position point for which the position accuracy is evaluated.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field semiMajorAxisLength: half of length of the major axis, i.e. distance between the centre point
 /// * and major axis point of the position accuracy ellipse. 
 /// *
 /// * @field semiMinorAxisLength: half of length of the minor axis, i.e. distance between the centre point
 /// * and minor axis point of the position accuracy ellipse. 
 /// *
 /// * @field semiMajorAxisOrientation: orientation direction of the ellipse major axis of the position accuracy
 /// * ellipse with regards to the WGS84 north. 
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// *
 /// * @category GeoReference information
 /// * @revision: created in V2.1.1 based on @ref PosConfidenceEllipse
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PositionConfidenceEllipse {
            pub semi_major_axis_length: SemiAxisLength,
                    pub semi_minor_axis_length: SemiAxisLength,
                    pub semi_major_axis_orientation: Wgs84AngleValue,
                    
        }

        impl PositionConfidenceEllipse {
        pub fn new(
            semi_major_axis_length: SemiAxisLength,
	semi_minor_axis_length: SemiAxisLength,
	semi_major_axis_orientation: Wgs84AngleValue,
        ) -> Self {
            Self {
                semi_major_axis_length,
	semi_minor_axis_length,
	semi_major_axis_orientation,
            }
        }
    }

        

///*
 /// * This DE indicates whether a passenger seat is occupied or whether the occupation status is detectable or not.
 /// * 
 /// * The number of row in vehicle seats layout is counted in rows from the driver row backwards from front to the rear
 /// * of the vehicle.
 /// * The left side seat of a row refers to the left hand side seen from vehicle rear to front.
 /// * Additionally, a bit is reserved for each seat row, to indicate if the seat occupation of a row is detectable or not,
 /// * i.e. `row1NotDetectable (3)`, `row2NotDetectable(8)`, `row3NotDetectable(13)` and `row4NotDetectable(18)`.
 /// * Finally, a bit is reserved for each row seat to indicate if the seat row is present or not in the vehicle,
 /// * i.e. `row1NotPresent (4)`, `row2NotPresent (9)`, `row3NotPresent(14)`, `row4NotPresent(19)`.
 /// * 
 /// * When a seat is detected to be occupied, the corresponding seat occupation bit shall be set to `1`.
 /// * For example, when the row 1 left seat is occupied, `row1LeftOccupied(0)` bit shall be set to `1`.
 /// * When a seat is detected to be not occupied, the corresponding seat occupation bit shall be set to `0`.
 /// * Otherwise, the value of seat occupation bit shall be set according to the following conditions:
 /// * - If the seat occupation of a seat row is not detectable, the corresponding bit shall be set to `1`.
 /// *   When any seat row not detectable bit is set to `1`, all corresponding seat occupation bits of the same row
 /// *   shall be set to `1`.
 /// * - If the seat row is not present, the corresponding not present bit of the same row shall be set to `1`.
 /// *   When any of the seat row not present bit is set to `1`, the corresponding not detectable bit for that row
 /// *   shall be set to `1`, and all the corresponding seat occupation bits in that row shall be set to `0`.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("20..=20"))]
        pub struct PositionOfOccupants(pub BitString);


        
///*
 /// * This DF shall contain a list of distances @ref PosPillar that refer to the perpendicular distance between centre of vehicle front bumper
 /// * and vehicle pillar A, between neighbour pillars until the last pillar of the vehicle.
 /// *
 /// * Vehicle pillars refer to the vertical or near vertical support of vehicle,
 /// * designated respectively as the A, B, C or D and other pillars moving in side profile view from the front to rear.
 /// * 
 /// * The first value of the DF refers to the perpendicular distance from the centre of vehicle front bumper to 
 /// * vehicle A pillar. The second value refers to the perpendicular distance from the centre position of A pillar
 /// * to the B pillar of vehicle and so on until the last pillar.
 /// *
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=3", extensible))]
        pub struct PositionOfPillars(pub SequenceOf<PosPillar>);


///*
 /// * This DE indicates the positioning technology being used to estimate a geographical position.
 /// *
 /// * The value shall be set to:
 /// * - 0 `noPositioningSolution`  - no positioning solution used,
 /// * - 1 `sGNSS`                  - Global Navigation Satellite System used,
 /// * - 2 `dGNSS`                  - Differential GNSS used,
 /// * - 3 `sGNSSplusDR`            - GNSS and dead reckoning used,
 /// * - 4 `dGNSSplusDR`            - Differential GNSS and dead reckoning used,
 /// * - 5 `dR`                     - dead reckoning used,
 /// * - 6 `manuallyByOperator`     - position set manually by a human operator.
 /// *
 /// * @category: GeoReference information
 /// * @revision: V1.3.1, extension with value 6 added in V2.2.1
 /// 

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
                #[rasn(extension_addition)]
             ManuallyByOperator = 6,
                
}


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `postCrash` .
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`                                               - in case further detailed information on post crash event is unavailable,
 /// * - 1 `accidentWithoutECallTriggered`                             - in case no eCall has been triggered for an accident,
 /// * - 2 `accidentWithECallManuallyTriggered`                        - in case eCall has been manually triggered and transmitted to eCall back end,
 /// * - 3 `accidentWithECallAutomaticallyTriggered`                   - in case eCall has been automatically triggered and transmitted to eCall back end,
 /// * - 4 `accidentWithECallTriggeredWithoutAccessToCellularNetwork`  - in case eCall has been triggered but cellular network is not accessible from triggering vehicle.
 /// * - 5-255                                                         - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct PostCrashSubCauseCode(pub u8);


///*
 ///* This DE represent the total amount of rain falling during one hour. It is measured in mm per hour at an area of 1 square metre.  
 ///* 
 ///* The following values are specified:
 ///* - `n` (`n > 0` and `n < 2000`) if the amount of rain falling is equal to or less than n x 0,1 mm/h and greater than (n-1) x 0,1 mm/h,
 ///* - `2000` if the amount of rain falling is greater than 199.9 mm/h, 
 ///* - `2001` if the information is not available.
 ///*
 ///* @unit: 0,1 mm/h 
 ///* @category: Basic Information
 ///* @revision: created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=2001"))]
        pub struct PrecipitationIntensity(pub u16);


        
        ///*
 /// * This DF describes a zone of protection inside which the ITS communication should be restricted.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field protectedZoneType: type of the protected zone. 
 /// * 
 /// * @field expiryTime: optional time at which the validity of the protected communication zone will expire.
 /// * 
 /// * @field protectedZoneLatitude: latitude of the centre point of the protected communication zone.
 /// * 
 /// * @field protectedZoneLongitude: longitude of the centre point of the protected communication zone.
 /// * 
 /// * @field protectedZoneRadius: optional radius of the protected communication zone in metres.
 /// * 
 /// * @field protectedZoneId: the optional ID of the protected communication zone.
 /// * 
 /// * @note: A protected communication zone may be defined around a CEN DSRC road side equipment.
 /// * 
 /// * @category: Infrastructure information, Communication information
 /// * @revision: revised in V2.1.1 (changed protectedZoneID to protectedZoneId)
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ProtectedCommunicationZone {
            pub protected_zone_type: ProtectedZoneType,
                    pub expiry_time: Option<TimestampIts>,
                    pub protected_zone_latitude: Latitude,
                    pub protected_zone_longitude: Longitude,
                    pub protected_zone_radius: Option<ProtectedZoneRadius>,
                    pub protected_zone_id: Option<ProtectedZoneId>,
                    
        }

        impl ProtectedCommunicationZone {
        pub fn new(
            protected_zone_type: ProtectedZoneType,
	expiry_time: Option<TimestampIts>,
	protected_zone_latitude: Latitude,
	protected_zone_longitude: Longitude,
	protected_zone_radius: Option<ProtectedZoneRadius>,
	protected_zone_id: Option<ProtectedZoneId>,
        ) -> Self {
            Self {
                protected_zone_type,
	expiry_time,
	protected_zone_latitude,
	protected_zone_longitude,
	protected_zone_radius,
	protected_zone_id,
            }
        }
    }

        

        
///*
 /// * This DF shall contain a list of @ref ProtectedCommunicationZone provided by a road side ITS-S (Road Side Unit RSU).
 /// *
 /// * It may provide up to 16 protected communication zones information.
 /// *
 /// * @category: Infrastructure information, Communication information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16"))]
        pub struct ProtectedCommunicationZonesRSU(pub SequenceOf<ProtectedCommunicationZone>);


///*
 /// * This DE represents the indentifier of a protected communication zone.
 /// * 
 /// * 
 /// * @category: Infrastructure information, Communication information
 /// * @revision: Revision in V2.1.1 (changed name from ProtectedZoneID to ProtectedZoneId)
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=134217727"))]
        pub struct ProtectedZoneId(pub u32);


///*
 /// * This DE represents the radius of a protected communication zone. 
 /// * 
 /// * 
 /// * @unit: metre
 /// * @category: Infrastructure information, Communication information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=255", extensible))]
        pub struct ProtectedZoneRadius(pub Integer);


///*
 /// * This DE indicates the type of a protected communication zone, so that an ITS-S is aware of the actions to do
 /// * while passing by such zone (e.g. reduce the transmit power in case of a DSRC tolling station).
 /// * 
 /// * The protected zone type is defined in ETSI TS 102 792 [14].
 /// * 
 /// * 
 /// * @category: Communication information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
                #[non_exhaustive]
pub enum ProtectedZoneType {
     PermanentCenDsrcTolling = 0,
                #[rasn(extension_addition)]
             TemporaryCenDsrcTolling = 1,
                
}


        
        ///*
 /// * This DF identifies an organization.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field countryCode: represents the country code that identifies the country of the national registration administrator for issuers according to ISO 14816.
 /// *
 /// * @field providerIdentifier: identifies the organization according to the national ISO 14816 register for issuers.
 /// *
 /// * @note: See https://www.itsstandards.eu/registries/register-of-nra-i-cs1/ for a list of national registration administrators and their respective registers
 /// * 
 /// * @category: Communication information
 /// * @revision: Created in V2.2.1 based on ISO 17573-3 [24]
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Provider {
            pub country_code: CountryCode,
                    pub provider_identifier: IssuerIdentifier,
                    
        }

        impl Provider {
        pub fn new(
            country_code: CountryCode,
	provider_identifier: IssuerIdentifier,
        ) -> Self {
            Self {
                country_code,
	provider_identifier,
            }
        }
    }

        

        
        ///*
 /// * This DF represents activation data for real-time systems designed for operations control, traffic light priorities, track switches, barriers, etc.
 /// * using a range of activation devices equipped in public transport vehicles.
 /// *
 /// * The activation of the corresponding equipment is triggered by the approach or passage of a public transport
 /// * vehicle at a certain point (e.g. a beacon).
 /// *
 /// * @field ptActivationType: type of activation. 
 /// *
 /// * @field ptActicationData: data of activation. 
 /// *
 /// * Today there are different payload variants defined for public transport activation-data. The R09.x is one of
 /// * the industry standard used by public transport vehicles (e.g. buses, trams) in Europe (e.g. Germany Austria)
 /// * for controlling traffic lights, barriers, bollards, etc. This DF shall include information like route, course,
 /// * destination, priority, etc.
 /// * 
 /// * The R09.x content is defined in VDV recommendation 420 [7]. It includes following information:
 /// * - Priority Request Information (pre-request, request, ready to start)
 /// * - End of Prioritization procedure
 /// * - Priority request direction
 /// * - Public Transport line number
 /// * - Priority of public transport
 /// * - Route line identifier of the public transport
 /// * - Route number identification
 /// * - Destination of public transport vehicle
 /// *
 /// * Other countries may use different message sets defined by the local administration.
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PtActivation {
            pub pt_activation_type: PtActivationType,
                    pub pt_activation_data: PtActivationData,
                    
        }

        impl PtActivation {
        pub fn new(
            pt_activation_type: PtActivationType,
	pt_activation_data: PtActivationData,
        ) -> Self {
            Self {
                pt_activation_type,
	pt_activation_data,
            }
        }
    }

        

///*
 /// * This DE is used for various tasks in the public transportation environment, especially for controlling traffic
 /// * signal systems to prioritize and speed up public transportation in urban area (e.g. intersection "_bottlenecks_").
 /// * The traffic lights may be controlled by an approaching bus or tram automatically. This permits "_In Time_" activation
 /// * of the green phase, will enable the individual traffic to clear a potential traffic jam in advance. Thereby the
 /// * approaching bus or tram may pass an intersection with activated green light without slowing down the speed due to
 /// * traffic congestion. Other usage of the DE is the provision of information like the public transport line number
 /// * or the schedule delay of a public transport vehicle.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=20"))]
        pub struct PtActivationData(pub OctetString);


///*
 /// * This DE indicates a certain coding type of the PtActivationData data.
 /// *
 /// * The folowing value are specified:
 /// * - 0 `undefinedCodingType`  : undefined coding type,
 /// * - 1 `r09-16CodingType`     : coding of PtActivationData conform to VDV recommendation 420 [7],
 /// * - 2 `vdv-50149CodingType`  : coding of PtActivationData based on VDV recommendation 420 [7].
 /// * - 3 - 255                  : reserved for alternative and future use.
 /// * 
 /// * @category: Vehicle information 
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct PtActivationType(pub u8);


        
        ///*
 /// * This DF describes a radial shape. The circular sector or cone is constructed by sweeping      
 /// * the provided range about the reference position specified outside of the context of this DF or 
 /// * about the optional shapeReferencePoint. The range is swept between a horizontal start and a 
 /// * horizontal end angle in the X-Y plane of a cartesian coordinate system specified outside of the 
 /// * context of this DF, in a right-hand positive angular direction w.r.t. the x-axis. 
 /// * A vertical opening angle in the X-Z plane may optionally be provided in a right-hand positive 
 /// * angular direction w.r.t. the x-axis. 
 /// *
 /// * It shall include the following components:
 /// * 
 /// * @field shapeReferencePoint: the optional reference point used for the definition of the shape, 
 /// * relative to an externally specified reference position. If this component is absent, the  
 /// * externally specified reference position represents the shape's reference point. 
 /// *
 /// * @field range: the radial range of the shape from the shape's reference point. 
 /// *
 /// * @field horizontalOpeningAngleStart: the start of the shape's horizontal opening angle. 
 /// *
 /// * @field horizontalOpeningAngleEnd: the end of the shape's horizontal opening angle. 
 /// *
 /// * @field verticalOpeningAngleStart: optional start of the shape's vertical opening angle. 
 /// *
 /// * @field verticalOpeningAngleEnd: optional end of the shape's vertical opening angle. 
 /// *
 /// * @category GeoReference information
 /// * @revision: created in V2.1.1, names and types of the horizontal opening angles changed, constraint added and description revised in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RadialShape {
            pub shape_reference_point: Option<CartesianPosition3d>,
                    pub range: StandardLength12b,
                    pub horizontal_opening_angle_start: CartesianAngleValue,
                    pub horizontal_opening_angle_end: CartesianAngleValue,
                    pub vertical_opening_angle_start: Option<CartesianAngleValue>,
                    pub vertical_opening_angle_end: Option<CartesianAngleValue>,
                    
        }

        impl RadialShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
	range: StandardLength12b,
	horizontal_opening_angle_start: CartesianAngleValue,
	horizontal_opening_angle_end: CartesianAngleValue,
	vertical_opening_angle_start: Option<CartesianAngleValue>,
	vertical_opening_angle_end: Option<CartesianAngleValue>,
        ) -> Self {
            Self {
                shape_reference_point,
	range,
	horizontal_opening_angle_start,
	horizontal_opening_angle_end,
	vertical_opening_angle_start,
	vertical_opening_angle_end,
            }
        }
    }

        

        
        ///*
 /// * This DF describes radial shape details. The circular sector or cone is
 /// * constructed by sweeping the provided range about the position specified outside of the  
 /// * context of this DF. The range is swept between a horizontal start and a horizontal end angle in 
 /// * the X-Y plane of a right-hand cartesian coordinate system specified outside of the context of 
 /// * this DF, in positive angular direction w.r.t. the x-axis. A vertical opening angle in the X-Z 
 /// * plane may optionally be provided in positive angular direction w.r.t. the x-axis.
 /// * 
 /// * It shall include the following components:
 /// * 
 /// * @field range: the radial range of the sensor from the reference point or sensor point offset. 
 /// *
 /// * @field horizontalOpeningAngleStart:  the start of the shape's horizontal opening angle.
 /// *
 /// * @field horizontalOpeningAngleEnd: the end of the shape's horizontal opening angle. 
 /// *
 /// * @field verticalOpeningAngleStart: optional start of the shape's vertical opening angle. 
 /// *
 /// * @field verticalOpeningAngleEnd: optional end of the shape's vertical opening angle. 
 /// *
 /// * @category: Georeference information
 /// * @revision: created in V2.1.1, description revised and constraint added in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RadialShapeDetails {
            pub range: StandardLength12b,
                    pub horizontal_opening_angle_start: CartesianAngleValue,
                    pub horizontal_opening_angle_end: CartesianAngleValue,
                    pub vertical_opening_angle_start: Option<CartesianAngleValue>,
                    pub vertical_opening_angle_end: Option<CartesianAngleValue>,
                    
        }

        impl RadialShapeDetails {
        pub fn new(
            range: StandardLength12b,
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

        

        
        ///*
 /// * This DF describes a list of radial shapes positioned w.r.t. to an offset position defined  
 /// * relative to a reference position specified outside of the context of this DF and oriented w.r.t.  
 /// * a cartesian coordinate system specified outside of the context of this DF. 
 /// *
 /// * It shall include the following components:
 /// *
 /// * @field refPointId: the identification of the reference point in case of a sensor mounted to trailer. Defaults to ITS ReferencePoint (0).
 /// * 
 /// * @field xCoordinate: the x-coordinate of the offset position.
 /// *
 /// * @field yCoordinate: the y-coordinate of the offset position.
 /// *
 /// * @field zCoordinate: the optional z-coordinate of the offset position.
 /// *
 /// * @field radialShapesList: the list of radial shape details.
 /// *
 /// * @category: Georeference information
 /// * @revision: created in V2.1.1, description revised in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RadialShapes {
            pub ref_point_id: Identifier1B,
                    pub x_coordinate: CartesianCoordinateSmall,
                    pub y_coordinate: CartesianCoordinateSmall,
                    pub z_coordinate: Option<CartesianCoordinateSmall>,
                    pub radial_shapes_list: RadialShapesList,
                    
        }

        impl RadialShapes {
        pub fn new(
            ref_point_id: Identifier1B,
	x_coordinate: CartesianCoordinateSmall,
	y_coordinate: CartesianCoordinateSmall,
	z_coordinate: Option<CartesianCoordinateSmall>,
	radial_shapes_list: RadialShapesList,
        ) -> Self {
            Self {
                ref_point_id,
	x_coordinate,
	y_coordinate,
	z_coordinate,
	radial_shapes_list,
            }
        }
    }

        

        
///* 
 /// * The DF contains a list of @ref RadialShapeDetails.
 /// *
 /// * @category: Georeference information
 /// * @revision: created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct RadialShapesList(pub SequenceOf<RadialShapeDetails>);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `railwayLevelCrossing` .
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`                   - in case no further detailed information on the railway level crossing status is available,
 /// * - 1 `doNotCrossAbnormalSituation`   - in case when something wrong is detected by equation or sensors of the railway level crossing, 
 ///                                         including level crossing is closed for too long (e.g. more than 10 minutes long ; default value),
 /// * - 2 `closed`                        - in case the crossing is closed (barriers down),
 /// * - 3 `unguarded`                     - in case the level crossing is unguarded (i.e a Saint Andrew cross level crossing without detection of train),
 /// * - 4 `nominal`                       - in case the barriers are up and lights are off.
 /// * - 5-255: reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct RailwayLevelCrossingSubCauseCode(pub u8);


        
        ///* 
 /// * This DF represents the shape of a rectangular area or a right rectangular prism that is centred 
 /// * on a reference position defined outside of the context of this DF and oriented w.r.t. a cartesian    
 /// * coordinate system defined outside of the context of this DF. 
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field shapeReferencePoint: represents an optional offset point which the rectangle is centred on with 
 /// * respect to the reference position. If this component is absent, the externally specified  
 /// * reference position represents the shape's reference point. 
 /// *
 /// * @field semiLength: represents half the length of the rectangle located in the X-Y Plane.
 /// * 
 /// * @field semiBreadth: represents half the breadth of the rectangle located in the X-Y Plane.
 /// *
 /// * @field orientation: represents the optional orientation of the length of the rectangle, 
 /// * measured with positive values turning around the Z-axis using the right-hand rule, starting from
 /// * the X-axis. 
 /// *
 /// * @field height: represents the optional height, present if the shape is a right rectangular prism 
 /// * with height extending in the positive Z-axis.
 /// *
 /// * @category GeoReference information
 /// * @revision: created in V2.1.1, centerPoint renamed to shapeReferencePoint, the type of the field orientation changed and description revised in V2.2.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RectangularShape {
            pub shape_reference_point: Option<CartesianPosition3d>,
                    pub semi_length: StandardLength12b,
                    pub semi_breadth: StandardLength12b,
                    pub orientation: Option<CartesianAngleValue>,
                    pub height: Option<StandardLength12b>,
                    
        }

        impl RectangularShape {
        pub fn new(
            shape_reference_point: Option<CartesianPosition3d>,
	semi_length: StandardLength12b,
	semi_breadth: StandardLength12b,
	orientation: Option<CartesianAngleValue>,
	height: Option<StandardLength12b>,
        ) -> Self {
            Self {
                shape_reference_point,
	semi_length,
	semi_breadth,
	orientation,
	height,
            }
        }
    }

        

        
        ///*
 /// * A position within a geographic coordinate system together with a confidence ellipse. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field latitude: the latitude of the geographical point.
 /// *
 /// * @field longitude: the longitude of the geographical point.
 /// *
 /// * @field positionConfidenceEllipse: the confidence ellipse associated to the geographical position.
 /// *
 /// * @field altitude: the altitude and an altitude accuracy of the geographical point.
 /// *
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref ReferencePositionWithConfidence instead. 
 /// * @category: GeoReference information
 /// * @revision: description updated in V2.1.1
 /// 

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

        

        
        ///*
 /// * A position within a geographic coordinate system together with a confidence ellipse. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field latitude: the latitude of the geographical point.
 /// *
 /// * @field longitude: the longitude of the geographical point.
 /// *
 /// * @field positionConfidenceEllipse: the confidence ellipse associated to the geographical position.
 /// *
 /// * @field altitude: the altitude and an altitude accuracy of the geographical point.
 /// *
 /// * @category: GeoReference information
 /// * @revision: created in V2.1.1 based on @ref ReferencePosition but using @ref PositionConfidenceEllipse.
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ReferencePositionWithConfidence {
            pub latitude: Latitude,
                    pub longitude: Longitude,
                    pub position_confidence_ellipse: PositionConfidenceEllipse,
                    pub altitude: Altitude,
                    
        }

        impl ReferencePositionWithConfidence {
        pub fn new(
            latitude: Latitude,
	longitude: Longitude,
	position_confidence_ellipse: PositionConfidenceEllipse,
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

        

///*
 /// * This DE describes a distance of relevance for information indicated in a message.
 /// *
 /// * The value shall be set to:
 /// * - 0 `lessThan50m`   - for distances below 50 m,
 /// * - 1 `lessThan100m`  - for distances below 100 m, 
 /// * - 2 `lessThan200m`  - for distances below 200 m, 
 /// * - 3 `lessThan500m`  - for distances below 300 m, 
 /// * - 4 `lessThan1000m` - for distances below 1 000 m, 
 /// * - 5 `lessThan5km`   - for distances below 5 000 m, 
 /// * - 6 `lessThan10km`  - for distances below 10 000 m, 
 /// * - 7 `over10km`      - for distances over 10 000 m. 
 /// * 
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref StandardLength3b instead. 
 /// *
 /// * @category: GeoReference information
 /// * @revision: Editorial update in V2.1.1
 /// 

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


///*
 /// * This DE indicates a traffic direction that is relevant to information indicated in a message.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `allTrafficDirections` - for all traffic directions, 
 /// * - 1 `upstreamTraffic`      - for upstream traffic, 
 /// * - 2 `downstreamTraffic`    - for downstream traffic, 
 /// * - 3 `oppositeTraffic`      - for traffic in the opposite direction. 
 /// *
 /// * The terms `upstream`, `downstream` and `oppositeTraffic` are relative to the event position.
 /// *
 /// * @note: Upstream traffic corresponds to the incoming traffic towards the event position,
 /// * and downstream traffic to the departing traffic away from the event position.
 /// *
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref TrafficDirection instead. 
 /// *
 /// * @category: GeoReference information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum RelevanceTrafficDirection {
     AllTrafficDirections = 0,
                 UpstreamTraffic = 1,
                 DownstreamTraffic = 2,
                 OppositeTraffic = 3,
                
}


///*
 /// * This DE indicates whether an ITS message is transmitted as request from ITS-S or a response transmitted from
 /// * ITS-S after receiving request from other ITS-Ss.
 /// *
 /// * The value shall be set to:
 /// * - 0 `request`  - for a request message, 
 /// * - 1 `response` - for a response message.  
 /// *
 /// * @category Communication information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum RequestResponseIndication {
     Request = 0,
                 Response = 1,
                
}


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `rescueAndRecoveryWorkInProgress` 
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`              - in case further detailed information on rescue and recovery work is unavailable,
 /// * - 1 `emergencyVehicles`        - in case rescue and/or safeguarding work is ongoing by emergency vehicles, i.e. by vehicles that have the absolute right of way,
 /// * - 2 `rescueHelicopterLanding`  - in case rescue helicopter is landing,
 /// * - 3 `policeActivityOngoing`    - in case police activity is ongoing (only to be used if a more specific sub cause than (1) is needed),
 /// * - 4 `medicalEmergencyOngoing`  - in case medical emergency recovery is ongoing (only to be used if a more specific sub cause than (1) is needed),
 /// * - 5 `childAbductionInProgress` - in case a child kidnapping alarm is activated and rescue work is ongoing (only to be used if a more specific sub cause than (1) is needed),
 /// * - 6 `prioritizedVehicle`       - in case rescue and/or safeguarding work is ongoing by prioritized vehicles, i.e. by vehicles that have priority but not the absolute right of way,
 /// * - 7 `rescueAndRecoveryVehicle` - in case technical rescue work is ongoing by rescue and recovery vehicles.
 /// * - 8-255: reserved for future usage.
 ///
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1, named values 6 and 7 added in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct RescueAndRecoveryWorkInProgressSubCauseCode(pub u8);


        
///*
 /// * This DF shall contain a list of @ref StationType. to which a certain traffic restriction, e.g. the speed limit, applies.
 /// * 
 /// * @category: Infrastructure information, Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=3", extensible))]
        pub struct RestrictedTypes(pub SequenceOf<StationType>);


        
        ///* 
 /// * This DF provides configuration information about a road section.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field roadSectionDefinition: the topological definition of the road section for which the information in the other components applies throughout its entire length.
 /// * 
 /// * @field roadType: the optional type of road on which the section is located.
 /// * 
 /// * @field laneConfiguration: the optional configuration of the road section in terms of basic information per lane.
 /// *
 /// * @field mapemConfiguration: the optional configuration of the road section in terms of MAPEM lanes or connections.
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RoadConfigurationSection {
            pub road_section_definition: RoadSectionDefinition,
                    pub road_type: Option<RoadType>,
                    pub lane_configuration: Option<BasicLaneConfiguration>,
                    pub mapem_configuration: Option<MapemConfiguration>,
                    
        }

        impl RoadConfigurationSection {
        pub fn new(
            road_section_definition: RoadSectionDefinition,
	road_type: Option<RoadType>,
	lane_configuration: Option<BasicLaneConfiguration>,
	mapem_configuration: Option<MapemConfiguration>,
        ) -> Self {
            Self {
                road_section_definition,
	road_type,
	lane_configuration,
	mapem_configuration,
            }
        }
    }

        

        
///*
 /// * This DF shall contain a list of @ref RoadConfigurationSection.
 /// * 
 /// * @category: Road Topology information
 /// * @revision: Created in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct RoadConfigurationSectionList(pub SequenceOf<RoadConfigurationSection>);


        
        ///* 
 /// * This DF provides the basic topological definition of a road section.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field startingPointSection: the position of the starting point of the section. 
 /// * 
 /// * @field lengthOfSection: the optional length of the section along the road profile (i.e. including curves).
 /// * 
 /// * @field endingPointSection: the optional position of the ending point of the section. 
 /// * If this component is absent, the ending position is implicitly defined by other means, e.g. the starting point of the next RoadConfigurationSection, or the section’s length.
 /// *
 /// * @field connectedPaths: the identifier(s) of the path(s) having one or an ordered subset of waypoints located upstream of the RoadConfigurationSection’ starting point. 
 /// * 
 /// * @field includedPaths: the identifier(s) of the trace(s) that covers (either with all its length or with a part of it) a RoadConfigurationSection. 
 /// *
 /// * @field isEventZoneIncluded: indicates, if set to TRUE, that the @ref EventZone incl. its reference position covers a RoadConfigurationSection (either with all its length or with a part of it). 
 /// * 
 /// * @field isEventZoneConnected: indicates, if set to TRUE, that the @ref EventZone incl. its reference position has one or an ordered subset of waypoints located upstream of the RoadConfigurationSection’ starting point.
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RoadSectionDefinition {
            pub starting_point_section: GeoPosition,
                    pub length_of_section: Option<StandardLength2B>,
                    pub ending_point_section: Option<GeoPosition>,
                    pub connected_paths: PathReferences,
                    pub included_paths: PathReferences,
                    pub is_event_zone_included: bool,
                    pub is_event_zone_connected: bool,
                    
        }

        impl RoadSectionDefinition {
        pub fn new(
            starting_point_section: GeoPosition,
	length_of_section: Option<StandardLength2B>,
	ending_point_section: Option<GeoPosition>,
	connected_paths: PathReferences,
	included_paths: PathReferences,
	is_event_zone_included: bool,
	is_event_zone_connected: bool,
        ) -> Self {
            Self {
                starting_point_section,
	length_of_section,
	ending_point_section,
	connected_paths,
	included_paths,
	is_event_zone_included,
	is_event_zone_connected,
            }
        }
    }

        

///* 
 /// * This DE indicates an ordinal number that represents the position of a component in the list @ref RoadConfigurationSectionList. 
 /// *
 /// * The value shall be set to:
 /// * - `0`     - if no road section is identified
 /// * - `1..8`  - for instances 1..8 of @ref RoadConfigurationSectionList 
 /// *
 /// * @category: Road topology information
 /// * @revision: Created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=8", extensible))]
        pub struct RoadSectionId(pub Integer);


        
        ///*
 /// * This DF represents a unique id for a road segment
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field region: the optional identifier of the entity that is responsible for the region in which the road segment is placed.
 /// * It is the duty of that entity to guarantee that the @ref Id is unique within the region.
 /// *
 /// * @field id: the identifier of the road segment.
 /// *
 /// * @note: when the component region is present, the RoadSegmentReferenceId is guaranteed to be globally unique.
 /// * @category: GeoReference information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RoadSegmentReferenceId {
            pub region: Option<Identifier2B>,
                    pub id: Identifier2B,
                    
        }

        impl RoadSegmentReferenceId {
        pub fn new(
            region: Option<Identifier2B>,
	id: Identifier2B,
        ) -> Self {
            Self {
                region,
	id,
            }
        }
    }

        

///*
 /// * This DE indicates the type of a road segment.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `urban-NoStructuralSeparationToOppositeLanes`       - for an urban road with no structural separation between lanes carrying traffic in opposite directions,
 /// * - 1 `urban-WithStructuralSeparationToOppositeLanes`     - for an urban road with structural separation between lanes carrying traffic in opposite directions,
 /// * - 2 `nonUrban-NoStructuralSeparationToOppositeLanes`    - for an non urban road with no structural separation between lanes carrying traffic in opposite directions,
 /// * - 3 `nonUrban-WithStructuralSeparationToOppositeLanes`  - for an non urban road with structural separation between lanes carrying traffic in opposite directions.
 /// *
 /// * @category: Road Topology Information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum RoadType {
     UrbanNoStructuralSeparationToOppositeLanes = 0,
                 UrbanWithStructuralSeparationToOppositeLanes = 1,
                 NonUrbanNoStructuralSeparationToOppositeLanes = 2,
                 NonUrbanWithStructuralSeparationToOppositeLanes = 3,
                
}


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `roadworks`.
 /// * 
 ///The value shall be set to:
 /// * - 0 `unavailable`                 - in case further detailed information on roadworks is unavailable,
 /// * - 1 `majorRoadworks`              - in case a major roadworks is ongoing,
 /// * - 2 `roadMarkingWork`             - in case a road marking work is ongoing,
 /// * - 3 `slowMovingRoadMaintenance`   - in case slow moving road maintenance work is ongoing,
 /// * - 4 `shortTermStationaryRoadworks`- in case a short term stationary roadwork is ongoing,
 /// * - 5 `streetCleaning`              - in case a vehicle street cleaning work is ongoing,
 /// * - 6 `winterService`               - in case winter service work is ongoing.
 /// * - 7-255                           - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct RoadworksSubCauseCode(pub u8);


        
        ///*
 /// * This DF provides the safe distance indication of a traffic participant with other traffic participant(s).
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field subjectStation: optionally indicates one "other" traffic participant identified by its ITS-S.
 /// *  
 /// * @field safeDistanceIndicator: indicates whether the distance between the ego ITS-S and the traffic participant(s) is safe.
 /// * If subjectStation is present then it indicates whether the distance between the ego ITS-S and the traffic participant indicated in the component subjectStation is safe. 
 /// *
 /// * @field timeToCollision: optionally indicated the time-to-collision calculated as sqrt(LaDi^2 + LoDi^2 + VDi^2/relative speed 
 /// * and represented in  the  nearest 100  ms. This component may be present only if subjectStation is present. 
 /// *
 /// * @note: the abbreviations used are Lateral Distance (LaD),  Longitudinal Distance (LoD) and Vertical Distance (VD) 
 /// * and their respective  thresholds, Minimum  Safe  Lateral  Distance (MSLaD), Minimum  Safe  Longitudinal Distance  (MSLoD),  and  Minimum  Safe Vertical Distance (MSVD).
 /// *
 /// * @category: Traffic information, Kinematic information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SafeDistanceIndication {
            pub subject_station: Option<StationId>,
                    pub safe_distance_indicator: SafeDistanceIndicator,
                    pub time_to_collision: Option<DeltaTimeTenthOfSecond>,
                    
        }

        impl SafeDistanceIndication {
        pub fn new(
            subject_station: Option<StationId>,
	safe_distance_indicator: SafeDistanceIndicator,
	time_to_collision: Option<DeltaTimeTenthOfSecond>,
        ) -> Self {
            Self {
                subject_station,
	safe_distance_indicator,
	time_to_collision,
            }
        }
    }

        

///*
 /// * This DE indicates if a distance is safe. 
 /// *
 /// * The value shall be set to:
 /// * -  `FALSE`  if  the triple  {LaD,  LoD, VD} < {MSLaD, MSLoD, MSVD} is simultaneously  satisfied with confidence level of  90 % or  more, 
 /// * -  `TRUE` otherwise. 
 /// *
 /// * @note: the abbreviations used are Lateral Distance (LaD),  Longitudinal Distance (LoD) and Vertical Distance (VD) 
 /// * and their respective  thresholds, Minimum  Safe  Lateral  Distance (MSLaD), Minimum  Safe  Longitudinal Distance  (MSLoD),  and  Minimum  Safe Vertical Distance (MSVD).
 /// *
 /// * @category: Traffic information, Kinematic information
 /// * @revision: created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct SafeDistanceIndicator(pub bool);


///*
 /// * This DE indicates the horizontal position confidence value which represents the estimated absolute position accuracy, in one of the axis direction as defined in a shape of ellipse with a 
 /// * confidence level of 95 %. 
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 4 094`) if the accuracy is equal to or less than n * 0,01 metre,
 /// * - `4 094` if the accuracy is out of range, i.e. greater than 4,093 m,
 /// * - `4 095` if the accuracy information is unavailable.
 /// *
 /// * The value 0 shall not be used.
 /// * 
 /// * @note: The fact that a position coordinate value is received with confidence value set to `unavailable(4095)`.
 /// * can be caused by several reasons, such as:
 /// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
 /// * - the sensor cannot calculate the accuracy due to lack of variables, or
 /// * - there has been a vehicle bus (e.g. CAN bus) error.
 /// * In all 3 cases above, the position coordinate value may be valid and used by the application.
 /// * If a position coordinate value is received and its confidence value is set to `outOfRange(4094)`, it means that
 /// * the position coordinate value is not valid and therefore cannot be trusted. Such value is not useful
 /// * for the application.
 ///
 /// * @unit 0,01 metre 
 /// * @category: GeoReference Information
 /// * @revision: Description revised in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=4095"))]
        pub struct SemiAxisLength(pub u16);


///* 
 /// * This DE indicates the type of sensor.
 /// * 
 /// * The value shall be set to:
 /// * - 0  `undefined`         - in case the sensor type is undefined. 
 /// * - 1  `radar`             - in case the sensor is a radar,
 /// * - 2  `lidar`             - in case the sensor is a lidar,
 /// * - 3  `monovideo`         - in case the sensor is mono video,
 /// * - 4  `stereovision`      - in case the sensor is stereo vision,
 /// * - 5  `nightvision`       - in case the sensor is night vision,
 /// * - 6  `ultrasonic`        - in case the sensor is ultrasonic,
 /// * - 7  `pmd`               - in case the sensor is photonic mixing device,
 /// * - 8  `inductionLoop`     - in case the sensor is an induction loop,
 /// * - 9  `sphericalCamera`   - in case the sensor is a spherical camera,
 /// * - 10 `uwb`               - in case the sensor is ultra wide band,
 /// * - 11 `acoustic`          - in case the sensor is acoustic,
 /// * - 12 `localAggregation`  - in case the information is provided by a system that aggregates information from different local sensors. Aggregation may include fusion,
 /// * - 13 `itsAggregation`    - in case the information is provided by a system that aggregates information from other received ITS messages.
 /// * - 14-31                  - are reserved for future usage.
 /// *
 /// * @category: Sensing Information
 /// * @revision: created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=31"))]
        pub struct SensorType(pub u8);


///* 
 /// * This DE indicates the type of sensor(s).
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * 
 /// * - 0  `undefined`         - in case the sensor type is undefined. 
 /// * - 1  `radar`             - in case the sensor is a radar,
 /// * - 2  `lidar`             - in case the sensor is a lidar,
 /// * - 3  `monovideo`         - in case the sensor is mono video,
 /// * - 4  `stereovision`      - in case the sensor is stereo vision,
 /// * - 5  `nightvision`       - in case the sensor is night vision,
 /// * - 6  `ultrasonic`        - in case the sensor is ultrasonic,
 /// * - 7  `pmd`               - in case the sensor is photonic mixing device,
 /// * - 8  `inductionLoop`     - in case the sensor is an induction loop,
 /// * - 9  `sphericalCamera`   - in case the sensor is a spherical camera,
 /// * - 10 `uwb`               - in case the sensor is ultra wide band,
 /// * - 11 `acoustic`          - in case the sensor is acoustic,
 /// * - 12 `localAggregation`  - in case the information is provided by a system that aggregates information from different local sensors. Aggregation may include fusion,
 /// * - 13 `itsAggregation`    - in case the information is provided by a system that aggregates information from other received ITS messages.
 /// * - 14-15                  - are reserved for future usage.
 /// * 
 /// * @note: If all bits are set to 0, then no sensor type is used
 /// *
 /// * @category: Sensing Information
 /// * @revision: created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16", extensible))]
        pub struct SensorTypes(pub BitString);


///*
 /// * This DE represents a sequence number.
 /// * 
 /// * @category: Basic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct SequenceNumber(pub u16);


        
///* 
 /// * This DF shall contain a list of DF @ref CartesianPosition3d.
 /// * 
 /// * @category: GeoReference information
 /// * @revision: created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct SequenceOfCartesianPosition3d(pub SequenceOf<CartesianPosition3d>);


        
///* 
 /// * The DF contains a list of DE @ref Identifier1B.
 /// *
 /// * @category: Basic information
 /// * @revision: created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=128", extensible))]
        pub struct SequenceOfIdentifier1B(pub SequenceOf<Identifier1B>);


        
///*
 /// * The DF contains a list of DF @ref SafeDistanceIndication.
 /// *
 /// * @category: Traffic information, Kinematic information
 /// * @revision: created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct SequenceOfSafeDistanceIndication(pub SequenceOf<SafeDistanceIndication>);


        
///*
 /// * The DF shall contain a list of DF @ref TrajectoryInterceptionIndication.
 /// *
 /// * @category: Traffic information, Kinematic information
 /// * @revision: created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct SequenceOfTrajectoryInterceptionIndication(pub SequenceOf<TrajectoryInterceptionIndication>);


///*
 /// * This DF provides the definition of a geographical area or volume, based on different options.
 /// *
 /// * It is a choice of the following components: 
 /// *
 /// * @field rectangular: definition of an rectangular area or a right rectangular prism (with a rectangular base) also called a cuboid, or informally a rectangular box.
 /// *
 /// * @field circular: definition of an area of circular shape or a right circular cylinder.
 /// *
 /// * @field polygonal: definition of an area of polygonal shape or a right prism.
 /// *
 /// * @field elliptical: definition of an area of elliptical shape or a right elliptical cylinder.
 /// *
 /// * @field radial: definition of a radial shape.
 /// *
 /// * @field radialList: definition of list of radial shapes.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum Shape {
   Rectangular(RectangularShape),
     Circular(CircularShape),
     Polygonal(PolygonalShape),
     Elliptical(EllipticalShape),
     Radial(RadialShape),
     RadialShapes(RadialShapes),
    
}


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `signalViolation`.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`               - in case further detailed information on signal violation event is unavailable,
 /// * - 1 `stopSignViolation`         - in case a stop sign violation is detected,
 /// * - 2 `trafficLightViolation`     - in case a traffic light violation is detected,
 /// * - 3 `turningRegulationViolation`- in case a turning regulation violation is detected.
 /// * - 4-255                         - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct SignalViolationSubCauseCode(pub u8);


///*
 /// * This DE represents the sub cause codes of the @ref CauseCode "slowVehicle".
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`                    - in case further detailed information on slow vehicle driving event is
 /// *                                        unavailable,
 /// * - 1 `maintenanceVehicle`             - in case of a slow driving maintenance vehicle on the road,
 /// * - 2 `vehiclesSlowingToLookAtAccident`- in case vehicle is temporally slowing down to look at accident, spot, etc.,
 /// * - 3 `abnormalLoad`                   - in case an abnormal loaded vehicle is driving slowly on the road,
 /// * - 4 `abnormalWideLoad`               - in case an abnormal wide load vehicle is driving slowly on the road,
 /// * - 5 `convoy`                         - in case of slow driving convoy on the road,
 /// * - 6 `snowplough`                     - in case of slow driving snow plough on the road,
 /// * - 7 `deicing`                        - in case of slow driving de-icing vehicle on the road,
 /// * - 8 `saltingVehicles`                - in case of slow driving salting vehicle on the road.
 /// * - 9-255                              - are reserved for future usage.
 /// * 
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct SlowVehicleSubCauseCode(pub u8);


///*
 /// * The DE indicates if a vehicle is carrying goods in the special transport conditions.
 /// *
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * - 0 `heavyLoad`        - the vehicle is carrying goods with heavy load,
 /// * - 1 `excessWidth`      - the vehicle is carrying goods in excess of width,
 /// * - 2 `excessLength`     - the vehicle is carrying goods in excess of length,
 /// * - 3 `excessHeight`     - the vehicle is carrying goods in excess of height.
 /// *
 /// * Otherwise, the corresponding bit shall be set to 0.
 /// * @category Vehicle information
 /// * @revision: Description revised in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("4..=4"))]
        pub struct SpecialTransportType(pub BitString);


        
        ///*
 /// * This DF represents the speed and associated confidence value.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field speedValue: the speed value.
 /// * 
 /// * @field speedConfidence: the confidence value of the speed value.
 /// *
 /// * @category: Kinematic information
 /// * @revision: V1.3.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Speed {
            pub speed_value: SpeedValue,
                    pub speed_confidence: SpeedConfidence,
                    
        }

        impl Speed {
        pub fn new(
            speed_value: SpeedValue,
	speed_confidence: SpeedConfidence,
        ) -> Self {
            Self {
                speed_value,
	speed_confidence,
            }
        }
    }

        

///*
 /// * This DE indicates the speed confidence value which represents the estimated absolute accuracy of a speed value with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n * 0,01 m/s.
 /// * - `126` if the confidence value is out of range, i.e. greater than 1,25 m/s,
 /// * - `127` if the confidence value information is not available.
 /// *  
 /// * @note: The fact that a speed value is received with confidence value set to `unavailable(127)` can be caused by several reasons, such as:
 /// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
 /// * - the sensor cannot calculate the accuracy due to lack of variables, or
 /// * - there has been a vehicle bus (e.g. CAN bus) error.
 /// * In all 3 cases above, the speed value may be valid and used by the application.
 /// * 
 /// * @note: If a speed value is received and its confidence value is set to `outOfRange(126)`, it means that the speed value is not valid 
 /// * and therefore cannot be trusted. Such is not useful for the application.
 /// *
 /// * @unit: 0,01 m/s
 /// * @category: Vehicle information
 /// * @revision: Description revised in V2.1.1 
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=127"))]
        pub struct SpeedConfidence(pub u8);


///*
 /// * This DE represents a speed limitation applied to a geographical position, a road section or a geographical region.
 /// * 
 /// * @unit: km/h
 /// * @category: Infrastructure information, Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=255"))]
        pub struct SpeedLimit(pub u8);


///*
 /// * This DE represents a speed value, i.e. the magnitude of the velocity-vector. 
 /// *
 /// * The value shall be set to:
 /// * - `0` in a standstill situation.
 /// * - `n` (`n > 0` and `n < 16 382`) if the applicable value is equal to or less than n x 0,01 m/s, and greater than (n-1) x 0,01 m/s,
 /// * - `16 382` for speed values greater than 163,81 m/s,
 /// * - `16 383` if the speed accuracy information is not available.
 /// * 
 /// * @note: the definition of standstill is out of scope of the present document.
 /// *
 /// * @unit: 0,01 m/s
 /// * @category: Kinematic information
 /// * @revision: Description revised in V2.1.1 (the meaning of 16382 has changed slightly) 
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=16383"))]
        pub struct SpeedValue(pub u16);


        
        ///*
 /// * This DF  provides the  indication of  change in stability.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field lossProbability: the probability of stability loss. 
 /// * 
 /// * @field actionDeltaTime: the period over which the the probability of stability loss is estimated. 
 /// *
 /// * @category: Kinematic information
 /// * @revision: V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct StabilityChangeIndication {
            pub loss_probability: StabilityLossProbability,
                    pub action_delta_time: DeltaTimeTenthOfSecond,
                    
        }

        impl StabilityChangeIndication {
        pub fn new(
            loss_probability: StabilityLossProbability,
	action_delta_time: DeltaTimeTenthOfSecond,
        ) -> Self {
            Self {
                loss_probability,
	action_delta_time,
            }
        }
    }

        

///*
 /// * This DE indicates the estimated probability of a stability level and conversely also the probability of a stability loss.
 /// *
 /// * The value shall be set to:
 /// * - `0` to indicate an estimated probability of a loss of stability of 0 %, i.e. "stable", 
 /// * - `n` (`n > 0` and `n < 50`) to indicate the actual stability level,
 /// * - `50` to indicate a estimated probability of a loss of stability of 100 %, i.e. "total loss of stability",
 /// * - the values between 51 and 62 are reserved for future use,
 /// * - `63`: this value indicates that the information is unavailable.
 /// *
 /// * @unit: 2 %
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=63"))]
        pub struct StabilityLossProbability(pub u8);


///*
 /// * The DE represents length as a measure of distance between points or as a dimension of an object or shape. 
 /// *
 /// * @unit: 0,1 metre
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=4095"))]
        pub struct StandardLength12b(pub u16);


///*
 /// * The DE represents length as a measure of distance between points or as a dimension of an object. 
 /// *
 /// * @unit: 0,1 metre
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct StandardLength1B(pub u8);


///*
 /// * The DE represents length as a measure of distance between points or as a dimension of an object.  
 /// *
 /// * @unit: 0,1 metre
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct StandardLength2B(pub u16);


///*
 /// * The DE represents length as a measure of distance between points. 
 /// *
 /// * The value shall be set to:
 /// * - 0 `lessThan50m`   - for distances below 50 m, 
 /// * - 1 `lessThan100m`  - for distances below 100 m,
 /// * - 2 `lessThan200m`  - for distances below 200 m, 
 /// * - 3 `lessThan500m`  - for distances below 300 m, 
 /// * - 4 `lessThan1000m` - for distances below 1 000 m,
 /// * - 5 `lessThan5km`   - for distances below 5 000 m, 
 /// * - 6 `lessThan10km`  - for distances below 10 000 m, 
 /// * - 7 `over10km`      - for distances over 10 000 m.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1 from RelevanceDistance
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum StandardLength3b {
     LessThan50m = 0,
                 LessThan100m = 1,
                 LessThan200m = 2,
                 LessThan500m = 3,
                 LessThan1000m = 4,
                 LessThan5km = 5,
                 LessThan10km = 6,
                 Over10km = 7,
                
}


///*
 /// * The DE represents length as a measure of distance between points or as a dimension of an object. 
 /// *
 /// * @unit: 0,1 metre
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=511"))]
        pub struct StandardLength9b(pub u16);


///*
 /// * This DE represents the identifier of an ITS-S.
 /// * The ITS-S ID may be a pseudonym. It may change over space and/or over time.
 /// *
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref StationId instead.
 /// * @category: Basic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=4294967295"))]
        pub struct StationID(pub u32);


///*
 /// * This DE represents the identifier of an ITS-S.
 /// * The ITS-S ID may be a pseudonym. It may change over space and/or over time.
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1 based on @ref StationID
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=4294967295"))]
        pub struct StationId(pub u32);


///*
 /// * This DE represents the type of technical context the ITS-S is integrated in.
 /// * The station type depends on the integration environment of ITS-S into vehicle, mobile devices or at infrastructure.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unknown`          - information about the ITS-S context is not provided,
 /// * - 1 `pedestrian`       - ITS-S carried by human being not using a mechanical device for their trip (VRU profile 1),
 /// * - 2 `cyclist`          - ITS-S mounted on non-motorized unicycles, bicycles , tricycles, quadracycles (VRU profile 2),
 /// * - 3 `moped`            - ITS-S mounted on light motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] 
 ///                            class L1, L2 (VRU Profile 3),
 /// * - 4 `motorcycles`      - ITS-S mounted on motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] 
 ///                            class L3, L4, L5, L6, L7 (VRU Profile 3),
 /// * - 5 `passengerCar`     - ITS-S mounted on small passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M1,
 /// * - 6 `bus`              - ITS-S mounted on large passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M2, M3,
 /// * - 7 `lightTruck`       - ITS-S mounted on light Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N1,
 /// * - 8 `heavyTruck`       - ITS-S mounted on Heavy Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N2 and N3,
 /// * - 9 `trailer`          - ITS-S mounted on an unpowered vehicle that is intended to be towed by a powered vehicle as defined in 
 ///                            UNECE/TRANS/WP.29/78/Rev.4 [16] class O,
 /// * - 10 `specialVehicles` - ITS-S mounted on vehicles which have special purposes other than the above (e.g. moving road works vehicle),
 /// * - 11 `tram`            - ITS-S mounted on a vehicle which runs on tracks along public streets,
 /// * - 12 `lightVruVehicle` - ITS-S carried by a human being traveling on light vehicle , incl. possible use of roller skates or skateboards (VRU profile 2),
 /// * - 13 `animal`          - ITS-S carried by an animal presenting a safety risk to other road users e.g. domesticated dog in a city or horse (VRU Profile 4),
 /// * - 14                   - reserved for future usage,
 /// * - 15 `roadSideUnit`    - ITS-S mounted on an infrastructure typically positioned outside of the drivable roadway (e.g. on a gantry, on a pole, 
 ///                            on a stationary road works trailer); the infrastructure is static during the entire operation period of the ITS-S (e.g. no stop and go activity),
 /// * - 16-255               - are reserved for future usage.
 /// * 
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref TrafficParticipantType instead.
 /// * @category: Communication information.
 /// * @revision: revised in V2.1.1 (named values 12 and 13 added and note to value 9 deleted)
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct StationType(pub u8);


///*
 /// * This DE indicates the duration in minutes since which something is stationary.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `lessThan1Minute`         - for being stationary since less than 1 minute, 
 /// * - 1 `lessThan2Minutes`        - for being stationary since less than 2 minute and for equal to or more than 1 minute,
 /// * - 2 `lessThan15Minutes`       - for being stationary since less than 15 minutes and for equal to or more than 1 minute,
 /// * - 3 `equalOrGreater15Minutes` - for being stationary since equal to or more than 15 minutes.
 /// *
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum StationarySince {
     LessThan1Minute = 0,
                 LessThan2Minutes = 1,
                 LessThan15Minutes = 2,
                 EqualOrGreater15Minutes = 3,
                
}


///*
 /// * This DE provides the value of the sub cause codes of the @ref CauseCode "stationaryVehicle". 
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`           - in case further detailed information on stationary vehicle is unavailable,
 /// * - 1 `humanProblem`          - in case stationary vehicle is due to health problem of driver or passenger,
 /// * - 2 `vehicleBreakdown`      - in case stationary vehicle is due to vehicle break down,
 /// * - 3 `postCrash`             - in case stationary vehicle is caused by collision,
 /// * - 4 `publicTransportStop`   - in case public transport vehicle is stationary at bus stop,
 /// * - 5 `carryingDangerousGoods`- in case the stationary vehicle is carrying dangerous goods,
 /// * - 6 `vehicleOnFire`         - in case of vehicle on fire.
 /// * - 7-255 reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct StationaryVehicleSubCauseCode(pub u8);


        
        ///*
 /// * This DF represents the steering wheel angle of the vehicle at certain point in time.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field steeringWheelAngleValue: steering wheel angle value.
 /// * 
 /// * @field steeringWheelAngleConfidence: confidence value of the steering wheel angle value.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1
 /// 

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

        

///*
 /// * This DE indicates the steering wheel angle confidence value which represents the estimated absolute accuracy for a  steering wheel angle value with a confidence level of 95 %.
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 126`) if the confidence value is equal to or less than n x 1,5 degrees,
 /// * - `126` if the confidence value is out of range, i.e. greater than 187,5 degrees,
 /// * - `127` if the confidence value is not available.
 /// * 
 /// * @note: The fact that a steering wheel angle value is received with confidence value set to `unavailable(127)`
 /// * can be caused by several reasons, such as:
 /// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
 /// * - the sensor cannot calculate the accuracy due to lack of variables, or
 /// * - there has been a vehicle bus (e.g. CAN bus) error.
 /// * In all 3 cases above, the steering wheel angle value may be valid and used by the application.
 /// * 
 /// * If a steering wheel angle value is received and its confidence value is set to `outOfRange(126)`,
 /// * it means that the steering wheel angle value is not valid and therefore cannot be trusted.
 /// * Such value is not useful for the application.
 /// * 
 /// * @unit: 1,5 degree
 /// * @category: Vehicle Information
 /// * @revision: Description revised in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=127"))]
        pub struct SteeringWheelAngleConfidence(pub u8);


///*
 /// * This DE represents the steering wheel angle of the vehicle at certain point in time.
 /// * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11.
 /// * 
 /// * The value shall be set to:
 /// * - `-511` if the steering wheel angle is equal to or greater than 511 x 1,5 degrees = 766,5 degrees to the right,
 /// * - `n` (`n > -511` and `n <= 0`) if  the steering wheel angle is equal to or less than n x 1,5 degrees, and greater than (n-1) x 1,5 degrees, 
 ///      turning clockwise (i.e. to the right),
 /// * - `n` (`n >= 1` and `n < 511`) if the steering wheel angle is equal to or less than n x 0,1 degrees, and greater than (n-1) x 0,1 degrees, 
 ///      turning counter-clockwise (i.e. to the left),
 /// * - `511` if the steering wheel angle is greater than 510 x 1,5 degrees = 765 degrees to the left,
 /// * - `512` if information is not available.
 /// *
 /// * @unit: 1,5 degree
 /// * @revision: Description revised in V2.1.1 (meaning of value 511 has changed slightly).
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-511..=512"))]
        pub struct SteeringWheelAngleValue(pub i16);


///* 
 /// * This DE indicates the type of stored information.
 /// *
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * 
 /// * - `0` undefined        - in case the stored information type is undefined. 
 /// * - `1` staticDb         - in case the stored information type is a static database.
 /// * - `2` dynamicDb        - in case the stored information type is a dynamic database
 /// * - `3` realTimeDb       - in case the stored information type is a real time updated database.
 /// * - `4` map              - in case the stored information type is a road topology map.
 /// * - Bits 5 to 7          - are reserved for future use
 /// *
 /// * @note: If all bits are set to 0, then no stored information type is used
 /// *
 /// * @category: Basic Information
 /// * @revision: created in V2.2.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=168", extensible))]
        pub struct StoredInformationType(pub BitString);


///*
 /// * This DE indicates the generic sub cause of a detected event.
 /// * 
 /// * @note: The sub cause code value assignment varies based on value of @ref CauseCode.
 /// *
 /// * @category: Traffic information
 /// * @revision: Description revised in V2.1.1 (this is  the generic sub cause type)
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct SubCauseCodeType(pub u8);


///*
 /// * This DE indicates a temperature value.
 ///
 /// * The value shall be set to:
 /// * - `-60` for temperature equal to or less than -60 degrees C,
 /// * - `n` (`n > -60` and `n < 67`) for the actual temperature n in degrees C,
 /// * - `67` for temperature equal to or greater than 67 degrees C.
 /// * 
 /// * @unit: degrees Celsius
 /// * @category: Basic information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-60..=67"))]
        pub struct Temperature(pub i8);


///*
 /// * This DE represents the number of elapsed (TAI) milliseconds since the ITS Epoch. 
 /// * The ITS epoch is `00:00:00.000 UTC, 1 January 2004`.
 /// * "Elapsed" means that the true number of milliseconds is continuously counted without interruption,
 /// *  i.e. it is not altered by leap seconds, which occur in UTC.
 /// * 
 /// * @note: International Atomic Time (TAI) is the time reference coordinate on the basis of the readings of atomic clocks, 
 /// * operated in accordance with the definition of the second, the unit of time of the International System of Units. 
 /// * TAI is a continuous time scale. UTC has discontinuities, as it is occasionally adjusted by leap seconds. 
 /// * As of 1 January, 2022, TimestampIts is 5 seconds ahead of UTC, because since the ITS epoch on 1 January 2004 00:00:00.000 UTC, 
 /// * further 5 leap seconds have been inserted in UTC.
 /// * 
 /// * EXAMPLE: The value for TimestampIts for 1 January 2007 00:00:00.000 UTC is `94 694 401 000` milliseconds,
 /// * which includes one leap second insertion since the ITS epoch.
 /// * @unit: 0,001 s
 /// * @category: Basic information
 /// * @revision: Description revised in in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=4398046511103"))]
        pub struct TimestampIts(pub u64);


        
///*
 /// * This DF represents one or more paths using @ref Path.
 /// * 
 /// * @category: GeoReference information
 /// * @revision: Description revised in V2.1.1. Is is now based on Path and not on PathHistory
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=7"))]
        pub struct Traces(pub SequenceOf<Path>);


        
///*
 /// * This DF represents one or more paths using @ref PathExtended.
 /// * 
 /// * @category: GeoReference information
 /// * @revision: Created in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=7"))]
        pub struct TracesExtended(pub SequenceOf<PathExtended>);


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `trafficCondition`. 
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`                  - in case further detailed information on the traffic condition is unavailable,
 /// * - 1 `increasedVolumeOfTraffic`     - in case the type of traffic condition is increased traffic volume,
 /// * - 2 `trafficJamSlowlyIncreasing`   - in case the type of traffic condition is a traffic jam which volume is increasing slowly,
 /// * - 3 `trafficJamIncreasing`         - in case the type of traffic condition is a traffic jam which volume is increasing,
 /// * - 4 `trafficJamStronglyIncreasing` - in case the type of traffic condition is a traffic jam which volume is strongly increasing,
 /// * - 5 `trafficJam`         `         - in case the type of traffic condition is a traffic jam and no further detailed information about its volume is available,
 /// * - 6 `trafficJamSlightlyDecreasing` - in case the type of traffic condition is a traffic jam which volume is decreasing slowly,
 /// * - 7 `trafficJamDecreasing`         - in case the type of traffic condition is a traffic jam which volume is decreasing,
 /// * - 8 `trafficJamStronglyDecreasing` - in case the type of traffic condition is a traffic jam which volume is decreasing rapidly,
 /// * - 9 `trafficJamStable`             - in case the traffic condition is a traffic jam with stable volume,
 /// * - 10-255: reserved for future usage.
 /// *
 /// * @category: Traffic information
 /// * @revision: V1.3.1, definition of value 0 and 1 changed in V2.2.1, name and definition of value 5 changed in V2.2.1, value 9 added in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct TrafficConditionSubCauseCode(pub u8);


///*
 /// * This DE indicates a traffic direction that is relevant to information indicated in a message.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `allTrafficDirections` - for all traffic directions, 
 /// * - 1 `upstreamTraffic`      - for upstream traffic, 
 /// * - 2 `downstreamTraffic`    - for downstream traffic, 
 /// * - 3 `oppositeTraffic`      - for traffic in the opposite direction. 
 /// *
 /// * The terms `upstream`, `downstream` and `oppositeTraffic` are relative to the event position.
 /// *
 /// * @note: Upstream traffic corresponds to the incoming traffic towards the event position,
 /// * and downstream traffic to the departing traffic away from the event position.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1 from RelevanceTrafficDirection
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum TrafficDirection {
     AllTrafficDirections = 0,
                 UpstreamTraffic = 1,
                 DownstreamTraffic = 2,
                 OppositeTraffic = 3,
                
}


        
        ///*
 /// * Ths DF represents the a position on a traffic island between two lanes. 
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field oneSide: represents one lane.
 /// * 
 /// * @field otherSide: represents the other lane.
 /// * 
 /// * @category: Road Topology information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct TrafficIslandPosition {
            pub one_side: LanePositionAndType,
                    pub other_side: LanePositionAndType,
                    
        }

        impl TrafficIslandPosition {
        pub fn new(
            one_side: LanePositionAndType,
	other_side: LanePositionAndType,
        ) -> Self {
            Self {
                one_side,
	other_side,
            }
        }
    }

        

///*
 /// * This DE represents the type of a traffic participant.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unknown`          - information about traffic participant is not provided,
 /// * - 1 `pedestrian`       - human being not using a mechanical device for their trip (VRU profile 1),
 /// * - 2 `cyclist`          - non-motorized unicycles, bicycles , tricycles, quadracycles (VRU profile 2),
 /// * - 3 `moped`            - light motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class L1, L2 (VRU Profile 3),
 /// * - 4 `motorcycles`      - motor vehicles with less than four wheels as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class L3, L4, L5, L6, L7 (VRU Profile 3),
 /// * - 5 `passengerCar`     - small passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M1,
 /// * - 6 `bus`              - large passenger vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class M2, M3,
 /// * - 7 `lightTruck`       - light Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N1,
 /// * - 8 `heavyTruck`       - Heavy Goods Vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class N2 and N3,
 /// * - 9 `trailer`          - unpowered vehicle that is intended to be towed by a powered vehicle as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class O,
 /// * - 10 `specialVehicles` - vehicles which have special purposes other than the above (e.g. moving road works vehicle),
 /// * - 11 `tram`            - vehicle which runs on tracks along public streets,
 /// * - 12 `lightVruVehicle` - human being traveling on light vehicle, incl. possible use of roller skates or skateboards (VRU profile 2),
 /// * - 13 `animal`          - animal presenting a safety risk to other road users e.g. domesticated dog in a city or horse (VRU Profile 4),
 /// * - 14 `agricultural`    - agricultural and forestry vehicles as defined in UNECE/TRANS/WP.29/78/Rev.4 [16] class T,
 /// * - 15 `roadSideUnit`    - infrastructure typically positioned outside of the drivable roadway (e.g. on a gantry, on a pole, 
 ///                            on a stationary road works trailer); the infrastructure is static during the entire operation period of the ITS-S (e.g. no stop and go activity),
 /// * - 16-255               - are reserved for future usage.
 /// * 
 /// * @category: Communication information.
 /// * @revision: Created in V2.1.1 based on StationType
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct TrafficParticipantType(pub u8);


///*
 /// * This DE indicates traffic rules that apply to vehicles at a certain position.
 /// *
 /// * The value shall be set to:
 /// * - `0` - if overtaking is prohibited for all vehicles,
 /// * - `1` - if overtaking is prohibited for trucks,
 /// * - `2` - if vehicles should pass to the right lane,
 /// * - `3` - if vehicles should pass to the left lane.
 /// * - `4` - if vehicles should pass to the left or right lane.
 /// *
 /// * @category: Infrastructure information, Traffic information
 /// * @revision: Editorial update in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
                #[non_exhaustive]
pub enum TrafficRule {
     NoPassing = 0,
                 NoPassingForTrucks = 1,
                 PassToRight = 2,
                 PassToLeft = 3,
                #[rasn(extension_addition)]
             PassToLeftOrRight = 4,
                
}


        
        ///* 
 /// * This DF provides detailed information about an attached trailer.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field refPointId: identifier of the reference point of the trailer.
 /// *
 /// * @field hitchPointOffset: optional position of the hitch point in negative x-direction (according to ISO 8855) from the
 /// * vehicle Reference Point.
 /// *
 /// * @field frontOverhang: optional length of the trailer overhang in the positive x direction (according to ISO 8855) from the
 /// * trailer Reference Point indicated by the refPointID. The value defaults to 0 in case the trailer
 /// * is not overhanging to the front with respect to the trailer reference point.
 /// *
 /// * @field rearOverhang: optional length of the trailer overhang in the negative x direction (according to ISO 8855) from the
 /// * trailer Reference Point indicated by the refPointID.
 /// *
 /// * @field trailerWidth: optional width of the trailer.
 /// *
 /// * @field hitchAngle: optional Value and confidence value of the angle between the trailer orientation (corresponding to the x
 /// * direction of the ISO 8855 [21] coordinate system centered on the trailer) and the direction of
 /// * the segment having as end points the reference point of the trailer and the reference point of
 /// * the pulling vehicle, which can be another trailer or a vehicle looking on the horizontal plane
 /// * xy, described in the local Cartesian coordinate system of the trailer. The
 /// * angle is measured with negative values considering the trailer orientation turning clockwise
 /// * starting from the segment direction. The angle value accuracy is provided with the
 /// * confidence level of 95 %.
 /// *
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct TrailerData {
            pub ref_point_id: Identifier1B,
                    pub hitch_point_offset: StandardLength1B,
                    pub front_overhang: Option<StandardLength1B>,
                    pub rear_overhang: Option<StandardLength1B>,
                    pub trailer_width: Option<VehicleWidth>,
                    pub hitch_angle: CartesianAngle,
                    
        }

        impl TrailerData {
        pub fn new(
            ref_point_id: Identifier1B,
	hitch_point_offset: StandardLength1B,
	front_overhang: Option<StandardLength1B>,
	rear_overhang: Option<StandardLength1B>,
	trailer_width: Option<VehicleWidth>,
	hitch_angle: CartesianAngle,
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

        

///*
 /// * This DE provides information about the presence of a trailer. 
 /// *
 /// * The value shall be set to:
 /// * - 0 `noTrailerPresent`                - to indicate that no trailer is present, i.e. either the vehicle is physically not enabled to tow a trailer or it has been detected that no trailer is present.
 /// * - 1 `trailerPresentWithKnownLength`   - to indicate that a trailer has been detected as present and the length is included in the vehicle length value.
 /// * - 2 `trailerPresentWithUnknownLength` - to indicate that a trailer has been detected as present and the length is not included in the vehicle length value.
 /// * - 3 `trailerPresenceIsUnknown`        - to indicate that information about the trailer presence is unknown, i.e. the vehicle is physically enabled to tow a trailer but the detection of trailer presence/absence is not possible.
 /// * - 4 `unavailable`                     - to indicate that the information about the presence of a trailer is not available, i.e. it is neither known whether the vehicle is able to tow a trailer 
 /// *                                         nor the detection of trailer presence/absence is possible.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1 based on VehicleLengthConfidenceIndication
 ///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum TrailerPresenceInformation {
     NoTrailerPresent = 0,
                 TrailerPresentWithKnownLength = 1,
                 TrailerPresentWithUnknownLength = 2,
                 TrailerPresenceIsUnknown = 3,
                 Unavailable = 4,
                
}


///*
 /// * This DE  defines  the  confidence level of the trajectoryInterceptionProbability.
 /// *
 /// * The value shall be set to:
 /// * - `0` - to indicate confidence level less than 50 %,
 /// * - `1` - to indicate confidence level greater than or equal to 50 % and less than 70 %,
 /// * - `2` - to indicate confidence level greater than or equal to 70 % and less than 90 %,
 /// * - `3` - to indicate confidence level greater than or equal to 90%.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3"))]
        pub struct TrajectoryInterceptionConfidence(pub u8);


        
        ///*
 /// * This DF  provides the trajectory  interception  indication  of  ego-VRU  ITS-S  with another ITS-Ss. 
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field subjectStation: indicates the subject station.
 /// * 
 /// * @field trajectoryInterceptionProbability: indicates the propbability of the interception of the subject station trajectory 
 /// * with the trajectory of the station indicated in the component subjectStation.
 /// *
 /// * @field trajectoryInterceptionConfidence: indicates the confidence of interception of the subject station trajectory 
 /// * with the trajectory of the station indicated in the component subjectStation.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct TrajectoryInterceptionIndication {
            pub subject_station: Option<StationId>,
                    pub trajectory_interception_probability: TrajectoryInterceptionProbability,
                    pub trajectory_interception_confidence: Option<TrajectoryInterceptionConfidence>,
                    
        }

        impl TrajectoryInterceptionIndication {
        pub fn new(
            subject_station: Option<StationId>,
	trajectory_interception_probability: TrajectoryInterceptionProbability,
	trajectory_interception_confidence: Option<TrajectoryInterceptionConfidence>,
        ) -> Self {
            Self {
                subject_station,
	trajectory_interception_probability,
	trajectory_interception_confidence,
            }
        }
    }

        

///*
 /// * This  DE  defines  the  probability  that the ego trajectory  intercepts  with any  other object's  trajectory  on the  road. 
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n >= 0` and `n <= 50`) to indicate the actual probability,
 /// * - the values between 51 and 62 are reserved,
 /// * - `63`: to indicate that the information is unavailable. 
 /// *
 /// * @unit: 2 %
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=63"))]
        pub struct TrajectoryInterceptionProbability(pub u8);


///*
 /// * This DE represents the time interval between two consecutive message transmissions.
 /// * 
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref DeltaTimeMilliSecondPos instead.
 /// * @unit: 0,001 s
 /// * @category: Basic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=10000"))]
        pub struct TransmissionInterval(pub u16);


///*
 /// * This DE provides the turning direction. 
 /// * 
 /// * The value shall be set to:
 /// * - `left`  for turning to te left.
 /// * - `right` for turing to the right.
 /// *
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum TurningDirection {
     Left = 0,
                 Right = 1,
                
}


///*
 /// * This DE represents the smallest circular turn (i.e. U-turn) that the vehicle is capable of making.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 254`) to indicate the applicable value is equal to or less than n x 0,4 metre, and greater than (n-1) x 0,4 metre,
 /// * - `254` to indicate that the turning radius is  greater than 253 x 0,4 metre = 101.2 metres,
 /// * - `255` to indicate that the information is unavailable.
 /// * 
 /// * For vehicle with tracker, the turning radius applies to the vehicle only.
 /// *
 /// * @category: Vehicle information
 /// * @unit 0,4 metre
 /// * @revision: Description revised V2.1.1 (the meaning of 254 has changed slightly)
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=255"))]
        pub struct TurningRadius(pub u8);


///*
 /// * This DE represents indication of how a certain path or area will be used. 
 /// * 
 /// * The value shall be set to:
 /// * - 0  - ` noIndication `     - in case it will remain free to be used,
 /// * - 1  - ` specialUse `       - in case it will be physically blocked by special use,
 /// * - 2  - ` rescueOperation`   - in case it is intended to be used for rescue operations,
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
                #[non_exhaustive]
pub enum UsageIndication {
     NoIndication = 0,
                 SpecialUse = 1,
                 RescueOperation = 2,
                
}


///*
 /// * This DE represents the Vehicle Descriptor Section (VDS). The values are assigned according to ISO 3779 [6].
 /// * 
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("6..=6"))]
        pub struct VDS(pub Ia5String);


///* 
 /// * This DE represents the duration of a traffic event validity. 
 /// *
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref DeltaTimeSecond instead.
 /// * @unit: 1 s
 /// * @category: Basic information
 /// * @revision: V1.3.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=86400"))]
        pub struct ValidityDuration(pub u32);


///*
 /// * This DF together with its sub DFs Ext1, Ext2 and the DE Ext3 provides the custom (i.e. not ASN.1 standard) definition of an integer with variable lenght, that can be used for example to encode the ITS-AID. 
 /// *
 /// * @category: Basic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum VarLengthNumber {
  #[rasn(value("0..=127"),tag(context, 0))]
         Content(u8),
    #[rasn(tag(context, 1))]
         Extension(Ext1),
    
}


///*
 /// * This DE represents the value of the sub cause codes of the @ref CauseCode `vehicleBreakdown`. 
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`         - in case further detailed information on cause of vehicle break down is unavailable,
 /// * - 1 `lackOfFuel`          - in case vehicle break down is due to lack of fuel,
 /// * - 2 `lackOfBatteryPower`  - in case vehicle break down is caused by lack of battery power,
 /// * - 3 `engineProblem`       - in case vehicle break down is caused by an engine problem,
 /// * - 4 `transmissionProblem` - in case vehicle break down is caused by transmission problem,
 /// * - 5 `engineCoolingProblem`- in case vehicle break down is caused by an engine cooling problem,
 /// * - 6 `brakingSystemProblem`- in case vehicle break down is caused by a braking system problem,
 /// * - 7 `steeringProblem`     - in case vehicle break down is caused by a steering problem,
 /// * - 8 `tyrePuncture`        - in case vehicle break down is caused by tyre puncture,
 /// * - 9 `tyrePressureProblem` - in case low tyre pressure in detected,
 /// * - 10 `vehicleOnFire`      - in case the vehicle is on fire.
 /// * - 11-255                  - are reserved for future usage.
 /// *
 /// * @category: Traffic information
 ///
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct VehicleBreakdownSubCauseCode(pub u8);


///* 
 /// * This DE represents the height of the vehicle, measured from the ground to the highest point, excluding any antennas.
 /// * In case vehicles are equipped with adjustable ride heights, camper shells, and any other
 /// * equipment which may result in varying height, the largest possible height shall be used.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n >0` and `n < 127`) indicates the applicable value is equal to or less than n x 0,05 metre, and greater than (n-1) x 0,05 metre,
 /// * - `127` indicates that the vehicle width is greater than 6,3 metres,
 /// * - `128` indicates that the information in unavailable.
 /// *
 /// * @unit: 0,05 metre 
 /// * @category: Vehicle information
 /// * @revision: created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=128"))]
        pub struct VehicleHeight(pub u8);


        
        ///*
 /// * This DF provides information related to the identification of a vehicle.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field wMInumber: World Manufacturer Identifier (WMI) code.
 /// * 
 /// * @field vDS: Vehicle Descriptor Section (VDS). 
 /// * 
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VehicleIdentification {
            pub w_m_inumber: Option<WMInumber>,
                    pub v_d_s: Option<VDS>,
                    
        }

        impl VehicleIdentification {
        pub fn new(
            w_m_inumber: Option<WMInumber>,
	v_d_s: Option<VDS>,
        ) -> Self {
            Self {
                w_m_inumber,
	v_d_s,
            }
        }
    }

        

        
        ///*
 /// * This DF represents the length of vehicle and accuracy indication information.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field vehicleLengthValue: length of vehicle. 
 /// * 
 /// * @field vehicleLengthConfidenceIndication: indication of the length value confidence.
 /// * 
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref VehicleLengthV2 instead.
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

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

        

///*
 /// * This DE provides information about the presence of a trailer. 
 /// *
 /// * The value shall be set to:
 /// * - 0 `noTrailerPresent`                - to indicate that no trailer is present, i.e. either the vehicle is physically not enabled to tow a trailer or it has been detected that no trailer is present,
 /// * - 1 `trailerPresentWithKnownLength`   - to indicate that a trailer has been detected as present and the length is  included in the vehicle length value,
 /// * - 2 `trailerPresentWithUnknownLength` - to indicate that a trailer has been detected as present and the length is not included in the vehicle length value,
 /// * - 3 `trailerPresenceIsUnknown`        - to indicate that information about the trailer presence is unknown, i.e. the vehicle is physically enabled to tow a trailer but the detection of trailer presence/absence is not possible,
 /// * - 4 `unavailable`                     - to indicate that the information about the presence of a trailer is not available, i.e. it is neither known whether the vehicle is able to tow a trailer, 
 /// *                                        nor the detection of trailer presence/absence is possible.
 /// * 
 /// * @note: this DE is kept for backwards compatibility reasons only. It is recommended to use the @ref TrailerPresenceInformation instead. 
 /// * @category: Vehicle information
 /// * @revision: Description revised in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum VehicleLengthConfidenceIndication {
     NoTrailerPresent = 0,
                 TrailerPresentWithKnownLength = 1,
                 TrailerPresentWithUnknownLength = 2,
                 TrailerPresenceIsUnknown = 3,
                 Unavailable = 4,
                
}


        
        ///*
 /// * This DF represents the length of vehicle and accuracy indication information.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field vehicleLengthValue: length of vehicle. 
 /// * 
 /// * @field trailerPresenceInformation: information about the trailer presence.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: created in V2.1.1 based on @ref VehicleLength but using @ref TrailerPresenceInformation.
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct VehicleLengthV2 {
            pub vehicle_length_value: VehicleLengthValue,
                    pub trailer_presence_information: TrailerPresenceInformation,
                    
        }

        impl VehicleLengthV2 {
        pub fn new(
            vehicle_length_value: VehicleLengthValue,
	trailer_presence_information: TrailerPresenceInformation,
        ) -> Self {
            Self {
                vehicle_length_value,
	trailer_presence_information,
            }
        }
    }

        

///*
 /// * This DE represents the length of a vehicle.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 1022`) to indicate the applicable value n is equal to or less than n x 0,1 metre, and greater than (n-1) x 0,1 metre,
 /// * - `1 022` to indicate that the vehicle length is greater than 102.1 metres,
 /// * - `1 023` to indicate that the information in unavailable.
 /// * 
 /// * 
 /// * @unit: 0,1 metre
 /// * @category: Vehicle information
 /// * @revision: Description updated in V2.1.1 (the meaning of 1 022 has changed slightly).
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=1023"))]
        pub struct VehicleLengthValue(pub u16);


///*
 /// * This DE represents the mass of an empty loaded vehicle.
 /// *
 /// * The value shall be set to: 
 /// * - `n` (`n > 0` and `n < 1023`) to indicate that the applicable value is equal to or less than n x 10^5 gramm, and greater than (n-1) x 10^5 gramm,
 /// * - `1 023` indicates that the vehicle mass is greater than 102 200 000 g,
 /// * - `1 024` indicates  the vehicle mass information is unavailable.
 /// * 
 /// * @note:	The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
 /// * 
 /// * @unit: 10^5 gramm
 /// * @category: Vehicle information
 /// * @revision: Description updated in V2.1.1 (the meaning of 1 023 has changed slightly).
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=1024"))]
        pub struct VehicleMass(pub u16);


///*
 /// * This DE indicates the role played by a vehicle at a point in time.
 /// *
 /// * The value shall be set to:
 /// * - 0 `default`          - to indicate the default vehicle role as indicated by the vehicle type,
 /// * - 1 `publicTransport`  - to indicate that the vehicle is used to operate public transport service,
 /// * - 2 `specialTransport` - to indicate that the vehicle is used for special transport purpose, e.g. oversized trucks,
 /// * - 3 `dangerousGoods`   - to indicate that the vehicle is used for dangerous goods transportation,
 /// * - 4 `roadWork`         - to indicate that the vehicle is used to realize roadwork or road maintenance mission,
 /// * - 5 `rescue`           - to indicate that the vehicle is used for rescue purpose in case of an accident, e.g. as a towing service,
 /// * - 6 `emergency`        - to indicate that the vehicle is used for emergency mission, e.g. ambulance, fire brigade,
 /// * - 7 `safetyCar`        - to indicate that the vehicle is used for public safety, e.g. patrol,
 /// * - 8 `agriculture`      - to indicate that the vehicle is used for agriculture, e.g. farm tractor, 
 /// * - 9 `commercial`       - to indicate that the vehicle is used for transportation of commercial goods,
 /// * - 10 `military`        - to indicate that the vehicle is used for military purpose, 
 /// * - 11 `roadOperator`    - to indicate that the vehicle is used in road operator missions,
 /// * - 12 `taxi`            - to indicate that the vehicle is used to provide an authorized taxi service.
 /// * - 13 `reserved`        - is reserved for future usage.
 /// * - 14 `reserved`        - is reserved for future usage.
 /// * - 15 `reserved`        - is reserved for future usage.
 /// * 
 /// * @category: Vehicle Information
 /// * @revision: Description updated in V2.1.1 (removed reference to CEN/TS 16157-3)
 /// 

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


///*
 /// * This DE represents the width of a vehicle, excluding side mirrors and possible similar extensions.
 ///
 /// * The value shall be set to:
 /// * - `n` (`n > 0` and `n < 61`) indicates the applicable value is equal to or less than n x 0,1 metre, and greater than (n-1) x 0,1 metre,
 /// * - `61` indicates that the vehicle width is greater than 6,0 metres,
 /// * - `62` indicates that the information in unavailable.
 /// * 
 /// * @unit: 0,1 metre
 /// * @category: Vehicle information 
 /// * @revision: Description updated in V2.1.1 (the meaning of 61 has changed slightly).
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=62"))]
        pub struct VehicleWidth(pub u8);


///*
 /// * This DF represents a velocity vector with associated confidence value.
 /// *
 /// * The following options are available:
 /// * 
 /// * @field polarVelocity: the representation of the velocity vector in a polar or cylindrical coordinate system. 
 /// * 
 /// * @field cartesianVelocity: the representation of the velocity vector in a cartesian coordinate system.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum Velocity3dWithConfidence {
   PolarVelocity(VelocityPolarWithZ),
     CartesianVelocity(VelocityCartesian),
    
}


        
        ///*
 /// * This DF represents a velocity vector in a cartesian coordinate system.
 /// 
 /// * It shall include the following components: 
 /// * 
 /// * @field xVelocity: the x component of the velocity vector with the associated confidence value.
 /// * 
 /// * @field yVelocity: the y component of the velocity vector with the associated confidence value.
 /// *
 /// * @field zVelocity: the optional z component of the velocity vector with the associated confidence value.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct VelocityCartesian {
            pub x_velocity: VelocityComponent,
                    pub y_velocity: VelocityComponent,
                    pub z_velocity: Option<VelocityComponent>,
                    
        }

        impl VelocityCartesian {
        pub fn new(
            x_velocity: VelocityComponent,
	y_velocity: VelocityComponent,
	z_velocity: Option<VelocityComponent>,
        ) -> Self {
            Self {
                x_velocity,
	y_velocity,
	z_velocity,
            }
        }
    }

        

        
        ///*
 /// * This DF represents a component of the velocity vector and the associated confidence value.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field value: the value of the component.
 /// * 
 /// * @field confidence: the confidence value of the value.
 /// *
 /// * @category: Kinematic information
 /// * @revision: V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct VelocityComponent {
            pub value: VelocityComponentValue,
                    pub confidence: SpeedConfidence,
                    
        }

        impl VelocityComponent {
        pub fn new(
            value: VelocityComponentValue,
	confidence: SpeedConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

///* 
 /// * This DE represents the value of a velocity component in a defined coordinate system.
 /// *
 /// * The value shall be set to:
 /// * - `-16 383` if the velocity is equal to or smaller than -163,83 m/s,
 /// * - `n` (`n > -16 383` and `n < 16 382`) if the applicable value is equal to or less than n x 0,01 m/s, and greater than (n-1) x 0,01 m/s,
 /// * - `16 382` for velocity values equal to or greater than 163,81 m/s,
 /// * - `16 383` if the velocity information is not available.
 /// * 
 /// * @unit: 0,01 m/s
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-16383..=16383"))]
        pub struct VelocityComponentValue(pub i16);


        
        ///*
 /// * This DF represents a velocity vector in a polar or cylindrical coordinate system.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field velocityMagnitude: magnitude of the velocity vector on the reference plane, with the associated confidence value.
 /// * 
 /// * @field velocityDirection: polar angle of the velocity vector on the reference plane, with the associated confidence value.
 /// *
 /// * @field zVelocity: the optional z component of the velocity vector along the reference axis of the cylindrical coordinate system, with the associated confidence value.
 /// * 
 /// * @category: Kinematic information
 /// * @revision: Created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct VelocityPolarWithZ {
            pub velocity_magnitude: Speed,
                    pub velocity_direction: CartesianAngle,
                    pub z_velocity: Option<VelocityComponent>,
                    
        }

        impl VelocityPolarWithZ {
        pub fn new(
            velocity_magnitude: Speed,
	velocity_direction: CartesianAngle,
	z_velocity: Option<VelocityComponent>,
        ) -> Self {
            Self {
                velocity_magnitude,
	velocity_direction,
	z_velocity,
            }
        }
    }

        

        
        /// four and more octets length
 ///*
 /// * This DF indicates the vehicle acceleration at vertical direction and the associated confidence value.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field verticalAccelerationValue: vertical acceleration value at a point in time.
 /// * 
 /// * @field verticalAccelerationConfidence: confidence value of the vertical acceleration value with a predefined confidence level.
 /// * 
 /// * @note: this DF is kept for backwards compatibility reasons only. It is recommended to use @ref AccelerationComponent instead.
 /// * @category Vehicle information
 /// * @revision: Description revised in V2.1.1
 /// 

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

        

///*
 /// * This DE represents the vehicle acceleration at vertical direction in the centre of the mass of the empty vehicle.
 /// * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11.
 /// *
 /// * The value shall be set to:
 /// * - `-160` for acceleration values equal to or less than -16 m/s^2,
 /// * - `n` (`n > -160` and `n <= 0`) to indicate downwards acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
 /// * - `n` (`n > 0` and `n < 160`) to indicate upwards acceleration equal to or less than n x 0,1 m/s^2, and greater than (n-1) x 0,1 m/s^2,
 /// * - `160` for acceleration values greater than 15,9 m/s^2,
 /// * - `161` when the data is unavailable.
 /// * 
 /// * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
 /// *
 /// * @category: Vehicle information
 /// * @unit: 0,1 m/s^2
 /// * @revision: Desciption updated in V2.1.1 (the meaning of 160 has changed slightly).
 /// *  
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-160..=161"))]
        pub struct VerticalAccelerationValue(pub i16);


        
        ///* 
 /// * This DF provides information about a VRU cluster.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field clusterId: optional identifier of a VRU cluster .
 /// *
 /// * @field clusterBoundingBoxShape: optionally indicates the shape of the cluster bounding box, per default inside an East-North-Up coordinate system 
 /// * centered around a reference point defined outside of the context of this DF.
 /// *
 /// * @field clusterCardinalitySize: indicates an estimation of the number of VRUs in the group, e.g. the known members in the cluster + 1 (for the cluster leader) .
 /// *
 /// * @field clusterProfiles: optionally identifies all the VRU profile types that are estimated to be within the cluster.
 /// * if this component is absent it means that the information is unavailable. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1, description revised in V2.2.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VruClusterInformation {
            pub cluster_id: Option<Identifier1B>,
                    #[rasn(value("0.."))]
        pub cluster_bounding_box_shape: Option<Shape>,
                    pub cluster_cardinality_size: CardinalNumber1B,
                    pub cluster_profiles: Option<VruClusterProfiles>,
                    
        }

        impl VruClusterInformation {
        pub fn new(
            cluster_id: Option<Identifier1B>,
	cluster_bounding_box_shape: Option<Shape>,
	cluster_cardinality_size: CardinalNumber1B,
	cluster_profiles: Option<VruClusterProfiles>,
        ) -> Self {
            Self {
                cluster_id,
	cluster_bounding_box_shape,
	cluster_cardinality_size,
	cluster_profiles,
            }
        }
    }

        

///* 
 /// * This DE Identifies all the VRU profile types within a cluster.
 /// * It consist of a Bitmap encoding VRU profiles, to allow multiple profiles to be indicated in a single cluster (heterogeneous cluster if more than one profile).
 /// * 
 /// * The corresponding bit shall be set to 1 under the following conditions:
 /// * - 0 `pedestrian`  - indicates that the VRU cluster contains at least one pedestrian VRU,
 /// * - 1 `bicycle`     - indicates that the VRU cluster contains at least one bicycle VRU member,
 /// * - 2 `motorcyclist`- indicates that the VRU cluster contains at least one motorcycle VRU member,
 /// * - 3 `animal`      - indicates that the VRU cluster contains at least one animal VRU member.
 /// * 
 /// * Otherwise, the corresponding bit shall be set to 0.
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("4..=4"))]
        pub struct VruClusterProfiles(pub BitString);


///*
 /// * This DE represents the possible usage conditions of the VRU device.
 ///
 /// * - The value shall be set to:
 /// * - 0 `unavailable`      - to indicate that the usage conditions are unavailable,
 /// * - 1 `other`            - to indicate that the VRU device is in a state not defined below,
 /// * - 2 `idle`             - to indicate that the human is currently not interacting with the device,
 /// * - 3 `listeningToAudio` - to indicate that any audio source other than calling is in use,
 /// * - 4 `typing`           - to indicate that the human is texting or performaing any other manual input activity,
 /// * - 5 `calling`          - to indicate that the VRU device is currently receiving a call,
 /// * - 6 `playingGames`     - to indicate that the human is playing games,
 /// * - 7 `reading`          - to indicate that the human is reading on the VRU device,
 /// * - 8 `viewing`          - to indicate that the human is watching dynamic content, including following navigation prompts, viewing videos or other visual contents that are not static.
 /// * - value 9 to 15        - are reserved for future usage. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct VruDeviceUsage(pub u8);


///*
 /// * This DE represents the possible VRU environment conditions.
 /// *
 /// * - The value shall be set to:
 /// * - 0 `unavailable`            - to indicate that the information on the type of environment is unavailable,
 /// * - 1 `intersectionCrossing`   - to indicate that the VRU is on an intersection or crossing,
 /// * - 2 `zebraCrossing`          - to indicate that the VRU is on a  zebra crossing (crosswalk),
 /// * - 3 `sidewalk`               - to indicate that the VRU is on a sidewalk,
 /// * - 4 `onVehicleRoad`          - to indicate that the VRU is on a traffic lane,
 /// * - 5 `protectedGeographicArea`- to indicate that the VRU is in a protected area.
 /// * - value 6 to 15              - are reserved for future usage. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct VruEnvironment(pub u8);


        
        ///*
 /// * This DF represents the status of the exterior light switches of a VRU.
 /// * This DF is an extension of the vehicular DE @ref ExteriorLights.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field vehicular:  represents the status of the exterior light switches of a road vehicle.
 /// * 
 /// * @field vruSpecific: represents the status of the exterior light switches of a VRU.
 /// *
 /// * @category: VRU information
 /// * @revision: created in V2.1.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VruExteriorLights {
            pub vehicular: ExteriorLights,
                    pub vru_specific: VruSpecificExteriorLights,
                    
        }

        impl VruExteriorLights {
        pub fn new(
            vehicular: ExteriorLights,
	vru_specific: VruSpecificExteriorLights,
        ) -> Self {
            Self {
                vehicular,
	vru_specific,
            }
        }
    }

        

///*
 /// *  This DE indicates the status of the possible human control over a VRU vehicle.
 /// *
 /// * The value shall be set to:
 /// * - 0 `unavailable`                 - to indicate that the information is unavailable,
 /// * - 1 `braking`                     - to indicate that the VRU is braking,
 /// * - 2 `hardBraking`                 - to indicate that the VRU is braking hard,
 /// * - 3 `stopPedaling`                - to indicate that the VRU stopped pedaling,
 /// * - 4 `brakingAndStopPedaling`      - to indicate that the VRU stopped pedaling an is braking,
 /// * - 5 `hardBrakingAndStopPedaling`  - to indicate that the VRU stopped pedaling an is braking hard,
 /// * - 6 `noReaction`                  - to indicate that the VRU is not changing its behavior.
 /// * - 7 to 15                         - are reserved for future usage. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1 and range changed from 0..255 to 0..15
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct VruMovementControl(pub u8);


///*
 /// * This DF indicates the profile of a VRU including sub-profile information
 /// * It identifies four options corresponding to the four types of VRU profiles specified in ETSI TS 103 300-2 [18]:
 /// *
 /// * @field pedestrian: VRU Profile 1 - Pedestrian.
 /// *
 /// * @field bicyclistAndLightVruVehicle: VRU Profile  2 - Bicyclist.
 /// *
 /// * @field motorcyclist: VRU Profile 3  - Motorcyclist.
 /// *
 /// * @field animal: VRU Profile  4 -  Animal.
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum VruProfileAndSubprofile {
   Pedestrian(VruSubProfilePedestrian),
     BicyclistAndLightVruVehicle(VruSubProfileBicyclist),
     Motorcyclist(VruSubProfileMotorcyclist),
     Animal(VruSubProfileAnimal),
    
}


///*
 /// * This DE indicates the approximate size of a VRU including the VRU vehicle used.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`    - to indicate that there is no matched size class or due to privacy reasons in profile 1, 
 /// * - 1 `low`            - to indicate that the VRU size class is low depending on the VRU profile,
 /// * - 2 `medium`         - to indicate that the VRU size class is medium depending on the VRU profile,
 /// * - 3 `high`           - to indicate that the VRU size class is high depending on the VRU profile.
 /// * - 4 to 15            - are reserved for future usage. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct VruSizeClass(pub u8);


///*
 /// * This DE describes the status of the exterior light switches of a VRU.
 /// *
 /// * The value of each bit indicates the state of the switch, which commands the corresponding light. 
 /// * The bit corresponding to a specific light shall be set to 1, when the corresponding switch is turned on, either manually by the driver or VRU 
 /// * or automatically by a vehicle or VRU system: 
 /// * - 0 `unavailable`     - indicates no information available, 
 /// * - 1 `backFlashLight ` - indicates the status of the back flash light,
 /// * - 2 `helmetLight`     - indicates the status of the helmet light,
 /// * - 3 `armLight`        - indicates the status of the arm light,
 /// * - 4 `legLight`        - indicates the status of the leg light,
 /// * - 5 `wheelLight`      - indicates the status of the wheel light. 
 /// * - Bits 6 to 8         - are reserved for future use. 
 /// * The bit values do not indicate if the corresponding lamps are alight or not.
 /// * If  VRU is not equipped with a certain light or if the light switch status information is not available, the corresponding bit shall be set to 0.
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("8..=8"))]
        pub struct VruSpecificExteriorLights(pub BitString);


///*
 /// * This DE indicates the profile of an animal
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`     - to indicate that the information  is unavailable,
 /// * - 1 `wild-animal`     - to indicate a animal living in the wildness, 
 /// * - 2 `farm-animal`     - to indicate an animal beloning to a farm,
 /// * - 3 `service-animal`  - to indicate an animal that supports a human being.
 /// * - 4 to 15             - are reserved for future usage. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct VruSubProfileAnimal(pub u8);


///*
 /// * This DE indicates the profile of a VRU and its light VRU vehicle / mounted animal. 
 /// *
 /// * The value shall be set to:
 /// * - 0 `unavailable`           - to indicate that the information  is unavailable,
 /// * - 1 `bicyclist `            - to indicate a cycle and bicyclist,
 /// * - 2 `wheelchair-user`       - to indicate a wheelchair and its user,
 /// * - 3 `horse-and-rider`       - to indicate a horse and rider,
 /// * - 4 `rollerskater`          - to indicate a rolleskater and skater,
 /// * - 5 `e-scooter`             - to indicate an e-scooter and rider,
 /// * - 6 `personal-transporter`  - to indicate a personal-transporter and rider,
 /// * - 7 `pedelec`               - to indicate a pedelec and rider,
 /// * - 8 `speed-pedelec`         - to indicate a speed-pedelec and rider.
 /// * - 9 to 15                   - are reserved for future usage. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct VruSubProfileBicyclist(pub u8);


///*
 /// * This DE indicates the profile of a motorcyclist and corresponding vehicle.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable `                  - to indicate that the information  is unavailable,
 /// * - 1 `moped`                         - to indicate a moped and rider,
 /// * - 2 `motorcycle`                    - to indicate a motorcycle and rider,
 /// * - 3 `motorcycle-and-sidecar-right`  - to indicate a motorcycle with sidecar on the right and rider,
 /// * - 4 `motorcycle-and-sidecar-left`   - to indicate  a motorcycle with sidecar on the left and rider.
 /// * - 5 to 15                           - are reserved for future usage. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct VruSubProfileMotorcyclist(pub u8);


///*
 /// * This DE indicates the profile of a pedestrian.
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`             - to indicate that the information on is unavailable,
 /// * - 1 `ordinary-pedestrian`     - to indicate a pedestrian to which no more-specific profile applies,
 /// * - 2 `road-worker`             - to indicate a pedestrian with the role of a road worker,
 /// * - 3 `first-responder`         - to indicate a pedestrian with the role of a first responder.
 /// * - value 4 to 15               - are reserved for future usage. 
 /// *
 /// * @category: VRU information
 /// * @revision: Created in V2.1.1, type changed from ENUMERATED to INTEGER in V2.2.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct VruSubProfilePedestrian(pub u8);


///*
 /// * This DE represents the World Manufacturer Identifier (WMI). The values are assigned according to ISO 3779 [6].
 /// * 
 /// *
 /// * @category: Vehicle information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=3"))]
        pub struct WMInumber(pub Ia5String);


        
        ///* 
 /// * This DF represents an angular component along with a confidence value in the WGS84 coordinate system.
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// *
 /// * It shall include the following components: 
 /// *
 /// * @field value: the angle value, which can be estimated as the mean of the current distribution.
 /// *
 /// * @field confidence: the confidence value associated to the angle value.
 /// *
 /// * @category: GeoReference information
 /// * @revision: Created in V2.1.1
 ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Wgs84Angle {
            pub value: Wgs84AngleValue,
                    pub confidence: Wgs84AngleConfidence,
                    
        }

        impl Wgs84Angle {
        pub fn new(
            value: Wgs84AngleValue,
	confidence: Wgs84AngleConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

///* 
 /// * This DE indicates the angle confidence value which represents the estimated accuracy of an angle value with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// * 
 /// * The value shall be set to:
 /// * - `n` (`n >= 1` and `n < 126`) if the confidence value is equal to or less than n x 0,1 degrees and more than (n-1) x 0,1 degrees,
 /// * - `126` if the confidence value is out of range, i.e. greater than 12,5 degrees,
 /// * - `127` if the confidence value is not available.
 /// *
 /// *
 /// * @unit 0,1 degrees
 /// * @category: GeoReference Information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=127"))]
        pub struct Wgs84AngleConfidence(pub u8);


///* 
 /// * This DE represents an angle value in degrees described in the WGS84 reference system with respect to the WGS84 north.
 /// * The specific WGS84 coordinate system is specified by the corresponding standards applying this DE.
 /// * When the information is not available, the DE shall be set to 3 601. The value 3600 shall not be used.
 /// *
 /// * @unit 0,1 degrees
 /// * @category: GeoReference Information
 /// * @revision: Created in V2.1.1
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3601"))]
        pub struct Wgs84AngleValue(pub u16);


///*
 /// * This DE indicates the perpendicular distance between front and rear axle of the wheel base of vehicle.
 /// *
 /// * The value shall be set to:
 /// * - `n` (`n >= 1` and `n < 126`) if the value is equal to or less than n x 0,1 metre  and more than (n-1) x 0,1 metre,
 /// * - `126` indicates that the wheel base distance is equal to or greater than 12,5 metres,
 /// * - `127` indicates that the information is unavailable.
 /// *
 /// * @unit 0,1 metre
 /// * @category: Vehicle information
 /// * @revision: Created in V2.1.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=127"))]
        pub struct WheelBaseVehicle(pub u8);


///*
 /// * This DE represents the sub cause codes of the @ref CauseCode `wrongWayDriving` .
 /// * 
 /// * The value shall be set to:
 /// * - 0 `unavailable`    - in case further detailed information on wrong way driving event is unavailable,
 /// * - 1 `wrongLane`      - in case vehicle is driving on a lane for which it has no authorization to use,
 /// * - 2 `wrongDirection` - in case vehicle is driving in a direction that it is not allowed,
 /// * - 3-255              - reserved for future usage.
 /// * 
 /// * @category: Traffic information
 /// * @revision: V1.3.1
 /// 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct WrongWayDrivingSubCauseCode(pub u8);


        
        ///*
 /// * This DF represents a yaw rate of vehicle at a point in time.
 /// *
 /// * It shall include the following components: 
 /// * 
 /// * @field yawRateValue: yaw rate value at a point in time.
 /// * 
 /// * @field yawRateConfidence: confidence value associated to the yaw rate value.
 /// * 
 /// * @category: Vehicle Information
 /// * @revision: V1.3.1
 /// 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct YawRate {
            pub yaw_rate_value: YawRateValue,
                    pub yaw_rate_confidence: YawRateConfidence,
                    
        }

        impl YawRate {
        pub fn new(
            yaw_rate_value: YawRateValue,
	yaw_rate_confidence: YawRateConfidence,
        ) -> Self {
            Self {
                yaw_rate_value,
	yaw_rate_confidence,
            }
        }
    }

        

///*
 /// * This DE indicates the yaw rate confidence value which represents the estimated accuracy for a yaw rate value with a default confidence level of 95 %.
 /// * If required, the confidence level can be defined by the corresponding standards applying this DE.
 /// * 
 /// * The value shall be set to:
 /// * - `0` if the confidence value is equal to or less than 0,01 degree/second,
 /// * - `1` if the confidence value is equal to or less than 0,05 degrees/second or greater than 0,01 degree/second,
 /// * - `2` if the confidence value is equal to or less than 0,1 degree/second or greater than 0,05 degree/second,
 /// * - `3` if the confidence value is equal to or less than 1 degree/second or greater than 0,1 degree/second,
 /// * - `4` if the confidence value is equal to or less than 5 degrees/second or greater than 1 degrees/second,
 /// * - `5` if the confidence value is equal to or less than 10 degrees/second or greater than 5 degrees/second,
 /// * - `6` if the confidence value is equal to or less than 100 degrees/second or greater than 10 degrees/second,
 /// * - `7` if the confidence value is out of range, i.e. greater than 100 degrees/second,
 /// * - `8` if the confidence value is unavailable.
 /// * 
 /// * NOTE: The fact that a yaw rate value is received with confidence value set to `unavailable(8)` can be caused by
 /// * several reasons, such as:
 /// * - the sensor cannot deliver the accuracy at the defined confidence level because it is a low-end sensor,
 /// * - the sensor cannot calculate the accuracy due to lack of variables, or
 /// * - there has been a vehicle bus (e.g. CAN bus) error.
 /// * In all 3 cases above, the yaw rate value may be valid and used by the application.
 /// * 
 /// * If a yaw rate value is received and its confidence value is set to `outOfRange(7)`, it means that the 
 /// * yaw rate value is not valid and therefore cannot be trusted. Such value is not useful the application.
 /// * 
 /// * @category: Vehicle information
 /// * @revision: Description revised in V2.1.1
 /// 

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


///*
 /// * This DE represents the vehicle rotation around z-axis of the coordinate system centred on the centre of mass of the empty-loaded
 /// * vehicle. The leading sign denotes the direction of rotation.
 /// * 
 /// * The value shall be provided in the vehicle coordinate system as defined in ISO 8855 [21], clause 2.11.
 /// *
 /// * The value shall be set to:
 /// * - `-32 766` to indicate that the yaw rate is equal to or greater than 327,66 degrees/second to the right,
 /// * - `n` (`n > -32 766` and `n <= 0`) to indicate that the rotation is clockwise (i.e. to the right) and is equal to or less than n x 0,01 degrees/s, 
 ///      and greater than (n-1) x 0,01 degrees/s,
 /// * - `n` (`n > 0` and `n < 32 766`) to indicate that the rotation is anti-clockwise (i.e. to the left) and is equal to or less than n x 0,01 degrees/s, 
 ///      and greater than (n-1) x 0,01 degrees/s,
 /// * - `32 766` to indicate that the yaw rate is greater than 327.65 degrees/second to the left,
 /// * - `32 767` to indicate that the information is not available.
 /// * 
 /// * The yaw rate value shall be a raw data value, i.e. not filtered, smoothed or otherwise modified.
 /// * The reading instant should be the same as for the vehicle acceleration.
 /// * 
 /// * @note: The empty load vehicle is defined in ISO 1176 [8], clause 4.6.
 /// * 
 /// * @unit: 0,01 degree per second. 
 /// * @category: Vehicle Information
 /// * @revision: Desription revised in V2.1.1 (the meaning of 32766 has changed slightly). 
 ///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-32766..=32767"))]
        pub struct YawRateValue(pub i16);


            // =====================================================
            // Etsi-Schema
            // { iso(1) standard(0) signalizedIntersection(19091) profilec(2) addgrpc(0) version2(2) }
            // =====================================================
            
        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ACKResponseService {
            #[rasn(value("-32768..=32767"))]
        pub ack_resp_delay_adjust: i16,
                    #[rasn(value("0..=65535"))]
        pub ack_resp_delay_std_dev: u16,
                    
        }

        impl ACKResponseService {
        pub fn new(
            ack_resp_delay_adjust: i16,
	ack_resp_delay_std_dev: u16,
        ) -> Self {
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
        pub fn new(
            latitude: Latitude,
	longitude: Longitude,
        ) -> Self {
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
        pub fn new(
            latitude: Latitude,
	longitude: Longitude,
	altitude: Altitude,
        ) -> Self {
            Self {
                latitude,
	longitude,
	altitude,
            }
        }
    }

        

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct AbsolutePositions(pub SequenceOf<AbsolutePosition>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct AbsolutePositionsWithAltitude(pub SequenceOf<AbsolutePositionWAltitude>);



#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum AccelOrDecel {
     Accelerate = 0,
                 Decelerate = 1,
                
}



#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum AckNackIndication {
     ACK = 0,
                 NACK = 1,
                
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=127"))]
        pub struct ActionDeltaTime(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=32"))]
        pub struct AdvertiserIdentifier(pub Utf8String);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct AdvertiserPermissions(pub SequenceOf<ChannelIdentifier>);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct AdvisorySpeedRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct AdvisorySpeed {
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
#[rasn(delegate,size("1..=16"))]
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
#[rasn(delegate,size("12..=12"))]
        pub struct AllowedManeuvers(pub BitString);




#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
                #[non_exhaustive]
pub enum AmbientOrRoadConditionPictogram {
     AmbientCondition = 0,
                 RoadCondition = 1,
                
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=28800"))]
        pub struct Angle(pub u16);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct AnimalSubclass {
            #[rasn(default = "animal_subclass_r_type_default")]
        pub r_type: AnimalSubclassType,
                    #[rasn(default = "animal_subclass_confidence_default")]
        pub confidence: ClassConfidence,
                    
        }

        impl AnimalSubclass {
        pub fn new(
            r_type: AnimalSubclassType,
	confidence: ClassConfidence,
        ) -> Self {
            Self {
                r_type,
	confidence,
            }
        }
    }

        fn animal_subclass_r_type_default() -> AnimalSubclassType {
                AnimalSubclassType(0).into()
            }
            
            fn animal_subclass_confidence_default() -> ClassConfidence {
                ClassConfidence(0).into()
            }
            
            


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct AnimalSubclassType(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct AntennaOffsetSet {
            pub ant_offset_x: OffsetB12,
                    pub ant_offset_y: OffsetB09,
                    pub ant_offset_z: OffsetB10,
                    
        }

        impl AntennaOffsetSet {
        pub fn new(
            ant_offset_x: OffsetB12,
	ant_offset_y: OffsetB09,
	ant_offset_z: OffsetB10,
        ) -> Self {
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
#[rasn(delegate,value("0..=15"))]
        pub struct ApproachID(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct AreaCircular {
            pub node_center_point: Option<OffsetPoint>,
                    pub radius: Radius,
                    
        }

        impl AreaCircular {
        pub fn new(
            node_center_point: Option<OffsetPoint>,
	radius: Radius,
        ) -> Self {
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
        pub fn new(
            poly_point_list: PolyPointList,
        ) -> Self {
            Self {
                poly_point_list,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct AreaRadial {
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
#[rasn(delegate,value("0..=127", extensible))]
        pub struct AnonymousAttributeIdList(pub Integer);


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("0..=127", extensible))]
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
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum BasePublicEncryptionKey {
   EciesNistP256(EccP256CurvePoint),
     EciesBrainpoolP256r1(EccP256CurvePoint),
    
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
#[rasn(delegate,size("0..=31"))]
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
        pub fn new(
            ssp_value: OctetString,
	ssp_bitmask: OctetString,
        ) -> Self {
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
        pub fn new(
            header: ItsPduHeader,
	cam: CoopAwareness,
        ) -> Self {
            Self {
                header,
	cam,
            }
        }
    }

        


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CPM {
            pub header: ItsPduHeader,
                    pub cpm: CollectivePerceptionMessage,
                    
        }

        impl CPM {
        pub fn new(
            header: ItsPduHeader,
	cpm: CollectivePerceptionMessage,
        ) -> Self {
            Self {
                header,
	cpm,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CS5 {
            pub vin: VisibleString,
                    #[rasn(size("9..=9"))]
        pub fill: BitString,
                    
        }

        impl CS5 {
        pub fn new(
            vin: VisibleString,
	fill: BitString,
        ) -> Self {
            Self {
                vin,
	fill,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct CamParameters {
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
#[rasn(delegate,value("0..=255"))]
        pub struct ChannelAccess80211(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ChannelIdentifier {
            #[rasn(size("3..=3"))]
        pub country_string: OctetString,
                    pub operating_class: Uint8,
                    pub channel_number: Uint8,
                    
        }

        impl ChannelIdentifier {
        pub fn new(
            country_string: OctetString,
	operating_class: Uint8,
	channel_number: Uint8,
        ) -> Self {
            Self {
                country_string,
	operating_class,
	channel_number,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=31"))]
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
#[rasn(delegate,value("0..=255"))]
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
        
                #[non_exhaustive]pub struct ChannelSpecificProviderPermission {
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
        pub fn new(
            center: Dot2TwoDLocation,
	radius: Uint16,
        ) -> Self {
            Self {
                center,
	radius,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=101"))]
        pub struct ClassConfidence(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum ClusterBoundingBoxShape {
   ClusterRectangle(AreaRectangle),
     ClusterCircle(AreaCircular),
     ClusterPolygon(AreaPolygon),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct ClusterId(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("4..=4"))]
        pub struct ClusterProfiles(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum Code {
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
        pub fn new(
            generation_delta_time: GenerationDeltaTime,
	cpm_parameters: CpmParameters,
        ) -> Self {
            Self {
                generation_delta_time,
	cpm_parameters,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3"))]
        pub struct ComparisonOperator(pub u8);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=3"))]
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
#[rasn(delegate,size("1..=4"))]
        pub struct ComputedLaneRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ComputedLane {
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
        pub fn new(
            lane: LaneID,
	maneuver: Option<AllowedManeuvers>,
        ) -> Self {
            Self {
                lane,
	maneuver,
            }
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
#[rasn(delegate,size("1..=4"))]
        pub struct ConnectionManeuverAssistRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ConnectionManeuverAssist {
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
        
                #[non_exhaustive]pub struct ConnectionManeuverAssistAddGrpC {
            pub its_station_position: Option<ItsStationPositionList>,
                    
        }

        impl ConnectionManeuverAssistAddGrpC {
        pub fn new(
            its_station_position: Option<ItsStationPositionList>,
        ) -> Self {
            Self {
                its_station_position,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ConnectionTrajectoryAddGrpC {
            pub nodes: NodeSetXY,
                    pub connection_i_d: LaneConnectionID,
                    
        }

        impl ConnectionTrajectoryAddGrpC {
        pub fn new(
            nodes: NodeSetXY,
	connection_i_d: LaneConnectionID,
        ) -> Self {
            Self {
                nodes,
	connection_i_d,
            }
        }
    }

        

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16"))]
        pub struct ConnectsToList(pub SequenceOf<Connection>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct CoopAwareness {
            pub generation_delta_time: GenerationDeltaTime,
                    pub cam_parameters: CamParameters,
                    
        }

        impl CoopAwareness {
        pub fn new(
            generation_delta_time: GenerationDeltaTime,
	cam_parameters: CamParameters,
        ) -> Self {
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
        pub fn new(
            country_only: CountryOnly,
	regions: SequenceOfUint8,
        ) -> Self {
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
        pub fn new(
            country: CountryOnly,
	region_and_subregions: SequenceOfRegionAndSubregions,
        ) -> Self {
            Self {
                country,
	region_and_subregions,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct CountryOnly(pub Uint16);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct CpmManagementContainer {
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
        
                #[non_exhaustive]pub struct CpmParameters {
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
#[rasn(delegate,value("0..=255"))]
        pub struct CtxRef(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct DBV(pub Distance);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
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
        pub fn new(
            dcj: Option<u8>,
	dcr: Option<u8>,
	tpl: Option<u8>,
	io_list: DDDIoList,
        ) -> Self {
            Self {
                dcj,
	dcr,
	tpl,
	io_list,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15", extensible))]
        pub struct DDDDEP(pub Integer);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15", extensible))]
        pub struct DDDDER(pub Integer);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
        pub struct DDDIODp(pub SequenceOf<DestinationPlace>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
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
#[rasn(delegate,value("0..=31"))]
        pub struct DDay(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=8"))]
        pub struct DFL(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=31"))]
        pub struct DHour(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=60"))]
        pub struct DMinute(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=12"))]
        pub struct DMonth(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-840..=840"))]
        pub struct DOffset(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=32767"))]
        pub struct DSRCmsgID(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
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
#[rasn(delegate,value("0..=4095"))]
        pub struct DYear(pub u16);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct DangerousGoodsContainer {
            pub dangerous_goods_basic: DangerousGoodsBasic,
                    
        }

        impl DangerousGoodsContainer {
        pub fn new(
            dangerous_goods_basic: DangerousGoodsBasic,
        ) -> Self {
            Self {
                dangerous_goods_basic,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct DataParameters {
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
#[rasn(delegate,value("0..=255"))]
        pub struct DataRate80211(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("8..=8"))]
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
#[rasn(delegate,value("-150..=150"))]
        pub struct DeltaAngle(pub i16);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct DeltaPosition {
            pub delta_latitude: DeltaLatitude,
                    pub delta_longitude: DeltaLongitude,
                    
        }

        impl DeltaPosition {
        pub fn new(
            delta_latitude: DeltaLatitude,
	delta_longitude: DeltaLongitude,
        ) -> Self {
            Self {
                delta_latitude,
	delta_longitude,
            }
        }
    }

        

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=32", extensible))]
        pub struct DeltaPositions(pub SequenceOf<DeltaPosition>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=32", extensible))]
        pub struct DeltaPositionsWithAltitude(pub SequenceOf<DeltaReferencePosition>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-122..=121"))]
        pub struct DeltaTime(pub i8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=63"))]
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
        pub fn new(
            der_type: DDDDER,
	ron_id: Option<u16>,
	ron_text: Option<Utf8String>,
        ) -> Self {
            Self {
                der_type,
	ron_id,
	ron_text,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum DetectionArea {
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
        pub fn new(
            particulate: Particulate,
	absorption_coeff: Int2,
        ) -> Self {
            Self {
                particulate,
	absorption_coeff,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Distance {
            #[rasn(value("1..=16384"))]
        pub value: u16,
                    pub unit: RSCUnit,
                    
        }

        impl Distance {
        pub fn new(
            value: u16,
	unit: RSCUnit,
        ) -> Self {
            Self {
                value,
	unit,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=102"))]
        pub struct DistanceConfidence(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct DistanceOrDuration {
            #[rasn(value("1..=16384"))]
        pub value: u16,
                    pub unit: RSCUnit,
                    
        }

        impl DistanceOrDuration {
        pub fn new(
            value: u16,
	unit: RSCUnit,
        ) -> Self {
            Self {
                value,
	unit,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-132768..=132767"))]
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
        pub fn new(
            latitude: Dot2Latitude,
	longitude: Dot2Longitude,
	elevation: Dot2Elevation,
        ) -> Self {
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
        pub fn new(
            latitude: Dot2Latitude,
	longitude: Dot2Longitude,
        ) -> Self {
            Self {
                latitude,
	longitude,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-32767..=32767"))]
        pub struct DrivenLineOffsetLg(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-2047..=2047"))]
        pub struct DrivenLineOffsetSm(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3"))]
        pub struct DriverCharacteristics(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct DtmHourMinutes {
            pub shm: HoursMinutes,
                    pub ehm: HoursMinutes,
                    
        }

        impl DtmHourMinutes {
        pub fn new(
            shm: HoursMinutes,
	ehm: HoursMinutes,
        ) -> Self {
            Self {
                shm,
	ehm,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct DtmMonthDay {
            pub smd: MonthDay,
                    pub emd: MonthDay,
                    
        }

        impl DtmMonthDay {
        pub fn new(
            smd: MonthDay,
	emd: MonthDay,
        ) -> Self {
            Self {
                smd,
	emd,
            }
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
        pub fn new(
            syr: u16,
	eyr: u16,
        ) -> Self {
            Self {
                syr,
	eyr,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
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
#[rasn(delegate,value("0..=2"))]
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
        pub fn new(
            poi_header: ItsPOIHeader,
	evcsn_data: ItsEVCSNData,
        ) -> Self {
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
            #[rasn(size("32..=32"))]
        pub x: OctetString,
                    #[rasn(size("32..=32"))]
        pub y: OctetString,
                    
        }

        impl EccP256CurvePointUncompressedP256 {
        pub fn new(
            x: OctetString,
	y: OctetString,
        ) -> Self {
            Self {
                x,
	y,
            }
        }
    }

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum EccP256CurvePoint {
  #[rasn(size("32..=32"))]
         XOnly(OctetString),
     Fill(()),
    #[rasn(size("32..=32"))]
         CompressedY0(OctetString),
    #[rasn(size("32..=32"))]
         CompressedY1(OctetString),
     UncompressedP256(EccP256CurvePointUncompressedP256),
    
}


        
        /// Inner type 

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct EccP384CurvePointUncompressedP384 {
            #[rasn(size("48..=48"))]
        pub x: OctetString,
                    #[rasn(size("48..=48"))]
        pub y: OctetString,
                    
        }

        impl EccP384CurvePointUncompressedP384 {
        pub fn new(
            x: OctetString,
	y: OctetString,
        ) -> Self {
            Self {
                x,
	y,
            }
        }
    }

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum EccP384CurvePoint {
  #[rasn(size("48..=48"))]
         XOnly(OctetString),
     Fill(()),
    #[rasn(size("48..=48"))]
         CompressedY0(OctetString),
    #[rasn(size("48..=48"))]
         CompressedY1(OctetString),
     UncompressedP384(EccP384CurvePointUncompressedP384),
    
}


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct EcdsaP256Signature {
            pub r_sig: EccP256CurvePoint,
                    #[rasn(size("32..=32"))]
        pub s_sig: OctetString,
                    
        }

        impl EcdsaP256Signature {
        pub fn new(
            r_sig: EccP256CurvePoint,
	s_sig: OctetString,
        ) -> Self {
            Self {
                r_sig,
	s_sig,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct EcdsaP384Signature {
            pub r_sig: EccP384CurvePoint,
                    #[rasn(size("48..=48"))]
        pub s_sig: OctetString,
                    
        }

        impl EcdsaP384Signature {
        pub fn new(
            r_sig: EccP384CurvePoint,
	s_sig: OctetString,
        ) -> Self {
            Self {
                r_sig,
	s_sig,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct EciesP256EncryptedKey {
            pub v: EccP256CurvePoint,
                    #[rasn(size("16..=16"))]
        pub c: OctetString,
                    #[rasn(size("16..=16"))]
        pub t: OctetString,
                    
        }

        impl EciesP256EncryptedKey {
        pub fn new(
            v: EccP256CurvePoint,
	c: OctetString,
	t: OctetString,
        ) -> Self {
            Self {
                v,
	c,
	t,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum EdcaIdentifier {
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
#[rasn(delegate,value("-4096..=61439"))]
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
#[rasn(delegate,size("1..=16"))]
        pub struct EnabledLaneList(pub SequenceOf<LaneID>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum EncryptionKey {
   Public(PublicEncryptionKey),
     Symmetric(SymmetricEncryptionKey),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
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
        pub fn new(
            euro_value: EuroValue,
	cop_value: CopValue,
        ) -> Self {
            Self {
                euro_value,
	cop_value,
            }
        }
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
        pub fn new(
            header: ItsPduHeader,
	evcsn: EVChargingSpotNotificationPOIMessage,
        ) -> Self {
            Self {
                header,
	evcsn,
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
                    #[rasn(size("4..=4"))]
        pub set1: OctetString,
                    #[rasn(size("4..=4"))]
        pub set2: OctetString,
                    #[rasn(size("4..=4"))]
        pub set3: OctetString,
                    #[rasn(size("4..=4"))]
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
        #[rasn(automatic_tags)]
        pub struct ExtendedChannelInfo {
            pub med_id: MedType,
                    pub value: Any,
                    
        }

        impl ExtendedChannelInfo {
        pub fn new(
            med_id: MedType,
	value: Any,
        ) -> Self {
            Self {
                med_id,
	value,
            }
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
        pub fn new(
            extension_id: RefExt,
	value: Any,
        ) -> Self {
            Self {
                extension_id,
	value,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct FirstIntersection {
            #[rasn(value("1..=32"))]
        pub number_of_intersections: u8,
                    pub partial_intersection: PartialIntersection,
                    
        }

        impl FirstIntersection {
        pub fn new(
            number_of_intersections: u8,
	partial_intersection: PartialIntersection,
        ) -> Self {
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
        
                #[non_exhaustive]pub struct FreeSpaceAddendum {
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
#[rasn(delegate,size("1..=128", extensible))]
        pub struct FreeSpaceAddendumContainer(pub SequenceOf<FreeSpaceAddendum>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum FreeSpaceArea {
   FreeSpacePolygon(AreaPolygon),
     FreeSpaceCircular(AreaCircular),
     FreeSpaceEllipse(AreaEllipse),
     FreeSpaceRectangle(AreaRectangle),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=101"))]
        pub struct FreeSpaceConfidence(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=50"))]
        pub struct FrontOverhang(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct FuelType(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct FullPositionVector {
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
#[rasn(delegate,size("8..=8"))]
        pub struct GNSSstatus(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct GatewayMacAddress(pub MACaddress);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct GeneralIviContainer(pub SequenceOf<GicPart>);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct GenericLaneRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct GenericLane {
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
#[rasn(delegate,size("1..=16", extensible))]
        pub struct GeographicLocationContainerParts(pub SequenceOf<GlcPart>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct GeographicLocationContainer {
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

        

///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum GeographicRegion {
   CircularRegion(CircularRegion),
     RectangularRegion(SequenceOfRectangularRegion),
     PolygonalRegion(PolygonalRegion),
     IdentifiedRegion(SequenceOfIdentifiedRegion),
    
}


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct GicPartDetectionZoneIds(pub SequenceOf<Zid>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct GicPartRelevanceZoneIds(pub SequenceOf<Zid>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct GicPartDriverAwarenessZoneIds(pub SequenceOf<Zid>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct GicPartApplicableLanes(pub SequenceOf<LanePosition>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct GicPartVehicleCharacteristics(pub SequenceOf<CompleteVehicleCharacteristics>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
        pub struct GicPartRoadSignCodes(pub SequenceOf<RSCode>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
        pub struct GicPartExtraText(pub SequenceOf<Text>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct GicPart {
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
        
                #[non_exhaustive]pub struct GlcPart {
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
#[rasn(delegate,value("0..=15", extensible))]
        pub struct GoodsType(pub Integer);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct GroupLinkageValue {
            #[rasn(size("4..=4"))]
        pub j_value: OctetString,
                    #[rasn(size("9..=9"))]
        pub value: OctetString,
                    
        }

        impl GroupLinkageValue {
        pub fn new(
            j_value: OctetString,
	value: OctetString,
        ) -> Self {
            Self {
                j_value,
	value,
            }
        }
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
#[rasn(delegate,size("10..=10"))]
        pub struct HashedId10(pub OctetString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("3..=3"))]
        pub struct HashedId3(pub OctetString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("8..=8"))]
        pub struct HashedId8(pub OctetString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum HighFrequencyContainer {
   BasicVehicleContainerHighFrequency(BasicVehicleContainerHighFrequency),
     RsuContainerHighFrequency(RSUContainerHighFrequency),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=100"))]
        pub struct HitchPointOffset(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("0..=255"))]
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
        pub fn new(
            hours: u8,
	mins: u8,
        ) -> Self {
            Self {
                hours,
	mins,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct IPv6Address(pub OctetString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
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
#[rasn(delegate,size("1..=8", extensible))]
        pub struct ISO14823Attributes(pub SequenceOf<ISO14823Attribute>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ISO14823Code {
            pub pictogram_code: PictogramCode,
                    pub attributes: Option<ISO14823Attributes>,
                    
        }

        impl ISO14823Code {
        pub fn new(
            pictogram_code: PictogramCode,
	attributes: Option<ISO14823Attributes>,
        ) -> Self {
            Self {
                pictogram_code,
	attributes,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ITSRangingSAMAppData {
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
#[rasn(delegate,value("0..=1023"))]
        pub struct IVILaneWidth(pub u16);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct IVIM {
            pub header: ItsPduHeader,
                    pub ivi: IviStructure,
                    
        }

        impl IVIM {
        pub fn new(
            header: ItsPduHeader,
	ivi: IviStructure,
        ) -> Self {
            Self {
                header,
	ivi,
            }
        }
    }

        

        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8"))]
        pub struct IVIManagementContainerConnectedIviStructures(pub SequenceOf<IviIdentificationNumber>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct IVIManagementContainer {
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

        

///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct IValue(pub Uint16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum IdentifiedRegion {
   CountryOnly(CountryOnly),
     CountryAndRegions(CountryAndRegions),
     CountryAndSubregions(CountryAndSubregions),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct Identifier(pub u8);



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
#[rasn(delegate,value("0..=255"))]
        pub struct Int1(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct Int2(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum IntersectionAccessPoint {
   Lane(LaneID),
     Approach(ApproachID),
     Connection(LaneConnectionID),
    
}


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct IntersectionGeometryRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct IntersectionGeometry {
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
#[rasn(delegate,size("1..=32"))]
        pub struct IntersectionGeometryList(pub SequenceOf<IntersectionGeometry>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct IntersectionID(pub u16);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct IntersectionReferenceID {
            pub region: Option<RoadRegulatorID>,
                    pub id: IntersectionID,
                    
        }

        impl IntersectionReferenceID {
        pub fn new(
            region: Option<RoadRegulatorID>,
	id: IntersectionID,
        ) -> Self {
            Self {
                region,
	id,
            }
        }
    }

        

        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct IntersectionStateRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct IntersectionState {
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
        
                #[non_exhaustive]pub struct IntersectionStateAddGrpC {
            pub active_prioritizations: Option<PrioritizationResponseList>,
                    
        }

        impl IntersectionStateAddGrpC {
        pub fn new(
            active_prioritizations: Option<PrioritizationResponseList>,
        ) -> Self {
            Self {
                active_prioritizations,
            }
        }
    }

        

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=32"))]
        pub struct IntersectionStateList(pub SequenceOf<IntersectionState>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct IntersectionStatusObject(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct IpV6Prefix(pub OctetString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct IpV6PrefixLength(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ItsAidCtxRef {
            pub itsaid: ITSaid,
                    pub ctx: CtxRef,
                    
        }

        impl ItsAidCtxRef {
        pub fn new(
            itsaid: ITSaid,
	ctx: CtxRef,
        ) -> Self {
            Self {
                itsaid,
	ctx,
            }
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
#[rasn(delegate,size("1..=16"))]
        pub struct ItsChargingSpots(pub SequenceOf<ItsChargingSpotDataElements>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ItsChargingStationData {
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
#[rasn(delegate,size("1..=256"))]
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
        pub fn new(
            poi_type: POIType,
	time_stamp: TimestampIts,
	relay_capable: bool,
        ) -> Self {
            Self {
                poi_type,
	time_stamp,
	relay_capable,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct ItsStationPosition {
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
#[rasn(delegate,size("1..=5"))]
        pub struct ItsStationPositionList(pub SequenceOf<ItsStationPosition>);


///Definition of Containers

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum IviContainer {
   Glc(GeographicLocationContainer),
     Giv(GeneralIviContainer),
     Rcc(RoadConfigurationContainer),
     Tc(TextContainer),
     Lac(LayoutContainer),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3"))]
        pub struct IviPurpose(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=7"))]
        pub struct IviStatus(pub u8);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct IviStructureOptional(pub SequenceOf<IviContainer>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct IviStructure {
            pub mandatory: IVIManagementContainer,
                    pub optional: Option<IviStructureOptional>,
                    
        }

        impl IviStructure {
        pub fn new(
            mandatory: IVIManagementContainer,
	optional: Option<IviStructureOptional>,
        ) -> Self {
            Self {
                mandatory,
	optional,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=7"))]
        pub struct IviType(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,value("-900000000..=900000000"))]
        pub struct KnownLatitude(pub NinetyDegreeInt);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,value("-1799999999..=1800000000"))]
        pub struct KnownLongitude(pub OneEightyDegreeInt);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct LMchannelBusyRatio(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("2..=2"))]
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
#[rasn(delegate,size("16..=16"))]
        pub struct LaneAttributesBarrier(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct LaneAttributesBike(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct LaneAttributesCrosswalk(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct LaneAttributesParking(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct LaneAttributesSidewalk(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct LaneAttributesStriping(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("16..=16"))]
        pub struct LaneAttributesTrackedVehicle(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("8..=8", extensible))]
        pub struct LaneAttributesVehicle(pub BitString);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct LaneAttributesAddGrpC {
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
#[rasn(delegate,value("0..=255"))]
        pub struct LaneConnectionID(pub u8);


        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct LaneDataAttributeRegional(pub SequenceOf<RegionalExtension>);


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum LaneDataAttribute {
   PathEndPointAngle(DeltaAngle),
     LaneCrownPointCenter(RoadwayCrownAngle),
     LaneCrownPointLeft(RoadwayCrownAngle),
     LaneCrownPointRight(RoadwayCrownAngle),
     LaneAngle(MergeDivergeNodeAngle),
     SpeedLimits(SpeedLimitList),
     Regional(LaneDataAttributeRegional),
    
}


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8"))]
        pub struct LaneDataAttributeList(pub SequenceOf<LaneDataAttribute>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("2..=2"))]
        pub struct LaneDirection(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct LaneID(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct LaneInformation {
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
#[rasn(delegate,size("1..=255"))]
        pub struct LaneList(pub SequenceOf<GenericLane>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("10..=10"))]
        pub struct LaneSharing(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=7", extensible))]
        pub struct LaneStatus(pub Integer);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum LaneTypeAttributes {
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
#[rasn(delegate,value("0..=100"))]
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
#[rasn(delegate,size("1..=4", extensible))]
        pub struct LayoutContainerLayoutComponents(pub SequenceOf<LayoutComponent>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct LayoutContainer {
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
#[rasn(delegate,size("16..=16"))]
        pub struct LinkageSeed(pub OctetString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("9..=9"))]
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
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum LowFrequencyContainer {
   BasicVehicleContainerLowFrequency(BasicVehicleContainerLowFrequency),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("6..=6"))]
        pub struct MACaddress(pub OctetString);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct MAPEM {
            pub header: ItsPduHeader,
                    pub map: MapData,
                    
        }

        impl MAPEM {
        pub fn new(
            header: ItsPduHeader,
	map: MapData,
        ) -> Self {
            Self {
                header,
	map,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MCDMApplicationContainer {
            
        }

        impl MCDMApplicationContainer {
        pub fn new(
            
        ) -> Self {
            Self {
                
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MCDMLocationContainer {
            pub event_position: ReferencePosition,
                    
        }

        impl MCDMLocationContainer {
        pub fn new(
            event_position: ReferencePosition,
        ) -> Self {
            Self {
                event_position,
            }
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
        
                #[non_exhaustive]pub struct MCDMManagementContainer {
            pub action_i_d: ActionID,
                    pub request: Option<RequestResponseIndication>,
                    pub ack: Option<AckNackIndication>,
                    pub detection_time: Option<TimestampIts>,
                    pub reference_time: TimestampIts,
                    pub linked_denm: Option<ActionID>,
                    pub validity_duration: Option<ValidityDuration>,
                    pub station_type: Option<StationType>,
                    #[rasn(value("0..=4294967295"),default = "m_c_d_m_management_container_number_of_m_d_us_default")]
        pub number_of_m_d_us: u32,
                    #[rasn(value("1..=4294967295"),default = "m_c_d_m_management_container_number_of_p_d_us_default")]
        pub number_of_p_d_us: u32,
                    #[rasn(value("1..=4294967295"),default = "m_c_d_m_management_container_pdu_sequence_number_default")]
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
                1.into()
            }
            
            fn m_c_d_m_management_container_number_of_p_d_us_default() -> u32 {
                1.into()
            }
            
            fn m_c_d_m_management_container_pdu_sequence_number_default() -> u32 {
                1.into()
            }
            
            fn m_c_d_m_management_container_real_time_default() -> bool {
                false.into()
            }
            
            

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=7"))]
        pub struct MCDMMultimediaContainer(pub SequenceOf<MultimediaDataUnit>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MCDMSituationContainer {
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
#[rasn(delegate)]
        pub struct MandAppCtx(pub SequenceOf<ItsAidCtxRef>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16"))]
        pub struct ManeuverAssistList(pub SequenceOf<ConnectionManeuverAssist>);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct MapDataRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MapData {
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
        
                #[non_exhaustive]pub struct MapDataAddGrpC {
            pub signal_head_locations: Option<SignalHeadLocationList>,
                    
        }

        impl MapDataAddGrpC {
        pub fn new(
            signal_head_locations: Option<SignalHeadLocationList>,
        ) -> Self {
            Self {
                signal_head_locations,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MatchedPosition {
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
        pub fn new(
            header: ItsPduHeader,
	mcdm_info: McdmInfo,
        ) -> Self {
            Self {
                header,
	mcdm_info,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
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
#[rasn(delegate,value("-180..=180"))]
        pub struct MergeDivergeNodeAngle(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=527040"))]
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
        pub fn new(
            month: u8,
	day: u8,
        ) -> Self {
            Self {
                month,
	day,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MotorcylistSpecialContainer {
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
#[rasn(delegate,size("1..=4"))]
        pub struct MovementEventRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MovementEvent {
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
        
                #[non_exhaustive]pub struct MovementEventAddGrpC {
            pub state_change_reason: Option<ExceptionalCondition>,
                    
        }

        impl MovementEventAddGrpC {
        pub fn new(
            state_change_reason: Option<ExceptionalCondition>,
        ) -> Self {
            Self {
                state_change_reason,
            }
        }
    }

        

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16"))]
        pub struct MovementEventList(pub SequenceOf<MovementEvent>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=255"))]
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
#[rasn(delegate,size("1..=4"))]
        pub struct MovementStateRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct MovementState {
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
#[rasn(delegate,value("0..=127"))]
        pub struct MsgCount(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum MultimediaDataUnit {
   MediaContentUTF8(Utf8String),
     MediaContentOctet(OctetString),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-900000000..=900000001"))]
        pub struct NinetyDegreeInt(pub i32);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct Node {
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
        pub fn new(
            lon: Longitude,
	lat: Latitude,
        ) -> Self {
            Self {
                lon,
	lat,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct NodeXY20b {
            pub x: OffsetB10,
                    pub y: OffsetB10,
                    
        }

        impl NodeXY20b {
        pub fn new(
            x: OffsetB10,
	y: OffsetB10,
        ) -> Self {
            Self {
                x,
	y,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct NodeXY22b {
            pub x: OffsetB11,
                    pub y: OffsetB11,
                    
        }

        impl NodeXY22b {
        pub fn new(
            x: OffsetB11,
	y: OffsetB11,
        ) -> Self {
            Self {
                x,
	y,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct NodeXY24b {
            pub x: OffsetB12,
                    pub y: OffsetB12,
                    
        }

        impl NodeXY24b {
        pub fn new(
            x: OffsetB12,
	y: OffsetB12,
        ) -> Self {
            Self {
                x,
	y,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct NodeXY26b {
            pub x: OffsetB13,
                    pub y: OffsetB13,
                    
        }

        impl NodeXY26b {
        pub fn new(
            x: OffsetB13,
	y: OffsetB13,
        ) -> Self {
            Self {
                x,
	y,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct NodeXY28b {
            pub x: OffsetB14,
                    pub y: OffsetB14,
                    
        }

        impl NodeXY28b {
        pub fn new(
            x: OffsetB14,
	y: OffsetB14,
        ) -> Self {
            Self {
                x,
	y,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct NodeXY32b {
            pub x: OffsetB16,
                    pub y: OffsetB16,
                    
        }

        impl NodeXY32b {
        pub fn new(
            x: OffsetB16,
	y: OffsetB16,
        ) -> Self {
            Self {
                x,
	y,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct NodeAttributeSetAddGrpC {
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
#[rasn(delegate,size("1..=4"))]
        pub struct NodeAttributeSetXYRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct NodeAttributeSetXY {
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
#[rasn(delegate,size("1..=8"))]
        pub struct NodeAttributeXYList(pub SequenceOf<NodeAttributeXY>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=5"))]
        pub struct NodeLink(pub SequenceOf<Node>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum NodeListXY {
   Nodes(NodeSetXY),
     Computed(ComputedLane),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
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
#[rasn(choice,automatic_tags)]
        pub enum NodeOffsetPointZ {
   NodeZ1(OffsetB10),
     NodeZ2(OffsetB11),
     NodeZ3(OffsetB12),
     NodeZ4(OffsetB13),
     NodeZ5(OffsetB14),
     NodeZ6(OffsetB16),
    
}


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("2..=63"))]
        pub struct NodeSetXY(pub SequenceOf<NodeXY>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct NodeXY {
            pub delta: NodeOffsetPointXY,
                    pub attributes: Option<NodeAttributeSetXY>,
                    
        }

        impl NodeXY {
        pub fn new(
            delta: NodeOffsetPointXY,
	attributes: Option<NodeAttributeSetXY>,
        ) -> Self {
            Self {
                delta,
	attributes,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum NonIslandLanePosition {
   OffRoadLanePosition(OffRoadLanePosition),
     VehicularLanePosition(LanePosition),
     MapPosition(MapPosition),
    
}



#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct NullCtx(());



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct NumberOfPerceivedObjects(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=256"))]
        pub struct NumberStations(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=1500"))]
        pub struct ObjectAge(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum ObjectClassChoice {
   Vehicle(VehicleSubclass),
     Person(PersonSubclass),
     Animal(AnimalSubclass),
     Other(OtherSubclass),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=101"))]
        pub struct ObjectConfidence(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ObjectDistanceWithConfidence {
            pub value: DistanceValue,
                    pub confidence: DistanceConfidence,
                    
        }

        impl ObjectDistanceWithConfidence {
        pub fn new(
            value: DistanceValue,
	confidence: DistanceConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=8"))]
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
#[rasn(delegate,value("-256..=255"))]
        pub struct OffsetB09(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-512..=511"))]
        pub struct OffsetB10(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-1024..=1023"))]
        pub struct OffsetB11(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-2048..=2047"))]
        pub struct OffsetB12(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-4096..=4095"))]
        pub struct OffsetB13(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-8192..=8191"))]
        pub struct OffsetB14(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-32768..=32767"))]
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
#[rasn(choice,automatic_tags)]
        pub enum OffsetXaxis {
   Small(DrivenLineOffsetSm),
     Large(DrivenLineOffsetLg),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum OffsetYaxis {
   Small(DrivenLineOffsetSm),
     Large(DrivenLineOffsetLg),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-1799999999..=1800000001"))]
        pub struct OneEightyDegreeInt(pub i32);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct Opaque(pub OctetString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct OperatingClass80211(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum OriginatingRSUContainer {
   IntersectionReferenceId(IntersectionReferenceID),
     RoadSegmentReferenceId(RoadSegmentReferenceID),
    
}


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct OriginatingVehicleContainer {
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
        pub fn new(
            r_type: OtherSublassType,
	confidence: ClassConfidence,
        ) -> Self {
            Self {
                r_type,
	confidence,
            }
        }
    }

        fn other_subclass_r_type_default() -> OtherSublassType {
                OtherSublassType(0).into()
            }
            
            fn other_subclass_confidence_default() -> ClassConfidence {
                ClassConfidence(0).into()
            }
            
            


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct OtherSublassType(pub u8);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=5"))]
        pub struct OverlayLaneList(pub SequenceOf<LaneID>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("4..=4"))]
        pub struct PMD(pub BitString);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct POIType(pub u16);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct ParkingPlacesData(pub SequenceOf<SpotAvailability>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct PartialIntersection {
            #[rasn(size("4..=4"))]
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
        
                #[non_exhaustive]pub struct PartialMapData {
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
        pub fn new(
            header: ItsPduHeader,
	map: PartialMapData,
        ) -> Self {
            Self {
                header,
	map,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct PartialSpat {
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
        
                #[non_exhaustive]pub struct PartialSpatIntersection {
            #[rasn(size("5..=5"))]
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
        pub fn new(
            header: ItsPduHeader,
	spat: PartialSpat,
        ) -> Self {
            Self {
                header,
	spat,
            }
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
        pub fn new(
            unit_type: UnitType,
	value: u16,
        ) -> Self {
            Self {
                unit_type,
	value,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PassengerCapacity {
            pub number_of_seats: Int1,
                    pub number_of_standing_places: Int1,
                    
        }

        impl PassengerCapacity {
        pub fn new(
            number_of_seats: Int1,
	number_of_standing_places: Int1,
        ) -> Self {
            Self {
                number_of_seats,
	number_of_standing_places,
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
#[rasn(delegate,size("1..=128", extensible))]
        pub struct PerceivedObjectContainer(pub SequenceOf<PerceivedObject>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PerceivedObjectContainerSegmentInfo {
            pub total_msg_segments: SegmentCount,
                    pub this_segment_num: SegmentCount,
                    
        }

        impl PerceivedObjectContainerSegmentInfo {
        pub fn new(
            total_msg_segments: SegmentCount,
	this_segment_num: SegmentCount,
        ) -> Self {
            Self {
                total_msg_segments,
	this_segment_num,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PersonSubclass {
            #[rasn(default = "person_subclass_r_type_default")]
        pub r_type: PersonSubclassType,
                    #[rasn(default = "person_subclass_confidence_default")]
        pub confidence: ClassConfidence,
                    
        }

        impl PersonSubclass {
        pub fn new(
            r_type: PersonSubclassType,
	confidence: ClassConfidence,
        ) -> Self {
            Self {
                r_type,
	confidence,
            }
        }
    }

        fn person_subclass_r_type_default() -> PersonSubclassType {
                PersonSubclassType(0).into()
            }
            
            fn person_subclass_confidence_default() -> ClassConfidence {
                ClassConfidence(0).into()
            }
            
            


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct PersonSubclassType(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PictogramCategoryCode {
            #[rasn(value("1..=9"))]
        pub nature: u8,
                    #[rasn(value("0..=99"))]
        pub serial_number: u8,
                    
        }

        impl PictogramCategoryCode {
        pub fn new(
            nature: u8,
	serial_number: u8,
        ) -> Self {
            Self {
                nature,
	serial_number,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PictogramCode {
            #[rasn(size("2..=2"))]
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
#[rasn(delegate,size("3..=16", extensible))]
        pub struct PolyPointList(pub SequenceOf<OffsetPoint>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum PolygonalLine {
   DeltaPositions(DeltaPositions),
     DeltaPositionsWithAltitude(DeltaPositionsWithAltitude),
     AbsolutePositions(AbsolutePositions),
     AbsolutePositionsWithAltitude(AbsolutePositionsWithAltitude),
    
}


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("3.."))]
        pub struct PolygonalRegion(pub SequenceOf<Dot2TwoDLocation>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct PortNumber(pub u16);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct Position3DRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct Position3D {
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
        
                #[non_exhaustive]pub struct Position3DAddGrpC {
            pub altitude: Altitude,
                    
        }

        impl Position3DAddGrpC {
        pub fn new(
            altitude: Altitude,
        ) -> Self {
            Self {
                altitude,
            }
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
        pub fn new(
            pos: PositionConfidence,
	elevation: ElevationConfidence,
        ) -> Self {
            Self {
                pos,
	elevation,
            }
        }
    }

        

        
        
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

        

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=32"))]
        pub struct PreemptPriorityList(pub SequenceOf<SignalControlZone>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct PrioritizationResponse {
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
#[rasn(delegate,size("1..=10"))]
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



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=134217727"))]
        pub struct ProtectedZoneID(pub u32);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct ProtocolType(pub VarLengthNumber);



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
            #[rasn(size("3..=3"))]
        pub fill_bit: BitString,
                    #[rasn(size("0..=31"))]
        pub psc: OctetString,
                    
        }

        impl ProviderServiceContext {
        pub fn new(
            fill_bit: BitString,
	psc: OctetString,
        ) -> Self {
            Self {
                fill_bit,
	psc,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0.."))]
        pub struct Psid(pub Integer);


        
        ///

        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PsidSsp {
            pub psid: Psid,
                    pub ssp: Option<ServiceSpecificPermissions>,
                    
        }

        impl PsidSsp {
        pub fn new(
            psid: Psid,
	ssp: Option<ServiceSpecificPermissions>,
        ) -> Self {
            Self {
                psid,
	ssp,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct PsidSspRange {
            pub psid: Psid,
                    pub ssp_range: Option<SspRange>,
                    
        }

        impl PsidSspRange {
        pub fn new(
            psid: Psid,
	ssp_range: Option<SspRange>,
        ) -> Self {
            Self {
                psid,
	ssp_range,
            }
        }
    }

        


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
        pub fn new(
            supported_symm_alg: SymmAlgorithm,
	public_key: BasePublicEncryptionKey,
        ) -> Self {
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
        pub fn new(
            embarkation_status: EmbarkationStatus,
	pt_activation: Option<PtActivation>,
        ) -> Self {
            Self {
                embarkation_status,
	pt_activation,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum PublicVerificationKey {
   EcdsaNistP256(EccP256CurvePoint),
     EcdsaBrainpoolP256r1(EccP256CurvePoint),
    #[rasn(extension_addition)]
         EcdsaBrainpoolP384r1(EccP384CurvePoint),
    
}




#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=32"))]
        pub struct ROI(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct RSCUnit(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RSCode {
            #[rasn(value("1..=4", extensible))]
        pub layout_component_id: Option<u8>,
                    pub code: Code,
                    
        }

        impl RSCode {
        pub fn new(
            layout_component_id: Option<u8>,
	code: Code,
        ) -> Self {
            Self {
                layout_component_id,
	code,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RSUContainerHighFrequency {
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
        pub fn new(
            header: ItsPduHeader,
	rtcmc: RTCMcorrections,
        ) -> Self {
            Self {
                header,
	rtcmc,
            }
        }
    }

        

        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct RTCMcorrectionsRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RTCMcorrections {
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
        pub fn new(
            status: GNSSstatus,
	offset_set: AntennaOffsetSet,
        ) -> Self {
            Self {
                status,
	offset_set,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=1023"))]
        pub struct RTCMmessage(pub OctetString);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=5"))]
        pub struct RTCMmessageList(pub SequenceOf<RTCMmessage>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=10000"))]
        pub struct Radius(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=10000"))]
        pub struct Range(pub u16);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct RccPartZoneIds(pub SequenceOf<Zid>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16", extensible))]
        pub struct RccPartLaneConfiguration(pub SequenceOf<LaneInformation>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RccPart {
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
#[rasn(delegate,value("0..=255"))]
        pub struct RcpiThreshold(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=150"))]
        pub struct RearOverhang(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RectangularRegion {
            pub north_west: Dot2TwoDLocation,
                    pub south_east: Dot2TwoDLocation,
                    
        }

        impl RectangularRegion {
        pub fn new(
            north_west: Dot2TwoDLocation,
	south_east: Dot2TwoDLocation,
        ) -> Self {
            Self {
                north_west,
	south_east,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct RefExt(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct RefPointId(pub u8);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct ReferenceDenms(pub SequenceOf<ActionID>);





























        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RegionAndSubregions {
            pub region: Uint8,
                    pub subregions: SequenceOfUint16,
                    
        }

        impl RegionAndSubregions {
        pub fn new(
            region: Uint8,
	subregions: SequenceOfUint16,
        ) -> Self {
            Self {
                region,
	subregions,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct RegionId(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RegionalExtension {
            pub region_id: RegionId,
                    pub reg_ext_value: Any,
                    
        }

        impl RegionalExtension {
        pub fn new(
            region_id: RegionId,
	reg_ext_value: Any,
        ) -> Self {
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
        pub fn new(
            r_type: SpeedLimitType,
	speed: Velocity,
        ) -> Self {
            Self {
                r_type,
	speed,
            }
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



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct RepeatRate(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct ReplyAddress(pub PortNumber);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
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
#[rasn(delegate,size("1..=4"))]
        pub struct RequesterDescriptionRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RequesterDescription {
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
        
                #[non_exhaustive]pub struct RequesterDescriptionAddGrpC {
            pub fuel: Option<FuelType>,
                    pub battery_status: Option<BatteryStatus>,
                    
        }

        impl RequesterDescriptionAddGrpC {
        pub fn new(
            fuel: Option<FuelType>,
	battery_status: Option<BatteryStatus>,
        ) -> Self {
            Self {
                fuel,
	battery_status,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RequesterPositionVector {
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
        
                #[non_exhaustive]pub struct RequesterType {
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

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RescueContainer {
            pub light_bar_siren_in_use: LightBarSirenInUse,
                    
        }

        impl RescueContainer {
        pub fn new(
            light_bar_siren_in_use: LightBarSirenInUse,
        ) -> Self {
            Self {
                light_bar_siren_in_use,
            }
        }
    }

        


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
        pub fn new(
            id: RestrictionClassID,
	users: RestrictionUserTypeList,
        ) -> Self {
            Self {
                id,
	users,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct RestrictionClassID(pub u8);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=254"))]
        pub struct RestrictionClassList(pub SequenceOf<RestrictionClassAssignment>);


        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct RestrictionUserTypeRegional(pub SequenceOf<RegionalExtension>);


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum RestrictionUserType {
   BasicType(RestrictionAppliesTo),
     Regional(RestrictionUserTypeRegional),
    
}


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RestrictionUserTypeAddGrpC {
            pub emission: Option<EmissionType>,
                    pub fuel: Option<FuelType>,
                    
        }

        impl RestrictionUserTypeAddGrpC {
        pub fn new(
            emission: Option<EmissionType>,
	fuel: Option<FuelType>,
        ) -> Self {
            Self {
                emission,
	fuel,
            }
        }
    }

        

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=16"))]
        pub struct RestrictionUserTypeList(pub SequenceOf<RestrictionUserType>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct RoadAngles(pub SequenceOf<Heading>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=255"))]
        pub struct RoadLaneSetList(pub SequenceOf<GenericLane>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct RoadRegulatorID(pub u16);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct RoadSegmentRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct RoadSegment {
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
#[rasn(delegate,value("0..=65535"))]
        pub struct RoadSegmentID(pub u16);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=32"))]
        pub struct RoadSegmentList(pub SequenceOf<RoadSegment>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct RoadSegmentReferenceID {
            pub region: Option<RoadRegulatorID>,
                    pub id: RoadSegmentID,
                    
        }

        impl RoadSegmentReferenceID {
        pub fn new(
            region: Option<RoadRegulatorID>,
	id: RoadSegmentID,
        ) -> Self {
            Self {
                region,
	id,
            }
        }
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

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-128..=127"))]
        pub struct RoadwayCrownAngle(pub i8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct RoutAdvertExt(pub Extension);



        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct RoutAdvertExts(pub SequenceOf<RoutAdvertExt>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
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
#[rasn(delegate,value("0..=15"))]
        pub struct RsvAdvPrtVersion(pub u8);




#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=28800"))]
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
#[rasn(delegate,value("-4096..=61439"))]
        pub struct SAElevation(pub i32);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct SALatitude {
            #[rasn(size("1..=1"))]
        pub fill_bit: BitString,
                    #[rasn(value("-900000000..=900000001"))]
        pub lat: i32,
                    
        }

        impl SALatitude {
        pub fn new(
            fill_bit: BitString,
	lat: i32,
        ) -> Self {
            Self {
                fill_bit,
	lat,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-1800000000..=1800000001"))]
        pub struct SALongitude(pub i32);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct SAMapplicationData(pub OctetString);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct SPATRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SPAT {
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
        pub fn new(
            header: ItsPduHeader,
	spat: SPAT,
        ) -> Self {
            Self {
                header,
	spat,
            }
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
        pub fn new(
            spm: Option<u8>,
	mns: Option<u8>,
	unit: RSCUnit,
        ) -> Self {
            Self {
                spm,
	mns,
	unit,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct SREM {
            pub header: ItsPduHeader,
                    pub srm: SignalRequestMessage,
                    
        }

        impl SREM {
        pub fn new(
            header: ItsPduHeader,
	srm: SignalRequestMessage,
        ) -> Self {
            Self {
                header,
	srm,
            }
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
        pub fn new(
            header: ItsPduHeader,
	ssm: SignalStatusMessage,
        ) -> Self {
            Self {
                header,
	ssm,
            }
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
        pub fn new(
            version: RsvAdvPrtVersion,
	body: SamBody,
        ) -> Self {
            Self {
                version,
	body,
            }
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
        pub fn new(
            itsaid_ctx_ref: ItsAidCtxRef,
	context: Any,
        ) -> Self {
            Self {
                itsaid_ctx_ref,
	context,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-2048..=2047"))]
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
        pub fn new(
            line: PolygonalLine,
	lane_width: Option<IVILaneWidth>,
        ) -> Self {
            Self {
                line,
	lane_width,
            }
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
#[rasn(delegate,size("1..=8"))]
        pub struct SegmentAttributeXYList(pub SequenceOf<SegmentAttributeXY>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=127"))]
        pub struct SegmentCount(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct SemiMajorAxisAccuracy(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=65535"))]
        pub struct SemiMajorAxisOrientation(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct SemiMinorAxisAccuracy(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=10000"))]
        pub struct SemiRangeLength(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-5000..=5000"))]
        pub struct SensorHeight(pub i16);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=128", extensible))]
        pub struct SensorIdList(pub SequenceOf<Identifier>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SensorInformation {
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
#[rasn(delegate,size("1..=128", extensible))]
        pub struct SensorInformationContainer(pub SequenceOf<SensorInformation>);


        

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
#[rasn(delegate)]
        pub struct SequenceOfUint16(pub SequenceOf<Uint16>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct SequenceOfUint8(pub SequenceOf<Uint8>);


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("0..=8"))]
        pub struct SequenceOfVruSafeDistanceIndication(pub SequenceOf<VruSafeDistanceIndication>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum ServiceCategoryCode {
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
#[rasn(delegate,value("0..=65535"))]
        pub struct ServicePort(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum ServiceSpecificPermissions {
   Opaque(OctetString),
    #[rasn(extension_addition)]
         BitmapSsp(BitmapSsp),
    
}



#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct ShadowingApplies(pub bool);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SignalControlZone {
            #[rasn(value("0.."))]
        pub zone: RegionalExtension,
                    
        }

        impl SignalControlZone {
        pub fn new(
            zone: RegionalExtension,
        ) -> Self {
            Self {
                zone,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct SignalGroupID(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SignalHeadLocation {
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
#[rasn(delegate,size("1..=64"))]
        pub struct SignalHeadLocationList(pub SequenceOf<SignalHeadLocation>);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct SignalRequestRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SignalRequest {
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
#[rasn(delegate,size("1..=32"))]
        pub struct SignalRequestList(pub SequenceOf<SignalRequestPackage>);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct SignalRequestMessageRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SignalRequestMessage {
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
#[rasn(delegate,size("1..=4"))]
        pub struct SignalRequestPackageRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SignalRequestPackage {
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
        
                #[non_exhaustive]pub struct SignalRequesterInfo {
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
#[rasn(delegate,size("1..=4"))]
        pub struct SignalStatusRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SignalStatus {
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
#[rasn(delegate,size("1..=32"))]
        pub struct SignalStatusList(pub SequenceOf<SignalStatus>);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct SignalStatusMessageRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SignalStatusMessage {
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
#[rasn(delegate,size("1..=4"))]
        pub struct SignalStatusPackageRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct SignalStatusPackage {
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
        
                #[non_exhaustive]pub struct SignalStatusPackageAddGrpC {
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
#[rasn(delegate,size("1..=32"))]
        pub struct SignalStatusPackageList(pub SequenceOf<SignalStatusPackage>);


///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum Signature {
   EcdsaNistP256Signature(EcdsaP256Signature),
     EcdsaBrainpoolP256r1Signature(EcdsaP256Signature),
    #[rasn(extension_addition)]
         EcdsaBrainpoolP384r1Signature(EcdsaP384Signature),
    
}


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct SoundLevel {
            pub soundstationary: Int1,
                    pub sounddriveby: Int1,
                    
        }

        impl SoundLevel {
        pub fn new(
            soundstationary: Int1,
	sounddriveby: Int1,
        ) -> Self {
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
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum SpecialVehicleContainer {
   PublicTransportContainer(PublicTransportContainer),
     SpecialTransportContainer(SpecialTransportContainer),
     DangerousGoodsContainer(DangerousGoodsContainer),
     RoadWorksContainerBasic(RoadWorksContainerBasic),
     RescueContainer(RescueContainer),
     EmergencyContainer(EmergencyContainer),
     SafetyCarContainer(SafetyCarContainer),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=500"))]
        pub struct SpeedAdvice(pub u16);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct SpeedExtended {
            pub value: SpeedValueExtended,
                    pub confidence: SpeedConfidence,
                    
        }

        impl SpeedExtended {
        pub fn new(
            value: SpeedValueExtended,
	confidence: SpeedConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        

        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=9"))]
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
#[rasn(delegate,value("-16383..=16383"))]
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
        pub fn new(
            max_waiting_time_minutes: u16,
	blocking: bool,
        ) -> Self {
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
        pub fn new(
            header: RsvAdvPrtVersion,
	body: SrmBody,
        ) -> Self {
            Self {
                header,
	body,
            }
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
        pub fn new(
            context: SamContext,
	client_port: PortNumber,
        ) -> Self {
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
        pub fn new(
            port_dyn_sam: PortNumber,
	alloc_reqs: SrmPrvChAllocReq,
        ) -> Self {
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
        pub fn new(
            sa_i_d: SrvAdvID,
	content_count: SrvAdvContentCount,
        ) -> Self {
            Self {
                sa_i_d,
	content_count,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct SrvAdvContentCount(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=15"))]
        pub struct SrvAdvID(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct SrvAdvMsgHeaderExt(pub Extension);



        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct SrvAdvMsgHeaderExts(pub SequenceOf<SrvAdvMsgHeaderExt>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum SspRange {
   Opaque(SequenceOfOctetString),
     All(()),
    #[rasn(extension_addition)]
         BitmapSspRange(BitmapSspRange),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum StationDataContainer {
   OriginatingVehicleContainer(OriginatingVehicleContainer),
     OriginatingRSUContainer(OriginatingRSUContainer),
    
}


///

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=1"))]
        pub struct SubjectAssurance(pub OctetString);



#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
                #[non_exhaustive]
pub enum SymmAlgorithm {
     Aes128Ccm = 0,
                
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum SymmetricEncryptionKey {
  #[rasn(size("16..=16"))]
         Aes128Ccm(OctetString),
    
}


        

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct SystemService(pub SequenceOf<SystemServiceAndContext>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct SystemServiceAndContext(pub SamContext);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-128..=127"))]
        pub struct TXpower80211(pub i8);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct TcPartDetectionZoneIds(pub SequenceOf<Zid>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct TcPartRelevanceZoneIds(pub SequenceOf<Zid>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct TcPartDriverAwarenessZoneIds(pub SequenceOf<Zid>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
        pub struct TcPartApplicableLanes(pub SequenceOf<LanePosition>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
        pub struct TcPartText(pub SequenceOf<Text>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct TcPart {
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

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("4..=4"))]
        pub struct TemporaryID(pub OctetString);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4"))]
        pub struct TestDataRegional(pub SequenceOf<RegionalExtension>);

        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct TestData {
            pub regional: Option<TestDataRegional>,
                    
        }

        impl TestData {
        pub fn new(
            regional: Option<TestDataRegional>,
        ) -> Self {
            Self {
                regional,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct Text {
            #[rasn(value("1..=4", extensible))]
        pub layout_component_id: Option<u8>,
                    #[rasn(size("10..=10"))]
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
#[rasn(delegate,size("1..=16", extensible))]
        pub struct TextContainer(pub SequenceOf<TcPart>);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ThreeDLocation {
            pub latitude: SALatitude,
                    pub longitude: SALongitude,
                    pub elevation: SAElevation,
                    
        }

        impl ThreeDLocation {
        pub fn new(
            latitude: SALatitude,
	longitude: SALongitude,
	elevation: SAElevation,
        ) -> Self {
            Self {
                latitude,
	longitude,
	elevation,
            }
        }
    }

        

///

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
#[rasn(delegate,value("0..=15"))]
        pub struct TimeIntervalConfidence(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=36001"))]
        pub struct TimeMark(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-1500..=1500"))]
        pub struct TimeOfMeasurement(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=60000"))]
        pub struct TimeReference(pub u16);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
        pub struct TractorCharacteristicsEqualTo(pub SequenceOf<VehicleCharacteristicsFixValues>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
        pub struct TractorCharacteristicsNotEqualTo(pub SequenceOf<VehicleCharacteristicsFixValues>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
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
#[rasn(delegate,size("1..=4", extensible))]
        pub struct TrailerCharacteristicsEqualTo(pub SequenceOf<VehicleCharacteristicsFixValues>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
        pub struct TrailerCharacteristicsNotEqualTo(pub SequenceOf<VehicleCharacteristicsFixValues>);

    
    
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=4", extensible))]
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
#[rasn(delegate,size("1..=2"))]
        pub struct TrailerDataContainer(pub SequenceOf<TrailerData>);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate)]
        pub struct TrainCharacteristics(pub TractorCharacteristics);



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
#[rasn(delegate,size("8..=8"))]
        pub struct TransitVehicleStatus(pub BitString);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct TransmissionAndSpeed {
            pub transmisson: TransmissionState,
                    pub speed: Velocity,
                    
        }

        impl TransmissionAndSpeed {
        pub fn new(
            transmisson: TransmissionState,
	speed: Velocity,
        ) -> Self {
            Self {
                transmisson,
	speed,
            }
        }
    }

        


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


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct TwoDLocation {
            pub latitude: SALatitude,
                    pub longitude: SALongitude,
                    
        }

        impl TwoDLocation {
        pub fn new(
            latitude: SALatitude,
	longitude: SALongitude,
        ) -> Self {
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
#[rasn(delegate,value("0..=65535"))]
        pub struct Uint16(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=7"))]
        pub struct Uint3(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=4294967295"))]
        pub struct Uint32(pub u32);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=18446744073709551615"))]
        pub struct Uint64(pub u64);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct Uint8(pub u8);



#[derive(AsnType, Debug, Clone, Copy, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(enumerated)]
        
pub enum UnitType {
     MgKm = 0,
                 MgKWh = 1,
                
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,value("900000001..=900000001"))]
        pub struct UnknownLatitude(pub NinetyDegreeInt);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,value("1800000001..=1800000001"))]
        pub struct UnknownLongitude(pub OneEightyDegreeInt);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct VAM {
            pub header: ItsPduHeader,
                    pub vam: VruAwareness,
                    
        }

        impl VAM {
        pub fn new(
            header: ItsPduHeader,
	vam: VruAwareness,
        ) -> Self {
            Self {
                header,
	vam,
            }
        }
    }

        

        
        
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
            Self {
                hei,
	wid,
	vln,
	wei,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct ValidityPeriod {
            pub start: Time32,
                    pub duration: Duration,
                    
        }

        impl ValidityPeriod {
        pub fn new(
            start: Time32,
	duration: Duration,
        ) -> Self {
            Self {
                start,
	duration,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VamParameters {
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

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=7"))]
        pub struct VcClass(pub u8);


        
        
/// Inner type 

#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(delegate,size("1..=8", extensible))]
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
#[rasn(delegate,value("0..=7"))]
        pub struct VcOption(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum VehicleCharacteristicsFixValues {
   SimpleVehicleType(StationType),
     EuVehicleCategoryCode(EuVehicleCategoryCode),
     Iso3833VehicleType(Iso3833VehicleType),
     EuroAndCo2value(EnvironmentalCharacteristics),
     EngineCharacteristics(EngineCharacteristics),
     LoadType(LoadType),
     Usage(VehicleRole),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum VehicleCharacteristicsRangeLimits {
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

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        pub enum VehicleID {
   EntityID(TemporaryID),
     StationID(StationID),
    
}


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VehicleSensor {
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
        
                #[non_exhaustive]pub struct VehicleSensorProperties {
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
#[rasn(delegate,size("1..=10"))]
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
        pub fn new(
            r_type: VehicleSubclassType,
	confidence: ClassConfidence,
        ) -> Self {
            Self {
                r_type,
	confidence,
            }
        }
    }

        fn vehicle_subclass_r_type_default() -> VehicleSubclassType {
                VehicleSubclassType(0).into()
            }
            
            fn vehicle_subclass_confidence_default() -> ClassConfidence {
                ClassConfidence(0).into()
            }
            
            


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
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
#[rasn(delegate,value("0..=8191"))]
        pub struct Velocity(pub u16);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct VruAwareness {
            pub generation_delta_time: GenerationDeltaTime,
                    pub vam_parameters: VamParameters,
                    
        }

        impl VruAwareness {
        pub fn new(
            generation_delta_time: GenerationDeltaTime,
	vam_parameters: VamParameters,
        ) -> Self {
            Self {
                generation_delta_time,
	vam_parameters,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VruClusterInformationContainer {
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
#[rasn(delegate,value("1..=255"))]
        pub struct VruClusterOpTimestamp(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VruClusterOperationContainer {
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

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VruHighFrequencyContainer {
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
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum VruLanePosition {
   OffRoadLanePosition(OffRoadLanePosition),
     VehicularLanePosition(LanePosition),
     TrafficIslandPosition(TrafficIslandPosition),
     MapPosition(MapPosition),
    
}


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VruLowFrequencyContainer {
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
        
                #[non_exhaustive]pub struct VruMotionPredictionContainer {
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
#[rasn(delegate)]
        pub struct VruRollAngle(pub SteeringWheelAngle);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct VruSafeDistanceIndication {
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

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct WGS84Angle {
            pub value: WGS84AngleValue,
                    pub confidence: AngleConfidence,
                    
        }

        impl WGS84Angle {
        pub fn new(
            value: WGS84AngleValue,
	confidence: AngleConfidence,
        ) -> Self {
            Self {
                value,
	confidence,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=3601"))]
        pub struct WGS84AngleValue(pub u16);



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
        pub fn new(
            value: u16,
	unit: RSCUnit,
        ) -> Self {
            Self {
                value,
	unit,
            }
        }
    }

        

        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        pub struct WsaChInfoDataRate {
            #[rasn(size("1..=1"))]
        pub adaptable: BitString,
                    #[rasn(value("0..=127"))]
        pub data_rate: u8,
                    
        }

        impl WsaChInfoDataRate {
        pub fn new(
            adaptable: BitString,
	data_rate: u8,
        ) -> Self {
            Self {
                adaptable,
	data_rate,
            }
        }
    }

        


#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct WsaCountThreshold(pub u8);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=255"))]
        pub struct WsaCountThresholdInterval(pub u8);


        
        
        #[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
        #[rasn(automatic_tags)]
        
                #[non_exhaustive]pub struct WsaSsp {
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
#[rasn(delegate,value("-5000..=0"))]
        pub struct XSensorOffset(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("-1000..=1000"))]
        pub struct YSensorOffset(pub i16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=1000"))]
        pub struct ZSensorOffset(pub u16);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("1..=32", extensible))]
        pub struct Zid(pub Integer);



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq)]
#[rasn(choice,automatic_tags)]
        
                #[non_exhaustive]pub enum Zone {
   Segment(Segment),
     Area(PolygonalLine),
     ComputedSegment(ComputedSegment),
    
}



#[derive(AsnType, Debug, Clone, Decode, Encode, PartialEq, PartialOrd, Eq, Ord, Hash)]
#[rasn(delegate,value("0..=10000"))]
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

