use crate::util::{datetime_to_epoch, number_embedded_in_string};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, Serializer};
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone)]
pub enum OrganizationIdentifier {
    Global(Uuid),
    Local(u64),
    Named(String),
}

impl Serialize for OrganizationIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_url_id())
    }
}

impl OrganizationIdentifier {
    pub fn to_url_id(&self) -> String {
        match self {
            OrganizationIdentifier::Global(uuid) => uuid
                .to_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string(),
            OrganizationIdentifier::Local(v) => v.to_string(),
            OrganizationIdentifier::Named(s) => s.clone(),
        }
    }
}

impl Into<OrganizationIdentifier> for u64 {
    fn into(self) -> OrganizationIdentifier {
        OrganizationIdentifier::Local(self)
    }
}

impl Into<OrganizationIdentifier> for Uuid {
    fn into(self) -> OrganizationIdentifier {
        OrganizationIdentifier::Global(self)
    }
}

impl Into<OrganizationIdentifier> for OrganizationTemporary {
    fn into(self) -> OrganizationIdentifier {
        OrganizationIdentifier::Global(self.uuid)
    }
}

impl Into<OrganizationIdentifier> for Organization {
    fn into(self) -> OrganizationIdentifier {
        OrganizationIdentifier::Global(self.organization.uuid)
    }
}

impl Into<OrganizationIdentifier> for &str {
    fn into(self) -> OrganizationIdentifier {
        OrganizationIdentifier::Named(self.to_string())
    }
}

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
