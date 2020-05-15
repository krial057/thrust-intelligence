use super::{datetime_to_epoch, number_embedded_in_string};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationTemporary {
    #[serde(with = "number_embedded_in_string")]
    id: u64,
    name: String,
    uuid: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Organization {
    #[serde(flatten)]
    organization: OrganizationTemporary,

    #[serde(with = "datetime_to_epoch")]
    date_crated: DateTime<Utc>,
    #[serde(with = "datetime_to_epoch")]
    date_modified: DateTime<Utc>,
    description: String,
    nationality: String,
    sector: String,
    #[serde(with = "number_embedded_in_string")]
    created_by: u64,
    contacts: String,
    local: bool,
    restricted_to_domain: String,
    landingpage: String,
}
