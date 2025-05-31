use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let queries = [
            // Add hospitalIc to hospital
            ("hospital", "hospital_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add departmentIc to department
            ("department", "department_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add staffIc to staff
            ("staff", "staff_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add patientIc to patient
            ("patient", "patient_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add roomIc to room
            ("room", "room_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add bedIc to bed
            ("bed", "bed_ic", "INTEGER NOT NULL UNIQUE"),
            // Add amenitiesIc to amenities
            ("amenities", "amenities_ic", "INTEGER NOT NULL UNIQUE"),
            // Add customerIc to customers
            ("customers", "customer_ic", "INTEGER NOT NULL UNIQUE"),
            // Add inventoryIc to inventory
            ("inventory", "inventory_ic", "INTEGER NOT NULL UNIQUE"),
            // Add appointmentIc to appointment
            ("appointment", "appointment_ic", "INTEGER NOT NULL UNIQUE"),
            // Add patientDoctorIc to patient_doctor
            ("patient_doctor", "patient_doctor_ic", "INTEGER NOT NULL UNIQUE"),
            // Add medicalRecordIc to medical_record
            ("medical_record", "medical_record_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add supplierIc to supplier
            ("supplier", "supplier_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add staffScheduleIc to staff_schedule
            ("staff_schedule", "staff_schedule_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add prescriptionIc to prescription
            ("prescription", "prescription_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add prescriptionOrderIc to prescription_order
            ("prescription_order", "prescription_order_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add patientInfoIc to patient_info
            ("patient_info", "patient_info_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add guardIc to guard
            ("guard", "guard_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add dashboardIc to dashboard
            ("dashboard", "dashboard_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add treatmentIc to treatment
            ("treatment", "treatment_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add admissionIc to admission
            ("admission", "admission_ic", "VARCHAR NOT NULL UNIQUE"),
            // Add ambulanceIc to ambulance
            ("ambulance", "ambulance_ic", "INTEGER NOT NULL UNIQUE"),
            // Add cardIc to card
            ("card", "card_ic", "INTEGER NOT NULL UNIQUE"),
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
            ("hospital", "hospital_ic"),
            ("department", "department_ic"),
            ("staff", "staff_ic"),
            ("patient", "patient_ic"),
            ("room", "room_ic"),
            ("bed", "bed_ic"),
            ("amenities", "amenities_ic"),
            ("customers", "customer_ic"),
            ("inventory", "inventory_ic"),
            ("appointment", "appointment_ic"),
            ("patient_doctor", "patient_doctor_ic"),
            ("medical_record", "medical_record_ic"),
            ("supplier", "supplier_ic"),
            ("staff_schedule", "staff_schedule_ic"),
            ("prescription", "prescription_ic"),
            ("prescription_order", "prescription_order_ic"),
            ("patient_info", "patient_info_ic"),
            ("guard", "guard_ic"),
            ("dashboard", "dashboard_ic"),
            ("treatment", "treatment_ic"),
            ("admission", "admission_ic"),
            ("ambulance", "ambulance_ic"),
            ("card", "card_ic"),
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
