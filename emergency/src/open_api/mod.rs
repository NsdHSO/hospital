use utoipa::OpenApi;
use utoipa::openapi::{self, HttpMethod, Response};

// Import necessary types for schema components
use crate::emergency::NewEmergencyRequest;
use crate::schema::guard::name;

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

/// Initialize the OpenAPI documentation with manually defined paths
pub fn init() -> utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();

    // Ambulance endpoints - simple approach
    let ambulance_get_op = openapi::path::OperationBuilder::new()
        .operation_id(Some("list_ambulances"))
        .tag("ambulance")
        .response("200", Response::new("List of ambulances retrieved successfully"))
        .response("500", Response::new("Internal Server Error"))
        .build();

    doc.paths.paths.insert(
        "/v1/ambulance".to_string(),
        openapi::PathItem::new(HttpMethod::Get, ambulance_get_op)
    );

    // Emergency endpoints - find by ID
    let emergency_get_op = openapi::path::OperationBuilder::new()
        .operation_id(Some("find_emergency"))
        .tag("emergency")
        .response("200", Response::new("Emergency found successfully"))
        .response("404", Response::new("Emergency not found"))
        .response("500", Response::new("Internal Server Error"))
        .build();

    doc.paths.paths.insert(
        "/v1/emergency/{id}".to_string(),
        openapi::PathItem::new(HttpMethod::Get, emergency_get_op)
    );

    // Emergency endpoints - Get all emergencies
    let emergency_get_all_op = openapi::path::OperationBuilder::new()
        .operation_id(Some("list_emergencies"))
        .tag("emergency")
        .response("200", Response::new("List of emergencies retrieved successfully"))
        .response("500", Response::new("Internal Server Error"))
        .build();

    // First, add the GET operation
    doc.paths.paths.insert(
        "/v1/emergency".to_string(),
        openapi::PathItem::new(HttpMethod::Get, emergency_get_all_op)
    );

    // Emergency endpoints - Create emergency
    let emergency_post_op = openapi::path::OperationBuilder::new()
        .operation_id(Some("create_emergency"))
        .tag("emergency")
        .response("201", Response::new("Emergency created successfully"))
        .response("400", Response::new("Bad request"))
        .response("500", Response::new("Internal Server Error"))
        .build();

    // We need to merge the POST operation to the existing path
    // First, check if the path already exists
    if let Some(existing_path) = doc.paths.paths.get_mut("/v1/emergency") {
        // Add the POST operation to the existing path
    } else {
        // If the path doesn't exist yet (which shouldn't happen), create it
        doc.paths.paths.insert(
            "/v1/emergency".to_string(),
            openapi::PathItem::new(HttpMethod::Post, emergency_post_op)
        );
    }

    doc
}