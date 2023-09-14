#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]
use rasn_compiler_derive::asn1;

asn1! {
    r#"Etsi-Schema {
        iso (1) standard (0) signalizedIntersection (19091) profilec (2) addgrpc (0)
        version2 (2)
        }
      
      DEFINITIONS AUTOMATIC TAGS ::=
      
      BEGIN
      
      PartialMapem ::= SEQUENCE {
        header ItsPduHeader,
        map PartialMapData
      }
      
      PartialMapData ::= SEQUENCE {
       timeStamp MinuteOfTheYear OPTIONAL, 
       msgIssueRevision MsgCount, 
       layerType LayerType OPTIONAL,
       layerID LayerID OPTIONAL,
       firstIntersection FirstIntersection OPTIONAL,
       dummy1 NULL OPTIONAL,
       dummy2 NULL OPTIONAL,
       dummy3 NULL OPTIONAL,
       dummy4 NULL OPTIONAL,
       ...
      }
      
      FirstIntersection ::= SEQUENCE {
        numberOfIntersections INTEGER(1..32),
        partialIntersection PartialIntersection,
      }
      
      PartialIntersection ::= SEQUENCE {
        dummyBitmap BIT STRING (SIZE(4)),
        name DescriptiveName OPTIONAL, 
        id IntersectionReferenceID, 
        ...
      }
      
      FirstSpat ::= SEQUENCE {
        numberOfIntersections INTEGER(1..32),
        partialSpatIntersection PartialSpatIntersection,
      }

      PartialSpatIntersection ::= SEQUENCE {
        dummyBitmap BIT STRING (SIZE (5)),
        name DescriptiveName OPTIONAL,
        id IntersectionReferenceID,
        ...
      }

      PartialSpatem ::= SEQUENCE {
        header ItsPduHeader,
        spat PartialSpat
      }

      PartialSpat ::= SEQUENCE {
        timeStamp MinuteOfTheYear OPTIONAL,
        name DescriptiveName OPTIONAL,
        intersections FirstSpat,
        dummy NULL OPTIONAL,
        ...
      }

      CAM ::= SEQUENCE {
       header ItsPduHeader,
       cam CoopAwareness
      }
      
      CoopAwareness ::= SEQUENCE {
       generationDeltaTime GenerationDeltaTime,
       camParameters CamParameters
      }
      
      CamParameters ::= SEQUENCE {
       basicContainer BasicContainer,
       highFrequencyContainer HighFrequencyContainer,
       lowFrequencyContainer LowFrequencyContainer OPTIONAL,
       specialVehicleContainer SpecialVehicleContainer OPTIONAL,
       ...
      }
      
      HighFrequencyContainer ::= CHOICE {
       basicVehicleContainerHighFrequency BasicVehicleContainerHighFrequency,
       rsuContainerHighFrequency RSUContainerHighFrequency,
       ...
      }
      
       LowFrequencyContainer ::= CHOICE {
       basicVehicleContainerLowFrequency BasicVehicleContainerLowFrequency,
       ...
      }
      
      SpecialVehicleContainer ::= CHOICE {
       publicTransportContainer PublicTransportContainer,
       specialTransportContainer SpecialTransportContainer,
       dangerousGoodsContainer DangerousGoodsContainer,
       roadWorksContainerBasic RoadWorksContainerBasic,
       rescueContainer RescueContainer,
       emergencyContainer EmergencyContainer,
       safetyCarContainer SafetyCarContainer,
       ...
      }
      
      BasicContainer ::= SEQUENCE {
       stationType StationType,
       referencePosition ReferencePosition,
       ...
      }
      
      BasicVehicleContainerHighFrequency ::= SEQUENCE {
       heading Heading,
       speed Speed,
       driveDirection DriveDirection,
       vehicleLength VehicleLength,
       vehicleWidth VehicleWidth,
       longitudinalAcceleration LongitudinalAcceleration,
       curvature Curvature,
       curvatureCalculationMode CurvatureCalculationMode,
       yawRate YawRate,
       accelerationControl AccelerationControl OPTIONAL,
       lanePosition LanePosition OPTIONAL,
       steeringWheelAngle SteeringWheelAngle OPTIONAL,
       lateralAcceleration LateralAcceleration OPTIONAL,
       verticalAcceleration VerticalAcceleration OPTIONAL,
       performanceClass PerformanceClass OPTIONAL,
       cenDsrcTollingZone CenDsrcTollingZone OPTIONAL
      }
      
      BasicVehicleContainerLowFrequency ::= SEQUENCE {
       vehicleRole VehicleRole,
       exteriorLights ExteriorLights,
       pathHistory PathHistory
      }
      
      PublicTransportContainer ::= SEQUENCE {
       embarkationStatus EmbarkationStatus,
       ptActivation PtActivation OPTIONAL
      }
      
      SpecialTransportContainer ::= SEQUENCE {
       specialTransportType SpecialTransportType,
       lightBarSirenInUse LightBarSirenInUse
      }
      
       DangerousGoodsContainer ::= SEQUENCE {
       dangerousGoodsBasic DangerousGoodsBasic
       }
       
       RoadWorksContainerBasic ::= SEQUENCE {
       roadworksSubCauseCode RoadworksSubCauseCode OPTIONAL,
       lightBarSirenInUse LightBarSirenInUse,
       closedLanes ClosedLanes OPTIONAL
       }
      
      RescueContainer ::= SEQUENCE {
       lightBarSirenInUse LightBarSirenInUse
      }
      
      EmergencyContainer ::= SEQUENCE {
       lightBarSirenInUse LightBarSirenInUse,
       incidentIndication CauseCode OPTIONAL,
       emergencyPriority EmergencyPriority OPTIONAL
      }
      
      SafetyCarContainer ::= SEQUENCE {
       lightBarSirenInUse LightBarSirenInUse,
       incidentIndication CauseCode OPTIONAL,
       trafficRule TrafficRule OPTIONAL,
       speedLimit SpeedLimit OPTIONAL
      }
      
      RSUContainerHighFrequency ::= SEQUENCE {
       protectedCommunicationZonesRSU ProtectedCommunicationZonesRSU OPTIONAL,
       ...
      }
      
      GenerationDeltaTime ::= INTEGER { oneMilliSec(1) } (0..65535)
      
      
      
      DENM ::= SEQUENCE {
       header ItsPduHeader,
       denm DecentralizedEnvironmentalNotificationMessage
      }
      
      DecentralizedEnvironmentalNotificationMessage ::= SEQUENCE {
       management ManagementContainer,
       situation SituationContainer OPTIONAL,
       location LocationContainer OPTIONAL,
       alacarte AlacarteContainer OPTIONAL
      }
      
      ManagementContainer ::= SEQUENCE {
       actionID ActionID,
       detectionTime TimestampIts,
       referenceTime TimestampIts,
       termination Termination OPTIONAL,
       eventPosition ReferencePosition,
       relevanceDistance RelevanceDistance OPTIONAL,
       relevanceTrafficDirection RelevanceTrafficDirection OPTIONAL,
       validityDuration ValidityDuration DEFAULT defaultValidity,
       transmissionInterval TransmissionInterval OPTIONAL,
       stationType StationType,
       ...
      }
      
      SituationContainer ::= SEQUENCE {
       informationQuality InformationQuality,
       eventType CauseCode,
       linkedCause CauseCode OPTIONAL,
       eventHistory EventHistory OPTIONAL,
       ...
      }
      
      LocationContainer ::= SEQUENCE {
       eventSpeed Speed OPTIONAL,
       eventPositionHeading Heading OPTIONAL,
       traces Traces,
       roadType RoadType OPTIONAL,
       ...
      }
      
      ImpactReductionContainer ::= SEQUENCE {
       heightLonCarrLeft HeightLonCarr,
       heightLonCarrRight HeightLonCarr,
       posLonCarrLeft PosLonCarr,
       posLonCarrRight PosLonCarr,
       positionOfPillars PositionOfPillars,
       posCentMass PosCentMass,
       wheelBaseVehicle WheelBaseVehicle,
       turningRadius TurningRadius,
       posFrontAx PosFrontAx,
       positionOfOccupants PositionOfOccupants,
       vehicleMass VehicleMass,
       requestResponseIndication RequestResponseIndication
      }
      
      RoadWorksContainerExtended ::= SEQUENCE {
       lightBarSirenInUse LightBarSirenInUse OPTIONAL,
       closedLanes ClosedLanes OPTIONAL,
       restriction RestrictedTypes OPTIONAL,
       speedLimit SpeedLimit OPTIONAL,
       incidentIndication CauseCode OPTIONAL,
       recommendedPath ItineraryPath OPTIONAL,
       startingPointSpeedLimit DeltaReferencePosition OPTIONAL,
       trafficFlowRule TrafficRule OPTIONAL,
       referenceDenms ReferenceDenms OPTIONAL
       }
      
      StationaryVehicleContainer ::= SEQUENCE {
       stationarySince StationarySince OPTIONAL,
       stationaryCause CauseCode OPTIONAL,
       carryingDangerousGoods DangerousGoodsExtended OPTIONAL,
       numberOfOccupants NumberOfOccupants OPTIONAL,
       vehicleIdentification VehicleIdentification OPTIONAL,
       energyStorageType EnergyStorageType OPTIONAL
      }
      
      PreCrashContainer ::= SEQUENCE {
       perceivedObject PerceivedObject,
       objectStationId StationID OPTIONAL,
       timeToCollision TransmissionInterval OPTIONAL,
       impactSection ImpactSection OPTIONAL,
       hostVehicleOrientation WGS84Angle,
       ...
      }
      
      ImpactSection ::= ENUMERATED { unavailable(0), rear(1), front(2), sideLeftFront(3), sideLeftBack(4), sideRightFront(5), sideRightBack(6) }
      
      AlacarteContainer ::= SEQUENCE {
       lanePosition LanePosition OPTIONAL,
       impactReduction ImpactReductionContainer OPTIONAL,
       externalTemperature Temperature OPTIONAL,
       roadWorks RoadWorksContainerExtended OPTIONAL,
       positioningSolution PositioningSolutionType OPTIONAL,
       stationaryVehicle StationaryVehicleContainer OPTIONAL,
       ...,
       preCrashContainer PreCrashContainer OPTIONAL
      }
      
      defaultValidity INTEGER ::= 600
      
      Termination ::= ENUMERATED {isCancellation(0), isNegation (1)}
      
      ReferenceDenms ::= SEQUENCE (SIZE(1..8, ...)) OF ActionID
      
      
      ItsPduHeader ::= SEQUENCE {
       protocolVersion INTEGER (0..255),
       messageID INTEGER (0..255),
       stationID StationID
      }
      
      StationID ::= INTEGER (0..4294967295)
      
      ReferencePosition ::= SEQUENCE {
       latitude Latitude,
       longitude Longitude,
       positionConfidenceEllipse PosConfidenceEllipse ,
       altitude Altitude
      }
      
      DeltaReferencePosition ::= SEQUENCE {
       deltaLatitude DeltaLatitude,
       deltaLongitude DeltaLongitude,
       deltaAltitude DeltaAltitude
      }
      
      Longitude ::= INTEGER {oneMicrodegreeEast (10), oneMicrodegreeWest (-10), unavailable(1800000001)} (-1800000000..1800000001)
      
      Latitude ::= INTEGER {oneMicrodegreeNorth (10), oneMicrodegreeSouth (-10), unavailable(900000001)} (-900000000..900000001)
      
      Altitude ::= SEQUENCE {
       altitudeValue AltitudeValue,
       altitudeConfidence AltitudeConfidence
      }
      
      AltitudeValue ::= INTEGER {referenceEllipsoidSurface(0), oneCentimeter(1), unavailable(800001)} (-100000..800001)
      
      AltitudeConfidence ::= ENUMERATED {
       alt-000-01 (0),
       alt-000-02 (1),
       alt-000-05 (2),
       alt-000-10 (3),
       alt-000-20 (4),
       alt-000-50 (5),
       alt-001-00 (6),
       alt-002-00 (7),
       alt-005-00 (8),
       alt-010-00 (9),
       alt-020-00 (10),
       alt-050-00 (11),
       alt-100-00 (12),
       alt-200-00 (13),
       outOfRange (14),
       unavailable (15)
      }
      
      DeltaLongitude ::= INTEGER {oneMicrodegreeEast (10), oneMicrodegreeWest (-10), unavailable(131072)} (-131071..131072)
      
      DeltaLatitude ::= INTEGER {oneMicrodegreeNorth (10), oneMicrodegreeSouth (-10) , unavailable(131072)} (-131071..131072)
      
      DeltaAltitude ::= INTEGER {oneCentimeterUp (1), oneCentimeterDown (-1), unavailable(12800)} (-12700..12800)
      
      PosConfidenceEllipse ::= SEQUENCE {
       semiMajorConfidence SemiAxisLength,
       semiMinorConfidence SemiAxisLength,
       semiMajorOrientation HeadingValue
      }
      
      PathPoint ::= SEQUENCE {
       pathPosition DeltaReferencePosition,
       pathDeltaTime PathDeltaTime OPTIONAL
      }
      
      PathDeltaTime ::= INTEGER {tenMilliSecondsInPast(1)} (1..65535, ...)
      
      PtActivation ::= SEQUENCE {
       ptActivationType PtActivationType,
       ptActivationData PtActivationData
      }
      
      PtActivationType ::= INTEGER {undefinedCodingType(0), r09-16CodingType(1), vdv-50149CodingType(2)} (0..255)
      
      PtActivationData ::= OCTET STRING (SIZE(1..20))
      
      AccelerationControl ::= BIT STRING {
       brakePedalEngaged (0),
       gasPedalEngaged (1),
       emergencyBrakeEngaged (2),
       collisionWarningEngaged (3),
       accEngaged (4),
       cruiseControlEngaged (5),
       speedLimiterEngaged (6)
      } (SIZE(7))
      
      
      SemiAxisLength ::= INTEGER{oneCentimeter(1), outOfRange(4094), unavailable(4095)} (0..4095)
      
      CauseCode ::= SEQUENCE {
       causeCode CauseCodeType,
       subCauseCode SubCauseCodeType,
       ...
      }
      
      CauseCodeType ::= INTEGER {
       reserved (0),
       trafficCondition (1),
       accident (2),
       roadworks (3),
       impassability (5),
       adverseWeatherCondition-Adhesion (6),
       aquaplannning (7),
       hazardousLocation-SurfaceCondition (9),
       hazardousLocation-ObstacleOnTheRoad (10),
       hazardousLocation-AnimalOnTheRoad (11),
       humanPresenceOnTheRoad (12),
       wrongWayDriving (14),
       rescueAndRecoveryWorkInProgress (15),
       adverseWeatherCondition-ExtremeWeatherCondition (17),
       adverseWeatherCondition-Visibility (18),
       adverseWeatherCondition-Precipitation (19),
       slowVehicle (26),
       dangerousEndOfQueue (27),
       vehicleBreakdown (91),
       postCrash (92),
       humanProblem (93),
       stationaryVehicle (94),
       emergencyVehicleApproaching (95),
       hazardousLocation-DangerousCurve (96),
       collisionRisk (97),
       signalViolation (98),
       dangerousSituation (99)
      } (0..255)
      
      SubCauseCodeType ::= INTEGER (0..255)
      
      TrafficConditionSubCauseCode ::= INTEGER {unavailable(0), increasedVolumeOfTraffic(1), trafficJamSlowlyIncreasing(2), trafficJamIncreasing(3), trafficJamStronglyIncreasing(4), trafficStationary(5), trafficJamSlightlyDecreasing(6), trafficJamDecreasing(7), trafficJamStronglyDecreasing(8)} (0..255)
      
      AccidentSubCauseCode ::= INTEGER {unavailable(0), multiVehicleAccident(1), heavyAccident(2), accidentInvolvingLorry(3), accidentInvolvingBus(4), accidentInvolvingHazardousMaterials(5), accidentOnOppositeLane(6), unsecuredAccident(7), assistanceRequested(8)} (0..255)
      
      RoadworksSubCauseCode ::= INTEGER {unavailable(0), majorRoadworks(1), roadMarkingWork(2), slowMovingRoadMaintenance(3), shortTermStationaryRoadworks(4), streetCleaning(5), winterService(6)} (0..255)
      
      HumanPresenceOnTheRoadSubCauseCode ::= INTEGER {unavailable(0), childrenOnRoadway(1), cyclistOnRoadway(2), motorcyclistOnRoadway(3)} (0..255)
      
      WrongWayDrivingSubCauseCode ::= INTEGER {unavailable(0), wrongLane(1), wrongDirection(2)} (0..255)
      
      AdverseWeatherCondition-ExtremeWeatherConditionSubCauseCode ::= INTEGER {unavailable(0), strongWinds(1), damagingHail(2), hurricane(3), thunderstorm(4), tornado(5), blizzard(6)} (0..255)
      
      AdverseWeatherCondition-AdhesionSubCauseCode ::= INTEGER {unavailable(0), heavyFrostOnRoad(1), fuelOnRoad(2), mudOnRoad(3), snowOnRoad(4), iceOnRoad(5), blackIceOnRoad(6), oilOnRoad(7), looseChippings(8), instantBlackIce(9), roadsSalted(10)} (0..255)
      
      AdverseWeatherCondition-VisibilitySubCauseCode ::= INTEGER {unavailable(0), fog(1), smoke(2), heavySnowfall(3), heavyRain(4), heavyHail(5), lowSunGlare(6), sandstorms(7), swarmsOfInsects(8)} (0..255)
      
      AdverseWeatherCondition-PrecipitationSubCauseCode ::= INTEGER {unavailable(0), heavyRain(1), heavySnowfall(2), softHail(3)} (0..255)
      
      SlowVehicleSubCauseCode ::= INTEGER {unavailable(0), maintenanceVehicle(1), vehiclesSlowingToLookAtAccident(2), abnormalLoad(3), abnormalWideLoad(4), convoy(5), snowplough(6), deicing(7), saltingVehicles(8)} (0..255)
       
      StationaryVehicleSubCauseCode ::= INTEGER {unavailable(0), humanProblem(1), vehicleBreakdown(2), postCrash(3), publicTransportStop(4), carryingDangerousGoods(5)} (0..255)
      
      HumanProblemSubCauseCode ::= INTEGER {unavailable(0), glycemiaProblem(1), heartProblem(2)} (0..255)
      
      EmergencyVehicleApproachingSubCauseCode ::= INTEGER {unavailable(0), emergencyVehicleApproaching(1), prioritizedVehicleApproaching(2)} (0..255)
      
      HazardousLocation-DangerousCurveSubCauseCode ::= INTEGER {unavailable(0), dangerousLeftTurnCurve(1), dangerousRightTurnCurve(2), multipleCurvesStartingWithUnknownTurningDirection(3), multipleCurvesStartingWithLeftTurn(4), multipleCurvesStartingWithRightTurn(5)} (0..255)
      
      HazardousLocation-SurfaceConditionSubCauseCode ::= INTEGER {unavailable(0), rockfalls(1), earthquakeDamage(2), sewerCollapse(3), subsidence(4), snowDrifts(5), stormDamage(6), burstPipe(7), volcanoEruption(8), fallingIce(9)} (0..255)
      
      HazardousLocation-ObstacleOnTheRoadSubCauseCode ::= INTEGER {unavailable(0), shedLoad(1), partsOfVehicles(2), partsOfTyres(3), bigObjects(4), fallenTrees(5), hubCaps(6), waitingVehicles(7)} (0..255)
      
      HazardousLocation-AnimalOnTheRoadSubCauseCode ::= INTEGER {unavailable(0), wildAnimals(1), herdOfAnimals(2), smallAnimals(3), largeAnimals(4)} (0..255)
      
      CollisionRiskSubCauseCode ::= INTEGER {unavailable(0), longitudinalCollisionRisk(1), crossingCollisionRisk(2), lateralCollisionRisk(3), vulnerableRoadUser(4)} (0..255)
       
      SignalViolationSubCauseCode ::= INTEGER {unavailable(0), stopSignViolation(1), trafficLightViolation(2), turningRegulationViolation(3)} (0..255)
      
      RescueAndRecoveryWorkInProgressSubCauseCode ::= INTEGER {unavailable(0), emergencyVehicles(1), rescueHelicopterLanding(2), policeActivityOngoing(3), medicalEmergencyOngoing(4), childAbductionInProgress(5)} (0..255)
      
      DangerousEndOfQueueSubCauseCode ::= INTEGER {unavailable(0), suddenEndOfQueue(1), queueOverHill(2), queueAroundBend(3), queueInTunnel(4)} (0..255)
      
      DangerousSituationSubCauseCode ::= INTEGER {unavailable(0), emergencyElectronicBrakeEngaged(1), preCrashSystemEngaged(2), espEngaged(3), absEngaged(4), aebEngaged(5), brakeWarningEngaged(6), collisionRiskWarningEngaged(7)} (0..255)
      
      VehicleBreakdownSubCauseCode ::= INTEGER {unavailable(0), lackOfFuel (1), lackOfBatteryPower (2), engineProblem(3), transmissionProblem(4), engineCoolingProblem(5), brakingSystemProblem(6), steeringProblem(7), tyrePuncture(8), tyrePressureProblem(9)} (0..255)
      
      PostCrashSubCauseCode ::= INTEGER {unavailable(0), accidentWithoutECallTriggered (1), accidentWithECallManuallyTriggered (2), accidentWithECallAutomaticallyTriggered (3), accidentWithECallTriggeredWithoutAccessToCellularNetwork(4)} (0..255)
      
      Curvature ::= SEQUENCE {
       curvatureValue CurvatureValue,
       curvatureConfidence CurvatureConfidence
      }
      
      CurvatureValue ::= INTEGER {straight(0),unavailable(1023)} (-1023..1023)
      
      CurvatureConfidence ::= ENUMERATED {
       onePerMeter-0-00002 (0),
       onePerMeter-0-0001 (1),
       onePerMeter-0-0005 (2),
       onePerMeter-0-002 (3),
       onePerMeter-0-01 (4),
       onePerMeter-0-1 (5),
       outOfRange (6),
       unavailable (7)
      }
      
      CurvatureCalculationMode ::= ENUMERATED {yawRateUsed(0), yawRateNotUsed(1), unavailable(2), ...}
      
      Heading ::= SEQUENCE {
       headingValue HeadingValue,
       headingConfidence HeadingConfidence
      }
      
      HeadingValue ::= INTEGER {wgs84North(0), wgs84East(900), wgs84South(1800), wgs84West(2700), unavailable(3601)} (0..3601)
      
      HeadingConfidence ::= INTEGER {equalOrWithinZeroPointOneDegree (1), equalOrWithinOneDegree (10), outOfRange(126), unavailable(127)} (1..127)
      
      LanePosition ::= INTEGER {offTheRoad(-1), innerHardShoulder(0),
      innermostDrivingLane(1), secondLaneFromInside(2), outerHardShoulder(14) } (-1..14)
      
      ClosedLanes ::= SEQUENCE {
       innerhardShoulderStatus HardShoulderStatus OPTIONAL,
       outerhardShoulderStatus HardShoulderStatus OPTIONAL,
       drivingLaneStatus DrivingLaneStatus OPTIONAL,
       ...
      }
      
      HardShoulderStatus ::= ENUMERATED {availableForStopping(0), closed(1), availableForDriving(2)}
      
      DrivingLaneStatus ::= BIT STRING (SIZE (1..13))
      
      
      PerformanceClass ::= INTEGER {unavailable(0), performanceClassA(1), performanceClassB(2)} (0..7)
      
      SpeedValue ::= INTEGER {standstill(0), oneCentimeterPerSec(1), unavailable(16383)} (0..16383)
      
      SpeedConfidence ::= INTEGER {equalOrWithinOneCentimeterPerSec(1), equalOrWithinOneMeterPerSec(100), outOfRange(126), unavailable(127)} (1..127)
      
      VehicleMass ::= INTEGER {hundredKg(1), unavailable(1024)} (1..1024) 
      
      Speed ::= SEQUENCE {
       speedValue SpeedValue,
       speedConfidence SpeedConfidence
      }
      
      DriveDirection ::= ENUMERATED {forward (0), backward (1), unavailable (2)}
      
      EmbarkationStatus ::= BOOLEAN
      
      LongitudinalAcceleration ::= SEQUENCE {
       longitudinalAccelerationValue LongitudinalAccelerationValue,
       longitudinalAccelerationConfidence AccelerationConfidence
      }
      
      LongitudinalAccelerationValue ::= INTEGER {pointOneMeterPerSecSquaredForward(1), pointOneMeterPerSecSquaredBackward(-1), unavailable(161)} (-160 .. 161)
      
      AccelerationConfidence ::= INTEGER {pointOneMeterPerSecSquared(1), outOfRange(101), unavailable(102)} (0 .. 102)
      
      LateralAcceleration ::= SEQUENCE {
       lateralAccelerationValue LateralAccelerationValue,
       lateralAccelerationConfidence AccelerationConfidence
      }
      
      LateralAccelerationValue ::= INTEGER {pointOneMeterPerSecSquaredToRight(-1), pointOneMeterPerSecSquaredToLeft(1), unavailable(161)} (-160 .. 161)
      
      VerticalAcceleration ::= SEQUENCE {
       verticalAccelerationValue VerticalAccelerationValue,
       verticalAccelerationConfidence AccelerationConfidence
      }
      
      VerticalAccelerationValue ::= INTEGER {pointOneMeterPerSecSquaredUp(1), pointOneMeterPerSecSquaredDown(-1), unavailable(161)} (-160 .. 161)
      
      StationType ::= INTEGER {unknown(0), pedestrian(1), cyclist(2), moped(3), motorcycle(4), passengerCar(5), bus(6), 
      lightTruck(7), heavyTruck(8), trailer(9), specialVehicles(10), tram(11), roadSideUnit(15)} (0..255)
      
      ExteriorLights ::= BIT STRING {
       lowBeamHeadlightsOn (0),
       highBeamHeadlightsOn (1),
       leftTurnSignalOn (2),
       rightTurnSignalOn (3),
       daytimeRunningLightsOn (4),
       reverseLightOn (5),
       fogLightOn (6),
       parkingLightsOn (7)
      } (SIZE(8))
      
      DangerousGoodsBasic::= ENUMERATED {
       explosives1(0),
       explosives2(1),
       explosives3(2),
       explosives4(3),
       explosives5(4),
       explosives6(5),
       flammableGases(6),
       nonFlammableGases(7),
       toxicGases(8),
       flammableLiquids(9),
       flammableSolids(10),
       substancesLiableToSpontaneousCombustion(11),
       substancesEmittingFlammableGasesUponContactWithWater(12),
       oxidizingSubstances(13),
       organicPeroxides(14),
       toxicSubstances(15),
       infectiousSubstances(16),
       radioactiveMaterial(17),
       corrosiveSubstances(18),
       miscellaneousDangerousSubstances(19)
      }
      
      DangerousGoodsExtended ::= SEQUENCE {
       dangerousGoodsType DangerousGoodsBasic,
       unNumber INTEGER (0..9999),
       elevatedTemperature BOOLEAN,
       tunnelsRestricted BOOLEAN,
       limitedQuantity BOOLEAN,
       emergencyActionCode IA5String (SIZE (1..24)) OPTIONAL,
       phoneNumber PhoneNumber OPTIONAL,
       companyName UTF8String (SIZE (1..24)) OPTIONAL,
       ...
      }
      
      SpecialTransportType ::= BIT STRING {heavyLoad(0), excessWidth(1), excessLength(2), excessHeight(3)} (SIZE(4))
      
      LightBarSirenInUse ::= BIT STRING {
       lightBarActivated (0),
       sirenActivated (1)
      } (SIZE(2))
      
      HeightLonCarr ::= INTEGER {oneCentimeter(1), unavailable(100)} (1..100)
      
      PosLonCarr ::= INTEGER {oneCentimeter(1), unavailable(127)} (1..127)
      
      PosPillar ::= INTEGER {tenCentimeters(1), unavailable(30)} (1..30)
      
      PosCentMass ::= INTEGER {tenCentimeters(1), unavailable(63)} (1..63)
      
      RequestResponseIndication ::= ENUMERATED {request(0), response(1)}
      
      SpeedLimit ::= INTEGER {oneKmPerHour(1)} (1..255)
      
      StationarySince ::= ENUMERATED {lessThan1Minute(0), lessThan2Minutes(1), lessThan15Minutes(2), equalOrGreater15Minutes(3)}
      
      Temperature ::= INTEGER {equalOrSmallerThanMinus60Deg (-60), oneDegreeCelsius(1), equalOrGreaterThan67Deg(67)} (-60..67)
      
      TrafficRule ::= ENUMERATED {noPassing(0), noPassingForTrucks(1), passToRight(2), passToLeft(3), ...
      }
      
      WheelBaseVehicle ::= INTEGER {tenCentimeters(1), unavailable(127)} (1..127)
      
      TurningRadius ::= INTEGER {point4Meters(1), unavailable(255)} (1..255)
      
      PosFrontAx ::= INTEGER {tenCentimeters(1), unavailable(20)} (1..20)
      
      PositionOfOccupants ::= BIT STRING {
       row1LeftOccupied (0),
       row1RightOccupied (1),
       row1MidOccupied (2),
       row1NotDetectable (3),
       row1NotPresent (4),
       row2LeftOccupied (5),
       row2RightOccupied (6),
       row2MidOccupied (7),
       row2NotDetectable (8),
       row2NotPresent (9),
       row3LeftOccupied (10),
       row3RightOccupied (11),
       row3MidOccupied (12),
       row3NotDetectable (13),
       row3NotPresent (14),
       row4LeftOccupied (15),
       row4RightOccupied (16),
       row4MidOccupied (17),
       row4NotDetectable (18),
       row4NotPresent (19)} (SIZE(20))
      
      PositioningSolutionType ::= ENUMERATED {noPositioningSolution(0), sGNSS(1), dGNSS(2), sGNSSplusDR(3), dGNSSplusDR(4), dR(5), ...}
      
      VehicleIdentification ::= SEQUENCE {
       wMInumber WMInumber OPTIONAL,
       vDS VDS OPTIONAL,
       ...
      }
      
      WMInumber ::= IA5String (SIZE(1..3))
      
      VDS ::= IA5String (SIZE(6))
      
      EnergyStorageType ::= BIT STRING {hydrogenStorage(0), electricEnergyStorage(1), liquidPropaneGas(2), compressedNaturalGas(3), diesel(4), gasoline(5), ammonia(6)} (SIZE(7))
      
      VehicleLength ::= SEQUENCE {
       vehicleLengthValue VehicleLengthValue,
       vehicleLengthConfidenceIndication VehicleLengthConfidenceIndication
      }
      
      VehicleLengthValue ::= INTEGER {tenCentimeters(1), outOfRange(1022), unavailable(1023)} (1..1023)
      
      VehicleLengthConfidenceIndication ::= ENUMERATED {noTrailerPresent(0), trailerPresentWithKnownLength(1), trailerPresentWithUnknownLength(2), trailerPresenceIsUnknown(3), unavailable(4)}
      
      VehicleWidth ::= INTEGER {tenCentimeters(1), outOfRange(61), unavailable(62)} (1..62)
      
      PathHistory::= SEQUENCE (SIZE(0..40)) OF PathPoint
      
      EmergencyPriority ::= BIT STRING {requestForRightOfWay(0), requestForFreeCrossingAtATrafficLight(1)} (SIZE(2))
      
      InformationQuality ::= INTEGER {unavailable(0), lowest(1), highest(7)} (0..7)
      
      RoadType ::= ENUMERATED {
       urban-NoStructuralSeparationToOppositeLanes(0),
       urban-WithStructuralSeparationToOppositeLanes(1),
       nonUrban-NoStructuralSeparationToOppositeLanes(2),
       nonUrban-WithStructuralSeparationToOppositeLanes(3)}
      
      SteeringWheelAngle ::= SEQUENCE {
       steeringWheelAngleValue SteeringWheelAngleValue,
       steeringWheelAngleConfidence SteeringWheelAngleConfidence
      }
      
      SteeringWheelAngleValue ::= INTEGER {straight(0), onePointFiveDegreesToRight(-1), onePointFiveDegreesToLeft(1), unavailable(512)} (-511..512)
      
      SteeringWheelAngleConfidence ::= INTEGER {equalOrWithinOnePointFiveDegree (1), outOfRange(126), unavailable(127)} (1..127)
      
      TimestampIts ::= INTEGER {utcStartOf2004(0), oneMillisecAfterUTCStartOf2004(1)} (0..4398046511103)
      
      VehicleRole ::= ENUMERATED {default(0), publicTransport(1), specialTransport(2), dangerousGoods(3), roadWork(4), rescue(5), emergency(6), safetyCar(7), agriculture(8),commercial(9),military(10),roadOperator(11),taxi(12), reserved1(13), reserved2(14), reserved3(15)}
      
      YawRate::= SEQUENCE {
       yawRateValue YawRateValue,
       yawRateConfidence YawRateConfidence
      }
      
      YawRateValue ::= INTEGER {straight(0), degSec-000-01ToRight(-1), degSec-000-01ToLeft(1), unavailable(32767)} (-32766..32767)
      
      YawRateConfidence ::= ENUMERATED {
       degSec-000-01 (0),
       degSec-000-05 (1),
       degSec-000-10 (2),
       degSec-001-00 (3),
       degSec-005-00 (4),
       degSec-010-00 (5),
       degSec-100-00 (6),
       outOfRange (7),
       unavailable (8)
      }
      
      ProtectedZoneType::= ENUMERATED { permanentCenDsrcTolling (0), ..., temporaryCenDsrcTolling (1) }
      
      RelevanceDistance ::= ENUMERATED {lessThan50m(0), lessThan100m(1), lessThan200m(2), lessThan500m(3), lessThan1000m(4), lessThan5km(5), lessThan10km(6), over10km(7)}
      
      RelevanceTrafficDirection ::= ENUMERATED {allTrafficDirections(0), upstreamTraffic(1), downstreamTraffic(2), oppositeTraffic(3)}
      
      TransmissionInterval ::= INTEGER {oneMilliSecond(1), tenSeconds(10000)} (1..10000)
      
      ValidityDuration ::= INTEGER {timeOfDetection(0), oneSecondAfterDetection(1)} (0..86400)
      
      ActionID ::= SEQUENCE {
       originatingStationID StationID,
       sequenceNumber SequenceNumber
      }
      
      ItineraryPath ::= SEQUENCE SIZE(1..40) OF ReferencePosition
      
      ProtectedCommunicationZone ::= SEQUENCE {
       protectedZoneType ProtectedZoneType,
       expiryTime TimestampIts OPTIONAL,
       protectedZoneLatitude Latitude,
       protectedZoneLongitude Longitude,
       protectedZoneRadius ProtectedZoneRadius OPTIONAL,
       protectedZoneID ProtectedZoneID OPTIONAL,
       ...
      }
      
      Traces ::= SEQUENCE SIZE(1..7) OF PathHistory
      
      NumberOfOccupants ::= INTEGER {oneOccupant (1), unavailable(127)} (0 .. 127)
      
      SequenceNumber ::= INTEGER (0..65535)
      
      PositionOfPillars ::= SEQUENCE (SIZE(1..3, ...)) OF PosPillar
      
      RestrictedTypes ::= SEQUENCE (SIZE(1..3, ...)) OF StationType
      
      EventHistory::= SEQUENCE (SIZE(1..23)) OF EventPoint
      
      EventPoint ::= SEQUENCE {
       eventPosition DeltaReferencePosition,
       eventDeltaTime PathDeltaTime OPTIONAL,
       informationQuality InformationQuality
      }
      
      ProtectedCommunicationZonesRSU ::= SEQUENCE (SIZE(1..16)) OF ProtectedCommunicationZone 
      
      
      
      CenDsrcTollingZone ::= SEQUENCE {
       protectedZoneLatitude Latitude,
       protectedZoneLongitude Longitude,
       cenDsrcTollingZoneID CenDsrcTollingZoneID OPTIONAL,
       ...
      }
      
      ProtectedZoneRadius ::= INTEGER {oneMeter(1)} (1..255,...)
      
      ProtectedZoneID ::= INTEGER (0.. 134217727)
      
      CenDsrcTollingZoneID ::= ProtectedZoneID
      
      OpeningDaysHours ::= UTF8String 
      
      PhoneNumber ::= NumericString (SIZE(1..16))
      
      
      SPATEM ::= SEQUENCE {
       header ItsPduHeader,
       spat SPAT
      }
      
      MAPEM ::= SEQUENCE {
       header ItsPduHeader,
       map MapData
      }
      
      IVIM ::= SEQUENCE {
       header ItsPduHeader,
       ivi IviStructure
      }
      
      SREM ::= SEQUENCE {
       header ItsPduHeader,
       srm SignalRequestMessage
      }
      
      SSEM ::= SEQUENCE {
       header ItsPduHeader,
       ssm SignalStatusMessage
      }
      
      
      RTCMEM ::= SEQUENCE {
       header ItsPduHeader,
       rtcmc RTCMcorrections
      }
      
      ConnectionManeuverAssist-addGrpC ::= SEQUENCE {
       itsStationPosition ItsStationPositionList OPTIONAL,
       ...
      }
      
      ConnectionTrajectory-addGrpC ::= SEQUENCE { 
       nodes NodeSetXY,
       connectionID LaneConnectionID,
       ...
      }
      
      IntersectionState-addGrpC ::= SEQUENCE {
       activePrioritizations PrioritizationResponseList OPTIONAL,
       ...
      }
      
      LaneAttributes-addGrpC ::= SEQUENCE {
       maxVehicleHeight VehicleHeight OPTIONAL,
       maxVehicleWeight VehicleMass OPTIONAL,
       ...
      }
      
      MapData-addGrpC ::= SEQUENCE {
       signalHeadLocations SignalHeadLocationList OPTIONAL,
       ...
      }
      
      MovementEvent-addGrpC ::= SEQUENCE {
       stateChangeReason ExceptionalCondition OPTIONAL,
       ...
      }
      
      NodeAttributeSet-addGrpC ::= SEQUENCE { 
       ptvRequest PtvRequestType OPTIONAL,
       nodeLink NodeLink OPTIONAL,
       node Node OPTIONAL,
       ...
      }
      
      
      Position3D-addGrpC ::= SEQUENCE {
       altitude Altitude,
       ...
      }
      
      RestrictionUserType-addGrpC ::= SEQUENCE { 
       emission EmissionType OPTIONAL,
       fuel FuelType OPTIONAL,
       ...
      }
      
      RequesterDescription-addGrpC ::= SEQUENCE {
       fuel FuelType OPTIONAL,
       batteryStatus BatteryStatus OPTIONAL,
       ...
      }
      
      SignalStatusPackage-addGrpC ::= SEQUENCE {
       synchToSchedule DeltaTime OPTIONAL,
       rejectedReason RejectedReason OPTIONAL,
       ...
      }
      
      
      
      
      
      ItsStationPosition ::= SEQUENCE {
       stationID StationID,
       laneID LaneID OPTIONAL,
       nodeXY NodeOffsetPointXY OPTIONAL,
       timeReference TimeReference OPTIONAL,
       ...
      }
      
      ItsStationPositionList ::= SEQUENCE SIZE(1..5) OF ItsStationPosition
      
      Node ::= SEQUENCE {
       id INTEGER,
       lane LaneID OPTIONAL,
       connectionID LaneConnectionID OPTIONAL,
       intersectionID IntersectionID OPTIONAL,
       ...
      }
      
      NodeLink ::= SEQUENCE SIZE (1..5) OF Node
      
      PrioritizationResponse ::= SEQUENCE {
       stationID StationID,
       priorState PrioritizationResponseStatus,
       signalGroup SignalGroupID,
       ...
      }
      
      PrioritizationResponseList ::= SEQUENCE SIZE(1..10) OF PrioritizationResponse
      
      SignalHeadLocation ::= SEQUENCE {
       nodeXY NodeOffsetPointXY,
       nodeZ DeltaAltitude,
       signalGroupID SignalGroupID,
       ...
      }
      
      SignalHeadLocationList ::= SEQUENCE (SIZE(1..64)) OF SignalHeadLocation
      
      
      
      
      
      
      BatteryStatus ::= ENUMERATED {
       unknown,
       critical,
       low,
       good, 
       ...
      }
      
      EmissionType ::= ENUMERATED {
       euro1,
       euro2,
       euro3,
       euro4,
       euro5,
       euro6,
       ...
      }
      
      ExceptionalCondition ::= ENUMERATED {
       unknown,
       publicTransportPriority,
       emergencyVehiclePriority,
       trainPriority,
       bridgeOpen,
       vehicleHeight,
       weather,
       trafficJam,
       tunnelClosure,
       meteringActive,
       truckPriority,
       bicyclePlatoonPriority,
       vehiclePlatoonPriority,
       ...
      }
      
      PtvRequestType ::= ENUMERATED {
       preRequest,
       mainRequest,
       doorCloseRequest,
       cancelRequest,
       emergencyRequest,
       ...
      }
      
      RejectedReason ::= ENUMERATED {
       unknown,
       exceptionalCondition,
       maxWaitingTimeExceeded,
       ptPriorityDisabled,
       higherPTPriorityGranted,
       vehicleTrackingUnknown,
       ...
      }
      
      
      
      
      TimeReference ::= INTEGER { oneMilliSec(1) } (0..60000)
      
      Reg-AdvisorySpeed REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-ComputedLane REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-ConnectionManeuverAssist REG-EXT-ID-AND-TYPE ::= {
       {ConnectionManeuverAssist-addGrpC IDENTIFIED BY addGrpC},
       ...
      }
      
      Reg-GenericLane REG-EXT-ID-AND-TYPE ::= {
       {ConnectionTrajectory-addGrpC IDENTIFIED BY addGrpC} ,
       ...
      }
      
      Reg-IntersectionGeometry REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-IntersectionState REG-EXT-ID-AND-TYPE ::= {
       {IntersectionState-addGrpC IDENTIFIED BY addGrpC},
       ...
      }
      
      Reg-LaneAttributes REG-EXT-ID-AND-TYPE ::= {
       {LaneAttributes-addGrpC IDENTIFIED BY addGrpC} ,
       ...
      }
      Reg-LaneDataAttribute REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-MapData REG-EXT-ID-AND-TYPE ::= {
       {MapData-addGrpC IDENTIFIED BY addGrpC},
       ...
      }
      
      Reg-MovementEvent REG-EXT-ID-AND-TYPE ::= {
       {MovementEvent-addGrpC IDENTIFIED BY addGrpC} ,
       ...
      }
      Reg-MovementState REG-EXT-ID-AND-TYPE ::= { ... }
      
      
      
      Reg-NodeAttributeSetXY REG-EXT-ID-AND-TYPE ::= {
       {NodeAttributeSet-addGrpC IDENTIFIED BY addGrpC},
       ...
      }
      
      
      
      Reg-NodeOffsetPointXY REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-Position3D REG-EXT-ID-AND-TYPE ::= {
       {Position3D-addGrpC IDENTIFIED BY addGrpC} ,
       ...
      }
      
      Reg-requesterDescription REG-EXT-ID-AND-TYPE ::= {
       { RequesterDescription-addGrpC IDENTIFIED BY addGrpC} ,
       ...
      }
      
      Reg-requesterType REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-RestrictionUserType REG-EXT-ID-AND-TYPE ::= {
       {RestrictionUserType-addGrpC IDENTIFIED BY addGrpC} ,
       ...
      }
      
      Reg-RoadSegment REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-RTCMcorrections REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-SignalControlZone REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-SignalRequest REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-SignalRequestMessage REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-SignalRequestPackage REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-SignalStatus REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-SignalStatusMessage REG-EXT-ID-AND-TYPE ::= { ... }
      
      Reg-SignalStatusPackage REG-EXT-ID-AND-TYPE ::= {
       { SignalStatusPackage-addGrpC IDENTIFIED BY addGrpC },
       ...
      }
      
      Reg-SPAT REG-EXT-ID-AND-TYPE ::= { ... }
      
      
      REG-EXT-ID-AND-TYPE ::= CLASS {
       &id RegionId UNIQUE,
       &Type
      } WITH SYNTAX {&Type IDENTIFIED BY &id}
      
      RegionalExtension {REG-EXT-ID-AND-TYPE : Set} ::= SEQUENCE {
       regionId REG-EXT-ID-AND-TYPE.&id( {Set} ),
       regExtValue REG-EXT-ID-AND-TYPE.&Type( {Set}{@regionId} )
      }
       
      MapData ::= SEQUENCE {
       timeStamp MinuteOfTheYear OPTIONAL, 
       msgIssueRevision MsgCount, 
       layerType LayerType OPTIONAL,
       layerID LayerID OPTIONAL,
       intersections IntersectionGeometryList OPTIONAL,
       roadSegments RoadSegmentList OPTIONAL,
       dataParameters DataParameters OPTIONAL,
       restrictionList RestrictionClassList OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-MapData}} OPTIONAL,
       ...
      }
      
      TestData ::= SEQUENCE {
        regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-MapData}} OPTIONAL,
       ...
      }
      
      RTCMcorrections ::= SEQUENCE {
       msgCnt MsgCount, 
       rev RTCM-Revision,
       timeStamp MinuteOfTheYear OPTIONAL,
       anchorPoint FullPositionVector OPTIONAL,
       rtcmHeader RTCMheader OPTIONAL, 
       msgs RTCMmessageList,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-RTCMcorrections}} OPTIONAL,
       ...
      }
      
      SPAT ::= SEQUENCE { 
       timeStamp MinuteOfTheYear OPTIONAL, 
       name DescriptiveName OPTIONAL, 
       intersections IntersectionStateList,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-SPAT}} OPTIONAL,
       ...
      }
      
      SignalRequestMessage ::= SEQUENCE { 
       timeStamp MinuteOfTheYear OPTIONAL,
       second DSecond,
       sequenceNumber MsgCount OPTIONAL,
       requests SignalRequestList OPTIONAL,
       requester RequesterDescription,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-SignalRequestMessage}} OPTIONAL,
       ...
      }
      
      SignalStatusMessage ::= SEQUENCE {
       timeStamp MinuteOfTheYear OPTIONAL,
       second DSecond,
       sequenceNumber MsgCount OPTIONAL, 
       status SignalStatusList,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-SignalStatusMessage}} OPTIONAL,
       ...
      }
      
      
      
      
      
      
      
      
      
      AdvisorySpeed ::= SEQUENCE {
       type AdvisorySpeedType,
       speed SpeedAdvice OPTIONAL,
       confidence SAESpeedConfidence OPTIONAL,
       distance ZoneLength OPTIONAL,
       class RestrictionClassID OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-AdvisorySpeed}} OPTIONAL,
       ...
      }
      
      AdvisorySpeedList ::= SEQUENCE (SIZE(1..16)) OF AdvisorySpeed
      
      AntennaOffsetSet ::= SEQUENCE { 
       antOffsetX Offset-B12,
       antOffsetY Offset-B09,
       antOffsetZ Offset-B10
       }
      
      OffsetXaxis ::= CHOICE {
       small DrivenLineOffsetSm, 
       large DrivenLineOffsetLg
      }
      
      OffsetYaxis ::= CHOICE {
       small DrivenLineOffsetSm, 
       large DrivenLineOffsetLg
      }
      
      ComputedLane ::= SEQUENCE {
       referenceLaneId LaneID,
       offsetXaxis OffsetXaxis, 
       offsetYaxis OffsetYaxis, 
       rotateXY Angle OPTIONAL, 
       scaleXaxis Scale-B12 OPTIONAL, 
       scaleYaxis Scale-B12 OPTIONAL, 
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-ComputedLane}} OPTIONAL,
       ... 
      }
      
      ConnectsToList ::= SEQUENCE (SIZE(1..16)) OF Connection
      
      ConnectingLane ::= SEQUENCE {
       lane LaneID,
       maneuver AllowedManeuvers OPTIONAL
      }
      
      Connection ::= SEQUENCE {
       connectingLane ConnectingLane, 
       remoteIntersection IntersectionReferenceID OPTIONAL, 
       signalGroup SignalGroupID OPTIONAL, 
       userClass RestrictionClassID OPTIONAL, 
       connectionID LaneConnectionID OPTIONAL
      }
      
      ConnectionManeuverAssist ::= SEQUENCE {
       connectionID LaneConnectionID,
       queueLength ZoneLength OPTIONAL,
       availableStorageLength ZoneLength OPTIONAL,
       waitOnStop WaitOnStopline OPTIONAL,
       pedBicycleDetect PedestrianBicycleDetect OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-ConnectionManeuverAssist}} OPTIONAL,
       ... 
      }
      
      DataParameters ::= SEQUENCE {
       processMethod IA5String(SIZE(1..255)) OPTIONAL, 
       processAgency IA5String(SIZE(1..255)) OPTIONAL, 
       lastCheckedDate IA5String(SIZE(1..255)) OPTIONAL, 
       geoidUsed IA5String(SIZE(1..255)) OPTIONAL, 
       ... 
      }
      
      
      DDateTime ::= SEQUENCE {
       year DYear OPTIONAL, 
       month DMonth OPTIONAL, 
       day DDay OPTIONAL, 
       hour DHour OPTIONAL, 
       minute DMinute OPTIONAL, 
       second DSecond OPTIONAL, 
       offset DOffset OPTIONAL
       }
      
      EnabledLaneList ::= SEQUENCE (SIZE(1..16)) OF LaneID
      
      FullPositionVector ::= SEQUENCE {
       utcTime DDateTime OPTIONAL,
       long Longitude,
       lat Latitude,
       elevation Elevation OPTIONAL,
       heading SAEHeading OPTIONAL, 
       speed TransmissionAndSpeed OPTIONAL,
       posAccuracy PositionalAccuracy OPTIONAL,
       timeConfidence TimeConfidence OPTIONAL,
       posConfidence PositionConfidenceSet OPTIONAL,
       speedConfidence SpeedandHeadingandThrottleConfidence OPTIONAL, 
       ... 
       }
      
      
      GenericLane ::= SEQUENCE { 
       laneID LaneID, 
       name DescriptiveName OPTIONAL, 
       ingressApproach ApproachID OPTIONAL,
       egressApproach ApproachID OPTIONAL,
       laneAttributes LaneAttributes, 
       maneuvers AllowedManeuvers OPTIONAL,
       nodeList NodeListXY,
       connectsTo ConnectsToList OPTIONAL, 
       overlays OverlayLaneList OPTIONAL, 
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-GenericLane}} OPTIONAL,
       ...
      }
      
      IntersectionAccessPoint ::= CHOICE {
       lane LaneID,
       approach ApproachID,
       connection LaneConnectionID,
       ...
      }
      
      IntersectionGeometry ::= SEQUENCE {
       name DescriptiveName OPTIONAL, 
       id IntersectionReferenceID, 
       revision MsgCount, 
       refPoint Position3D,
       laneWidth LaneWidth OPTIONAL, 
       speedLimits SpeedLimitList OPTIONAL, 
       laneSet LaneList,
       preemptPriorityData PreemptPriorityList OPTIONAL, 
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-IntersectionGeometry}} OPTIONAL,
       ... 
      }
      
      IntersectionGeometryList ::= SEQUENCE (SIZE(1..32)) OF IntersectionGeometry
       
      IntersectionReferenceID ::= SEQUENCE {
       region RoadRegulatorID OPTIONAL,
       id IntersectionID
      }
      
      IntersectionState ::= SEQUENCE {
       name DescriptiveName OPTIONAL, 
       id IntersectionReferenceID, 
       revision MsgCount, 
       status IntersectionStatusObject,
       moy MinuteOfTheYear OPTIONAL,
       timeStamp DSecond OPTIONAL, 
       enabledLanes EnabledLaneList OPTIONAL, 
       states MovementList,
       maneuverAssistList ManeuverAssistList OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-IntersectionState}} OPTIONAL,
       ... 
      }
      
      IntersectionStateList ::= SEQUENCE (SIZE(1..32)) OF IntersectionState
      
      LaneAttributes ::= SEQUENCE {
       directionalUse LaneDirection,
       sharedWith LaneSharing,
       laneType LaneTypeAttributes,
       regional RegionalExtension {{Reg-LaneAttributes}} OPTIONAL
      }
      
      LaneDataAttribute ::= CHOICE {
       pathEndPointAngle DeltaAngle, 
       laneCrownPointCenter RoadwayCrownAngle, 
       laneCrownPointLeft RoadwayCrownAngle, 
       laneCrownPointRight RoadwayCrownAngle, 
       laneAngle MergeDivergeNodeAngle, 
       speedLimits SpeedLimitList,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-LaneDataAttribute}}, 
       ...
      }
      
      LaneDataAttributeList ::= SEQUENCE (SIZE(1..8)) OF LaneDataAttribute
      
      LaneList ::= SEQUENCE (SIZE(1..255)) OF GenericLane
      
      LaneSharing ::= BIT STRING {
       overlappingLaneDescriptionProvided (0),
       multipleLanesTreatedAsOneLane (1),
       otherNonMotorizedTrafficTypes (2),
       individualMotorizedVehicleTraffic (3),
       busVehicleTraffic (4), 
       taxiVehicleTraffic (5),
       pedestriansTraffic (6),
       cyclistVehicleTraffic (7),
       trackedVehicleTraffic (8),
       pedestrianTraffic (9)
      } (SIZE (10))
      
      LaneTypeAttributes ::= CHOICE {
       vehicle LaneAttributes-Vehicle,
       crosswalk LaneAttributes-Crosswalk,
       bikeLane LaneAttributes-Bike,
       sidewalk LaneAttributes-Sidewalk,
       median LaneAttributes-Barrier,
       striping LaneAttributes-Striping,
       trackedVehicle LaneAttributes-TrackedVehicle,
       parking LaneAttributes-Parking,
       ...
      }
      
      ManeuverAssistList ::= SEQUENCE (SIZE(1..16)) OF ConnectionManeuverAssist
      
      MovementEvent ::= SEQUENCE {
       eventState MovementPhaseState,
       timing TimeChangeDetails OPTIONAL,
       speeds AdvisorySpeedList OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-MovementEvent}} OPTIONAL,
       ... 
      }
      
      MovementEventList ::= SEQUENCE (SIZE(1..16)) OF MovementEvent
      
      MovementList ::= SEQUENCE (SIZE(1..255)) OF MovementState
      
      MovementState ::= SEQUENCE {
       movementName DescriptiveName OPTIONAL,
       signalGroup SignalGroupID, 
       state-time-speed MovementEventList, 
       maneuverAssistList ManeuverAssistList OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-MovementState}} OPTIONAL,
       ... 
      }
      
      NodeAttributeSetXY ::= SEQUENCE {
       localNode NodeAttributeXYList OPTIONAL,
       disabled SegmentAttributeXYList OPTIONAL,
       enabled SegmentAttributeXYList OPTIONAL,
       data LaneDataAttributeList OPTIONAL,
       dWidth Offset-B10 OPTIONAL,
       dElevation Offset-B10 OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-NodeAttributeSetXY}} OPTIONAL,
       ...
      }
      
      NodeAttributeXY ::= ENUMERATED {
       reserved, 
       stopLine,
       roundedCapStyleA,
       roundedCapStyleB,
       mergePoint,
       divergePoint,
       downstreamStopLine,
       downstreamStartNode,
       closedToTraffic,
       safeIsland,
       curbPresentAtStepOff,
       hydrantPresent,
       ...
      }
      
      NodeAttributeXYList ::= SEQUENCE (SIZE(1..8)) OF NodeAttributeXY
      
      Node-LLmD-64b ::= SEQUENCE {
       lon Longitude,
       lat Latitude
      }
      
      Node-XY-20b ::= SEQUENCE {
       x Offset-B10,
       y Offset-B10
      }
      
      Node-XY-22b ::= SEQUENCE {
       x Offset-B11,
       y Offset-B11
      }
      
      Node-XY-24b ::= SEQUENCE {
       x Offset-B12,
       y Offset-B12
      }
      
      Node-XY-26b ::= SEQUENCE {
       x Offset-B13,
       y Offset-B13
      }
      
      Node-XY-28b ::= SEQUENCE {
       x Offset-B14,
       y Offset-B14
      }
      
      Node-XY-32b ::= SEQUENCE {
       x Offset-B16,
       y Offset-B16
      }
      
      NodeListXY ::= CHOICE {
       nodes NodeSetXY,
       computed ComputedLane,
       ...
      }
      
      NodeOffsetPointXY ::= CHOICE {
       node-XY1 Node-XY-20b,
       node-XY2 Node-XY-22b,
       node-XY3 Node-XY-24b,
       node-XY4 Node-XY-26b,
       node-XY5 Node-XY-28b,
       node-XY6 Node-XY-32b,
       node-LatLon Node-LLmD-64b,
       regional RegionalExtension {{Reg-NodeOffsetPointXY}}
      }
      
      NodeXY ::= SEQUENCE {
       delta NodeOffsetPointXY,
       attributes NodeAttributeSetXY OPTIONAL,
       ... 
      }
      
      NodeSetXY ::= SEQUENCE (SIZE(2..63)) OF NodeXY
      
      OverlayLaneList ::= SEQUENCE (SIZE(1..5)) OF LaneID
      
      PositionalAccuracy ::= SEQUENCE {
       semiMajor SemiMajorAxisAccuracy,
       semiMinor SemiMinorAxisAccuracy,
       orientation SemiMajorAxisOrientation
      }
      
      PositionConfidenceSet ::= SEQUENCE {
       pos PositionConfidence,
       elevation ElevationConfidence 
       }
      
      
      Position3D ::= SEQUENCE {
       lat Latitude,
       long Longitude,
       elevation Elevation OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-Position3D}} OPTIONAL,
       ...
      }
      
      PreemptPriorityList ::= SEQUENCE (SIZE(1..32)) OF SignalControlZone
      
      RegulatorySpeedLimit ::= SEQUENCE {
       type SpeedLimitType, 
       speed Velocity
      }
      
      RequesterDescription ::= SEQUENCE {
       id VehicleID, 
       type RequesterType OPTIONAL,
       position RequesterPositionVector OPTIONAL,
       name DescriptiveName OPTIONAL,
       routeName DescriptiveName OPTIONAL,
       transitStatus TransitVehicleStatus OPTIONAL,
       transitOccupancy TransitVehicleOccupancy OPTIONAL,
       transitSchedule DeltaTime OPTIONAL, 
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-requesterDescription}} OPTIONAL,
       ...
      }
      
      RequesterPositionVector ::= SEQUENCE {
       position Position3D,
       heading Angle OPTIONAL, 
       speed TransmissionAndSpeed OPTIONAL,
       ... 
      }
      
      RequesterType ::= SEQUENCE {
       role BasicVehicleRole,
       subrole RequestSubRole OPTIONAL,
       request RequestImportanceLevel OPTIONAL,
       iso3883 Iso3833VehicleType OPTIONAL,
       hpmsType VehicleType OPTIONAL,
       regional RegionalExtension {{Reg-requesterType}} OPTIONAL,
       ...
      }
      
      RestrictionClassAssignment ::= SEQUENCE {
       id RestrictionClassID,
       users RestrictionUserTypeList 
      }
      
      RestrictionClassList ::= SEQUENCE (SIZE(1..254)) OF RestrictionClassAssignment
      
      RestrictionUserType ::= CHOICE {
       basicType RestrictionAppliesTo,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-RestrictionUserType}}, 
       ...
      }
      
      RestrictionUserTypeList ::= SEQUENCE (SIZE(1..16)) OF RestrictionUserType
      
      RoadLaneSetList ::= SEQUENCE (SIZE(1..255)) OF GenericLane
      
      RoadSegmentReferenceID ::= SEQUENCE {
       region RoadRegulatorID OPTIONAL,
       id RoadSegmentID
      }
      
      RoadSegment ::= SEQUENCE {
       name DescriptiveName OPTIONAL,
       id RoadSegmentReferenceID,
       revision MsgCount, 
       refPoint Position3D,
       laneWidth LaneWidth OPTIONAL, 
       speedLimits SpeedLimitList OPTIONAL, 
       roadLaneSet RoadLaneSetList, 
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-RoadSegment}} OPTIONAL,
       ...
      }
      
      RoadSegmentList ::= SEQUENCE (SIZE(1..32)) OF RoadSegment
      
      RTCMheader ::= SEQUENCE {
       status GNSSstatus,
       offsetSet AntennaOffsetSet
       }
      
      RTCMmessageList ::= SEQUENCE (SIZE(1..5)) OF RTCMmessage
      
      SegmentAttributeXYList ::= SEQUENCE (SIZE(1..8)) OF SegmentAttributeXY
      
      SignalControlZone ::= SEQUENCE {
       zone RegionalExtension {{Reg-SignalControlZone}},
       ...
      }
      
      SignalRequesterInfo ::= SEQUENCE {
       id VehicleID,
       request RequestID,
       sequenceNumber MsgCount,
       role BasicVehicleRole OPTIONAL,
       typeData RequesterType OPTIONAL, 
       ...
      }
      
      SignalRequest ::= SEQUENCE {
       id IntersectionReferenceID, 
       requestID RequestID, 
       requestType PriorityRequestType, 
       inBoundLane IntersectionAccessPoint, 
       outBoundLane IntersectionAccessPoint OPTIONAL, 
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-SignalRequest}} OPTIONAL,
       ...
      }
      
      SignalRequestList ::= SEQUENCE (SIZE(1..32)) OF SignalRequestPackage
      
      SignalRequestPackage ::= SEQUENCE {
       request SignalRequest,
       minute MinuteOfTheYear OPTIONAL,
       second DSecond OPTIONAL,
       duration DSecond OPTIONAL,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-SignalRequestPackage}} OPTIONAL,
       ... 
      }
      
      SignalStatus ::= SEQUENCE {
       sequenceNumber MsgCount,
       id IntersectionReferenceID,
       sigStatus SignalStatusPackageList,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-SignalStatus}} OPTIONAL,
       ... 
      }
      
      SignalStatusList ::= SEQUENCE (SIZE(1..32)) OF SignalStatus
      
      SignalStatusPackageList ::= SEQUENCE (SIZE(1..32)) OF SignalStatusPackage
      
      SignalStatusPackage ::= SEQUENCE {
       requester SignalRequesterInfo OPTIONAL,
       inboundOn IntersectionAccessPoint,
       outboundOn IntersectionAccessPoint OPTIONAL,
      
       minute MinuteOfTheYear OPTIONAL,
       second DSecond OPTIONAL,
       duration DSecond OPTIONAL,
       status PrioritizationResponseStatus,
       regional SEQUENCE (SIZE(1..4)) OF 
       RegionalExtension {{Reg-SignalStatusPackage}} OPTIONAL,
       ... 
      }
      
      SpeedandHeadingandThrottleConfidence ::= SEQUENCE {
       heading SAEHeadingConfidence, 
       speed SAESpeedConfidence, 
       throttle SAEThrottleConfidence 
       }
      
       
      SpeedLimitList ::= SEQUENCE (SIZE(1..9)) OF RegulatorySpeedLimit
      
      SpeedLimitType ::= ENUMERATED {
       unknown,
       maxSpeedInSchoolZone,
       maxSpeedInSchoolZoneWhenChildrenArePresent,
       maxSpeedInConstructionZone,
       vehicleMinSpeed,
       vehicleMaxSpeed,
       vehicleNightMaxSpeed,
       truckMinSpeed,
       truckMaxSpeed,
       truckNightMaxSpeed,
       vehiclesWithTrailersMinSpeed,
       vehiclesWithTrailersMaxSpeed,
       vehiclesWithTrailersNightMaxSpeed,
       ...
      }
      
      TimeChangeDetails ::= SEQUENCE {
       startTime TimeMark OPTIONAL,
       minEndTime TimeMark,
       maxEndTime TimeMark OPTIONAL,
       likelyTime TimeMark OPTIONAL,
       confidence TimeIntervalConfidence OPTIONAL,
       nextTime TimeMark OPTIONAL
      }
      
      TimeMark ::= INTEGER (0..36001) 
      
      TransmissionAndSpeed ::= SEQUENCE {
       transmisson TransmissionState,
       speed Velocity
      }
      
      VehicleID ::= CHOICE {
       entityID TemporaryID,
       stationID StationID
      }
      
      
      
      
      
      AdvisorySpeedType ::= ENUMERATED {
       none (0),
       greenwave (1),
       ecoDrive (2),
       transit (3),
       ... 
      }
      
      AllowedManeuvers ::= BIT STRING {
       maneuverStraightAllowed (0), 
       maneuverLeftAllowed (1), 
       maneuverRightAllowed (2), 
       maneuverUTurnAllowed (3), 
       maneuverLeftTurnOnRedAllowed (4), 
       maneuverRightTurnOnRedAllowed (5), 
       maneuverLaneChangeAllowed (6), 
       maneuverNoStoppingAllowed (7), 
       yieldAllwaysRequired (8), 
       goWithHalt (9), 
       caution (10), 
       reserved1 (11)
      } (SIZE(12))
      
      Angle ::= INTEGER (0..28800) 
      
      ApproachID ::= INTEGER (0..15)
       
      BasicVehicleRole ::= ENUMERATED {
       basicVehicle (0),
       publicTransport (1),
       specialTransport (2),
       dangerousGoods (3),
       roadWork (4),
       roadRescue (5),
       emergency (6),
       safetyCar (7),
       none-unknown (8),
       truck (9),
       motorcycle (10),
       roadSideSource (11),
       police (12),
       fire (13),
       ambulance (14),
       dot (15),
       transit (16),
       slowMoving (17),
       stopNgo (18),
       cyclist (19),
       pedestrian (20),
       nonMotorized (21),
       military (22),
       ... 
      } 
      
      DDay ::= INTEGER (0..31)
      
      DeltaAngle ::= INTEGER (-150..150) 
      
      DeltaTime ::= INTEGER (-122 .. 121) 
      
      DescriptiveName ::= IA5String (SIZE(1..63))
      
      DHour ::= INTEGER (0..31)
      
      DMinute ::= INTEGER (0..60)
      
      DMonth ::= INTEGER (0..12)
      
      DOffset ::= INTEGER (-840..840)
      
      DrivenLineOffsetLg ::= INTEGER (-32767..32767) 
      
      DrivenLineOffsetSm ::= INTEGER (-2047..2047) 
      
      DSecond ::= INTEGER (0..65535)
      
      DSRCmsgID ::= INTEGER (0..32767)
       mapData DSRCmsgID ::= 18
       rtcmCorrections DSRCmsgID ::= 28
       signalPhaseAndTimingMessage DSRCmsgID ::= 19
       signalRequestMessage DSRCmsgID ::= 29
       signalStatusMessage DSRCmsgID ::= 30
      
      DYear ::= INTEGER (0..4095)
      
      Elevation ::= INTEGER (-4096..61439)
      
      ElevationConfidence ::= ENUMERATED {
       unavailable (0),
       elev-500-00 (1),
       elev-200-00 (2),
       elev-100-00 (3),
       elev-050-00 (4),
       elev-020-00 (5),
       elev-010-00 (6),
       elev-005-00 (7),
       elev-002-00 (8),
       elev-001-00 (9),
       elev-000-50 (10),
       elev-000-20 (11),
       elev-000-10 (12),
       elev-000-05 (13),
       elev-000-02 (14),
       elev-000-01 (15)
       }
      
      
      FuelType ::= INTEGER (0..15) 
       unknownFuel FuelType ::= 0
       gasoline FuelType ::= 1 
       ethanol FuelType ::= 2
       diesel FuelType ::= 3
       electric FuelType ::= 4 
       hybrid FuelType ::= 5
       hydrogen FuelType ::= 6 
       natGasLiquid FuelType ::= 7
       natGasComp FuelType ::= 8
       propane FuelType ::= 9
       
      GNSSstatus ::= BIT STRING {
       unavailable (0),
       isHealthy (1),
       isMonitored (2),
       baseStationType (3),
       aPDOPofUnder5 (4),
       inViewOfUnder5 (5),
       localCorrectionsPresent (6),
       networkCorrectionsPresent (7)
       } (SIZE(8))
      
      SAEHeadingConfidence ::= ENUMERATED {
       unavailable (0),
       prec10deg (1),
       prec05deg (2),
       prec01deg (3),
       prec0-1deg (4),
       prec0-05deg (5),
       prec0-01deg (6),
       prec0-0125deg (7)
       } 
       
      SAEHeading ::= INTEGER (0..28800)
      
      IntersectionID ::= INTEGER (0..65535)
      
      
      IntersectionStatusObject ::= BIT STRING {
       manualControlIsEnabled (0),
       stopTimeIsActivated (1),
       failureFlash (2), 
       preemptIsActive (3),
       signalPriorityIsActive (4), 
       fixedTimeOperation (5),
       trafficDependentOperation (6),
       standbyOperation (7),
       failureMode (8),
       off (9),
       recentMAPmessageUpdate (10),
       recentChangeInMAPassignedLanesIDsUsed (11),
       noValidMAPisAvailableAtThisTime (12),
       noValidSPATisAvailableAtThisTime (13)
      } (SIZE(16))
      
      LaneAttributes-Barrier ::= BIT STRING {
       median-RevocableLane (0),
       median (1),
       whiteLineHashing (2),
       stripedLines (3),
       doubleStripedLines (4),
       trafficCones (5),
       constructionBarrier (6),
       trafficChannels (7),
       lowCurbs (8),
       highCurbs (9)
      } (SIZE (16))
      
      LaneAttributes-Bike ::= BIT STRING {
       bikeRevocableLane (0),
       pedestrianUseAllowed (1),
       isBikeFlyOverLane (2),
       fixedCycleTime (3),
       biDirectionalCycleTimes (4),
       isolatedByBarrier (5),
       unsignalizedSegmentsPresent (6)
      } (SIZE (16))
      
      LaneAttributes-Crosswalk ::= BIT STRING { 
       crosswalkRevocableLane (0),
       bicyleUseAllowed (1),
       isXwalkFlyOverLane (2),
       fixedCycleTime (3),
       biDirectionalCycleTimes (4),
       hasPushToWalkButton (5),
       audioSupport (6),
       rfSignalRequestPresent (7),
       unsignalizedSegmentsPresent (8)
      } (SIZE (16))
      
      LaneAttributes-Parking ::= BIT STRING { 
       parkingRevocableLane (0),
       parallelParkingInUse (1),
       headInParkingInUse (2),
       doNotParkZone (3),
       parkingForBusUse (4),
       parkingForTaxiUse (5),
       noPublicParkingUse (6)
      } (SIZE (16))
      
      LaneAttributes-Sidewalk ::= BIT STRING { 
       sidewalk-RevocableLane (0),
       bicyleUseAllowed (1),
       isSidewalkFlyOverLane (2),
       walkBikes (3)
      } (SIZE (16))
      
      LaneAttributes-Striping ::= BIT STRING { 
       stripeToConnectingLanesRevocableLane (0),
       stripeDrawOnLeft (1),
       stripeDrawOnRight (2),
       stripeToConnectingLanesLeft (3),
       stripeToConnectingLanesRight (4),
       stripeToConnectingLanesAhead (5)
      } (SIZE (16))
      
      LaneAttributes-TrackedVehicle ::= BIT STRING { 
       spec-RevocableLane (0),
       spec-commuterRailRoadTrack (1), 
       spec-lightRailRoadTrack (2),
       spec-heavyRailRoadTrack (3),
       spec-otherRailType (4)
      } (SIZE (16))
      
      
      LaneAttributes-Vehicle ::= BIT STRING { 
       isVehicleRevocableLane (0),
       isVehicleFlyOverLane (1),
       hovLaneUseOnly (2),
       restrictedToBusUse (3),
       restrictedToTaxiUse (4),
       restrictedFromPublicUse (5),
       hasIRbeaconCoverage (6),
       permissionOnRequest (7)
      } (SIZE (8,...))
      
      LaneConnectionID ::= INTEGER (0..255)
      
      LaneDirection ::= BIT STRING {
       ingressPath (0), 
       egressPath (1)
      } (SIZE (2))
      
      LaneID ::= INTEGER (0..255)
      
      LayerID ::= INTEGER (0..100)
      
      LayerType ::= ENUMERATED {
       none, 
       mixedContent,
       generalMapData, 
       intersectionData, 
       curveData, 
       roadwaySectionData, 
       parkingAreaData, 
       sharedLaneData,
       ... 
      }
       
      LaneWidth ::= INTEGER (0..32767)
      
      MergeDivergeNodeAngle ::= INTEGER (-180..180) 
      
      MinuteOfTheYear ::= INTEGER (0..527040) 
      
      MovementPhaseState ::= ENUMERATED {
       unavailable (0), 
       dark (1), 
       stop-Then-Proceed (2), 
       stop-And-Remain (3),
       pre-Movement (4), 
       permissive-Movement-Allowed (5), 
       protected-Movement-Allowed (6), 
       permissive-clearance (7), 
       protected-clearance (8), 
       caution-Conflicting-Traffic (9)
      }
      
      MsgCount ::= INTEGER (0..127)
      
      Offset-B09 ::= INTEGER (-256..255)
      
      Offset-B10 ::= INTEGER (-512..511)
      
      Offset-B11 ::= INTEGER (-1024..1023)
      
      Offset-B12 ::= INTEGER (-2048..2047)
      
      Offset-B13 ::= INTEGER (-4096..4095)
      
      Offset-B14 ::= INTEGER (-8192..8191)
      
      Offset-B16 ::= INTEGER (-32768..32767)
      
      PedestrianBicycleDetect ::= BOOLEAN 
      
      PositionConfidence ::= ENUMERATED {
       unavailable (0),
       a500m (1),
       a200m (2),
       a100m (3),
       a50m (4),
       a20m (5),
       a10m (6),
       a5m (7),
       a2m (8),
       a1m (9),
       a50cm (10),
       a20cm (11),
       a10cm (12),
       a5cm (13),
       a2cm (14),
       a1cm (15)
       } 
      
      PrioritizationResponseStatus ::= ENUMERATED {
       unknown (0),
       requested (1),
       processing (2),
       watchOtherTraffic (3),
       granted (4),
       rejected (5),
       maxPresence (6),
       reserviceLocked (7),
       ...
      }
      
      PriorityRequestType ::= ENUMERATED {
       priorityRequestTypeReserved (0), 
       priorityRequest (1), 
       priorityRequestUpdate (2), 
       priorityCancellation (3), 
       ... 
      }
      
      RegionId ::= INTEGER (0..255)
       noRegion RegionId ::= 0
       addGrpA RegionId ::= 1
       addGrpB RegionId ::= 2
       addGrpC RegionId ::= 3
      
      RequestID ::= INTEGER (0..255)
      
      RequestImportanceLevel ::= ENUMERATED {
       requestImportanceLevelUnKnown (0),
       requestImportanceLevel1 (1),
       requestImportanceLevel2 (2),
       requestImportanceLevel3 (3),
       requestImportanceLevel4 (4),
       requestImportanceLevel5 (5),
       requestImportanceLevel6 (6),
       requestImportanceLevel7 (7),
       requestImportanceLevel8 (8),
       requestImportanceLevel9 (9),
       requestImportanceLevel10 (10),
       requestImportanceLevel11 (11),
       requestImportanceLevel12 (12),
       requestImportanceLevel13 (13),
       requestImportanceLevel14 (14),
       requestImportanceReserved (15)
      }
      
      
      RequestSubRole ::= ENUMERATED {
       requestSubRoleUnKnown (0),
       requestSubRole1 (1),
       requestSubRole2 (2),
       requestSubRole3 (3),
       requestSubRole4 (4),
       requestSubRole5 (5),
       requestSubRole6 (6),
       requestSubRole7 (7),
       requestSubRole8 (8),
       requestSubRole9 (9),
       requestSubRole10 (10),
       requestSubRole11 (11),
       requestSubRole12 (12),
       requestSubRole13 (13),
       requestSubRole14 (14),
       requestSubRoleReserved (15)
      }
      
      RestrictionAppliesTo ::= ENUMERATED {
       none,
       equippedTransit,
       equippedTaxis,
       equippedOther,
       emissionCompliant,
       equippedBicycle,
       weightCompliant,
       heightCompliant,
       pedestrians,
       slowMovingPersons,
       wheelchairUsers,
       visualDisabilities,
       audioDisabilities,
       otherUnknownDisabilities,
       ...
      }
      
      RestrictionClassID ::= INTEGER (0..255)
      
      RoadRegulatorID ::= INTEGER (0..65535)
      
      RoadSegmentID ::= INTEGER (0..65535)
      
      RoadwayCrownAngle ::= INTEGER (-128..127) 
      
      RTCMmessage ::= OCTET STRING (SIZE(1..1023))
       
      RTCM-Revision ::= ENUMERATED {
       unknown (0),
       rtcmRev2 (1),
       rtcmRev3 (2),
       reserved (3),
       ...
      }
       
      Scale-B12 ::= INTEGER (-2048..2047)
      
      SignalGroupID ::= INTEGER (0..255) 
      
      SegmentAttributeXY ::= ENUMERATED {
       reserved , 
       doNotBlock ,
       whiteLine ,
       mergingLaneLeft ,
       mergingLaneRight ,
       curbOnLeft ,
       curbOnRight ,
       loadingzoneOnLeft ,
       loadingzoneOnRight ,
       turnOutPointOnLeft ,
       turnOutPointOnRight ,
       adjacentParkingOnLeft ,
       adjacentParkingOnRight ,
       adjacentBikeLaneOnLeft ,
       adjacentBikeLaneOnRight ,
       sharedBikeLane ,
       bikeBoxInFront ,
       transitStopOnLeft ,
       transitStopOnRight ,
       transitStopInLane ,
       sharedWithTrackedVehicle ,
       safeIsland ,
       lowCurbsPresent ,
       rumbleStripPresent ,
       audibleSignalingPresent ,
       adaptiveTimingPresent ,
       rfSignalRequestPresent ,
       partialCurbIntrusion ,
       taperToLeft ,
       taperToRight ,
       taperToCenterLine ,
       parallelParking ,
       headInParking ,
       freeParking ,
       timeRestrictionsOnParking ,
       costToPark ,
       midBlockCurbPresent ,
       unEvenPavementPresent ,
       ...
      }
      
      SemiMajorAxisAccuracy ::= INTEGER (0..255)
      
      SemiMajorAxisOrientation ::= INTEGER (0..65535)
      
      SemiMinorAxisAccuracy ::= INTEGER (0..255)
      
      SpeedAdvice ::= INTEGER (0..500) 
      
      SAESpeedConfidence ::= ENUMERATED {
       unavailable (0),
       prec100ms (1),
       prec10ms (2),
       prec5ms (3),
       prec1ms (4),
       prec0-1ms (5),
       prec0-05ms (6),
       prec0-01ms (7)
       }
      
      TemporaryID ::= OCTET STRING (SIZE(4))
      
      SAEThrottleConfidence ::= ENUMERATED {
       unavailable (0),
       prec10percent (1),
       prec1percent (2),
       prec0-5percent (3)
       }
      
      TimeConfidence ::= ENUMERATED {
       unavailable (0), 
       time-100-000 (1),
       time-050-000 (2),
       time-020-000 (3),
       time-010-000 (4),
       time-002-000 (5),
       time-001-000 (6),
       time-000-500 (7),
       time-000-200 (8),
       time-000-100 (9),
       time-000-050 (10),
       time-000-020 (11),
       time-000-010 (12),
       time-000-005 (13),
       time-000-002 (14),
       time-000-001 (15),
       time-000-000-5 (16),
       time-000-000-2 (17),
       time-000-000-1 (18),
       time-000-000-05 (19),
       time-000-000-02 (20),
       time-000-000-01 (21),
       time-000-000-005 (22),
       time-000-000-002 (23),
       time-000-000-001 (24),
       time-000-000-000-5 (25),
       time-000-000-000-2 (26),
       time-000-000-000-1 (27),
       time-000-000-000-05 (28),
       time-000-000-000-02 (29),
       time-000-000-000-01 (30),
       time-000-000-000-005 (31),
       time-000-000-000-002 (32),
       time-000-000-000-001 (33),
       time-000-000-000-000-5 (34),
       time-000-000-000-000-2 (35),
       time-000-000-000-000-1 (36),
       time-000-000-000-000-05 (37),
       time-000-000-000-000-02 (38),
       time-000-000-000-000-01 (39) 
      }
      
      TimeIntervalConfidence ::= INTEGER (0..15) 
      
      TransitVehicleOccupancy ::= ENUMERATED {
       occupancyUnknown (0), 
       occupancyEmpty (1), 
       occupancyVeryLow (2), 
       occupancyLow (3), 
       occupancyMed (4), 
       occupancyHigh (5), 
       occupancyNearlyFull (6), 
       occupancyFull (7)
      }
      
      TransitVehicleStatus ::= BIT STRING {
       loading (0),
       anADAuse (1),
       aBikeLoad (2),
       doorOpen (3),
       charging (4),
       atStopLine (5)
      } (SIZE(8))
      
      TransmissionState ::= ENUMERATED {
       neutral (0),
       park (1),
       forwardGears (2),
       reverseGears (3),
       reserved1 (4), 
       reserved2 (5), 
       reserved3 (6), 
       unavailable (7)
      }
      
      VehicleHeight ::= INTEGER (0..127)
      
      VehicleType ::= ENUMERATED {
       none (0),
       unknown (1),
       special (2),
       moto (3),
       car (4),
       carOther (5),
       bus (6),
       axleCnt2 (7),
       axleCnt3 (8),
       axleCnt4 (9),
       axleCnt4Trailer (10),
       axleCnt5Trailer (11),
       axleCnt6Trailer (12),
       axleCnt5MultiTrailer (13),
       axleCnt6MultiTrailer (14),
       axleCnt7MultiTrailer (15),
       ... 
      } 
      
      Velocity ::= INTEGER (0..8191)
      
      WaitOnStopline ::= BOOLEAN 
      
      ZoneLength ::= INTEGER (0..10000)
      
      
      EuVehicleCategoryCode ::= CHOICE {
       euVehicleCategoryL EuVehicleCategoryL,
       euVehicleCategoryM EuVehicleCategoryM,
       euVehicleCategoryN EuVehicleCategoryN,
       euVehicleCategoryO EuVehicleCategoryO,
       euVehilcleCategoryT NULL,
       euVehilcleCategoryG NULL
       }
      
      EuVehicleCategoryL ::= ENUMERATED { l1, l2, l3, l4, l5, l6, l7 }
      
      EuVehicleCategoryM ::= ENUMERATED {m1, m2, m3}
      
      EuVehicleCategoryN ::= ENUMERATED {n1, n2, n3}
      
      EuVehicleCategoryO ::= ENUMERATED {o1, o2, o3, o4}
      
      Iso3833VehicleType ::= INTEGER {
       passengerCar (0),
       saloon (1),
       convertibleSaloon (2),
       pullmanSaloon (3),
       stationWagon (4),
       truckStationWagon (5),
       coupe (6),
       convertible (7),
       multipurposePassengerCar (8),
       forwardControlPassengerCar (9),
       specialPassengerCar (10),
       bus (11),
       minibus (12),
       urbanBus (13),
       interurbanCoach (14),
       longDistanceCoach (15),
       articulatedBus (16),
       trolleyBus (17),
       specialBus (18),
       commercialVehicle (19),
       specialCommercialVehicle (20),
       specialVehicle (21),
       trailingTowingVehicle (22),
       semiTrailerTowingVehicle (23),
       trailer (24),
       busTrailer (25),
       generalPurposeTrailer (26),
       caravan (27),
       specialTrailer (28),
       semiTrailer (29),
       busSemiTrailer (30),
       generalPurposeSemiTrailer (31),
       specialSemiTrailer (32),
       roadTrain (33),
       passengerRoadTrain (34),
       articulatedRoadTrain (35),
       doubleRoadTrain (36),
       compositeRoadTrain (37),
       specialRoadTrain (38),
       moped (39),
       motorCycle (40)
       } (0..255)
      
       
      IviStructure::= SEQUENCE{
       mandatory IVIManagementContainer,
       optional SEQUENCE (SIZE (1..8,...)) OF IviContainer OPTIONAL
       }
      
      --Definition of Containers
      
      IviContainer::= CHOICE {
       glc GeographicLocationContainer,
       giv GeneralIviContainer,
       rcc RoadConfigurationContainer,
       tc TextContainer,
       lac LayoutContainer,
       ...
       }
      
      
      IVIManagementContainer::= SEQUENCE {
       serviceProviderId Provider, 
       iviIdentificationNumber IviIdentificationNumber,
       timeStamp TimestampIts OPTIONAL,
       validFrom TimestampIts OPTIONAL,
       validTo TimestampIts OPTIONAL,
       connectedIviStructures SEQUENCE (SIZE(1..8)) OF IviIdentificationNumber OPTIONAL,
       iviStatus IviStatus,
       ...
       }
       
      GeographicLocationContainer::= SEQUENCE {
       referencePosition ReferencePosition,
       referencePositionTime TimestampIts OPTIONAL,
       referencePositionHeading Heading OPTIONAL, 
       referencePositionSpeed Speed OPTIONAL,
       parts SEQUENCE (SIZE (1..16,...)) OF GlcPart, 
       ...
       }
      
      GlcPart::= SEQUENCE {
       zoneId Zid, 
       laneNumber LanePosition OPTIONAL,
       zoneExtension INTEGER (0..255) OPTIONAL,
       zoneHeading HeadingValue OPTIONAL,
       zone Zone OPTIONAL,
       ... 
       }
      
      GeneralIviContainer::= SEQUENCE (SIZE (1..16,...)) OF GicPart 
      
      GicPart::= SEQUENCE {
       detectionZoneIds SEQUENCE (SIZE (1..8,...)) OF Zid OPTIONAL, 
       its-Rrid VarLengthNumber OPTIONAL,
       relevanceZoneIds SEQUENCE (SIZE (1..8,...)) OF Zid OPTIONAL, 
       direction Direction OPTIONAL,
       driverAwarenessZoneIds SEQUENCE (SIZE (1..8,...)) OF Zid OPTIONAL, 
       minimumAwarenessTime INTEGER (0..255) OPTIONAL,
       applicableLanes SEQUENCE (SIZE (1..8,...)) OF LanePosition OPTIONAL,
       iviType IviType,
       iviPurpose IviPurpose OPTIONAL,
       laneStatus LaneStatus OPTIONAL, 
       vehicleCharacteristics SEQUENCE (SIZE (1..8, ...)) OF CompleteVehicleCharacteristics OPTIONAL,
       driverCharacteristics DriverCharacteristics OPTIONAL,
       layoutId INTEGER(1..4,...) OPTIONAL,
       preStoredlayoutId INTEGER(1..64,...) OPTIONAL,
       roadSignCodes SEQUENCE (SIZE (1..4,...)) OF RSCode,
       extraText SEQUENCE (SIZE (1..4,...)) OF Text OPTIONAL,
       ...
       }
      
      RoadConfigurationContainer::= SEQUENCE (SIZE (1..16,...)) OF RccPart
      
      RccPart::= SEQUENCE{
       zoneIds SEQUENCE (SIZE (1..8,...)) OF Zid,
       roadType RoadType, 
       laneConfiguration SEQUENCE (SIZE (1..16,...)) OF LaneInformation,
       ...
       }
      
      TextContainer::= SEQUENCE (SIZE (1..16,...)) OF TcPart
      
      TcPart::= SEQUENCE {
       detectionZoneIds SEQUENCE (SIZE (1..8,...)) OF Zid OPTIONAL, 
       relevanceZoneIds SEQUENCE (SIZE (1..8,...)) OF Zid, 
       direction Direction OPTIONAL, 
       driverAwarenessZoneIds SEQUENCE (SIZE (1..8,...)) OF Zid OPTIONAL, 
       minimumAwarenessTime INTEGER (0..255) OPTIONAL,
       applicableLanes SEQUENCE (SIZE (1..8,...)) OF LanePosition OPTIONAL,
       layoutId INTEGER(1..4,...) OPTIONAL,
       preStoredlayoutId INTEGER(1..64,...) OPTIONAL,
       text SEQUENCE (SIZE (1..4,...)) OF Text OPTIONAL,
       data OCTET STRING,
       ...
       }
      
      LayoutContainer::=SEQUENCE{
       layoutId INTEGER(1..4,...),
       height INTEGER(10..73) OPTIONAL, 
       width INTEGER(10..265) OPTIONAL,
       layoutComponents SEQUENCE SIZE(1..4,...) OF LayoutComponent,
       ... 
       }
      
      
      
      AbsolutePosition::= SEQUENCE{
       latitude Latitude,
       longitude Longitude
       }
      
      AbsolutePositionWAltitude::= SEQUENCE{
       latitude Latitude,
       longitude Longitude,
       altitude Altitude 
       }
      
      AnyCatalogue::=SEQUENCE{
       owner Provider,
       version INTEGER(0..255),
       pictogramCode INTEGER(0..65535),
       value INTEGER (0..65535) OPTIONAL, 
       unit RSCUnit OPTIONAL,
       attributes ISO14823Attributes OPTIONAL
       } 
      
      ComparisonOperator ::= INTEGER {
       greaterThan (0),
       greaterThanOrEqualTo (1),
       lessThan (2),
       lessThanOrEqualTo (3)
      } (0..3)
      
      CompleteVehicleCharacteristics::= SEQUENCE{
       tractor TractorCharacteristics OPTIONAL,
       trailer SEQUENCE (SIZE (1..3)) OF TrailerCharacteristics OPTIONAL,
       train TrainCharacteristics OPTIONAL
       }
      
      ComputedSegment::= SEQUENCE {
       zoneId Zid, 
       laneNumber LanePosition, 
       laneWidth IVILaneWidth,
       offsetDistance INTEGER (-32768..32767) OPTIONAL, 
       offsetPosition DeltaReferencePosition OPTIONAL 
       }
      
      DeltaPosition::=SEQUENCE{
       deltaLatitude DeltaLatitude,
       deltaLongitude DeltaLongitude
       }
      
      Direction::= INTEGER{
       sameDirection (0),
       oppositeDirection (1),
       bothDirections (2),
       valueNotUsed (3)
       } (0..3)
      
      Distance::= SEQUENCE {
       value INTEGER(1..16384),
       unit RSCUnit
       }
      
      DistanceOrDuration::= SEQUENCE {
       value INTEGER(1..16384),
       unit RSCUnit
       } 
      
      DriverCharacteristics::= INTEGER{
       unexperiencedDrivers (0),
       experiencedDrivers (1),
       rfu1 (2), 
       rfu2 (3)
       } (0..3)
      
      GoodsType::= INTEGER {
       ammunition (0),
       chemicals (1),
       empty (2),
       fuel (3),
       glass (4),
       dangerous (5),
       liquid (6),
       liveStock (7),
       dangerousForPeople (8),
       dangerousForTheEnvironment (9),
       dangerousForWater (10),
       perishableProducts (11),
       pharmaceutical (12),
       vehicles (13)
      
       } (0..15,...)
      
      
      
      ISO14823Attributes::= SEQUENCE (SIZE(1..8,...)) OF ISO14823Attribute
      
      ISO14823Attribute ::= CHOICE{
       dtm DTM,
       edt EDT,
       dfl DFL,
       ved VED,
       spe SPE,
       roi ROI,
       dbv DBV,
       ddd DDD
      }
      
      ServiceCategoryCode ::= CHOICE { 
       trafficSignPictogram TrafficSignPictogram,
       publicFacilitiesPictogram PublicFacilitiesPictogram,
       ambientOrRoadConditionPictogram AmbientOrRoadConditionPictogram,
        ...
      }
      
      TrafficSignPictogram ::= ENUMERATED {dangerWarning, regulatory, informative,...}
      PublicFacilitiesPictogram ::= ENUMERATED {publicFacilities, ...}
      AmbientOrRoadConditionPictogram ::= ENUMERATED {ambientCondition, roadCondition,...}
      
      PictogramCategoryCode ::= SEQUENCE {
       nature INTEGER (1..9),
       serialNumber INTEGER (0..99)
       }
      
      PictogramCode ::= SEQUENCE { 
       countryCode OCTET STRING (SIZE (2)) OPTIONAL, 
       serviceCategoryCode ServiceCategoryCode,
       pictogramCategoryCode PictogramCategoryCode
       }
      
      ISO14823Code ::= SEQUENCE{
       pictogramCode PictogramCode,
       attributes ISO14823Attributes OPTIONAL 
       }
      
      
      
      IviIdentificationNumber::= INTEGER(1..32767,...)
      
      IviPurpose::= INTEGER { 
       safety (0), 
       environmental (1), 
       trafficOptimisation (2)
       } (0..3) 
      
      IviStatus::= INTEGER {
       new (0), 
       update (1), 
       cancellation (2), 
       negation (3) 
      
       }(0..7)
      
      IviType::= INTEGER {
       immediateDangerWarningMessages (0),
       regulatoryMessages (1),
       trafficRelatedInformationMessages (2), 
       pollutionMessages (3),
       notTrafficRelatedInformationMessages (4)
      
       } (0..7)
      
      LaneInformation::= SEQUENCE{
       laneNumber LanePosition, 
       direction Direction,
       validity DTM OPTIONAL,
       laneType LaneType, 
       laneTypeQualifier CompleteVehicleCharacteristics OPTIONAL,
       laneStatus LaneStatus, 
       laneWidth IVILaneWidth OPTIONAL, 
       ...
       }
      
      LaneStatus::= INTEGER {
       open (0),
       closed (1),
       mergeR (2),
       mergeL (3),
       mergeLR (4),
       provisionallyOpen (5),
       diverging (6) 
      
      } (0..7, ...)
      
      LaneType::= INTEGER{
       traffic (0),
       through (1),
       reversible (2),
       acceleration (3),
       deceleration (4),
       leftHandTurning (5),
       rightHandTurning (6),
       dedicatedVehicle (7),
       bus (8),
       taxi (9),
       hov (10),
       hot (11),
       pedestrian (12),
       bikeLane (13),
       median (14), 
       striping (15),
       trackedVehicle (16),
       parking (17),
       emergency (18),
       verge (19)
       }(0..31)
      
      
      DeltaPositions ::= SEQUENCE (SIZE(1..32,...)) OF DeltaPosition
      
      DeltaPositionsWithAltitude ::= SEQUENCE (SIZE(1..32,...)) OF DeltaReferencePosition
      
      AbsolutePositions ::= SEQUENCE (SIZE(1..8,...)) OF AbsolutePosition
      
      AbsolutePositionsWithAltitude ::= SEQUENCE (SIZE(1..8,...)) OF AbsolutePositionWAltitude
      
      IVILaneWidth::= INTEGER (0..1023)
      
      LayoutComponent::=SEQUENCE{
       layoutComponentId INTEGER(1..8,...),
       height INTEGER(10..73), 
       width INTEGER(10..265), 
       x INTEGER(10..265),
       y INTEGER(10..73), 
       textScripting INTEGER (0..1)
       } 
      
      LoadType::= SEQUENCE{
       goodsType GoodsType,
       dangerousGoodsType DangerousGoodsBasic,
       specialTransportType SpecialTransportType
       }
      
      PolygonalLine::= CHOICE {
       deltaPositions DeltaPositions, 
       deltaPositionsWithAltitude DeltaPositionsWithAltitude, 
       absolutePositions AbsolutePositions, 
       absolutePositionsWithAltitude AbsolutePositionsWithAltitude, 
       ...
       }
      
       
      Code ::= CHOICE {
       viennaConvention VcCode,
       iso14823 ISO14823Code, 
       itisCodes INTEGER (0..65535),
       anyCatalogue AnyCatalogue,
       ...
       }
      
      RSCode::= SEQUENCE{
       layoutComponentId INTEGER(1..4,...) OPTIONAL,
       code Code
       }
      
      RSCUnit::= INTEGER {
       kmperh (0),
       milesperh (1),
       kilometer (2),
       meter (3),
       decimeter (4),
       centimeter (5),
       mile (6),
       yard (7),
       foot (8),
       minutesOfTime (9),
       tonnes (10), --1000 kg, not Ton!
       hundredkg (11),
       pound (12), --lbs
       rateOfIncline (13)
      
       } (0..15)
      
      
      Segment::= SEQUENCE {
       line PolygonalLine,
       laneWidth IVILaneWidth OPTIONAL
       }
      
      Text::= SEQUENCE {
       layoutComponentId INTEGER(1..4,...) OPTIONAL,
       language BIT STRING (SIZE(10)), 
       textContent UTF8String
       }
      
      TractorCharacteristics::=SEQUENCE{
       equalTo SEQUENCE (SIZE (1..4,...)) OF VehicleCharacteristicsFixValues OPTIONAL,
       notEqualTo SEQUENCE (SIZE (1..4,...)) OF VehicleCharacteristicsFixValues OPTIONAL, 
       ranges SEQUENCE (SIZE (1..4,...)) OF VehicleCharacteristicsRanges OPTIONAL
      }
       
      TrailerCharacteristics::=SEQUENCE{
       equalTo SEQUENCE (SIZE (1..4,...)) OF VehicleCharacteristicsFixValues OPTIONAL,
       notEqualTo SEQUENCE (SIZE (1..4,...)) OF VehicleCharacteristicsFixValues  OPTIONAL, 
       ranges SEQUENCE (SIZE (1..4,...)) OF VehicleCharacteristicsRanges  OPTIONAL 
       }
      
      TrainCharacteristics::= TractorCharacteristics
      
      VcClass::= INTEGER {
       classA (0),
       classB (1),
       classC (2),
       classD (3),
       classE (4),
       classF (5),
       classG (6),
       classH (7)
       } (0..7)
      
      VcCode::= SEQUENCE {
       roadSignClass VcClass,
       roadSignCode INTEGER (1..64),
       vcOption VcOption,
       validity SEQUENCE (SIZE (1..8,...)) OF DTM OPTIONAL, 
       value INTEGER (0..65535) OPTIONAL, 
       unit RSCUnit OPTIONAL 
       }
      
      VcOption::= INTEGER {
       none (0),
       a (1),
       b (2),
       c (3),
       d (4),
       e (5),
       f (6),
       g (7)
       } (0..7)
      
      VehicleCharacteristicsFixValues::= CHOICE{
       simpleVehicleType StationType, 
       euVehicleCategoryCode EuVehicleCategoryCode,
       iso3833VehicleType Iso3833VehicleType,
       euroAndCo2value EnvironmentalCharacteristics,
       engineCharacteristics EngineCharacteristics,
       loadType LoadType,
       usage VehicleRole,
       ...}
      
      VehicleCharacteristicsRanges::= SEQUENCE{
       comparisonOperator ComparisonOperator,
       limits VehicleCharacteristicsRangeLimits
      }
      
       VehicleCharacteristicsRangeLimits ::= CHOICE{
        numberOfAxles INTEGER(0..7),
        vehicleDimensions VehicleDimensions,
        vehicleWeightLimits VehicleWeightLimits,
        axleWeightLimits AxleWeightLimits,
        passengerCapacity PassengerCapacity, 
        exhaustEmissionValues ExhaustEmissionValues,
        dieselEmissionValues DieselEmissionValues,
        soundLevel SoundLevel, 
        ...
      }
      
      Weight::= SEQUENCE {
       value INTEGER(1..16384),
       unit RSCUnit
       } 
      
      Zid::= INTEGER (1..32,...)
      
      Zone::= CHOICE {
       segment Segment,
       area PolygonalLine,
       computedSegment ComputedSegment,
       ...
       }
      
      
      DtmYear ::= SEQUENCE {
         syr INTEGER(2000..2127,...),
         eyr INTEGER(2000..2127,...)
       }
      
       DtmMonthDay ::= SEQUENCE {
         smd MonthDay,
         emd MonthDay
       }
      
       DtmHourMinutes ::= SEQUENCE {
         shm HoursMinutes,
         ehm HoursMinutes
       }
      
      DTM ::= SEQUENCE {
       year DtmYear OPTIONAL,
       month-day DtmMonthDay OPTIONAL,
       pmd PMD OPTIONAL, 
       hourMinutes DtmHourMinutes OPTIONAL,
       dayOfWeek DayOfWeek OPTIONAL,
       period HoursMinutes OPTIONAL
      }
      
      MonthDay ::= SEQUENCE {
       month INTEGER (1..12),
       day INTEGER (1..31) 
      }
      
      PMD::= BIT STRING {national-holiday (0), even-days(1), odd-days(2), market-day(3) } (SIZE (4))
      
      HoursMinutes ::= SEQUENCE {
       hours INTEGER (0..23),
       mins INTEGER (0..59)
       }
      
      
      DayOfWeek ::= BIT STRING {unused(0), monday(1), tuesday(2), wednesday(3), thursday(4), friday(5), saturday(6), sunday(7)} (SIZE (8))
      
      EDT ::= DTM
      
      
      
      DFL::= INTEGER {
       sDL (1),
       sLT (2),
       sRT (3),
       lTO (4),
       rTO (5),
       cLL (6),
       cRI (7),
       oVL (8)
       } (1..8) 
      
      VED::=SEQUENCE{
       hei Distance OPTIONAL,
       wid Distance OPTIONAL,
       vln Distance OPTIONAL,
       wei Weight OPTIONAL
       }
      
      SPE::=SEQUENCE{
       spm INTEGER(0..250) OPTIONAL,
       mns INTEGER(0..250) OPTIONAL,
       unit RSCUnit
       }
      
      ROI::= INTEGER(1..32)
      
      DBV::= Distance 
      
      DDD::= SEQUENCE{
       dcj INTEGER(1..128) OPTIONAL,
       dcr INTEGER(1..128) OPTIONAL,
       tpl INTEGER(1..128) OPTIONAL,
       ioList SEQUENCE (SIZE (1..8,...)) OF DDD-IO
       }
      
      DDD-IO::= SEQUENCE{ 
       drn INTEGER(0..7),
       dp SEQUENCE (SIZE (1..4,...)) OF DestinationPlace OPTIONAL,
       dr SEQUENCE (SIZE (1..4,...)) OF DestinationRoad OPTIONAL,
       rne INTEGER(1..999) OPTIONAL,
       stnId INTEGER(1..999) OPTIONAL,
       stnText UTF8String OPTIONAL, 
       dcp DistanceOrDuration OPTIONAL,
       ddp DistanceOrDuration OPTIONAL 
       }
      
      DestinationPlace::= SEQUENCE{
       depType DDD-DEP,
      
      
       depRSCode ISO14823Code OPTIONAL,
       depBlob OCTET STRING OPTIONAL, 
       plnId INTEGER(1..999) OPTIONAL,
       plnText UTF8String OPTIONAL
       } 
      
      DestinationRoad::=SEQUENCE{
       derType DDD-DER,
       ronId INTEGER(1..999) OPTIONAL,
       ronText UTF8String OPTIONAL 
       }
      
      DDD-DER::= INTEGER { 
       none (0), 
       nationalHighway (1), 
       localHighway (2), 
       tollExpresswayMotorway (3),
       internationalHighway (4), 
       highway (5),
       expressway (6),
       nationalRoad (7), 
       regionalProvincialRoad (8),
       localRoad (9), 
       motorwayJunction (10),
       diversion (11),
       rfu1 (12),
       rfu2 (13),
       rfu3 (14),
       rfu4 (15)
       } (0..15, ...) 
      
      DDD-DEP::= INTEGER { 
       none (0), 
       importantArea (1), 
       principalArea (2), 
       generalArea (3), 
       wellKnownPoint (4),
       country (5),
       city (6),
       street (7),
       industrialArea (8),
       historicArea (9),
       touristicArea (10),
       culturalArea (11),
       touristicRoute (12), 
       recommendedRoute (13), 
       touristicAttraction (14),
       geographicArea (15)
       } (0..15, ...) 
      
      
      
       
      AttributeIdList ::= SEQUENCE (SIZE(0.. 127,...)) OF INTEGER(0..127,...)
      
      AxleWeightLimits ::= SEQUENCE {
       maxLadenweightOnAxle1 Int2,
       maxLadenweightOnAxle2 Int2,
       maxLadenweightOnAxle3 Int2,
       maxLadenweightOnAxle4 Int2,
       maxLadenweightOnAxle5 Int2
      }
      
      Particulate ::= SEQUENCE {
       unitType UnitType,
       value INTEGER (0..32767)
       }
      
      DieselEmissionValues::= SEQUENCE {
       particulate Particulate,
       absorptionCoeff Int2
      }
      
      EnvironmentalCharacteristics::= SEQUENCE {
       euroValue EuroValue,
       copValue CopValue
      }
      
      EuroValue::= ENUMERATED {
       noEntry (0),
       euro-1 (1),
       euro-2 (2),
       euro-3 (3),
       euro-4 (4),
       euro-5 (5),
       euro-6 (6),
       reservedForUse1 (7),
       reservedForUse2 (8),
       reservedForUse3 (9),
       reservedForUse4 (10),
       reservedForUse5 (11),
       reservedForUse6 (12),
       reservedForUse7 (13),
       reservedForUse8 (14),
       eev (15)
      }
      
      
      
      CopValue::= ENUMERATED {
       noEntry (0),
       co2class1 (1),
       co2class2 (2),
       co2class3 (3),
       co2class4 (4),
       co2class5 (5),
       co2class6 (6),
       co2class7 (7),
       reservedforUse (8)
      }
      
      
      EngineCharacteristics::= INTEGER {
       noEntry (0),
       noEngine (1),
       petrolUnleaded (2),
       petrolLeaded (3),
       diesel (4),
       lPG (5),
       battery (6),
       solar (7),
       hybrid (8),
       hydrogen (9)
      } (0..255)
      
      ExhaustEmissionValues ::= SEQUENCE {
       unitType UnitType,
       emissionCO INTEGER (0..32767),
       emissionHC Int2,
       emissionNOX Int2,
       emissionHCNOX Int2
      }
      
      
      Int1 ::= INTEGER(0..255)
      
      Int2 ::= INTEGER(0..65535)
      
      PassengerCapacity ::= SEQUENCE{
       numberOfSeats Int1,
       numberOfStandingPlaces Int1
      }
      
      
      CountryCode::= BIT STRING(SIZE(10))
       
      IssuerIdentifier::= INTEGER(0 .. 16383)
      
      Provider ::= SEQUENCE {
       countryCode CountryCode,
       providerIdentifier IssuerIdentifier
      }
      
      SoundLevel ::= SEQUENCE{
       soundstationary Int1,
       sounddriveby Int1
      }
      
      UnitType::= ENUMERATED {
       mg-km (0),
       mg-kWh (1)
      }
      
      VehicleDimensions ::= SEQUENCE {
       vehicleLengthOverall Int1,
       vehicleHeigthOverall Int1,
       vehicleWidthOverall Int1
      }
      
      VehicleWeightLimits ::= SEQUENCE {
       vehicleMaxLadenWeight Int2,
       vehicleTrainMaximumWeight Int2,
       vehicleWeightUnladen Int2
      }
      
      CS5::= SEQUENCE {
       vin VisibleString,
       fill BIT STRING (SIZE(9))
      }
      
      
      VarLengthNumber ::= CHOICE {
       content INTEGER(0..127),
       extension Ext1
       }
       
      Ext1 ::= CHOICE {
       content INTEGER(128..16511),
       extension Ext2
      }
      
      Ext2 ::= CHOICE {
       content INTEGER(16512..2113663),
       extension Ext3
       }
      
      Ext3 ::= INTEGER(2113664..270549119,...)
       
      
      
      EvcsnPdu ::= SEQUENCE {
       header ItsPduHeader, 
       evcsn EVChargingSpotNotificationPOIMessage
      }
      
      EVChargingSpotNotificationPOIMessage ::= SEQUENCE {
       poiHeader ItsPOIHeader,
       evcsnData ItsEVCSNData
      }
      
      ItsPOIHeader ::= SEQUENCE {
       poiType POIType,
       timeStamp TimestampIts,
       relayCapable BOOLEAN
      }
      
      
      ItsEVCSNData ::= SEQUENCE {
       totalNumberOfStations NumberStations,
       chargingStationsData SEQUENCE (SIZE(1..256)) OF ItsChargingStationData
      }
      
      ItsChargingStationData ::= SEQUENCE {
       chargingStationID StationID,
       utilityDistributorId UTF8String (SIZE(1..32)) OPTIONAL,
       providerID UTF8String (SIZE(1..32)) OPTIONAL,
       chargingStationLocation ReferencePosition,
       address UTF8String OPTIONAL,
       phoneNumber NumericString (SIZE(1..16)) OPTIONAL,
       accessibility UTF8String (SIZE(1..32)),
       digitalMap DigitalMap OPTIONAL,
       openingDaysHours UTF8String,
       pricing UTF8String,
       bookingContactInfo UTF8String OPTIONAL,
       payment UTF8String OPTIONAL,
       chargingSpotsAvailable ItsChargingSpots,
       ...
      }
      
      
      ItsChargingSpots ::= SEQUENCE (SIZE(1..16)) OF ItsChargingSpotDataElements
      
      
      ItsChargingSpotDataElements ::= SEQUENCE {
       type ChargingSpotType,
       evEquipmentID UTF8String OPTIONAL,
       typeOfReceptacle TypeOfReceptacle,
       energyAvailability UTF8String,
       parkingPlacesData ParkingPlacesData OPTIONAL
      }
      
      DigitalMap ::= SEQUENCE (SIZE(1..256)) OF ReferencePosition
      
      ChargingSpotType ::= BIT STRING {
       standardChargeMode1(0),
       standardChargeMode2(1),
       standardOrFastChargeMode3(2),
       fastChargeWithExternalCharger(3), 
       quickDrop(8),
       inductiveChargeWhileStationary(12),
       inductiveChargeWhileDriving(14)
      }
      
      TypeOfReceptacle ::= BIT STRING
      
      ParkingPlacesData ::= SEQUENCE (SIZE(1..4)) OF SpotAvailability
      
      SpotAvailability ::= SEQUENCE {
       maxWaitingTimeMinutes INTEGER (0..1400),
       blocking BOOLEAN
      }
      
      POIType ::= INTEGER(0..65535)
      NumberStations ::= INTEGER(1..256)
      
      
      CPM ::= SEQUENCE {
       header ItsPduHeader,
       cpm CollectivePerceptionMessage
      }
      
      CollectivePerceptionMessage ::= SEQUENCE {
       generationDeltaTime GenerationDeltaTime,
       cpmParameters CpmParameters
      }
      
      CpmParameters ::= SEQUENCE {
       managementContainer CpmManagementContainer,
       stationDataContainer StationDataContainer OPTIONAL,
       sensorInformationContainer SensorInformationContainer OPTIONAL,
       perceivedObjectContainer PerceivedObjectContainer OPTIONAL,
       freeSpaceAddendumContainer FreeSpaceAddendumContainer OPTIONAL,
       numberOfPerceivedObjects NumberOfPerceivedObjects,
       ...
      }
      
      CpmManagementContainer ::= SEQUENCE {
       stationType StationType,
       perceivedObjectContainerSegmentInfo PerceivedObjectContainerSegmentInfo OPTIONAL,
       referencePosition ReferencePosition,
       ...
      }
      
      StationDataContainer ::= CHOICE {
       originatingVehicleContainer OriginatingVehicleContainer,
       originatingRSUContainer OriginatingRSUContainer,
       ...
      }
      
      OriginatingVehicleContainer ::= SEQUENCE {
       heading Heading,
       speed Speed,
       vehicleOrientationAngle WGS84Angle OPTIONAL,
       driveDirection DriveDirection DEFAULT forward,
       longitudinalAcceleration LongitudinalAcceleration OPTIONAL,
       lateralAcceleration LateralAcceleration OPTIONAL,
       verticalAcceleration VerticalAcceleration OPTIONAL,
       yawRate YawRate OPTIONAL,
       pitchAngle CartesianAngle OPTIONAL,
       rollAngle CartesianAngle OPTIONAL,
       vehicleLength VehicleLength OPTIONAL,
       vehicleWidth VehicleWidth OPTIONAL,
       vehicleHeight VehicleHeight OPTIONAL,
       trailerDataContainer TrailerDataContainer OPTIONAL,
       ...
      }
      
      OriginatingRSUContainer ::= CHOICE {
       intersectionReferenceId IntersectionReferenceID,
       roadSegmentReferenceId RoadSegmentReferenceID,
       ...
      }
      
      SensorInformationContainer ::= SEQUENCE SIZE(1..128, ...) OF SensorInformation
      
      SensorInformation ::= SEQUENCE {
       sensorID Identifier,
       type SensorType,
       detectionArea DetectionArea,
       freeSpaceConfidence FreeSpaceConfidence OPTIONAL,
       ...
      }
      
      PerceivedObjectContainer ::= SEQUENCE SIZE(1..128, ...) OF PerceivedObject
      
      PerceivedObject ::= SEQUENCE {
       objectID Identifier,
       sensorIDList SensorIdList OPTIONAL,
       timeOfMeasurement TimeOfMeasurement,
       objectAge ObjectAge OPTIONAL,
       objectConfidence ObjectConfidence DEFAULT 0,
       xDistance ObjectDistanceWithConfidence,
       yDistance ObjectDistanceWithConfidence,
       zDistance ObjectDistanceWithConfidence OPTIONAL,
       xSpeed SpeedExtended,
       ySpeed SpeedExtended,
       zSpeed SpeedExtended OPTIONAL,
       xAcceleration LongitudinalAcceleration OPTIONAL,
       yAcceleration LateralAcceleration OPTIONAL,
       zAcceleration VerticalAcceleration OPTIONAL,
       yawAngle CartesianAngle OPTIONAL,
       planarObjectDimension1 ObjectDimension OPTIONAL,
       planarObjectDimension2 ObjectDimension OPTIONAL,
       verticalObjectDimension ObjectDimension OPTIONAL,
       objectRefPoint ObjectRefPoint DEFAULT 0,
       dynamicStatus DynamicStatus OPTIONAL,
       classification ObjectClassDescription OPTIONAL,
       matchedPosition MatchedPosition OPTIONAL,
       ...
      }
      
      DetectionArea ::= CHOICE {
       vehicleSensor VehicleSensor,
       stationarySensorRadial AreaRadial,
       stationarySensorPolygon AreaPolygon,
       stationarySensorCircular AreaCircular,
       stationarySensorEllipse AreaEllipse,
       stationarySensorRectangle AreaRectangle,
       ...
      }
      
      VehicleSensor ::= SEQUENCE {
       refPointId RefPointId DEFAULT 0,
       xSensorOffset XSensorOffset,
       ySensorOffset YSensorOffset,
       zSensorOffset ZSensorOffset OPTIONAL,
       vehicleSensorPropertyList VehicleSensorPropertyList,
       ...
      }
      
      VehicleSensorPropertyList ::= SEQUENCE SIZE(1..10) OF VehicleSensorProperties
      
      VehicleSensorProperties ::= SEQUENCE {
       range Range,
       horizontalOpeningAngleStart CartesianAngleValue,
       horizontalOpeningAngleEnd CartesianAngleValue,
       verticalOpeningAngleStart CartesianAngleValue OPTIONAL,
       verticalOpeningAngleEnd CartesianAngleValue OPTIONAL,
       ...
      }
      
      AreaCircular ::= SEQUENCE {
       nodeCenterPoint OffsetPoint OPTIONAL,
       radius Radius
      }
      
      AreaEllipse ::= SEQUENCE {
       nodeCenterPoint OffsetPoint OPTIONAL,
       semiMinorRangeLength SemiRangeLength,
       semiMajorRangeLength SemiRangeLength,
       semiMajorRangeOrientation WGS84AngleValue,
       semiHeight SemiRangeLength OPTIONAL
      }
      
      AreaRectangle ::= SEQUENCE {
       nodeCenterPoint OffsetPoint OPTIONAL,
       semiMajorRangeLength SemiRangeLength,
       semiMinorRangeLength SemiRangeLength,
       semiMajorRangeOrientation WGS84AngleValue,
       semiHeight SemiRangeLength OPTIONAL
      }
      
      AreaPolygon ::= SEQUENCE {
       polyPointList PolyPointList
      }
      
      PolyPointList ::= SEQUENCE (SIZE(3..16, ...)) OF OffsetPoint
      
      AreaRadial ::= SEQUENCE {
       range Range,
       stationaryHorizontalOpeningAngleStart WGS84AngleValue,
       stationaryHorizontalOpeningAngleEnd WGS84AngleValue,
       verticalOpeningAngleStart CartesianAngleValue OPTIONAL,
       verticalOpeningAngleEnd CartesianAngleValue OPTIONAL,
       sensorPositionOffset OffsetPoint OPTIONAL,
       sensorHeight SensorHeight OPTIONAL,
       ...
      }
      
      FreeSpaceAddendumContainer ::= SEQUENCE SIZE(1..128, ...) OF FreeSpaceAddendum
      
      FreeSpaceAddendum ::= SEQUENCE {
       freeSpaceConfidence FreeSpaceConfidence,
       freeSpaceArea FreeSpaceArea,
       sensorIDList SensorIdList OPTIONAL,
       shadowingApplies ShadowingApplies DEFAULT TRUE,
       ...
      }
      
      FreeSpaceArea ::= CHOICE {
       freeSpacePolygon AreaPolygon,
       freeSpaceCircular AreaCircular,
       freeSpaceEllipse AreaEllipse,
       freeSpaceRectangle AreaRectangle,
       ...
      }
      
      ObjectDistanceWithConfidence ::= SEQUENCE {
       value DistanceValue,
       confidence DistanceConfidence
      }
      
      ObjectDimension ::= SEQUENCE {
       value ObjectDimensionValue,
       confidence ObjectDimensionConfidence
      }
      
      CartesianAngle ::= SEQUENCE {
       value CartesianAngleValue,
       confidence AngleConfidence
      }
      
      WGS84Angle ::= SEQUENCE {
       value WGS84AngleValue,
       confidence AngleConfidence
      }
      
      SpeedExtended ::= SEQUENCE {
       value SpeedValueExtended,
       confidence SpeedConfidence
      }
      
      SensorIdList ::= SEQUENCE SIZE(1..128, ...) OF Identifier
      
      TrailerDataContainer ::= SEQUENCE SIZE(1..2) OF TrailerData
      
      TrailerData ::= SEQUENCE {
       refPointId RefPointId,
       hitchPointOffset HitchPointOffset,
       frontOverhang FrontOverhang,
       rearOverhang RearOverhang,
       trailerWidth VehicleWidth OPTIONAL,
       hitchAngle CartesianAngle OPTIONAL,
       ...
      }
      
      LongitudinalLanePosition ::= SEQUENCE {
       longitudinalLanePositionValue LongitudinalLanePositionValue,
       longitudinalLanePositionConfidence LongitudinalLanePositionConfidence
      }
      
      MatchedPosition ::= SEQUENCE {
       laneID LaneID OPTIONAL,
       longitudinalLanePosition LongitudinalLanePosition OPTIONAL,
       ...
      }
      
      PerceivedObjectContainerSegmentInfo ::= SEQUENCE {
       totalMsgSegments SegmentCount,
       thisSegmentNum SegmentCount
      }
      
      ObjectClassDescription ::= SEQUENCE (SIZE(1..8)) OF ObjectClass
      
      ObjectClassChoice ::= CHOICE {
         vehicle VehicleSubclass,
         person PersonSubclass,
         animal AnimalSubclass,
         other OtherSubclass
       }
      
      ObjectClass ::= SEQUENCE {
       confidence ClassConfidence,
       class ObjectClassChoice
      }
      
      VehicleSubclass ::= SEQUENCE {
       type VehicleSubclassType DEFAULT 0,
       confidence ClassConfidence DEFAULT 0
      }
      
      PersonSubclass ::= SEQUENCE {
       type PersonSubclassType DEFAULT 0,
       confidence ClassConfidence DEFAULT 0
      }
      
      AnimalSubclass ::= SEQUENCE {
       type AnimalSubclassType DEFAULT 0,
       confidence ClassConfidence DEFAULT 0
      }
      
      OtherSubclass ::= SEQUENCE {
       type OtherSublassType DEFAULT 0,
       confidence ClassConfidence DEFAULT 0
      }
      
      OffsetPoint ::= SEQUENCE{
       nodeOffsetPointxy NodeOffsetPointXY,
       nodeOffsetPointZ NodeOffsetPointZ OPTIONAL
      }
      
      NodeOffsetPointZ ::= CHOICE {
       node-Z1 Offset-B10,
       node-Z2 Offset-B11,
       node-Z3 Offset-B12,
       node-Z4 Offset-B13,
       node-Z5 Offset-B14,
       node-Z6 Offset-B16
      }
      
      AnimalSubclassType ::= INTEGER {unknown(0)} (0..255)
      
      ClassConfidence ::= INTEGER {unknown(0), onePercent(1), oneHundredPercent(100), unavailable(101)} (0..101)
      
      WGS84AngleValue ::= INTEGER {wgs84North(0), wgs84East(900), wgs84South(1800), wgs84West(2700), unavailable(3601)} (0..3601)
      
      CartesianAngleValue ::= INTEGER {zeroPointOneDegree(1), oneDegree(10), unavailable(3601)} (0..3601)
      
      AngleConfidence ::= INTEGER {zeroPointOneDegree (1), oneDegree (10), outOfRange(126), unavailable(127)} (1..127)
      
      SemiRangeLength ::= INTEGER {zeroPointOneMeter(1), oneMeter(10)} (0..10000)
      
      DistanceValue ::= INTEGER {zeroPointZeroOneMeter(1), oneMeter(100)} (-132768..132767)
      
      DistanceConfidence ::= INTEGER {zeroPointZeroOneMeter(1), oneMeter(100), outOfRange(101), unavailable(102)} (0..102)
      
      DynamicStatus ::= INTEGER {dynamic(0), hasBeenDynamic(1), static(2)} (0..2)
      
      HitchPointOffset ::= INTEGER {zeroPointOneMeter(1), oneMeter(10)} (0..100)
      
      FrontOverhang ::= INTEGER {zeroPointOneMeter(1), oneMeter(10)} (0..50)
      
      FreeSpaceConfidence ::= INTEGER {unknown(0), onePercent(1), oneHundredPercent(100), unavailable(101)} (0..101)
      
      LongitudinalLanePositionValue ::= INTEGER {zeroPointOneMeter(1)} (0..32767)
      
      LongitudinalLanePositionConfidence ::= INTEGER {zeroPointZeroOneMeter(1), oneMeter(100), outOfRange(101), unavailable(102)} (0..102)
      
      ObjectAge ::= INTEGER {oneMiliSec(1)} (0..1500)
      
      ObjectConfidence ::= INTEGER {unknown(0), onePercent(1), oneHundredPercent(100), unavailable(101)} (0..101)
      
      ObjectDimensionValue ::= INTEGER {zeroPointOneMeter(1), oneMeter(10)} (0..1023)
      
      ObjectDimensionConfidence ::= INTEGER {zeroPointZeroOneMeter(1), oneMeter(100), outOfRange(101), unavailable(102)} (0..102)
      
      ObjectRefPoint ::= INTEGER {mid(0), bottomLeft(1), midLeft(2), topLeft(3), bottomMid(4), topMid(5), bottomRight(6), midRight(7), topRight(8)} (0..8)
      
      OtherSublassType ::= INTEGER {unknown(0), roadSideUnit(1)} (0..255)
      
      PersonSubclassType ::= INTEGER {unknown(0), pedestrian(1), personInWheelchair(2), cyclist(3), personWithStroller(4), personOnSkates(5), personGroup(6)} (0..255)
      
      Radius ::= INTEGER {zeroPointOneMeter(1), oneMeter(10)} (0..10000)
      
      Range ::= INTEGER {zeroPointOneMeter(1), oneMeter(10)} (0..10000)
      
      RearOverhang ::= INTEGER {zeroPointOneMeter(1), oneMeter(10)} (0..150)
      
      RefPointId ::= INTEGER (0..255)
      
      SensorHeight ::= INTEGER {zeroPointZeroOneMeter(1)} (-5000..5000)
      
      ShadowingApplies ::= BOOLEAN
      
      Identifier ::= INTEGER (0..255)
      
      NumberOfPerceivedObjects ::= INTEGER (0..255)
      
      SensorType ::= INTEGER {undefined(0), radar(1), lidar(2), monovideo(3), stereovision(4), nightvision(5), ultrasonic(6), pmd(7), fusion(8), inductionloop(9), sphericalCamera(10), itssaggregation(11)} (0..15)
      
      SegmentCount ::= INTEGER(1..127)
      
      SpeedValueExtended ::= INTEGER {standstill(0), oneCentimeterPerSec(1),unavailable(16383)} (-16383..16383)
      
      TimeOfMeasurement ::= INTEGER {oneMilliSecond(1)} (-1500..1500)
      
      VehicleSubclassType ::= INTEGER {unknown(0), moped(1), motorcycle(2), passengerCar(3), bus(4), lightTruck(5), heavyTruck(6), trailer(7), specialVehicles(8), tram(9), emergencyVehicle(10), agricultural(11)} (0..255)
      
      XSensorOffset ::= INTEGER {negativeZeroPointZeroOneMeter(-1), negativeOneMeter(-100)} (-5000..0)
      
      YSensorOffset ::= INTEGER {zeroPointZeroOneMeter(1), oneMeter(100)} (-1000..1000)
      
      ZSensorOffset ::= INTEGER {zeroPointZeroOneMeter(1), oneMeter(100)} (0..1000)
      
      
      McdmPdu ::= SEQUENCE {
       header ItsPduHeader,
       mcdmInfo McdmInfo
      }
      
      McdmInfo ::= SEQUENCE {
       management MCDM-ManagementContainer,
       situation MCDM-SituationContainer OPTIONAL,
       location MCDM-LocationContainer OPTIONAL,
       application MCDM-ApplicationContainer OPTIONAL,
       multimedia MCDM-MultimediaContainer OPTIONAL
      }
      
      MCDM-ManagementContainer ::= SEQUENCE {
       actionID ActionID,
       request RequestResponseIndication OPTIONAL,
       ack AckNackIndication OPTIONAL,
       detectionTime TimestampIts OPTIONAL,
       referenceTime TimestampIts,
       linkedDenm ActionID OPTIONAL,
       validityDuration ValidityDuration OPTIONAL,
       stationType StationType OPTIONAL,
       numberOfMDUs INTEGER (0..4294967295) DEFAULT 1,
       numberOfPDUs INTEGER (1..4294967295) DEFAULT 1,
       pduSequenceNumber INTEGER (1..4294967295) DEFAULT 1,
       mediaTypes SEQUENCE OF MediaTypeOfMDUs OPTIONAL,
       urls SEQUENCE OF URLOfMDUs OPTIONAL,
       realTime BOOLEAN DEFAULT FALSE,
       size INTEGER (0..4294967295) OPTIONAL,
       ...
      }
      
      AckNackIndication ::= ENUMERATED {
       aCK(0),
       nACK(1)
      }
      
      MediaTypeOfMDUs ::= SEQUENCE {
       startingMDU SequenceNumber OPTIONAL,
       endingMDU SequenceNumber OPTIONAL,
       mediaType IA5String
      }
      
      URLOfMDUs ::= SEQUENCE {
       startingMDU SequenceNumber OPTIONAL,
       endingMDU SequenceNumber OPTIONAL,
       url IA5String
      }
      
      MCDM-SituationContainer ::= SEQUENCE {
       eventType CauseCode,
       linkedCause CauseCode OPTIONAL,
       authorizedPercentageLoss INTEGER (0..100) OPTIONAL,
       informationQuality InformationQuality,
       ...
      }
      
      MCDM-LocationContainer ::= SEQUENCE {
       eventPosition ReferencePosition,
       ...
      }
      MCDM-ApplicationContainer ::= SEQUENCE {
       ...
      }
      
      MCDM-MultimediaContainer ::= SEQUENCE SIZE(1..7) OF MultimediaDataUnit
      
      MultimediaDataUnit ::= CHOICE {
       mediaContentUTF8 UTF8String,
       mediaContentOctet OCTET STRING
      }
      
      
       VAM ::= SEQUENCE {
       header ItsPduHeader,
       vam VruAwareness
       }
      
       VruAwareness ::= SEQUENCE {
       generationDeltaTime GenerationDeltaTime,
       vamParameters VamParameters
       }
      
       VamParameters ::= SEQUENCE {
       basicContainer BasicContainer,
       vruHighFrequencyContainer VruHighFrequencyContainer OPTIONAL,
       vruLowFrequencyContainer VruLowFrequencyContainer OPTIONAL,
       vruClusterInformationContainer VruClusterInformationContainer OPTIONAL,
       vruClusterOperationContainer VruClusterOperationContainer OPTIONAL,
       vruMotionPredictionContainer VruMotionPredictionContainer OPTIONAL,
       ...
       }
      
       VruProfile ::= ENUMERATED {
       unavailable(0), 
       pedestrian(1), 
       cyclist(2), 
       motorcyclist(3), 
       animal(4),
       max(15)
       }
      
       VruHighFrequencyContainer ::= SEQUENCE {
       heading Heading,
       speed Speed,
       longitudinalAcceleration LongitudinalAcceleration,
       curvature Curvature OPTIONAL,
       curvatureCalculationMode CurvatureCalculationMode OPTIONAL,
       yawRate YawRate OPTIONAL,
       lateralAcceleration LateralAcceleration OPTIONAL,
       verticalAcceleration VerticalAcceleration OPTIONAL,
       vruLanePosition VruLanePosition OPTIONAL,
       environment VruEnvironment OPTIONAL,
       movementControl VruMovementControl OPTIONAL,
       orientation VruOrientation OPTIONAL,
       rollAngle VruRollAngle OPTIONAL,
       deviceUsage VruDeviceUsage OPTIONAL,
       ...
       }
      
       VruLanePosition ::= CHOICE {
       offRoadLanePosition OffRoadLanePosition,
       vehicularLanePosition LanePosition,
       trafficIslandPosition TrafficIslandPosition,
       mapPosition MapPosition,
       ...
       }
      
       OffRoadLanePosition ::= ENUMERATED {
       unavailable(0), sidewalk(1), parkingLane(2), bikeLane(3),
       max(15)
       }
      
       TrafficIslandPosition ::= SEQUENCE {
       oneSide NonIslandLanePosition,
       otherSide NonIslandLanePosition
       }
      
       NonIslandLanePosition ::= CHOICE {
       offRoadLanePosition OffRoadLanePosition,
       vehicularLanePosition LanePosition,
       mapPosition MapPosition,
       ...
       }
      
       MapPosition ::= SEQUENCE {
       intersectionId IntersectionReferenceID,
       lane LaneID
       }
      
       VruEnvironment ::= ENUMERATED {
       unavailable (0), intersectionCrossing(1), zebraCrossing(2), sidewalk (3),
       onVehicleRoad(4), protectedGeographicArea(5), max (255)
      
       }
      
       VruMovementControl ::= ENUMERATED {
       unavailable (0), braking(1), hardBraking(2), stopPedaling (3),
       noReaction(4), max (255)
      
       }
      
       VruOrientation ::= Heading
      
       VruRollAngle ::= SteeringWheelAngle
      
       VruDeviceUsage ::= ENUMERATED {
       unavailable(0), other(1), idle(2), listeningToAudio(3), typing(4),
       calling(5), playingGames(6), reading(7), viewing(8), max(255)
      
       }
      
       VruLowFrequencyContainer ::= SEQUENCE {
       profileAndSubprofile VruProfileAndSubprofile OPTIONAL,
       exteriorLights VruExteriorLights OPTIONAL,
       sizeClass VruSizeClass OPTIONAL,
       ...
       }
      
       VruProfileAndSubprofile ::= CHOICE {
       pedestrian VruSubProfilePedestrian,
       bicyclist VruSubProfileBicyclist,
       motorcylist VruSubProfileMotorcyclist,
       animal VruSubProfileAnimal,
       ...
       }
      
       VruSubProfilePedestrian ::= ENUMERATED {
       unavailable(0), ordinary-pedestrian(1),
       road-worker(2), first-responder(3),
       max(15)
       }
      
       VruSubProfileBicyclist ::= ENUMERATED {
       unavailable(0), bicyclist(1), wheelchair-user(2), horse-and-rider(3),
       rollerskater(4), e-scooter(5), personal-transporter(6),
       pedelec(7), speed-pedelec(8),
       max(15)
       }
      
       VruSubProfileMotorcyclist ::= ENUMERATED {
       unavailable(0), moped(1), motorcycle(2), motorcycle-and-sidecar-right(3),
       motorcycle-and-sidecar-left(4), max(15)
       }
      
       VruSubProfileAnimal ::= ENUMERATED {
       unavailable(0), max(15)
       }
      
       VruExteriorLights ::= SEQUENCE {
       vruSpecific VruSpecificExteriorLights,
       generic ExteriorLights
      
       }
      
       VruSpecificExteriorLights ::= BIT STRING {
       unavailable (0),
       backFlashLight (1),
       helmetLight (2),
       armLight (3),
       legLight (4),
       wheelLight (5)
       } (SIZE(8))
      
       VruSizeClass ::= ENUMERATED {
       unavailable (0), low(1), medium(2), high (3), max(15)
      
       }
      
       VruClusterInformationContainer ::= SEQUENCE {
       clusterId ClusterId,
       clusterBoundingBoxShape ClusterBoundingBoxShape,
       clusterCardinalitySize INTEGER(0..255),
       clusterProfiles ClusterProfiles,
       ...
       }
      
       ClusterId ::= INTEGER(0..255)
      
       ClusterBoundingBoxShape::= CHOICE {
       clusterRectangle AreaRectangle,
       clusterCircle AreaCircular,
       clusterPolygon AreaPolygon,
       ...
       }
      
       ClusterProfiles ::= BIT STRING {
       pedestrian(0),
       bicyclist(1),
       motorcyclist(2),
       animal(3)
       } (SIZE(4))
      
      
      
       VruClusterOperationContainer ::= SEQUENCE {
       clusterJoinInfo ClusterJoinInfo OPTIONAL,
       clusterLeaveInfo ClusterLeaveInfo OPTIONAL,
       clusterBreakupInfo ClusterBreakupInfo OPTIONAL,
       clusterIdChangeInfo VruClusterOpTimestamp OPTIONAL,
       ...
       }
      
       VruClusterOpTimestamp ::= INTEGER (1..255)
      
       ClusterJoinInfo ::= SEQUENCE {
       clusterId ClusterId,
       joinTime VruClusterOpTimestamp,
       ...
       }
      
       ClusterLeaveInfo ::= SEQUENCE {
       clusterId ClusterId,
       clusterLeaveReason ClusterLeaveReason,
       ...
       }
      
       ClusterBreakupInfo ::= SEQUENCE {
       clusterBreakupReason ClusterBreakupReason,
       breakupTime VruClusterOpTimestamp,
       ...
       }
      
       ClusterLeaveReason ::= ENUMERATED {
       notProvided (0),
       clusterLeaderLost (1),
       clusterDisbandedByLeader (2),
       outOfClusterBoundingBox (3),
       outOfClusterSpeedRange (4),
       joiningAnotherCluster (5),
       max(15)
       }
      
       ClusterBreakupReason ::= ENUMERATED {
       notProvided (0),
       clusteringPurposeCompleted (1),
       leaderMovedOutOfClusterBoundingBox (2),
       joiningAnotherCluster (3),
       max(15)
       }
      
       VruMotionPredictionContainer ::= SEQUENCE {
       pathHistory PathHistory OPTIONAL,
       pathPrediction PathPrediction OPTIONAL,
       safeDistance SequenceOfVruSafeDistanceIndication OPTIONAL,
       trajectoryChangeIndication SequenceOfTrajectoryInterceptionIndication OPTIONAL,
       accelerationChangeIndication AccelerationChangeIndication OPTIONAL,
       headingChangeIndication HeadingChangeIndication OPTIONAL,
       stabilityChangeIndication StabilityChangeIndication OPTIONAL,
       ...
       }
      
       PathPrediction ::= PathHistory
      
       SequenceOfVruSafeDistanceIndication ::= SEQUENCE(SIZE(0..8)) OF VruSafeDistanceIndication
      
       VruSafeDistanceIndication ::= SEQUENCE {
       subjectStation StationID OPTIONAL,
       stationSafeDistanceIndication BOOLEAN,
       timeToCollision ActionDeltaTime OPTIONAL,
       ...
       }
      
       SequenceOfTrajectoryInterceptionIndication ::= SEQUENCE (SIZE(1..8)) OF TrajectoryInterceptionIndication
      
       TrajectoryInterceptionIndication ::= SEQUENCE {
       subjectStation StationID OPTIONAL,
       trajectoryInterceptionIndication BOOLEAN
       }
      
       HeadingChangeIndication ::= SEQUENCE {
       direction LeftOrRight,
       actionDeltaTime ActionDeltaTime
       }
      
       LeftOrRight ::= ENUMERATED { left, right }
      
       ActionDeltaTime ::= INTEGER {zero(0), hundredMs(1), twoHundredMs(2),
       unavailable (127) } (0..127)
      
       AccelerationChangeIndication ::= SEQUENCE {
       accelOrDecel AccelOrDecel,
       actionDeltaTime ActionDeltaTime
       }
      
       AccelOrDecel ::= ENUMERATED { accelerate, decelerate }
      
       StabilityChangeIndication ::= SEQUENCE {
       lossProbability StabilityLossProbability,
       actionDeltaTime ActionDeltaTime
       }
      
       StabilityLossProbability ::= INTEGER { zero(0), twoPercent (1),
       fourPercent(2), unavailable (63) } (0..63)
      
      
       
      
      MotorcylistSpecialContainer ::= SEQUENCE {
       vruSubProfileMotorcyclist VruSubProfileMotorcyclist,
       vruSizeClass VruSizeClass,
       rollAngle VruRollAngle OPTIONAL,
       vruSafeDistance SequenceOfVruSafeDistanceIndication OPTIONAL,
       pathHistory PathHistory OPTIONAL,
       pathPrediction PathPrediction OPTIONAL,
       stabilityChangeIndication StabilityChangeIndication OPTIONAL,
       ...
      }
      
      
      
      
      MedType::=INTEGER{
       unknown (0),
       any (1),
       iso21212 (2),
       iso21213 (3),
       iso21214 (4),
       iso21215 (5),
       iso21216 (6),
       iso25112 (7),
       iso25113 (8),
       iso29283 (9),
       iso17515 (10),
       iso19079 (11),
       iso15628 (128),
       can (254),
       ethernet (255)
      } (0..255)
      
      
      
      EXT-TYPE ::= CLASS {
       &extRef RefExt UNIQUE,
       &ExtValue
       }
       WITH SYNTAX {&ExtValue IDENTIFIED BY &extRef}
      
      
      Extension {EXT-TYPE : ExtensionTypes} ::= SEQUENCE {
       extensionId EXT-TYPE.&extRef({ExtensionTypes}),
       value EXT-TYPE.&ExtValue({ExtensionTypes}{@.extensionId})
       }
      
      
      
      RefExt::=INTEGER (0..255)
      
      c-Reserved RefExt ::= 0
      c-TxPowerUsed80211 RefExt ::= 4
      c-2Dlocation RefExt ::= 5
      c-3Dlocation RefExt ::= 6
      c-advertiserID RefExt ::= 7
      c-ProviderServContext RefExt ::= 8
      c-IPv6Address RefExt ::= 9
      c-servicePort RefExt ::= 10
      c-ProviderMACaddress RefExt ::= 11
      c-EDCAparameterSet RefExt ::= 12
      c-SecondaryDNS RefExt ::= 13
      c-GatewayMACaddress RefExt ::= 14
      c-ChannelNumber80211 RefExt ::= 15
      c-DataRate80211 RefExt ::= 16
      c-RepeatRate RefExt ::= 17
      c-CountryString RefExt ::= 18
      c-RCPIthreshold RefExt ::= 19
      c-WSAcountThreshold RefExt ::= 20
      c-ChannelAccess RefExt ::= 21
      c-WSAcountThresInt RefExt ::= 22
      c-ChannelLoad RefExt ::= 23
      c-ProtocolType RefExt ::= 24
      c-LMtxCip RefExt ::= 80
      c-LMrxCip RefExt ::= 81
      c-LMchannelBusyRatio RefExt ::= 82
      c-LMpacketID RefExt ::= 83
      c-ExtendedChannelInfos RefExt ::= 84
      c-SAMapplicationData RefExt ::= 85
      
      
      
      
      
      
      
      
      DataRate80211::=INTEGER(0..255)
      
      TXpower80211::=INTEGER(-128..127)
      
      ChannelNumber80211::=INTEGER(0..255)
      
      LMchannelBusyRatio::=INTEGER{
       zeroPercent (0),
       halfPercent (1),
       onePercent (2),
       hundredPercent (200),
       unknown (201)
       }(0..255)
      
      
      
      
      RepeatRate ::= INTEGER (0..255)
      
      TwoDLocation ::= SEQUENCE {
       latitude SALatitude,
       longitude SALongitude
       }
      
      ThreeDLocation ::= SEQUENCE {
       latitude SALatitude,
       longitude SALongitude,
       elevation SAElevation
       }
      
      
      
      AdvertiserIdentifier ::= UTF8String (SIZE (1..32))
      
      
      
      CHINFO-TYPE ::= CLASS {
       &id MedType UNIQUE,
       &Type
       }
       WITH SYNTAX {&Type IDENTIFIED BY &id}
      
      chInfoType-unknown MedType ::= unknown
      chInfoType-any MedType ::= any
      chInfoType-2G MedType ::= iso21212
      chInfoType-3G MedType ::= iso21213
      chInfoType-IR MedType ::= iso21214
      chInfoType-M5 MedType ::= iso21215
      chInfoType-MM MedType ::= iso21216
      chInfoType-80216e MedType ::= iso25112
      chInfoType-HC-SDMA MedType ::= iso25113
      chInfoType-80220 MedType ::= iso29283
      chInfoType-LTE MedType ::= iso17515
      chInfoType-6LowPan MedType ::= iso19079
      chInfoType-15628 MedType ::= iso15628
      chInfoType-CAN MedType ::= can
      chInfoType-Ethernet MedType ::= ethernet
      
      
      ChInfoTypes CHINFO-TYPE ::= {
       { NULL IDENTIFIED BY chInfoType-unknown } |
       { NULL IDENTIFIED BY chInfoType-any } |
       { ChannelInfo IDENTIFIED BY chInfoType-M5 } ,
       ...
       }
      
      ExtendedChannelInfos ::= SEQUENCE OF ExtendedChannelInfo
      
      ExtendedChannelInfo ::= SEQUENCE {
       medId CHINFO-TYPE.&id({ChInfoTypes}),
       value CHINFO-TYPE.&Type({ChInfoTypes}{@.medId})
       }
      
      --ServiceInfo extension elements
      ProviderServiceContext ::= SEQUENCE{
       fillBit BIT STRING (SIZE(3)),
       psc OCTET STRING (SIZE(0..31))
       }
      
      IPv6Address ::= OCTET STRING (SIZE (16))
      
      ServicePort ::= INTEGER (0..65535)
      
      ProviderMacAddress ::= MACaddress
      
      MACaddress ::= OCTET STRING (SIZE(6))
      
      RcpiThreshold ::= INTEGER (0..255)
      
      WsaCountThreshold ::= INTEGER (0..255)
      
      WsaCountThresholdInterval ::= INTEGER (0..255)
      
      SAMapplicationData ::= OCTET STRING 
      
      --ChannelInfo extension elements
      EdcaParameterSet ::= SEQUENCE{
       acbeRecord EdcaParameterRecord,
       acbkRecord EdcaParameterRecord,
       acviRecord EdcaParameterRecord,
       acvoRecord EdcaParameterRecord
       }
      
      EdcaParameterRecord ::= SEQUENCE {
       res INTEGER (0..1),
       aci INTEGER (0..3),
       acm INTEGER (0..1),
       aifsn INTEGER (0..15),
       ecwMax INTEGER (0..15),
       ecwMin INTEGER (0..15),
       txopLimit INTEGER (0..65535)
       }
      
      ChannelAccess80211 ::= INTEGER {
       continuous (0),
       alternatingSCH (1),
       alternatingCCH (2)
       } (0..255)
      
      
      
      
      SecondaryDns ::= IPv6Address
      
      GatewayMacAddress ::= MACaddress
      
      
      
      SALatitude ::= SEQUENCE{
       fillBit BIT STRING (SIZE(1)),
       lat INTEGER (-900000000..900000001)
       }
      
      SALongitude ::= INTEGER (-1800000000..1800000001)
      
      SAElevation ::= INTEGER (-4096..61439)
      
      
      
      
      ITSaid::=VarLengthNumber
      
      
      
      
      RsvAdvPrtVersion ::= INTEGER {
       c-rsvAdvPrtVersion2016 (3)
       }(0..15)
      
      
      Sam ::= SEQUENCE{
       version RsvAdvPrtVersion,
       body SamBody
       }
      
      SamBody ::= SEQUENCE{
       changeCount SrvAdvChangeCount,
       extensions SrvAdvMsgHeaderExts OPTIONAL,
       serviceInfos ServiceInfos OPTIONAL,
       channelInfos ChannelInfos OPTIONAL,
       routingAdvertisement RoutingAdvertisement OPTIONAL
       }
      
      SrvAdvChangeCount ::= SEQUENCE{
       saID SrvAdvID,
       contentCount SrvAdvContentCount
       }
      
      SrvAdvID ::= INTEGER(0..15)
      
      SrvAdvContentCount ::= INTEGER(0..15)
      
      
      SrvAdvMsgHeaderExts::= SEQUENCE OF SrvAdvMsgHeaderExt
      
      SrvAdvMsgHeaderExt ::= Extension {{SrvAdvMsgHeaderExtTypes}}
      
      SrvAdvMsgHeaderExtTypes EXT-TYPE ::= {
       { RepeatRate IDENTIFIED BY c-RepeatRate } |
       { TwoDLocation IDENTIFIED BY c-2Dlocation } |
       { ThreeDLocation IDENTIFIED BY c-3Dlocation } |
       { AdvertiserIdentifier IDENTIFIED BY c-advertiserID } |
       { ExtendedChannelInfos IDENTIFIED BY c-ExtendedChannelInfos },
       ...
       }
      
      
      ServiceInfos ::= SEQUENCE OF ServiceInfo
      
      ServiceInfo ::= SEQUENCE {
       serviceID ITSaid,
       channelIndex ChannelIndex,
       chOptions ChannelOptions 
       }
      
      ChannelOptions ::= SEQUENCE{
       systemService SystemService OPTIONAL, 
       serviceProviderPort ReplyAddress OPTIONAL,
       extensions ServiceInfoExts OPTIONAL 
       }
      
      ChannelIndex ::= INTEGER {
       notUsed (0),
       firstEntry (1) 
       }(0..31)
      
      ReplyAddress ::= PortNumber 
      
      SystemService ::= SEQUENCE OF SystemServiceAndContext
      
      SystemServiceAndContext ::= SamContext
      
      
      ServiceInfoExts ::= SEQUENCE OF ServiceInfoExt
      
      ServiceInfoExt ::= Extension {{ServiceInfoExtTypes}}
      
      ServiceInfoExtTypes EXT-TYPE ::= {
       { ProviderServiceContext IDENTIFIED BY c-ProviderServContext } |
       { IPv6Address IDENTIFIED BY c-IPv6Address } |
       { ServicePort IDENTIFIED BY c-servicePort } |
       { ProviderMacAddress IDENTIFIED BY c-ProviderMACaddress } |
       { RcpiThreshold IDENTIFIED BY c-RCPIthreshold } |
       { WsaCountThreshold IDENTIFIED BY c-WSAcountThreshold } |
       { WsaCountThresholdInterval IDENTIFIED BY c-WSAcountThresInt } |
       { SAMapplicationData IDENTIFIED BY c-SAMapplicationData } |
       { ProtocolType IDENTIFIED BY c-ProtocolType }, 
       ...
       }
      
      
      ChannelInfos ::= SEQUENCE OF ChannelInfo
      
      ChannelInfo ::= SEQUENCE{
       operatingClass OperatingClass80211,
       channelNumber ChannelNumber80211,
       powerLevel TXpower80211,
       dataRate WsaChInfoDataRate,
       extensions ChInfoOptions
       }
      
      OperatingClass80211 ::= INTEGER (0..255)
      
      WsaChInfoDataRate ::= SEQUENCE{
       adaptable BIT STRING (SIZE(1)),
       dataRate INTEGER (0..127)
       }
      
      ChInfoOptions ::= SEQUENCE{
       option1 NULL OPTIONAL,
       option2 NULL OPTIONAL,
       option3 NULL OPTIONAL,
       option4 NULL OPTIONAL,
       option5 NULL OPTIONAL,
       option6 NULL OPTIONAL,
       option7 NULL OPTIONAL,
       extensions ChannelInfoExts OPTIONAL
       } 
      
      ChannelInfoExts ::= SEQUENCE OF ChannelInfoExt
      
      ChannelInfoExt ::= Extension {{ChannelInfoExtTypes}}
      
      ChannelInfoExtTypes EXT-TYPE ::= {
       { EdcaParameterSet IDENTIFIED BY c-EDCAparameterSet } |
       { ChannelAccess80211 IDENTIFIED BY c-ChannelAccess },
       ...
       }
      
      
      RoutingAdvertisement ::= SEQUENCE {
       lifetime RouterLifetime,
       ipPrefix IpV6Prefix,
       ipPrefixLength IpV6PrefixLength,
       defaultGateway IPv6Address,
       primaryDns IPv6Address,
       extensions RoutAdvertExts
       }
      
      RouterLifetime ::= INTEGER (0..65535)
      
      IpV6Prefix ::= OCTET STRING (SIZE (16))
      
      IpV6PrefixLength ::= INTEGER (0..255)
      
      
      
      RoutAdvertExts ::= SEQUENCE OF RoutAdvertExt
      
      RoutAdvertExt ::= Extension {{RoutAdvertExtTypes}}
      
      RoutAdvertExtTypes EXT-TYPE ::= {
       { SecondaryDns IDENTIFIED BY c-SecondaryDNS } |
       { GatewayMacAddress IDENTIFIED BY c-GatewayMACaddress },
       ...
       }
      
      
      
      Srm ::= SEQUENCE{
       header RsvAdvPrtVersion,
       body SrmBody
       }
      
      SrmBody ::= SEQUENCE{
       extensions SRMextensions OPTIONAL,
       prvChannelsRq SrmPrivateChannelsRq OPTIONAL,
       contexts SrmContexts OPTIONAL,
       prvChannelsCf SrmPrvChAllocConf OPTIONAL 
       }
      
      SrmPrivateChannelsRq ::= SEQUENCE{
       portDynSam PortNumber,
       allocReqs SrmPrvChAllocReq
       }
      
      SrmPrvChAllocReq ::= SEQUENCE OF ITSaid
      
      SrmContexts ::= SEQUENCE OF SrmContext
      
      SrmContext ::= SEQUENCE{
       context SamContext,
       clientPort PortNumber
       }
      
      SrmPrvChAllocConf ::= SEQUENCE OF ITSaid
      
      
      
      
      
      
      SA-CONTEXT ::= CLASS{
       &itsaidCtxRef ItsAidCtxRef UNIQUE,
       &ContextInfo OPTIONAL
       }
       WITH SYNTAX {&ContextInfo IDENTIFIED BY &itsaidCtxRef}
      
      SamContext ::= SEQUENCE{
       itsaidCtxRef SA-CONTEXT.&itsaidCtxRef({AllsamContexts}),
       context SA-CONTEXT.&ContextInfo({AllsamContexts}{@itsaidCtxRef})
       }
      
      ItsAidCtxRef ::= SEQUENCE{
       itsaid ITSaid,
       ctx CtxRef
       }
      
      CtxRef ::= INTEGER(0..255)
      c-ctxRefNull CtxRef::=0
      c-ctxRefMandApp CtxRef::=1
      
      
      c-CtxTypeSystemNull ItsAidCtxRef::={itsaid content:0, ctx c-ctxRefNull}
      NullCtx ::= NULL
      
      
      c-CtxTypeSystemMandApp ItsAidCtxRef::={itsaid content:0, ctx c-ctxRefMandApp}
      MandAppCtx ::= SEQUENCE OF ItsAidCtxRef
      
      AllsamContexts SA-CONTEXT ::= {
       { NullCtx IDENTIFIED BY c-CtxTypeSystemNull} |
       { MandAppCtx IDENTIFIED BY c-CtxTypeSystemMandApp} ,
       ...
       }
      
      
      SRMextensions::=SEQUENCE OF SRMextension
      
      
      SRMextension::= Extension{{SRMexts}}
      
      
      
      SRMexts EXT-TYPE::={
      -- { IDENTIFIED BY } |
      -- { IDENTIFIED BY } |
      -- { IDENTIFIED BY },
       ...
       }
      
      ProtocolType ::= VarLengthNumber
      
      
      PortNumber::=INTEGER(0..65535)
      
      
      ITSRangingSAMAppData ::= SEQUENCE {
       protocolVersion INTEGER (0..255),
       ackResponseService ACKResponseService,
       groundAltitude Altitude OPTIONAL,
       roadAngles RoadAngles OPTIONAL,
       ... 
      }
      ACKResponseService::= SEQUENCE {
       ackRespDelayAdjust INTEGER (-32768..32767),
       ackRespDelayStdDev INTEGER (0..65535),
       ...
      }
      RoadAngles::= SEQUENCE OF Heading
      
      
       WsaSsp::= SEQUENCE {
       version Uint8,
       advertiserPermissions AdvertiserPermissions OPTIONAL,
       providerPermissions ProviderPermissions OPTIONAL,
       ...
       }
       
       AdvertiserPermissions ::= SEQUENCE OF ChannelIdentifier
      
       ChannelIdentifier ::= SEQUENCE {
       countryString OCTET STRING (SIZE(3)),
       operatingClass Uint8,
       channelNumber Uint8
       }
      
       ProviderPermissions ::= SEQUENCE OF ChannelSpecificProviderPermission
      
       ChannelSpecificProviderPermission ::= SEQUENCE {
       channelId ChannelIdentifier,
       permittedPsids SequenceOfPsid OPTIONAL,
       permittedEdcaParameters SequenceOfEdcaIdentifier OPTIONAL,
       maximumTransmitPower Uint8 OPTIONAL,
       ...
       }
      
       EdcaIdentifier ::= CHOICE {
       enum EnumeratedEdcaIdentifier,
       explicit ExplicitEdcaIdentifier,
       ...
       }
      
       EnumeratedEdcaIdentifier ::= ENUMERATED {us-j2945-bsm (0), ... }
      
       ExplicitEdcaIdentifier ::= SEQUENCE {
       qosInfo Uint8,
       reserved Uint8,
       set1 OCTET STRING (SIZE(4)),
       set2 OCTET STRING (SIZE(4)),
       set3 OCTET STRING (SIZE(4)),
       set4 OCTET STRING (SIZE(4))
       }
      
       SequenceOfEdcaIdentifier ::= SEQUENCE OF EdcaIdentifier
      
      
      
      
      Uint3 ::= INTEGER (0..7)
      Uint8 ::= INTEGER (0..255)
      Uint16 ::= INTEGER (0..65535)
      Uint32 ::= INTEGER (0..4294967295)
      Uint64 ::= INTEGER (0..18446744073709551615)
      
      SequenceOfUint8 ::= SEQUENCE OF Uint8
      SequenceOfUint16 ::= SEQUENCE OF Uint16
      
      
      
      
      Opaque ::= OCTET STRING
      
       
      HashedId10 ::= OCTET STRING (SIZE(10))
      HashedId8 ::= OCTET STRING (SIZE(8))
      HashedId3 ::= OCTET STRING (SIZE(3))
      SequenceOfHashedId3 ::= SEQUENCE OF HashedId3
      
      
      
      
      
      --
      
      
      Time32 ::= Uint32
      Time64 ::= Uint64
      
      ValidityPeriod ::= SEQUENCE {
       start Time32,
       duration Duration
      }
      
      Duration ::= CHOICE {
       microseconds Uint16,
       milliseconds Uint16,
       seconds Uint16,
       minutes Uint16,
       hours Uint16,
       sixtyHours Uint16,
       years Uint16
      } 
      
      
      
      
      --
      
      
      
      GeographicRegion ::= CHOICE {
       circularRegion CircularRegion,
       rectangularRegion SequenceOfRectangularRegion,
       polygonalRegion PolygonalRegion,
       identifiedRegion SequenceOfIdentifiedRegion,
       ...
      }
      
      CircularRegion ::= SEQUENCE {
       center Dot2TwoDLocation,
       radius Uint16
      }
      
      RectangularRegion ::= SEQUENCE {
       northWest Dot2TwoDLocation,
       southEast Dot2TwoDLocation
      }
      
      SequenceOfRectangularRegion ::= SEQUENCE OF RectangularRegion
      
      PolygonalRegion ::= SEQUENCE SIZE(3..MAX) OF Dot2TwoDLocation
      
      Dot2TwoDLocation ::= SEQUENCE {
       latitude Dot2Latitude,
       longitude Dot2Longitude
      }
      
      IdentifiedRegion ::= CHOICE {
       countryOnly CountryOnly,
       countryAndRegions CountryAndRegions,
       countryAndSubregions CountryAndSubregions,
       ...
      }
      
      SequenceOfIdentifiedRegion ::= SEQUENCE OF IdentifiedRegion
      
      CountryOnly ::= Uint16
      
      CountryAndRegions ::= SEQUENCE {
       countryOnly CountryOnly,
       regions SequenceOfUint8
      }
      
      CountryAndSubregions ::= SEQUENCE {
       country CountryOnly,
       regionAndSubregions SequenceOfRegionAndSubregions
      }
      
      RegionAndSubregions ::= SEQUENCE {
       region Uint8,
       subregions SequenceOfUint16
      }
      
      SequenceOfRegionAndSubregions ::= SEQUENCE OF RegionAndSubregions
      
      Dot2ThreeDLocation ::= SEQUENCE {
       latitude Dot2Latitude,
       longitude Dot2Longitude,
       elevation Dot2Elevation
      }
      
      Dot2Latitude ::= NinetyDegreeInt
      Dot2Longitude ::= OneEightyDegreeInt
      Dot2Elevation ::= ElevInt
      
      NinetyDegreeInt ::= INTEGER {
       min (-900000000),
       max (900000000),
       unknown (900000001)
      } (-900000000..900000001)
      
      KnownLatitude ::= NinetyDegreeInt (min..max)
      UnknownLatitude ::= NinetyDegreeInt (unknown)
       
      OneEightyDegreeInt ::= INTEGER {
       min (-1799999999),
       max (1800000000),
       unknown (1800000001)
      } (-1799999999..1800000001)
      
      KnownLongitude ::= OneEightyDegreeInt (min..max)
      UnknownLongitude ::= OneEightyDegreeInt (unknown)
       
      ElevInt ::= Uint16
      
      
      
      
      --
      
      
      Signature ::= CHOICE {
       ecdsaNistP256Signature EcdsaP256Signature,
       ecdsaBrainpoolP256r1Signature EcdsaP256Signature,
       ...,
       ecdsaBrainpoolP384r1Signature EcdsaP384Signature
      }
      
      EcdsaP256Signature ::= SEQUENCE {
       rSig EccP256CurvePoint,
       sSig OCTET STRING (SIZE (32))
      }
      
      EcdsaP384Signature ::= SEQUENCE {
       rSig EccP384CurvePoint,
       sSig OCTET STRING (SIZE (48))
      }
      
      EccP256CurvePoint ::= CHOICE {
       x-only OCTET STRING (SIZE (32)),
       fill NULL,
       compressed-y-0 OCTET STRING (SIZE (32)),
       compressed-y-1 OCTET STRING (SIZE (32)),
       uncompressedP256 SEQUENCE {
       x OCTET STRING (SIZE (32)),
       y OCTET STRING (SIZE (32))
       }
      }
      
      EccP384CurvePoint::= CHOICE {
       x-only OCTET STRING (SIZE (48)),
       fill NULL,
       compressed-y-0 OCTET STRING (SIZE (48)),
       compressed-y-1 OCTET STRING (SIZE (48)),
       uncompressedP384 SEQUENCE {
       x OCTET STRING (SIZE (48)),
       y OCTET STRING (SIZE (48))
       }
      }
      
      
      SymmAlgorithm ::= ENUMERATED { 
       aes128Ccm,
       ...
      }
      
      HashAlgorithm ::= ENUMERATED { 
       sha256,
       ...,
       sha384
      }
      
      EciesP256EncryptedKey ::= SEQUENCE {
       v EccP256CurvePoint,
       c OCTET STRING (SIZE (16)),
       t OCTET STRING (SIZE (16))
      }
      
      EncryptionKey ::= CHOICE {
       public PublicEncryptionKey,
       symmetric SymmetricEncryptionKey 
      }
      
      PublicEncryptionKey ::= SEQUENCE { 
       supportedSymmAlg SymmAlgorithm,
       publicKey BasePublicEncryptionKey
      }
      
      BasePublicEncryptionKey ::= CHOICE { 
       eciesNistP256 EccP256CurvePoint,
       eciesBrainpoolP256r1 EccP256CurvePoint,
       ...
      }
      
      PublicVerificationKey ::= CHOICE { 
       ecdsaNistP256 EccP256CurvePoint,
       ecdsaBrainpoolP256r1 EccP256CurvePoint,
       ...,
       ecdsaBrainpoolP384r1 EccP384CurvePoint
      }
      
      SymmetricEncryptionKey ::= CHOICE {
       aes128Ccm OCTET STRING(SIZE(16)),
       ...
      }
      
      
      
      
      --
      
      
      
      PsidSsp ::= SEQUENCE {
       psid Psid,
       ssp ServiceSpecificPermissions OPTIONAL
      }
      
      SequenceOfPsidSsp ::= SEQUENCE OF PsidSsp
      
      Psid ::= INTEGER (0..MAX)
      
      SequenceOfPsid ::= SEQUENCE OF Psid
      
      ServiceSpecificPermissions ::= CHOICE {
       opaque OCTET STRING (SIZE(0..MAX)),
       ...,
       bitmapSsp BitmapSsp
      }
      
      BitmapSsp ::= OCTET STRING (SIZE(0..31))
      
      PsidSspRange ::= SEQUENCE {
       psid Psid,
       sspRange SspRange OPTIONAL
      }
      
      SequenceOfPsidSspRange ::= SEQUENCE OF PsidSspRange
      
      SspRange ::= CHOICE {
       opaque SequenceOfOctetString,
       all NULL,
       ... ,
       bitmapSspRange BitmapSspRange
      }
       
      BitmapSspRange ::= SEQUENCE {
       sspValue OCTET STRING (SIZE(1..32)),
       sspBitmask OCTET STRING (SIZE(1..32))
      }
      
      SequenceOfOctetString ::= SEQUENCE (SIZE (0..MAX)) OF 
       OCTET STRING (SIZE(0..MAX))
      
      
      
      
      
      --
      
      
      SubjectAssurance ::= OCTET STRING (SIZE(1))
      
      CrlSeries ::= Uint16
       
      
      
      
      
      --
      
      
      IValue ::= Uint16
      Hostname ::= UTF8String (SIZE(0..255))
      LinkageValue ::= OCTET STRING (SIZE(9))
      GroupLinkageValue ::= SEQUENCE {
       jValue OCTET STRING (SIZE(4)),
       value OCTET STRING (SIZE(9))
      }
       
      LaId ::= OCTET STRING (SIZE(2)) 
      LinkageSeed ::= OCTET STRING (SIZE(16))
      
      
      
      
      
      END
      "#
}