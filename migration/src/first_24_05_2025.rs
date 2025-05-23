use sea_orm_migration::prelude::*;
use ::sea_orm::Statement;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Create enum types - each CREATE TYPE must be a separate execute call
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE ambulance_car_details_make_enum AS ENUM (
                'Mercedes-Benz', 'Ford', 'Chevrolet', 'Toyota', 'Volkswagen',
                'Ram', 'Nissan', 'Peugeot', 'Fiat', 'Iveco'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE ambulance_car_details_model_enum AS ENUM (
                'Sprinter', 'Transit', 'Express', 'HiAce', 'Crafter',
                'ProMaster', 'NV350', 'Boxer', 'Ducato', 'Daily'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE ambulance_status_enum AS ENUM (
                'AVAILABLE', 'IN_SERVICE', 'MAINTENANCE', 'DISPATCHED',
                'EN_ROUTE_TO_SCENE', 'AT_SCENE', 'TRANSPORTING_PATIENT',
                'EN_ROUTE_TO_HOSPITAL', 'AT_HOSPITAL', 'RETURNING_TO_BASE',
                'UNAVAILABLE', 'OUT_OF_SERVICE', 'ON_BREAK', 'FUELING',
                'CLEANING', 'AWAITING_DISPATCH', 'PREPARING_FOR_MISSION',
                'UNDER_REPAIR'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE ambulance_type_enum AS ENUM (
                'BASIC_LIFE_SUPPORT', 'ADVANCED_LIFE_SUPPORT', 'MOBILE_INTENSIVE_CARE_UNIT',
                'PEDIATRIC_AMBULANCE', 'NEONATAL_AMBULANCE', 'RESCUE_AMBULANCE',
                'BARIATRIC_AMBULANCE', 'WHEELCHAIR_VAN', 'AMBULATORY_TRANSPORT',
                'PSYCHIATRIC_TRANSPORT', 'LONG_DISTANCE_TRANSPORT', 'AIR_AMBULANCE',
                'WATER_AMBULANCE', 'HAZMAT_AMBULANCE', 'EVENT_MEDICAL_SERVICES',
                'CRITICAL_CARE_TRANSPORT', 'RAPID_RESPONSE_VEHICLE', 'SUPERVISOR_VEHICLE',
                'UTILITY_VEHICLE', 'COMMAND_VEHICLE', 'TRAINING_AMBULANCE'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE amenities_name_enum AS ENUM (
                'TV', 'WIFI', 'BATHROOM', 'FRIDGE', 'MICROWAVE',
                'COFFEE_MAKER', 'SOFA', 'DESK', 'BALCONY', 'VIEW'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE appointment_status_enum AS ENUM (
                'SCHEDULED', 'CONFIRMED', 'COMPLETED', 'CANCELLED', 'NO_SHOW'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE bed_type_enum AS ENUM (
                'SINGLE', 'DOUBLE', 'KING', 'QUEEN', 'BUNK', 'CRIB', 'HOSPITAL'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE bill_status_enum AS ENUM (
                'PENDING', 'PARTIAL', 'COMPLETED', 'REFUNDED'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE card_cardtype_enum AS ENUM (
                'text', 'chart', 'table', 'image'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE card_size_enum AS ENUM (
                'small', 'medium', 'large'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE department_name_enum AS ENUM (
                'CARDIOLOGY', 'ONCOLOGY', 'NEUROLOGY', 'PEDIATRICS', 'SURGERY',
                'INTERNAL_MEDICINE', 'OBSTETRICS_GYNECOLOGY', 'OPHTHALMOLOGY',
                'DERMATOLOGY', 'UROLOGY'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE emergency_incidenttype_enum AS ENUM (
                'CAR_ACCIDENT', 'MOTORCYCLE_ACCIDENT', 'PEDESTRIAN_ACCIDENT',
                'TRAIN_ACCIDENT', 'AIRPLANE_CRASH', 'SHIP_ACCIDENT', 'HEART_ATTACK',
                'STROKE', 'SEIZURE', 'DIABETIC_EMERGENCY', 'ALLERGIC_REACTION',
                'BREATHING_PROBLEM', 'SEVERE_BURNS', 'ELECTROCUTION', 'DROWNING',
                'POISONING', 'FALL_INJURY', 'FRACTURE', 'BLEEDING', 'HOUSE_FIRE',
                'FOREST_FIRE', 'GAS_LEAK', 'EXPLOSION', 'INDUSTRIAL_ACCIDENT',
                'EARTHQUAKE', 'FLOOD', 'TORNADO', 'HURRICANE', 'LANDSLIDE',
                'TSUNAMI', 'SHOOTING', 'STABBING', 'ROBBERY', 'DOMESTIC_VIOLENCE',
                'KIDNAPPING', 'ASSAULT', 'HOSTAGE_SITUATION', 'PANDEMIC',
                'INFECTIOUS_DISEASE_OUTBREAK', 'BIOLOGICAL_HAZARD', 'CHEMICAL_SPILL',
                'RADIATION_EXPOSURE', 'BUILDING_COLLAPSE', 'BRIDGE_COLLAPSE',
                'DAM_FAILURE', 'UNKNOWN', 'OTHER'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE emergency_severity_enum AS ENUM (
                'LOW', 'MEDIUM', 'HIGH', 'CRITICAL', 'SEVERE', 'EXTREME',
                'UNKNOWN', 'STABLE', 'UNSTABLE', 'DECEASED'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE emergency_status_enum AS ENUM (
                'PENDING', 'IN_PROGRESS', 'RESOLVED', 'CANCELLED', 'ESCALATED',
                'WAITING_FOR_RESPONSE', 'ON_HOLD', 'FAILED', 'AT_SCENE',
                'IN_AMBULANCE', 'IN_TRANSIT_TO_HOSPITAL', 'ARRIVED_AT_HOSPITAL',
                'TREATED_AT_HOME'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE guard_area_enum AS ENUM (
                'MAIN_ENTRANCE', 'ER', 'ICU', 'WARDS', 'PARKING_LOT',
                'CAFETERIA', 'PHARMACY', 'HELIPAD', 'LAB', 'RADIOLOGY'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE guard_shift_enum AS ENUM (
                'DAY', 'NIGHT', 'WEEKEND', 'EVENING', 'MORNING'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE prescription_order_status_enum AS ENUM (
                'PENDING', 'PROCESSING', 'SHIPPED', 'RECEIVED', 'CANCELLED'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE room_type_enum AS ENUM (
                'SINGLE', 'DOUBLE', 'SUITE', 'ICU', 'EMERGENCY', 'PEDIATRIC',
                'MATERNITY', 'SURGICAL', 'RECOVERY', 'ISOLATION'
            );"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"CREATE TYPE staff_role_enum AS ENUM (
                'DOCTOR', 'NURSE', 'ADMIN', 'TECHNICIAN', 'RECEPTIONIST',
                'CLEANER', 'SECURITY'
            );"#,
        ))
            .await?;

        // Create tables - each CREATE TABLE must be a separate execute call
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE hospital (
                id UUID PRIMARY KEY,
                name VARCHAR NOT NULL,
                address VARCHAR NOT NULL,
                phone VARCHAR,
                email VARCHAR,
                website VARCHAR,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE department (
                id UUID PRIMARY KEY,
                name department_name_enum NOT NULL,
                description VARCHAR,
                "hospitalId" UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE staff (
                id UUID PRIMARY KEY,
                name VARCHAR NOT NULL,
                role staff_role_enum NOT NULL,
                email VARCHAR,
                phone VARCHAR,
                "departmentId" UUID NOT NULL REFERENCES department(id) ON DELETE CASCADE,
                "hospitalId" UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE patient (
                id UUID PRIMARY KEY,
                name VARCHAR NOT NULL,
                "dateOfBirth" DATE,
                gender VARCHAR,
                address VARCHAR,
                phone VARCHAR,
                email VARCHAR,
                "bloodType" VARCHAR,
                "emergencyContact" VARCHAR,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE room (
                id UUID PRIMARY KEY,
                number VARCHAR NOT NULL,
                floor INTEGER,
                type room_type_enum NOT NULL,
                capacity INTEGER NOT NULL,
                "hospitalId" UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                "departmentId" UUID NOT NULL REFERENCES department(id) ON DELETE CASCADE,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE bed (
                id UUID PRIMARY KEY,
                number VARCHAR NOT NULL,
                type bed_type_enum NOT NULL,
                "isOccupied" BOOLEAN NOT NULL DEFAULT FALSE,
                "roomId" UUID NOT NULL REFERENCES room(id) ON DELETE CASCADE,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE admission (
                id UUID PRIMARY KEY,
                "patientId" UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
                "bedId" UUID NOT NULL REFERENCES bed(id) ON DELETE CASCADE,
                "admissionDate" TIMESTAMPTZ NOT NULL,
                "dischargeDate" TIMESTAMPTZ,
                diagnosis VARCHAR,
                notes VARCHAR,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE ambulance (
                id UUID PRIMARY KEY,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL,
                "hospitalId" VARCHAR NOT NULL,
                "ambulanceIc" INTEGER NOT NULL UNIQUE,
                "vehicleNumber" VARCHAR NOT NULL UNIQUE,
                make VARCHAR,
                year INTEGER,
                capacity INTEGER,
                mission VARCHAR,
                passengers JSONB,
                "driverName" VARCHAR,
                "driverLicense" VARCHAR,
                "lastServiceDate" TIMESTAMPTZ,
                "nextServiceDate" TIMESTAMPTZ,
                mileage INTEGER,
                "fuelType" VARCHAR,
                "registrationNumber" VARCHAR,
                "insuranceProvider" VARCHAR,
                "insuranceExpiryDate" TIMESTAMPTZ,
                notes VARCHAR,
                "carDetailsYear" INTEGER NOT NULL,
                "carDetailsColor" VARCHAR NOT NULL,
                "carDetailsIsambulance" BOOLEAN NOT NULL,
                "carDetailsLicenseplate" VARCHAR,
                "carDetailsMileage" DOUBLE PRECISION,
                "locationLatitude" DECIMAL(9,6) NOT NULL,
                "locationLongitude" DECIMAL(9,6) NOT NULL,
                type ambulance_type_enum NOT NULL,
                status ambulance_status_enum NOT NULL,
                "carDetailsMake" ambulance_car_details_make_enum NOT NULL,
                "carDetailsModel" ambulance_car_details_model_enum NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE amenities (
                id UUID PRIMARY KEY,
                name amenities_name_enum NOT NULL,
                description VARCHAR,
                "roomId" UUID NOT NULL REFERENCES room(id) ON DELETE CASCADE,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE appointment (
                id UUID PRIMARY KEY,
                "patientId" UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
                "staffId" UUID NOT NULL REFERENCES staff(id) ON DELETE CASCADE,
                "appointmentDate" TIMESTAMPTZ NOT NULL,
                status appointment_status_enum NOT NULL,
                notes VARCHAR,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE bill (
                id UUID PRIMARY KEY,
                "patientId" UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
                amount DECIMAL(10,2) NOT NULL,
                status bill_status_enum NOT NULL,
                "billDate" TIMESTAMPTZ NOT NULL,
                "dueDate" TIMESTAMPTZ NOT NULL,
                "paidAmount" DECIMAL(10,2),
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE card (
                id UUID PRIMARY KEY,
                title VARCHAR NOT NULL,
                content VARCHAR,
                "dashboardId" UUID NOT NULL,
                "cardType" card_cardtype_enum NOT NULL,
                size card_size_enum NOT NULL,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE customers (
                id UUID PRIMARY KEY,
                name VARCHAR NOT NULL,
                email VARCHAR NOT NULL,
                phone VARCHAR,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE dashboard (
                id UUID PRIMARY KEY,
                title VARCHAR NOT NULL,
                description VARCHAR,
                "userId" UUID NOT NULL,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE emergency (
                id UUID PRIMARY KEY,
                "createdAt" TIMESTAMPTZ NOT NULL,
                "updatedAt" TIMESTAMPTZ NOT NULL,
                "emergencyIc" VARCHAR NOT NULL,
                "reportedBy" INTEGER,
                notes VARCHAR,
                "resolvedAt" TIMESTAMPTZ,
                "modificationAttempts" JSONB,
                "idAmbulance" UUID REFERENCES ambulance(id),
                "emergencyLatitude" DECIMAL(9,6) NOT NULL,
                "emergencyLongitude" DECIMAL(9,6) NOT NULL,
                status emergency_status_enum NOT NULL,
                severity emergency_severity_enum NOT NULL,
                "incidentType" emergency_incidenttype_enum NOT NULL,
                description VARCHAR
            );
            "#,
        ))
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Drop tables first - each DROP TABLE must be a separate execute call
        // Drop in reverse order of creation to handle foreign key dependencies
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS emergency CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS dashboard CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS customers CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS card CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS bill CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS appointment CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS amenities CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS ambulance CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS admission CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS bed CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS room CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS patient CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS staff CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS department CASCADE;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TABLE IF EXISTS hospital CASCADE;"#,
        ))
            .await?;


        // Drop enums in reverse order - each DROP TYPE must be a separate execute call
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS staff_role_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS room_type_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS prescription_order_status_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS guard_shift_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS guard_area_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS emergency_status_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS emergency_severity_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS emergency_incidenttype_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS department_name_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS card_size_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS card_cardtype_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS bill_status_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS bed_type_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS appointment_status_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS amenities_name_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS ambulance_type_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS ambulance_status_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS ambulance_car_details_model_enum CASCADE;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DROP TYPE IF EXISTS ambulance_car_details_make_enum CASCADE;"#,
        ))
            .await?;

        Ok(())
    }
}
