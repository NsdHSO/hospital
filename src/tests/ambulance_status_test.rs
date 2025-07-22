#[cfg(test)]
/// Tests for ambulance status functionality, verifying both the AmbulanceStatusEnum 
/// and the /status API endpoint.
/// 
/// These tests ensure that:
/// 1. The AmbulanceStatusEnum contains exactly 18 status values as expected
/// 2. The /status endpoint successfully returns all 18 statuses
/// 3. The response format matches our API contract with correct value and label fields
/// 4. All expected status values are present in the API response
mod ambulance_status_tests {
    use crate::components;
    use crate::entity::sea_orm_active_enums::AmbulanceStatusEnum;
    use crate::tests::db_config;
    use actix_web::{App, test, web};
    use sea_orm::Iterable;
    use serde_json::Value;

    /// Tests that the AmbulanceStatusEnum has exactly 18 variants.
    /// 
    /// This test ensures that our enum definition contains all expected ambulance statuses
    /// and matches the number of statuses that should be returned by the API.
    /// It also prints all variants for debugging and verifies that all expected status
    /// names are present in the enum.
    #[test]
    async fn test_ambulance_status_enum_has_18_values() {
        // Count the number of variants in the AmbulanceStatusEnum
        let status_count = AmbulanceStatusEnum::iter().count();

        // We expect 18 status values based on the enum definition
        assert_eq!(
            status_count, 18,
            "Expected 18 status enum values, got {}",
            status_count
        );

        // Print the status values for debugging
        println!("Status values:");
        for status in AmbulanceStatusEnum::iter() {
            println!("  - {:?}", status);
        }

        // Verify that we have all expected statuses
        let expected_statuses = [
            "Available",
            "InService",
            "Maintenance",
            "Dispatched",
            "EnRouteToScene",
            "AtScene",
            "TransportingPatient",
            "EnRouteToHospital",
            "AtHospital",
            "ReturningToBase",
            "Unavailable",
            "OutOfService",
            "OnBreak",
            "Fueling",
            "Cleaning",
            "AwaitingDispatch",
            "PreparingForMission",
            "UnderRepair",
        ];

        // Count the statuses to make sure we have exactly 18
        assert_eq!(
            expected_statuses.len(),
            18,
            "Expected status list should have 18 items"
        );

        // Check that all expected statuses are in the enum
        for expected in expected_statuses.iter() {
            let found =
                AmbulanceStatusEnum::iter().any(|status| format!("{:?}", status) == *expected);
            assert!(found, "Expected status '{}' not found in enum", expected);
        }
    }
    /// Tests the /status API endpoint for ambulance statuses.
    /// 
    /// This integration test:
    /// 1. Sets up a real database connection
    /// 2. Configures an Actix-Web test server with the ambulance routes
    /// 3. Sends a GET request to the /status endpoint
    /// 4. Verifies the response is successful and properly formatted
    /// 5. Checks that exactly 18 statuses are returned
    /// 6. Validates that each status has both 'value' and 'label' fields
    /// 7. Confirms all expected statuses are present in the response
    /// 
    /// Note: This test requires a working database connection as the routes
    /// are configured to use the database, even though the actual method
    /// doesn't query the database (it uses enum iteration).
    #[actix_rt::test]
    async fn test_ambulance_statuses() {
        // Set up the database connection for testing
        // This is required because the Actix-Web route expects a database connection,
        // even though the service method doesn't actually query the database
        let db = db_config::setup_test_db().await;

        // Configure the app with the database connection and ambulance routes
        // This creates a test server instance with our API routes registered
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(db.clone())) // Provide the database connection to the app
                .configure(components::ambulance::init_routes), // Register ambulance routes
        )
        .await;

        // Send a GET request to the `/status` endpoint
        let req = test::TestRequest::get().uri("/ambulance/status").to_request();
        let resp = test::call_service(&app, req).await;

        // Check that the response is successful
        assert!(
            resp.status().is_success(),
            "Expected successful response, got {}",
            resp.status()
        );

        // Read and parse the response body
        let body = test::read_body(resp).await;
        let response: Value = serde_json::from_slice(&body).expect("Failed to parse JSON response");

        // Extract the statuses from the 'message' field of the response object
        let statuses = response["message"]
            .as_array()
            .expect("Expected 'message' field to be an array");

        // Print the entire response for debugging
        println!(
            "Full response: {}",
            serde_json::to_string_pretty(&response).unwrap()
        );

        // Assert the number of statuses
        assert_eq!(
            statuses.len(),
            AmbulanceStatusEnum::iter().count(),
            "Expected {} statuses, got {}",
            AmbulanceStatusEnum::iter().count(),
            statuses.len()
        );

        // Check if each status has 'value' and 'label' fields
        for (i, status) in statuses.iter().enumerate() {
            assert!(
                status.get("value").is_some(),
                "Status at index {} is missing 'value' field",
                i
            );
            assert!(
                status.get("label").is_some(),
                "Status at index {} is missing 'label' field",
                i
            );
            println!(
                "Status {}: value='{}', label='{}'",
                i,
                status["value"].as_str().unwrap_or("<not a string>"),
                status["label"].as_str().unwrap_or("<not a string>")
            );
        }

        // Verify that all expected statuses are present (in any order)
        let expected_statuses = [
            "AVAILABLE",
            "IN_SERVICE",
            "MAINTENANCE",
            "DISPATCHED",
            "EN_ROUTE_TO_SCENE",
            "AT_SCENE",
            "TRANSPORTING_PATIENT",
            "EN_ROUTE_TO_HOSPITAL",
            "AT_HOSPITAL",
            "RETURNING_TO_BASE",
            "UNAVAILABLE",
            "OUT_OF_SERVICE",
            "ON_BREAK",
            "FUELING",
            "CLEANING",
            "AWAITING_DISPATCH",
            "PREPARING_FOR_MISSION",
            "UNDER_REPAIR",
        ];

        let status_values: Vec<&str> = statuses
            .iter()
            .map(|s| s["value"].as_str().unwrap_or(""))
            .collect();

        // Print all status values for debugging
        println!("Found status values: {:?}", status_values);

        // Verify that we have the correct number of statuses
        assert_eq!(
            status_values.len(),
            18,
            "Expected 18 statuses, got {}",
            status_values.len()
        );

        // Check each expected status is present
        for expected in expected_statuses.iter() {
            let found = status_values.iter().any(|s| *s == *expected);
            assert!(
                found,
                "Expected status '{}' not found in response",
                expected
            );
        }
    }
}
