use super::attribute::AttributeFull;
use super::object::ObjectFull;
use super::organization::{OrganizationIdentifier, OrganizationTemporary};
use super::serialization_helpers::{
    date_to_mispdate, datetime_to_epoch, number_embedded_in_string,
};
use crate::model::distribution::Distribution;
use crate::model::threat_level::ThreatLevel;
use chrono::{Date, DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Copy, Clone)]
pub struct EventIdentifier(pub u64);

pub enum GenericEventIdentifier {
    Global(Uuid),
    Local(EventIdentifier),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    id: EventIdentifier,
    org_id: OrganizationIdentifier,
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
    orgc_id: OrganizationIdentifier,
    #[serde(with = "datetime_to_epoch")]
    timestamp: DateTime<Utc>,
    distribution: Distribution,
    #[serde(with = "number_embedded_in_string")]
    sharing_group_id: u64,
    proposal_email_lock: bool,
    locked: bool,
    threat_level_id: ThreatLevel,
    #[serde(with = "datetime_to_epoch")]
    publish_timestamp: DateTime<Utc>,
    //#[serde(with = "datetime_to_epoch")]
    //sighting_timestamp: DateTime<Utc>,
    disable_correlation: bool,
    extends_uuid: String, //TODO change to Option<Uuid>
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
    objects: Vec<ObjectFull>,
    #[serde(rename = "Tag")]
    tags: Value,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EventFullEmbedded {
    #[serde(rename = "Event")]
    pub event: EventFull,
}

impl Serialize for EventIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> Deserialize<'de> for EventIdentifier {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        number_embedded_in_string::deserialize(deserializer).map(|v| EventIdentifier(v))
    }
}

impl fmt::Display for EventIdentifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl GenericEventIdentifier {
    pub fn to_url_id(&self) -> String {
        match self {
            GenericEventIdentifier::Global(uuid) => uuid
                .to_hyphenated()
                .encode_lower(&mut Uuid::encode_buffer())
                .to_string(),
            GenericEventIdentifier::Local(v) => v.to_string(),
        }
    }
}

impl Into<GenericEventIdentifier> for u64 {
    fn into(self) -> GenericEventIdentifier {
        GenericEventIdentifier::Local(EventIdentifier(self))
    }
}

impl Into<GenericEventIdentifier> for Uuid {
    fn into(self) -> GenericEventIdentifier {
        GenericEventIdentifier::Global(self)
    }
}

impl Into<GenericEventIdentifier> for Event {
    fn into(self) -> GenericEventIdentifier {
        GenericEventIdentifier::Global(self.uuid)
    }
}

impl Into<GenericEventIdentifier> for EventFull {
    fn into(self) -> GenericEventIdentifier {
        GenericEventIdentifier::Global(self.event.uuid)
    }
}

impl Into<GenericEventIdentifier> for EventFullEmbedded {
    fn into(self) -> GenericEventIdentifier {
        GenericEventIdentifier::Global(self.event.event.uuid)
    }
}

impl Event {
    pub fn id(&self) -> EventIdentifier {
        self.id
    }

    // Returns the organization that is currently handling the event
    pub fn organization_identifer(&self) -> OrganizationIdentifier {
        self.org_id
    }

    pub fn date(&self) -> &Date<Utc> {
        &self.date
    }

    pub fn info(&self) -> &str {
        &self.info
    }

    pub fn uuid(&self) -> Uuid {
        self.uuid
    }

    pub fn published(&self) -> bool {
        self.published
    }

    pub fn analysis(&self) -> u16 {
        self.analysis
    }

    pub fn attribute_count(&self) -> u64 {
        self.attribute_count
    }

    // Returns the organization that initially created the event
    pub fn organization_creator_identifier(&self) -> OrganizationIdentifier {
        self.orgc_id
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        &self.timestamp
    }

    pub fn distribution(&self) -> &Distribution {
        &self.distribution
    }

    pub fn sharing_group(&self) -> u64 {
        // TODO Return Group Identifier instead of u64
        self.sharing_group_id
    }

    pub fn proposal_email_lock(&self) -> bool {
        self.proposal_email_lock
    }

    pub fn locked(&self) -> bool {
        self.locked
    }

    pub fn threat_level(&self) -> &ThreatLevel {
        &self.threat_level_id
    }

    pub fn publish_timestamp(&self) -> &DateTime<Utc> {
        &self.publish_timestamp
    }

    pub fn disable_correlation(&self) -> bool {
        self.disable_correlation
    }

    pub fn extends(&self) -> &String {
        //TODO replace with Option<EventIdentifier>
        &self.extends_uuid
    }
}

impl EventFull {
    pub fn attributes(&self) -> &Vec<AttributeFull> {
        &self.attributes
    }

    pub fn objects(&self) -> &Vec<ObjectFull> {
        &self.objects
    }

    //
    // The following functions are copied from Event. They just call the embedded event methods.
    // This is needed for easier access, so that you can use EventFull.date() instead of
    // EventFull.event.date()
    //

    pub fn id(&self) -> EventIdentifier {
        self.event.id()
    }

    // Returns the organization that is currently handling the event
    pub fn organization_identifer(&self) -> OrganizationIdentifier {
        self.event.organization_identifer()
    }

    pub fn date(&self) -> &Date<Utc> {
        self.event.date()
    }

    pub fn info(&self) -> &str {
        self.event.info()
    }

    pub fn uuid(&self) -> Uuid {
        self.event.uuid()
    }

    pub fn published(&self) -> bool {
        self.event.published()
    }

    pub fn analysis(&self) -> u16 {
        self.event.analysis()
    }

    pub fn attribute_count(&self) -> u64 {
        self.event.attribute_count()
    }

    // Returns the organization that initially created the event
    pub fn organization_creator_identifier(&self) -> OrganizationIdentifier {
        self.event.organization_creator_identifier()
    }

    pub fn timestamp(&self) -> &DateTime<Utc> {
        self.event.timestamp()
    }

    pub fn distribution(&self) -> &Distribution {
        self.event.distribution()
    }

    pub fn sharing_group(&self) -> u64 {
        // TODO Return Group Identifier instead of u64
        self.event.sharing_group()
    }

    pub fn proposal_email_lock(&self) -> bool {
        self.event.proposal_email_lock()
    }

    pub fn locked(&self) -> bool {
        self.event.locked()
    }

    pub fn threat_level(&self) -> &ThreatLevel {
        self.event.threat_level()
    }

    pub fn publish_timestamp(&self) -> &DateTime<Utc> {
        self.event.publish_timestamp()
    }

    pub fn disable_correlation(&self) -> bool {
        self.event.disable_correlation()
    }

    pub fn extends(&self) -> &String {
        //TODO replace with Option<EventIdentifier>
        self.event.extends()
    }
}
