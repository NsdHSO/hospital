use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "ambulance", description = "Ambulance management endpoints"),
    )
)]
pub struct ApiDoc;