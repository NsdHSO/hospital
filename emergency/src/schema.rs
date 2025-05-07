// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ambulance_cardetailsmake_enum"))]
    pub struct AmbulanceCardetailsmakeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ambulance_cardetailsmodel_enum"))]
    pub struct AmbulanceCardetailsmodelEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ambulance_status_enum"))]
    pub struct AmbulanceStatusEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "ambulance_type_enum"))]
    pub struct AmbulanceTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "amenities_name_enum"))]
    pub struct AmenitiesNameEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "appointment_status_enum"))]
    pub struct AppointmentStatusEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "bed_type_enum"))]
    pub struct BedTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "bill_status_enum"))]
    pub struct BillStatusEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "card_cardtype_enum"))]
    pub struct CardCardtypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "card_size_enum"))]
    pub struct CardSizeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "department_name_enum"))]
    pub struct DepartmentNameEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "emergency_incidenttype_enum"))]
    pub struct EmergencyIncidenttypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "emergency_severity_enum"))]
    pub struct EmergencySeverityEnum;

    #[derive(diesel::sql_types::SqlType, QueryId)]
    #[diesel(postgres_type(name = "emergency_status_enum"))]
    pub struct EmergencyStatusEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "guard_area_enum"))]
    pub struct GuardAreaEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "guard_shift_enum"))]
    pub struct GuardShiftEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "prescription_order_status_enum"))]
    pub struct PrescriptionOrderStatusEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "room_type_enum"))]
    pub struct RoomTypeEnum;

    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "staff_role_enum"))]
    pub struct StaffRoleEnum;
}

