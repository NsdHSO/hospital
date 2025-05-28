use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let queries = [
            // Add hospitalIc to hospital
            ("hospital", "hospitalIc", "VARCHAR"),
            // Add departmentIc to department
            ("department", "departmentIc", "VARCHAR"),
            // Add staffIc to staff
            ("staff", "staffIc", "VARCHAR"),
            // Add patientIc to patient
            ("patient", "patientIc", "VARCHAR"),
            // Add roomIc to room
            ("room", "roomIc", "VARCHAR"),
            // Add bedIc to bed
            ("bed", "bedIc", "INTEGER"),
            // Add amenitiesIc to amenities
            ("amenities", "amenitiesIc", "INTEGER"),
            // Add customerIc to customers
            ("customers", "customerIc", "INTEGER"),
            // Add inventoryIc to inventory
            ("inventory", "inventoryIc", "INTEGER"),
            // Add appointmentIc to appointment
            ("appointment", "appointmentIc", "INTEGER"),
            // Add patientDoctorIc to patient_doctor
            ("patient_doctor", "patientDoctorIc", "INTEGER"),
            // Add medicalRecordIc to medical_record
            ("medical_record", "medicalRecordIc", "VARCHAR"),
            // Add supplierIc to supplier
            ("supplier", "supplierIc", "VARCHAR"),
            // Add staffScheduleIc to staff_schedule
            ("staff_schedule", "staffScheduleIc", "VARCHAR"),
            // Add prescriptionIc to prescription
            ("prescription", "prescriptionIc", "VARCHAR"),
            // Add prescriptionOrderIc to prescription_order
            ("prescription_order", "prescriptionOrderIc", "VARCHAR"),
            // Add patientInfoIc to patient_info
            ("patient_info", "patientInfoIc", "VARCHAR"),
            // Add guardIc to guard
            ("guard", "guardIc", "VARCHAR"),
            // Add dashboardIc to dashboard
            ("dashboard", "dashboardIc", "VARCHAR"),
            // Add treatmentIc to treatment
            ("treatment", "treatmentIc", "VARCHAR"),
            // Add admissionIc to admission
            ("admission", "admissionIc", "VARCHAR"),
            // Add ambulanceIc to ambulance
            ("ambulance", "ambulanceIc", "INTEGER"),
            // Add cardIc to card
            ("card", "cardIc", "INTEGER")
        ];
        for (table, column, coltype) in queries.iter() {
            let sql = format!(
                "DO $$ BEGIN IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = '{}') AND NOT EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='{}' AND column_name='{}') THEN ALTER TABLE {} ADD COLUMN \"{}\" {}{}; END IF; END $$;",
                table,
                table,
                column,
                table,
                column,
                coltype,
                if *coltype == "INTEGER" { " UNIQUE" } else { "" }
            );
            manager.get_connection().execute_unprepared(&sql).await?;
        }
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let queries = [
            ("hospital", "hospitalIc"),
            ("department", "departmentIc"),
            ("staff", "staffIc"),
            ("patient", "patientIc"),
            ("room", "roomIc"),
            ("bed", "bedIc"),
            ("amenities", "amenitiesIc"),
            ("customers", "customerIc"),
            ("inventory", "inventoryIc"),
            ("appointment", "appointmentIc"),
            ("patient_doctor", "patientDoctorIc"),
            ("medical_record", "medicalRecordIc"),
            ("supplier", "supplierIc"),
            ("staff_schedule", "staffScheduleIc"),
            ("prescription", "prescriptionIc"),
            ("prescription_order", "prescriptionOrderIc"),
            ("patient_info", "patientInfoIc"),
            ("guard", "guardIc"),
            ("dashboard", "dashboardIc"),
            ("treatment", "treatmentIc"),
            ("admission", "admissionIc"),
            ("ambulance", "ambulanceIc"),
            ("card", "cardIc")
        ];
        for (table, column) in queries.iter() {
            let sql = format!(
                "DO $$ BEGIN IF EXISTS (SELECT 1 FROM information_schema.tables WHERE table_name = '{}') AND EXISTS (SELECT 1 FROM information_schema.columns WHERE table_name='{}' AND column_name='{}') THEN ALTER TABLE {} DROP COLUMN \"{}\"; END IF; END $$;",
                table, table, column, table, column
            );
            manager.get_connection().execute_unprepared(&sql).await?;
        }
        Ok(())
    }
}
