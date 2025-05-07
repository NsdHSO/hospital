use diesel::deserialize::FromSql;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::EmergencyStatusEnum"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmergencyStatus {
    Pending,
    InProgress,
    Resolved,
    Cancelled,
    Escalated,
    WaitingForResponse,
    OnHold,
    Failed,
    AtScene,
    InAmbulance,
    InTransitToHospital,
    ArrivedAtHospital,
    TreatedAtHome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::EmergencySeverityEnum"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[repr(i32)]
pub enum EmergencySeverity {
    Low,      // Minor injury or discomfort, no immediate danger
    Medium,   // Requires attention but not life-threatening
    High,     // Serious emergency needing fast response
    Critical, // Life-threatening situation
    Severe,   // Mass casualty or widespread crisis
    Extreme,  // Catastrophic emergency (e.g., natural disaster)
    Unknown,  // Severity is not yet determined

    // Special Cases
    Stable,   // Condition is under control, but still being monitored
    Unstable, // Condition is deteriorating, requiring urgent care
    Deceased, // Fatality recorded in the emergency case
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, DbEnum, Serialize, Deserialize)]
#[ExistingTypePath = "crate::schema::sql_types::EmergencyIncidenttypeEnum"]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EmergencyType {
    // Traffic & Transport
    CarAccident,
    MotorcycleAccident,
    PedestrianAccident,
    TrainAccident,
    AirplaneCrash,
    ShipAccident,

    // Medical Emergencies
    HeartAttack,
    Stroke,
    Seizure,
    DiabeticEmergency,
    AllergicReaction,
    BreathingProblem,
    SevereBurns,
    Electrocution,
    Drowning,
    Poisoning,
    FallInjury,
    Fracture,
    Bleeding,

    // Fires & Explosions
    HouseFire,
    ForestFire,
    GasLeak,
    Explosion,
    IndustrialAccident,

    // Natural Disasters
    Earthquake,
    Flood,
    Tornado,
    Hurricane,
    Landslide,
    Tsunami,

    // Crime & Violence
    Shooting,
    Stabbing,
    Robbery,
    DomesticViolence,
    Kidnapping,
    Assault,
    HostageSituation,

    // Public Health
    Pandemic,
    InfectiousDiseaseOutbreak,
    BiologicalHazard,
    ChemicalSpill,
    RadiationExposure,

    // Structural Failures
    BuildingCollapse,
    BridgeCollapse,
    DamFailure,

    // Other
    Unknown,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportedBy {
    User,
    Computer,
    Operator,
    AutomatedSystem,
    ThirdParty,
}
