use crate::http_response::Claims;
use crate::shared::permissions::perm_marker::PermMarker;
use crate::shared::PermissionCode;
use std::marker::PhantomData;


// Generic extractor that enforces the permission and gives you Claims back
pub struct Require<M: PermMarker> {
    pub claims: Claims,
    pub _m: PhantomData<M>,
}
