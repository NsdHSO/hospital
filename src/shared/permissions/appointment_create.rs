use crate::http_response::Claims;
use crate::shared::permissions::perm_marker::PermMarker;
use crate::shared::PermissionCode;
use std::marker::PhantomData;

// markers
pub struct AppointmentCreatePermission;
impl PermMarker for AppointmentCreatePermission {
    fn code() -> &'static str {
        PermissionCode::AppointmentCreate.as_str()
    }
}

// Generic extractor that enforces the permission and gives you Claims back
pub struct Require<M: PermMarker> {
    pub claims: Claims,
    pub _m: PhantomData<M>,
}
