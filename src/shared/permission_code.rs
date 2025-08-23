use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// PermissionCode enumerates all known permission codes used by the service.
/// These map to the `auth.permissions.code` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PermissionCode {
    #[serde(rename = "user.read")]         UserRead,
    #[serde(rename = "user.write")]        UserWrite,
    #[serde(rename = "session.read")]      SessionRead,
    #[serde(rename = "session.terminate")] SessionTerminate,
    #[serde(rename = "token.read")]        TokenRead,
    #[serde(rename = "token.revoke")]      TokenRevoke,
    #[serde(rename = "project.read")]      ProjectRead,
    #[serde(rename = "project.write")]     ProjectWrite,
    #[serde(rename = "project.delete")]    ProjectDelete,
    #[serde(rename = "appointment.create")] AppointmentCreate,
    #[serde(rename = "appointment.read")]   AppointmentRead,
    #[serde(rename = "appointment.update")] AppointmentUpdate,
}

impl PermissionCode {
    /// A static list of all permission codes.
    pub const ALL: [PermissionCode; 12] = [
        PermissionCode::UserRead,
        PermissionCode::UserWrite,
        PermissionCode::SessionRead,
        PermissionCode::SessionTerminate,
        PermissionCode::TokenRead,
        PermissionCode::TokenRevoke,
        PermissionCode::ProjectRead,
        PermissionCode::ProjectWrite,
        PermissionCode::ProjectDelete,
        PermissionCode::AppointmentCreate,
        PermissionCode::AppointmentRead,
        PermissionCode::AppointmentUpdate,
    ];

    pub fn as_str(&self) -> &'static str {
        match self {
            PermissionCode::UserRead => "user.read",
            PermissionCode::UserWrite => "user.write",
            PermissionCode::SessionRead => "session.read",
            PermissionCode::SessionTerminate => "session.terminate",
            PermissionCode::TokenRead => "token.read",
            PermissionCode::TokenRevoke => "token.revoke",
            PermissionCode::ProjectRead => "project.read",
            PermissionCode::ProjectWrite => "project.write",
            PermissionCode::ProjectDelete => "project.delete",
            PermissionCode::AppointmentCreate => "appointment.create",
            PermissionCode::AppointmentRead => "appointment.read",
            PermissionCode::AppointmentUpdate => "appointment.update",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "user.read" => Some(Self::UserRead),
            "user.write" => Some(Self::UserWrite),
            "session.read" => Some(Self::SessionRead),
            "session.terminate" => Some(Self::SessionTerminate),
            "token.read" => Some(Self::TokenRead),
            "token.revoke" => Some(Self::TokenRevoke),
            "project.read" => Some(Self::ProjectRead),
            "project.write" => Some(Self::ProjectWrite),
            "project.delete" => Some(Self::ProjectDelete),
            "appointment.create" => Some(Self::AppointmentCreate),
            "appointment.read" => Some(Self::AppointmentRead),
            "appointment.update" => Some(Self::AppointmentUpdate),
            _ => None,
        }
    }
}

impl Display for PermissionCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}