diesel::table! {
    admission (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        patientId -> Int4,
        roomId -> Int4,
        doctorId -> Int4,
        hospitalId -> Int4,
        admissionDate -> Timestamp,
        dischargeDate -> Nullable<Timestamp>,
        reason -> Text,
        diagnosis -> Nullable<Text>,
        notes -> Nullable<Text>,
        totalCost -> Numeric,
        admittingDoctorNotes -> Nullable<Varchar>,
        dischargeSummary -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AmbulanceTypeEnum;
    use super::sql_types::AmbulanceStatusEnum;
    use super::sql_types::AmbulanceCardetailsmakeEnum;
    use super::sql_types::AmbulanceCardetailsmodelEnum;

    ambulance (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Varchar,
        ambulanceIc -> Int4,
        vehicleNumber -> Varchar,
        make -> Nullable<Varchar>,
        year -> Nullable<Int4>,
        capacity -> Nullable<Int4>,
        #[sql_name = "type"]
        type_ -> AmbulanceTypeEnum,
        status -> AmbulanceStatusEnum,
        mission -> Nullable<Varchar>,
        passengers -> Nullable<Jsonb>,
        driverName -> Nullable<Varchar>,
        driverLicense -> Nullable<Varchar>,
        lastServiceDate -> Nullable<Timestamp>,
        nextServiceDate -> Nullable<Timestamp>,
        mileage -> Nullable<Int4>,
        fuelType -> Nullable<Varchar>,
        registrationNumber -> Nullable<Varchar>,
        insuranceProvider -> Nullable<Varchar>,
        insuranceExpiryDate -> Nullable<Timestamp>,
        notes -> Nullable<Varchar>,
        carDetailsMake -> AmbulanceCardetailsmakeEnum,
        carDetailsModel -> AmbulanceCardetailsmodelEnum,
        carDetailsYear -> Int4,
        carDetailsColor -> Varchar,
        carDetailsIsambulance -> Bool,
        carDetailsLicenseplate -> Nullable<Varchar>,
        carDetailsMileage -> Nullable<Float8>,
        locationLatitude -> Numeric,
        locationLongitude -> Numeric,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AmenitiesNameEnum;

    amenities (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        name -> AmenitiesNameEnum,
        description -> Nullable<Text>,
        cost -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AppointmentStatusEnum;

    appointment (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        patientId -> Int4,
        doctorId -> Int4,
        hospitalId -> Int4,
        appointmentDate -> Timestamp,
        status -> AppointmentStatusEnum,
        reason -> Nullable<Text>,
        notes -> Nullable<Text>,
        cost -> Numeric,
        scheduledBy -> Nullable<Varchar>,
        appointmentType -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BedTypeEnum;

    bed (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        roomId -> Int4,
        #[sql_name = "type"]
        type_ -> BedTypeEnum,
        isOccupied -> Nullable<Bool>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::BillStatusEnum;

    bill (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        admissionId -> Int4,
        patientId -> Int4,
        hospitalId -> Int4,
        amount -> Numeric,
        status -> BillStatusEnum,
        insuranceCoverage -> Nullable<Numeric>,
        patientResponsibility -> Numeric,
        dueDate -> Nullable<Date>,
        lineItems -> Nullable<Jsonb>,
        paymentDate -> Nullable<Timestamp>,
        paymentMethod -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CardCardtypeEnum;
    use super::sql_types::CardSizeEnum;

    card (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        title -> Varchar,
        content -> Varchar,
        icon -> Nullable<Varchar>,
        cardType -> Nullable<CardCardtypeEnum>,
        position -> Nullable<Int4>,
        size -> Nullable<CardSizeEnum>,
        dataConfig -> Nullable<Jsonb>,
        dashboardId -> Nullable<Uuid>,
    }
}

diesel::table! {
    customers (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Nullable<Varchar>,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    dashboard (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        name -> Text,
        description -> Nullable<Varchar>,
        isActive -> Bool,
        ownerId -> Nullable<Int4>,
        layoutConfig -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::DepartmentNameEnum;

    department (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Int4,
        name -> DepartmentNameEnum,
        floor -> Nullable<Int4>,
        headOfDepartment -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        capacity -> Nullable<Int4>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::EmergencyStatusEnum;
    use super::sql_types::EmergencySeverityEnum;
    use super::sql_types::EmergencyIncidenttypeEnum;

    emergency (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        emergencyIc -> Text,
        description -> Text,
        status -> EmergencyStatusEnum,
        severity -> EmergencySeverityEnum,
        reportedBy -> Nullable<Int4>,
        incidentType -> EmergencyIncidenttypeEnum,
        notes -> Nullable<Text>,
        resolvedAt -> Nullable<Timestamp>,
        modificationAttempts -> Nullable<Jsonb>,
        idAmbulance -> Nullable<Uuid>,
        emergencyLatitude -> Numeric,
        emergencyLongitude -> Numeric,
        additional_info -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::GuardShiftEnum;
    use super::sql_types::GuardAreaEnum;

    guard (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Int4,
        name -> Varchar,
        shift -> GuardShiftEnum,
        area -> GuardAreaEnum,
        employeeId -> Nullable<Varchar>,
    }
}

diesel::table! {
    hospital (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        name -> Varchar,
        address -> Varchar,
        phone -> Nullable<Varchar>,
        website -> Nullable<Varchar>,
        description -> Nullable<Text>,
        capacity -> Nullable<Int4>,
        established -> Nullable<Int4>,
        ceo -> Nullable<Varchar>,
        traumaLevel -> Nullable<Varchar>,
        revenue -> Nullable<Int4>,
        nonProfit -> Nullable<Bool>,
        licenseNumber -> Nullable<Varchar>,
        accreditation -> Nullable<Varchar>,
        patientSatisfactionRating -> Nullable<Int4>,
        averageStayLength -> Nullable<Int4>,
        annualBudget -> Nullable<Int4>,
        owner -> Nullable<Varchar>,
        latitude -> Nullable<Numeric>,
        longitude -> Nullable<Numeric>,
    }
}

diesel::table! {
    inventory (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Int4,
        itemName -> Varchar,
        quantity -> Int4,
        unitPrice -> Nullable<Int4>,
        reorderPoint -> Nullable<Int4>,
        lastReceivedDate -> Nullable<Timestamp>,
    }
}

diesel::table! {
    medical_record (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        patientId -> Int4,
        hospitalId -> Int4,
        recordData -> Nullable<Text>,
        recordDate -> Nullable<Timestamp>,
    }
}

diesel::table! {
    patient (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Int4,
        firstName -> Varchar,
        lastName -> Varchar,
        dateOfBirth -> Date,
        gender -> Varchar,
        phone -> Varchar,
        email -> Nullable<Varchar>,
        address -> Varchar,
        emergencyContact -> Nullable<Varchar>,
        bloodType -> Nullable<Varchar>,
        allergies -> Nullable<Array<Nullable<Text>>>,
        medicalHistory -> Nullable<Text>,
    }
}

diesel::table! {
    patientInfo (id) {
        id -> Uuid,
        name -> Nullable<Varchar>,
        medicalInfo -> Nullable<Text>,
    }
}

diesel::table! {
    patient_doctor (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        patientId -> Int4,
        doctorId -> Int4,
        hospitalId -> Int4,
        assignedDate -> Date,
        notes -> Nullable<Varchar>,
    }
}

diesel::table! {
    prescription (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        patientId -> Int4,
        doctorId -> Int4,
        hospitalId -> Int4,
        medication -> Varchar,
        dosage -> Varchar,
        frequency -> Varchar,
        startDate -> Date,
        endDate -> Date,
        instructions -> Nullable<Text>,
        cost -> Numeric,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::PrescriptionOrderStatusEnum;

    prescription_order (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Int4,
        supplierId -> Int4,
        orderDate -> Timestamp,
        orderItems -> Nullable<Text>,
        totalAmount -> Nullable<Int4>,
        status -> Nullable<PrescriptionOrderStatusEnum>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoomTypeEnum;

    room (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Int4,
        #[sql_name = "type"]
        type_ -> RoomTypeEnum,
        roomNumber -> Nullable<Varchar>,
        ratePerDay -> Nullable<Int4>,
        description -> Nullable<Text>,
        floor -> Nullable<Int4>,
        view -> Nullable<Varchar>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StaffRoleEnum;

    staff (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Int4,
        departmentId -> Int4,
        name -> Varchar,
        role -> StaffRoleEnum,
        specialization -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
    }
}

diesel::table! {
    staff_schedule (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        staffId -> Int4,
        departmentId -> Int4,
        hospitalId -> Int4,
        scheduleDate -> Date,
        startTime -> Time,
        endTime -> Time,
    }
}

diesel::table! {
    supplier (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        hospitalId -> Int4,
        name -> Varchar,
        contactPerson -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
    }
}

diesel::table! {
    treatment (id) {
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        id -> Uuid,
        admissionId -> Int4,
        doctorId -> Int4,
        hospitalId -> Int4,
        description -> Text,
        treatmentDate -> Timestamp,
        cost -> Numeric,
        notes -> Nullable<Text>,
    }
}

diesel::joinable!(card -> dashboard (dashboardId));
diesel::joinable!(emergency -> ambulance (idAmbulance));

diesel::allow_tables_to_appear_in_same_query!(
    admission,
    ambulance,
    amenities,
    appointment,
    bed,
    bill,
    card,
    customers,
    dashboard,
    department,
    emergency,
    guard,
    hospital,
    inventory,
    medical_record,
    patient,
    patientInfo,
    patient_doctor,
    prescription,
    prescription_order,
    room,
    staff,
    staff_schedule,
    supplier,
    treatment,
);
