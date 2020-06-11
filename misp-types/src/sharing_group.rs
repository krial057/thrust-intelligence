use crate::model::serialization_helpers::number_embedded_in_string;
use serde::{Deserialize, Deserializer, Serialize, Serializer};


#[derive(Debug, Copy, Clone)]
pub struct SharingGroupIdentifier(pub u64);

// TODO
