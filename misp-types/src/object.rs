use crate::attribute::AttributeFull;
use crate::event::EventIdentifier;
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
pub struct ObjectIdentifier(pub u64);

pub enum GenericEventIdentifier {
    Global(Uuid),
    Local(ObjectIdentifier),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Object {
    id: ObjectIdentifier,
    name: String,
    #[serde(rename = "meta-category")]
    meta_category: String,
    description: String,
    template_uuid: String, //Todo change to Option<Uuid>
    #[serde(with = "number_embedded_in_string")]
    template_version: u64,
    event_id: EventIdentifier,
    uuid: Uuid,
    #[serde(with = "datetime_to_epoch")]
    timestamp: DateTime<Utc>,
    #[serde(with = "number_embedded_in_string")]
    distribution: u16,
    #[serde(with = "number_embedded_in_string")]
    sharing_group_id: u64,
    comment: String,
    deleted: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ObjectFull {
    #[serde(flatten)]
    object: Object,

    #[serde(with = "option_datetime_to_epoch")]
    first_seen: Option<DateTime<Utc>>,
    #[serde(with = "option_datetime_to_epoch")]
    last_seen: Option<DateTime<Utc>>,
    #[serde(rename = "ObjectReference")]
    object_reference: Value,
    #[serde(rename = "Attribute")]
    attributes: Vec<AttributeFull>,
}

impl Serialize for ObjectIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for ObjectIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        number_embedded_in_string::deserialize(deserializer).map(|v| ObjectIdentifier(v))
    }
}

impl fmt::Display for ObjectIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Object {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }
}

impl ObjectFull {
    pub fn attributes(&self) -> &Vec<AttributeFull> {
        &self.attributes
    }

    /// A shortcut function to access an attribute of an object that has a specific object_relation.
    pub fn attribute(&self, str: impl AsRef<str>) -> Option<&AttributeFull> {
        self.attributes()
            .iter()
            .find(|a| a.object_relation() == Some(str.as_ref()))
    }

    //
    // The following functions are copied from Object. They just call the embedded object methods.
    // This is needed for easier access, so that you can use ObjectFull.name() instead of
    // ObjectFull.object.date()
    //
    pub fn name(&self) -> &str {
        self.object.name()
    }

    pub fn description(&self) -> &str {
        self.object.description()
    }

    pub fn comment(&self) -> &str {
        self.object.comment()
    }
}
