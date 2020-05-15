use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerInfo {
    pub version: String,
    perm_sync: bool,
    perm_sighting: bool,
}
