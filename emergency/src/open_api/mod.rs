use utoipa::openapi::{self, HttpMethod, Response};
use utoipa::OpenApi;

// Import necessary types for schema components
use crate::emergency::NewEmergencyRequest;

/// API Documentation
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Emergency Service API",
        version = "1.0.0",
        description = "API for managing emergencies and ambulances"
    ),
    components(
        schemas(NewEmergencyRequest)
    ),
    tags(
        (name = "ambulance", description = "Ambulance management endpoints"),
        (name = "emergency", description = "Emergency management endpoints")
    ),
    paths()
)]
pub struct ApiDoc;
// TODO: Add here every api what you added

pub fn init() -> utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();

    // Ambulance paths
    doc.paths.paths.insert(
        "/v1/ambulance".to_string(),
        openapi::PathItem::new(
            HttpMethod::Get,
            openapi::path::OperationBuilder::new()
                .operation_id(Some("list_ambulances"))
                .tag("ambulance")
                .response(
                    "200",
                    Response::new("List of ambulances retrieved successfully"),
                )
                .response("500", Response::new("Internal Server Error"))
                .build(),
        ),
    );

    // Emergency paths - Find by ID
    doc.paths.paths.insert(
        "/v1/emergency/{id}".to_string(),
        openapi::PathItem::new(
            HttpMethod::Get,
            openapi::path::OperationBuilder::new()
                .operation_id(Some("find_emergency"))
                .tag("emergency")
                .response("200", Response::new("Emergency found successfully"))
                .response("404", Response::new("Emergency not found"))
                .response("500", Response::new("Internal Server Error"))
                .build(),
        ),
    );

    // Emergency paths - Find all
    doc.paths.paths.insert(
        "/v1/emergency".to_string(),
        openapi::PathItem::new(
            HttpMethod::Get,
            openapi::path::OperationBuilder::new()
                .operation_id(Some("list_emergencies"))
                .tag("emergency")
                .response(
                    "200",
                    Response::new("List of emergencies retrieved successfully"),
                )
                .response("500", Response::new("Internal Server Error"))
                .build(),
        ),
    );

    // Add the POST path for Emergency - Create
    let post_path_item = openapi::PathItem::new(
        HttpMethod::Post,
        openapi::path::OperationBuilder::new()
            .operation_id(Some("create_emergency"))
            .tag("emergency")
            .response("201", Response::new("Emergency created successfully"))
            .response("400", Response::new("Bad request"))
            .response("500", Response::new("Internal Server Error"))
            .build(),
    );

    doc
}
