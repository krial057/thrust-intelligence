use crate::util::{datetime_to_epoch, number_embedded_in_string, option_datetime_to_epoch};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    #[serde(with = "number_embedded_in_string")]
    id: u64,
    #[serde(with = "number_embedded_in_string")]
    event_id: u64,
    #[serde(with = "number_embedded_in_string")]
    object_id: u64,
    object_relation: Option<String>,
    category: String,
    #[serde(rename = "type")]
    kind: String,
    //value1: String,
    //value2: String,
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
