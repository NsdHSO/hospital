use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AmbulanceTypeEnum"]
pub enum AmbulanceType {
    #[db_rename = "BASIC_LIFE_SUPPORT"]
    BasicLifeSupport,
    #[db_rename = "ADVANCED_LIFE_SUPPORT"]
    AdvancedLifeSupport,
    #[db_rename = "MOBILE_INTENSIVE_CARE_UNIT"]
    MobileIntensiveCareUnit,
    #[db_rename = "PEDIATRIC_AMBULANCE"]
    PediatricAmbulance,
    #[db_rename = "NEONATAL_AMBULANCE"]
    NeonatalAmbulance,
    #[db_rename = "RESCUE_AMBULANCE"]
    RescueAmbulance,
    #[db_rename = "BARIATRIC_AMBULANCE"]
    BariatricAmbulance,
    #[db_rename = "WHEELCHAIR_VAN"]
    WheelchairVan,
    #[db_rename = "AMBULATORY_TRANSPORT"]
    AmbulatoryTransport,
    #[db_rename = "PSYCHIATRIC_TRANSPORT"]
    PsychiatricTransport,
    #[db_rename = "LONG_DISTANCE_TRANSPORT"]
    LongDistanceTransport,
    #[db_rename = "AIR_AMBULANCE"]
    AirAmbulance,
    #[db_rename = "WATER_AMBULANCE"]
    WaterAmbulance,
    #[db_rename = "HAZMAT_AMBULANCE"]
    HazmatAmbulance,
    #[db_rename = "EVENT_MEDICAL_SERVICES"]
    EventMedicalServices,
    #[db_rename = "CRITICAL_CARE_TRANSPORT"]
    CriticalCareTransport,
    #[db_rename = "RAPID_RESPONSE_VEHICLE"]
    RapidResponseVehicle,
    #[db_rename = "SUPERVISOR_VEHICLE"]
    SupervisorVehicle,
    #[db_rename = "UTILITY_VEHICLE"]
    UtilityVehicle,
    #[db_rename = "COMMAND_VEHICLE"]
    CommandVehicle,
    #[db_rename = "TRAINING_AMBULANCE"]
    TrainingAmbulance,
}

#[derive(Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AmbulanceStatusEnum"]
pub enum AmbulanceStatus {
    #[db_rename = "AVAILABLE"]
    Available,
    #[db_rename = "IN_SERVICE"]
    InService,
    #[db_rename = "MAINTENANCE"]
    Maintenance,
    #[db_rename = "DISPATCHED"]
    Dispatched,
    #[db_rename = "EN_ROUTE_TO_SCENE"]
    EnRouteToScene,
    #[db_rename = "AT_SCENE"]
    AtScene,
    #[db_rename = "TRANSPORTING_PATIENT"]
    TransportingPatient,
    #[db_rename = "EN_ROUTE_TO_HOSPITAL"]
    EnRouteToHospital,
    #[db_rename = "AT_HOSPITAL"]
    AtHospital,
    #[db_rename = "RETURNING_TO_BASE"]
    ReturningToBase,
    #[db_rename = "UNAVAILABLE"]
    Unavailable,
    #[db_rename = "OUT_OF_SERVICE"]
    OutOfService,
    #[db_rename = "ON_BREAK"]
    OnBreak,
    #[db_rename = "FUELING"]
    Fueling,
    #[db_rename = "CLEANING"]
    Cleaning,
    #[db_rename = "AWAITING_DISPATCH"]
    AwaitingDispatch,
    #[db_rename = "PREPARING_FOR_MISSION"]
    PreparingForMission,
    #[db_rename = "UNDER_REPAIR"]
    UnderRepair,
}
#[derive(Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AmbulanceCardetailsmakeEnum"]
pub enum AmbulanceCarDetailsMake {
    #[db_rename = "Toyota"]
    Toyota,
    #[db_rename = "Ford"]
    Ford,
    #[db_rename = "Mercedes-Benz"]
    MercedesBenz,
    #[db_rename = "Volkswagen"]
    Volkswagen,
    #[db_rename = "Chevrolet"]
    Chevrolet,
    #[db_rename = "Ram"]
    Ram,
    #[db_rename = "Nissan"]
    Nissan,
    #[db_rename = "Peugeot"]
    Peugeot,
    #[db_rename = "Fiat"]
    Fiat,
    #[db_rename = "Iveco"]
    Iveco,
}

#[derive(Debug, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::AmbulanceCardetailsmodelEnum"]
pub enum AmbulanceCarDetailsModel {
    #[db_rename = "Sprinter"]
    Sprinter,
    #[db_rename = "Transit"]
    Transit,
    #[db_rename = "Express"]
    Express,
    #[db_rename = "HiAce"]
    HiAce,
    #[db_rename = "Crafter"]
    Crafter,
    #[db_rename = "ProMaster"]
    ProMaster,
    #[db_rename = "NV350"]
    Nv350,
    #[db_rename = "Boxer"]
    Boxer,
    #[db_rename = "Ducato"]
    Ducato,
    #[db_rename = "Daily"]
    Daily,
}
