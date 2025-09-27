use crate::shared::{PermMarker, PermissionCode};

// markers
pub struct AppointmentReadPermission;
impl PermMarker for AppointmentReadPermission {
    fn code() -> &'static str {
        PermissionCode::AppointmentRead.as_str()
    }
}