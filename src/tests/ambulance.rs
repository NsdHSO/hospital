#[cfg(test)]
mod ambulance {
    use crate::entity::sea_orm_active_enums::{AmbulanceStatusEnum, AmbulanceTypeEnum};

    use crate::components::ambulance::generate_payload_to_create_ambulance;
    use crate::entity::ambulance::AmbulancePayload;

    #[tokio::test]
    async fn test_ambulance_with_hospital_name() {
        let payload = AmbulancePayload {
            ambulance_ic: None, // Will use generate_ic()
            vehicleNumber: None,
            make: Some("Toyota".to_string()),
            year: Some(2023),
            capacity: Some(4),
            mission: Some("Emergency response".to_string()),
            passengers: None,
            driver_name: Some("Test Driver".to_string()),
            driver_license: Some("DL-12345".to_string()),
            hospital_name: None,
            last_service_date: None,
            next_service_date: None,
            mileage: Some(1000),
            fuel_type: Some("Gasoline".to_string()),
            registration_number: Some("REG-12345".to_string()),
            insurance_provider: Some("Test Insurance".to_string()),
            insurance_expiry_date: None,
            notes: Some("Test notes".to_string()),
            car_details_year: Some(2023),
            car_details_color: Some("White".to_string()),
            car_details_isambulance: Some(true),
            car_details_licenseplate: Some("AMB-123".to_string()),
            car_details_mileage: Some(1000.0),
            location_latitude: None,
            location_longitude: None,
            r#type: Some(AmbulanceTypeEnum::BasicLifeSupport),
            status: Some(AmbulanceStatusEnum::Available),
            car_details_make: None,
            car_details_model: None,
        };

        let active_model = generate_payload_to_create_ambulance(Some(payload.clone()));

        assert_eq!(active_model.year.unwrap(), Some(2023));
    }
}
