use chrono::{DateTime, Utc};
use std::fmt;
use uuid::Uuid;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "serde")]
use super::serialization_helpers::{datetime_to_epoch, number_embedded_in_string};

#[derive(Debug, Copy, Clone)]
pub struct OrganizationIdentifier(pub u64);

#[derive(Deserialize, Debug, Clone)]
pub enum GenericOrganizationIdentifier {
    Global(Uuid),
    Local(u64),
    Named(String),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OrganizationTemporary {
    id: OrganizationIdentifier,
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

impl Serialize for OrganizationIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for OrganizationIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        number_embedded_in_string::deserialize(deserializer).map(|v| OrganizationIdentifier(v))
    }
}

impl fmt::Display for OrganizationIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for GenericOrganizationIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_url_id())
    }
}

impl GenericOrganizationIdentifier {
    pub fn to_url_id(&self) -> String {
        match self {
            GenericOrganizationIdentifier::Global(uuid) => uuid
                .to_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string(),
            GenericOrganizationIdentifier::Local(v) => v.to_string(),
            GenericOrganizationIdentifier::Named(s) => s.clone(),
        }
    }
}

impl Into<GenericOrganizationIdentifier> for u64 {
    fn into(self) -> GenericOrganizationIdentifier {
        GenericOrganizationIdentifier::Local(self)
    }
}

impl Into<GenericOrganizationIdentifier> for Uuid {
    fn into(self) -> GenericOrganizationIdentifier {
        GenericOrganizationIdentifier::Global(self)
    }
}

impl Into<GenericOrganizationIdentifier> for OrganizationTemporary {
    fn into(self) -> GenericOrganizationIdentifier {
        GenericOrganizationIdentifier::Global(self.uuid)
    }
}

impl Into<GenericOrganizationIdentifier> for Organization {
    fn into(self) -> GenericOrganizationIdentifier {
        GenericOrganizationIdentifier::Global(self.organization.uuid)
    }
}

impl Into<GenericOrganizationIdentifier> for &str {
    fn into(self) -> GenericOrganizationIdentifier {
        GenericOrganizationIdentifier::Named(self.to_string())
    }
}
