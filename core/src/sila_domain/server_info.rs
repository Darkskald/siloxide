use url::Url;
use uuid::Uuid;
use crate::sila_domain::common::{Identifier, VersionWithPatch};

pub struct ServerInfo {
    pub name: Identifier,
    pub server_type: Identifier,
    pub uuid: Uuid,
    pub vendor_url: Url,
    pub description: String,
    pub version: VersionWithPatch,
}

