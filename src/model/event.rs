use super::attribute::AttributeFull;
use super::organization::OrganizationTemporary;
use crate::util::{date_to_mispdate, datetime_to_epoch, number_embedded_in_string};
use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

#[derive(Debug)]
pub enum EventIdentifier {
    Global(Uuid),
    Local(u64),
}

impl fmt::Display for EventIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl EventIdentifier {
    pub fn to_url_id(&self) -> String {
        match self {
            EventIdentifier::Global(uuid) => uuid
                .to_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string(),
            EventIdentifier::Local(v) => v.to_string(),
        }
    }
}

impl Into<EventIdentifier> for u64 {
    fn into(self) -> EventIdentifier {
        EventIdentifier::Local(self)
    }
}

impl Into<EventIdentifier> for Uuid {
    fn into(self) -> EventIdentifier {
        EventIdentifier::Global(self)
    }
}

impl Into<EventIdentifier> for Event {
    fn into(self) -> EventIdentifier {
        EventIdentifier::Global(self.uuid)
    }
}

impl Into<EventIdentifier> for EventFull {
    fn into(self) -> EventIdentifier {
        EventIdentifier::Global(self.event.uuid)
    }
}

impl Into<EventIdentifier> for EventFullEmbedded {
    fn into(self) -> EventIdentifier {
        EventIdentifier::Global(self.event.event.uuid)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    //Direct members
    #[serde(with = "number_embedded_in_string")]
    id: u64,
    #[serde(with = "number_embedded_in_string")]
    org_id: u64,
    #[serde(with = "date_to_mispdate")]
    date: Date<Utc>,
    info: String,
    //#[serde(with = "number_embedded_in_string")]
    //user_id: u64,
    uuid: Uuid,
    published: bool,
    #[serde(with = "number_embedded_in_string")]
    analysis: u16,
    #[serde(with = "number_embedded_in_string")]
    attribute_count: u64,
    #[serde(with = "number_embedded_in_string")]
    orgc_id: u64,
    #[serde(with = "datetime_to_epoch")]
    timestamp: DateTime<Utc>,
    #[serde(with = "number_embedded_in_string")]
    distribution: u16,
    #[serde(with = "number_embedded_in_string")]
    sharing_group_id: u64,
    proposal_email_lock: bool,
    locked: bool,
    #[serde(with = "number_embedded_in_string")]
    threat_level_id: u64,
    #[serde(with = "datetime_to_epoch")]
    publish_timestamp: DateTime<Utc>,
    //#[serde(with = "datetime_to_epoch")]
    //sighting_timestamp: DateTime<Utc>,
    disable_correlation: bool,
    extends_uuid: String,
    //Indirect members:
    //TODO
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EventFull {
    #[serde(flatten)]
    event: Event,

    #[serde(rename = "Org")]
    org: OrganizationTemporary,
    #[serde(rename = "Orgc")]
    orgc: OrganizationTemporary,
    #[serde(rename = "Attribute")]
    attributes: Vec<AttributeFull>,
    #[serde(rename = "ShadowAttribute")]
    shadow_attributes: Value,
    #[serde(rename = "RelatedEvent")]
    related_events: Value,
    #[serde(rename = "Galaxy")]
    galaxies: Value,
    #[serde(rename = "Object")]
    objects: Value,
    #[serde(rename = "Tag")]
    tags: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventFullEmbedded {
    #[serde(rename = "Event")]
    pub event: EventFull,
}

impl Event {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn info(&self) -> &str {
        &self.info
    }
}

impl EventFull {
    pub fn id(&self) -> u64 {
        self.event.id()
    }

    pub fn uuid(&self) -> Uuid {
        self.event.uuid()
    }

    pub fn info(&self) -> &str {
        self.event.info()
    }
}
