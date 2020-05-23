use crate::model::serialization_helpers::number_embedded_in_string;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ThreatLevel {
    High,
    Medium,
    Low,
    Undefined,
    Custom(u64),
}

impl Serialize for ThreatLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v = match *self {
            ThreatLevel::High => 1,
            ThreatLevel::Medium => 2,
            ThreatLevel::Low => 3,
            ThreatLevel::Undefined => 4,
            ThreatLevel::Custom(i) => i,
        };
        number_embedded_in_string::serialize(v, serializer)
    }
}

impl<'de> Deserialize<'de> for ThreatLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        number_embedded_in_string::deserialize(deserializer).map(|v| match v {
            1 => ThreatLevel::High,
            2 => ThreatLevel::Medium,
            3 => ThreatLevel::Low,
            4 => ThreatLevel::Undefined,
            _ => ThreatLevel::Custom(v),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::model::threat_level::ThreatLevel;
    #[test]
    pub fn value_to_thread_level() {
        assert_eq!(ThreatLevel::High, serde_json::from_str("\"1\"").unwrap());
        assert_eq!(ThreatLevel::Medium, serde_json::from_str("\"2\"").unwrap());
        assert_eq!(ThreatLevel::Low, serde_json::from_str("\"3\"").unwrap());
        assert_eq!(
            ThreatLevel::Undefined,
            serde_json::from_str("\"4\"").unwrap()
        );
        assert_eq!(
            ThreatLevel::Custom(666),
            serde_json::from_str("\"666\"").unwrap()
        );
    }

    #[test]
    pub fn thread_level_to_value() {
        assert_eq!("\"1\"", serde_json::to_string(&ThreatLevel::High).unwrap());
        assert_eq!(
            "\"2\"",
            serde_json::to_string(&ThreatLevel::Medium).unwrap()
        );
        assert_eq!("\"3\"", serde_json::to_string(&ThreatLevel::Low).unwrap());
        assert_eq!(
            "\"4\"",
            serde_json::to_string(&ThreatLevel::Undefined).unwrap()
        );
        assert_eq!(
            "\"666\"",
            serde_json::to_string(&ThreatLevel::Custom(666)).unwrap()
        );
    }
}
