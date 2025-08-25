use crate::shared::{PermMarker, PermissionCode};

// markers
pub struct AppointmentCreatePermission;
impl PermMarker for AppointmentCreatePermission {
    fn code() -> &'static str {
        PermissionCode::AppointmentCreate.as_str()
    }
}