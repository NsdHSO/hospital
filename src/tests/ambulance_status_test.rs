#[cfg(test)]
mod ambulance_status_tests {
    use sea_orm::Iterable;
    use crate::entity::sea_orm_active_enums::AmbulanceStatusEnum;
    
    // This test verifies that the AmbulanceStatusEnum has 18 variants
    // which matches the number of statuses that should be returned by the API
    #[test]
    fn test_ambulance_status_enum_has_18_values() {
        // Count the number of variants in the AmbulanceStatusEnum
        let status_count = AmbulanceStatusEnum::iter().count();
        
        // We expect 18 status values based on the enum definition
        assert_eq!(status_count, 18, "Expected 18 status enum values, got {}", status_count);
        
        // Print the status values for debugging
        println!("Status values:");
        for status in AmbulanceStatusEnum::iter() {
            println!("  - {:?}", status);
        }
        
        // Verify that we have all expected statuses
        let expected_statuses = [
            "Available", "InService", "Maintenance", "Dispatched", 
            "EnRouteToScene", "AtScene", "TransportingPatient", "EnRouteToHospital",
            "AtHospital", "ReturningToBase", "Unavailable", "OutOfService",
            "OnBreak", "Fueling", "Cleaning", "AwaitingDispatch",
            "PreparingForMission", "UnderRepair"
        ];
        
        // Count the statuses to make sure we have exactly 18
        assert_eq!(expected_statuses.len(), 18, "Expected status list should have 18 items");
        
        // Check that all expected statuses are in the enum
        for expected in expected_statuses.iter() {
            let found = AmbulanceStatusEnum::iter().any(|status| {
                format!("{:?}", status) == *expected
            });
            assert!(found, "Expected status '{}' not found in enum", expected);
        }
    }
}
