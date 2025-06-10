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
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'ambulance_car_details_make_enum') THEN
                    CREATE TYPE ambulance_car_details_make_enum AS ENUM (
                        'Mercedes-Benz', 'Ford', 'Chevrolet', 'Toyota', 'Volkswagen',
                        'Ram', 'Nissan', 'Peugeot', 'Fiat', 'Iveco'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'ambulance_car_details_model_enum') THEN
                    CREATE TYPE ambulance_car_details_model_enum AS ENUM (
                        'Sprinter', 'Transit', 'Express', 'HiAce', 'Crafter',
                        'ProMaster', 'NV350', 'Boxer', 'Ducato', 'Daily'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'ambulance_status_enum') THEN
                    CREATE TYPE ambulance_status_enum AS ENUM (
                        'AVAILABLE', 'IN_SERVICE', 'MAINTENANCE', 'DISPATCHED',
                        'EN_ROUTE_TO_SCENE', 'AT_SCENE', 'TRANSPORTING_PATIENT',
                        'EN_ROUTE_TO_HOSPITAL', 'AT_HOSPITAL', 'RETURNING_TO_BASE',
                        'UNAVAILABLE', 'OUT_OF_SERVICE', 'ON_BREAK', 'FUELING',
                        'CLEANING', 'AWAITING_DISPATCH', 'PREPARING_FOR_MISSION',
                        'UNDER_REPAIR'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'ambulance_type_enum') THEN
                    CREATE TYPE ambulance_type_enum AS ENUM (
                        'BASIC_LIFE_SUPPORT', 'ADVANCED_LIFE_SUPPORT', 'MOBILE_INTENSIVE_CARE_UNIT',
                        'PEDIATRIC_AMBULANCE', 'NEONATAL_AMBULANCE', 'RESCUE_AMBULANCE',
                        'BARIATRIC_AMBULANCE', 'WHEELCHAIR_VAN', 'AMBULATORY_TRANSPORT',
                        'PSYCHIATRIC_TRANSPORT', 'LONG_DISTANCE_TRANSPORT', 'AIR_AMBULANCE',
                        'WATER_AMBULANCE', 'HAZMAT_AMBULANCE', 'EVENT_MEDICAL_SERVICES',
                        'CRITICAL_CARE_TRANSPORT', 'RAPID_RESPONSE_VEHICLE', 'SUPERVISOR_VEHICLE',
                        'UTILITY_VEHICLE', 'COMMAND_VEHICLE', 'TRAINING_AMBULANCE'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'amenities_name_enum') THEN
                    CREATE TYPE amenities_name_enum AS ENUM (
                        'TV', 'WIFI', 'BATHROOM', 'FRIDGE', 'MICROWAVE',
                        'COFFEE_MAKER', 'SOFA', 'DESK', 'BALCONY', 'VIEW'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'appointment_status_enum') THEN
                    CREATE TYPE appointment_status_enum AS ENUM (
                        'SCHEDULED', 'CONFIRMED', 'COMPLETED', 'CANCELLED', 'NO_SHOW'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'bed_type_enum') THEN
                    CREATE TYPE bed_type_enum AS ENUM (
                        'SINGLE', 'DOUBLE', 'KING', 'QUEEN', 'BUNK', 'CRIB', 'HOSPITAL'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'bill_status_enum') THEN
                    CREATE TYPE bill_status_enum AS ENUM (
                        'PENDING', 'PARTIAL', 'COMPLETED', 'REFUNDED'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'card_cardtype_enum') THEN
                    CREATE TYPE card_cardtype_enum AS ENUM (
                        'text', 'chart', 'table', 'image'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'card_size_enum') THEN
                    CREATE TYPE card_size_enum AS ENUM (
                        'small', 'medium', 'large'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'department_name_enum') THEN
                    CREATE TYPE department_name_enum AS ENUM (
                        'CARDIOLOGY', 'ONCOLOGY', 'NEUROLOGY', 'PEDIATRICS', 'SURGERY',
                        'INTERNAL_MEDICINE', 'OBSTETRICS_GYNECOLOGY', 'OPHTHALMOLOGY',
                        'DERMATOLOGY', 'UROLOGY'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'emergency_incidenttype_enum') THEN
                    CREATE TYPE emergency_incidenttype_enum AS ENUM (
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
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'emergency_severity_enum') THEN
                    CREATE TYPE emergency_severity_enum AS ENUM (
                        'LOW', 'MEDIUM', 'HIGH', 'CRITICAL', 'SEVERE', 'EXTREME',
                        'UNKNOWN', 'STABLE', 'UNSTABLE', 'DECEASED'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'emergency_status_enum') THEN
                    CREATE TYPE emergency_status_enum AS ENUM (
                        'PENDING', 'IN_PROGRESS', 'RESOLVED', 'CANCELLED', 'ESCALATED',
                        'WAITING_FOR_RESPONSE', 'ON_HOLD', 'FAILED', 'AT_SCENE',
                        'IN_AMBULANCE', 'IN_TRANSIT_TO_HOSPITAL', 'ARRIVED_AT_HOSPITAL',
                        'TREATED_AT_HOME'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'guard_area_enum') THEN
                    CREATE TYPE guard_area_enum AS ENUM (
                        'MAIN_ENTRANCE', 'ER', 'ICU', 'WARDS', 'PARKING_LOT',
                        'CAFETERIA', 'PHARMACY', 'HELIPAD', 'LAB', 'RADIOLOGY'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'guard_shift_enum') THEN
                    CREATE TYPE guard_shift_enum AS ENUM (
                        'DAY', 'NIGHT', 'WEEKEND', 'EVENING', 'MORNING'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'prescription_order_status_enum') THEN
                    CREATE TYPE prescription_order_status_enum AS ENUM (
                        'PENDING', 'PROCESSING', 'SHIPPED', 'RECEIVED', 'CANCELLED'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'room_type_enum') THEN
                    CREATE TYPE room_type_enum AS ENUM (
                        'SINGLE', 'DOUBLE', 'SUITE', 'ICU', 'EMERGENCY', 'PEDIATRIC',
                        'MATERNITY', 'SURGICAL', 'RECOVERY', 'ISOLATION'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ 
            BEGIN 
                IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'staff_role_enum') THEN
                    CREATE TYPE staff_role_enum AS ENUM (
                        'DOCTOR', 'NURSE', 'ADMIN', 'TECHNICIAN', 'RECEPTIONIST',
                        'CLEANER', 'SECURITY'
                    );
                END IF;
            END $$;"#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'gender_enum') THEN CREATE TYPE gender_enum AS ENUM ('MALE', 'FEMALE'); END IF; END $$;"#,
        ))
            .await?;
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM pg_type WHERE typname = 'blood_type_enum') THEN CREATE TYPE blood_type_enum AS ENUM ('A_POSITIVE', 'A_NEGATIVE', 'B_POSITIVE', 'B_NEGATIVE', 'AB_POSITIVE', 'AB_NEGATIVE', 'O_POSITIVE', 'O_NEGATIVE'); END IF; END $$;"#,
        ))
            .await?;
        // Create tables - each CREATE TABLE must be a separate execute call
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS hospital (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR NOT NULL UNIQUE,
                address VARCHAR NOT NULL,
                phone VARCHAR,
                website VARCHAR,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                description TEXT,
                capacity INTEGER,
                established INTEGER,
                ceo VARCHAR,
                trauma_level VARCHAR,
                revenue INTEGER,
                non_profit BOOLEAN,
                license_number VARCHAR,
                accreditation VARCHAR,
                patient_satisfaction_rating INTEGER,
                average_stay_length INTEGER,
                annual_budget INTEGER,
                owner VARCHAR,
                latitude DECIMAL(10, 6),
                longitude DECIMAL(10, 6)
            );
            "#,
        ))
            .await?;
            
        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS patient (
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                hospital_id UUID NULL REFERENCES hospital(id) ON DELETE CASCADE,
                first_name VARCHAR NOT NULL,
                last_name VARCHAR NOT NULL,
                date_of_birth DATE NOT NULL,
                gender gender_enum NULL,
                phone VARCHAR NOT NULL,
                email VARCHAR NULL,
                address VARCHAR NOT NULL,
                emergency_contact VARCHAR NULL,
                blood_type blood_type_enum NULL,
                allergies TEXT[] NULL,
                medical_history TEXT NULL,
                patient_ic VARCHAR NOT NULL
            );
            "#,
        ))
            .await?;


        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS department (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name department_name_enum NOT NULL,
                description VARCHAR,
                hospital_id UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS staff (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                name VARCHAR NOT NULL,
                role staff_role_enum NOT NULL,
                email VARCHAR,
                phone VARCHAR,
                department_id UUID NOT NULL REFERENCES department(id) ON DELETE CASCADE,
                hospital_id UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS room (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                number VARCHAR NOT NULL,
                floor INTEGER,
                type room_type_enum NOT NULL,
                capacity INTEGER NOT NULL,
                hospital_id UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                department_id UUID NOT NULL REFERENCES department(id) ON DELETE CASCADE,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS bed (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                hospital_id UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                bed_ic INTEGER NOT NULL UNIQUE,
                number VARCHAR NOT NULL,
                type bed_type_enum NOT NULL,
                is_occupied BOOLEAN NOT NULL DEFAULT FALSE,
                room_id UUID NOT NULL REFERENCES room(id) ON DELETE CASCADE,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS admission (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
                room_id UUID NOT NULL REFERENCES room(id) ON DELETE CASCADE,
                doctor_id UUID NOT NULL REFERENCES staff(id) ON DELETE CASCADE,
                hospital_id UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                admission_date TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                discharge_date TIMESTAMP WITHOUT TIME ZONE,
                reason TEXT NOT NULL,
                diagnosis TEXT,
                notes TEXT,
                total_cost DECIMAL(10, 2) NOT NULL,
                admitting_doctor_notes TEXT,
                discharge_summary TEXT,
                admission_ic TEXT,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS ambulance (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                hospital_id UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                ambulance_ic INTEGER NOT NULL UNIQUE,
                vehicle_number VARCHAR NOT NULL UNIQUE,
                make VARCHAR,
                year INTEGER,
                capacity INTEGER,
                mission VARCHAR,
                passengers JSONB,
                driver_name VARCHAR,
                driver_license VARCHAR,
                last_service_date TIMESTAMP WITHOUT TIME ZONE,
                next_service_date TIMESTAMP WITHOUT TIME ZONE,
                mileage INTEGER,
                fuel_type VARCHAR,
                registration_number VARCHAR,
                insurance_provider VARCHAR,
                insurance_expiry_date TIMESTAMP WITHOUT TIME ZONE,
                notes VARCHAR,
                car_details_year INTEGER NOT NULL,
                car_details_color VARCHAR NOT NULL,
                car_details_is_ambulance BOOLEAN NOT NULL,
                car_details_license_plate VARCHAR,
                car_details_mileage DOUBLE PRECISION,
                location_latitude DECIMAL(9,6) NOT NULL,
                location_longitude DECIMAL(9,6) NOT NULL,
                type ambulance_type_enum NOT NULL,
                status ambulance_status_enum NOT NULL,
                car_details_make ambulance_car_details_make_enum NOT NULL,
                car_details_model ambulance_car_details_model_enum NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS amenities (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                amenities_ic INTEGER NOT NULL UNIQUE,
                name amenities_name_enum NOT NULL,
                description VARCHAR,
                room_id UUID NOT NULL REFERENCES room(id) ON DELETE CASCADE,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS appointment (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                appointment_ic INTEGER NOT NULL UNIQUE,
                patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
                staff_id UUID NOT NULL REFERENCES staff(id) ON DELETE CASCADE,
                appointment_date TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                status appointment_status_enum NOT NULL,
                notes VARCHAR,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS bill (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
                amount DECIMAL(10,2) NOT NULL,
                status bill_status_enum NOT NULL,
                bill_date TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                due_date TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                paid_amount DECIMAL(10,2),
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS card (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
                title VARCHAR NOT NULL,
                content VARCHAR NOT NULL,
                icon VARCHAR,
                position INTEGER,
                data_config JSONB,
                dashboard_id UUID,
                card_type card_cardtype_enum,
                size card_size_enum,
                card_ic INTEGER UNIQUE
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS customers (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                customer_ic INTEGER NOT NULL UNIQUE,
                name VARCHAR NOT NULL,
                email VARCHAR NOT NULL,
                phone VARCHAR,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS dashboard (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                title VARCHAR NOT NULL,
                description VARCHAR,
                user_id UUID NOT NULL,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS emergency (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                emergency_ic VARCHAR NOT NULL,
                reported_by INTEGER,
                ambulance_id UUID NULL REFERENCES ambulance(id),
                hospital_id UUID NULL REFERENCES hospital(id),
                notes VARCHAR,
                resolved_at TIMESTAMP WITHOUT TIME ZONE,
                modification_attempts JSONB,
                emergency_latitude DECIMAL(9,6) NOT NULL,
                emergency_longitude DECIMAL(9,6) NOT NULL,
                status emergency_status_enum NOT NULL,
                severity emergency_severity_enum NOT NULL,
                incident_type emergency_incidenttype_enum NOT NULL,
                description VARCHAR
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS inventory (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                inventory_ic INTEGER NOT NULL UNIQUE,
                hospital_id UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                item_name VARCHAR NOT NULL,
                quantity INTEGER NOT NULL,
                unit_price INTEGER,
                reorder_point INTEGER,
                last_received_date TIMESTAMP WITHOUT TIME ZONE,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
            );
            "#,
        ))
            .await?;

        db.execute(Statement::from_string(
            manager.get_database_backend(),
            r#"
            CREATE TABLE IF NOT EXISTS patient_doctor (
                id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
                patient_doctor_ic INTEGER NOT NULL UNIQUE,
                patient_id UUID NOT NULL REFERENCES patient(id) ON DELETE CASCADE,
                doctor_id UUID NOT NULL REFERENCES staff(id) ON DELETE CASCADE,
                hospital_id UUID NOT NULL REFERENCES hospital(id) ON DELETE CASCADE,
                assigned_date DATE NOT NULL,
                notes VARCHAR,
                created_at TIMESTAMP WITHOUT TIME ZONE NOT NULL,
                updated_at TIMESTAMP WITHOUT TIME ZONE NOT NULL
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
