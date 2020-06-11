#[cfg(feature = "serde")]
use crate::serialization_helpers::number_embedded_in_string;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum ThreatLevel {
    High,
    Medium,
    Low,
    Undefined,
    Custom(u64),
}

impl From<u64> for ThreatLevel {
    /// Creates a MISP ThreadLevel type from a number
    fn from(thread_level: u64) -> ThreatLevel {
        match thread_level {
            1 => ThreatLevel::High,
            2 => ThreatLevel::Medium,
            3 => ThreatLevel::Low,
            4 => ThreatLevel::Undefined,
            _ => ThreatLevel::Custom(thread_level),
        }
    }
}

impl From<&ThreatLevel> for u64 {
    /// Converts a MISP Analysis type to a number.
    fn from(thread_level: &ThreatLevel) -> u64 {
        match thread_level {
            ThreatLevel::High => 1,
            ThreatLevel::Medium => 2,
            ThreatLevel::Low => 3,
            ThreatLevel::Undefined => 4,
            ThreatLevel::Custom(thread_level) => *thread_level,
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for ThreatLevel {
    /// Serializes the MISP ThreadLevel into a JSON string
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        number_embedded_in_string::serialize(u64::from(self), serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for ThreatLevel {
    /// Deserializes the MISP ThreadLevel from a JSON string
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(number_embedded_in_string::deserialize::<u64, D>(deserializer)?.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::threat_level::ThreatLevel;
    #[test]
    #[cfg(feature = "serde")]
    pub fn json_to_thread_level() {
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
    #[cfg(feature = "serde")]
    pub fn thread_level_to_json() {
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
