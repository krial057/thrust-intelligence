use crate::event::EventIdentifier;
use crate::object::ObjectIdentifier;
use chrono::{DateTime, Utc};
use core::fmt;
use uuid::Uuid;

#[cfg(feature = "serde")]
use super::serialization_helpers::{
    datetime_to_epoch, number_embedded_in_string, option_datetime_to_epoch,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "serde")]
use serde_json::Value;

#[derive(Debug, Copy, Clone)]
pub struct AttributeIdentifier(pub u64);

pub enum GenericAttributeIdentifier {
    Global(Uuid),
    Local(AttributeIdentifier),
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Attribute {
    id: AttributeIdentifier,
    event_id: EventIdentifier,
    object_id: ObjectIdentifier, // TODO Make it an Option?
    object_relation: Option<String>,
    category: String,
    #[serde(rename = "type")]
    kind: String,
    //value1: String,
    //value2: String,
    value: String,

    to_ids: bool,
    uuid: Uuid,
    #[serde(with = "datetime_to_epoch")]
    timestamp: DateTime<Utc>,
    #[serde(with = "number_embedded_in_string")]
    distribution: u16,
    #[serde(with = "number_embedded_in_string")]
    sharing_group_id: u64,
    comment: String,
    deleted: bool,
    disable_correlation: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttributeFull {
    #[serde(flatten)]
    attribute: Attribute,

    #[serde(rename = "Galaxy")]
    galaxies: Value,
    #[serde(rename = "ShadowAttribute")]
    shadow_attributes: Value,
    #[serde(with = "option_datetime_to_epoch")]
    first_seen: Option<DateTime<Utc>>,
    #[serde(with = "option_datetime_to_epoch")]
    last_seen: Option<DateTime<Utc>>,
}

impl Serialize for AttributeIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for AttributeIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        number_embedded_in_string::deserialize(deserializer).map(|v| AttributeIdentifier(v))
    }
}

impl fmt::Display for AttributeIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Attribute {
    pub fn id(&self) -> AttributeIdentifier {
        self.id
    }

    // Returns the organization that is currently handling the event
    pub fn event_identifer(&self) -> EventIdentifier {
        self.event_id
    }

    pub fn category(&self) -> &str {
        &self.category
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn object_relation(&self) -> Option<&str> {
        self.object_relation.as_ref().map(String::as_str)
    }
}

impl AttributeFull {
    pub fn category(&self) -> &str {
        self.attribute.category()
    }

    pub fn kind(&self) -> &str {
        self.attribute.kind()
    }
    pub fn value(&self) -> &str {
        self.attribute.value()
    }

    pub fn object_relation(&self) -> Option<&str> {
        self.attribute.object_relation()
    }
}
