use url::Url;
use uuid::Uuid;

pub struct ServerInfo {
    pub name: String,
    pub server_type: String,
    pub uuid: Uuid,
    pub vendor_url: Url,
    pub description: String,
    pub version: String,
}
