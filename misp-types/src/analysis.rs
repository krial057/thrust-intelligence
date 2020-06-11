#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[cfg(feature = "serde")]
use crate::serialization_helpers::number_embedded_in_string;

/// Represents the analysis level
/// [RFC](https://github.com/MISP/misp-rfc/blob/master/misp-core-format/raw.md#analysis))
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Analysis {
    Initial,
    Ongoing,
    Complete,
    Custom(u16),
}

impl From<u16> for Analysis {
    /// Creates a MISP Analysis type from a number
    fn from(analysis: u16) -> Analysis {
        match analysis {
            0 => Analysis::Initial,
            1 => Analysis::Ongoing,
            2 => Analysis::Complete,
            _ => Analysis::Custom(analysis),
        }
    }
}

impl From<&Analysis> for u16 {
    /// Converts a MISP Analysis type to a number.
    fn from(analysis: &Analysis) -> u16 {
        match analysis {
            Analysis::Initial => 0,
            Analysis::Ongoing => 1,
            Analysis::Complete => 2,
            Analysis::Custom(v) => *v,
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Analysis {
    /// Serializes the MISP Analysis type into a JSON string
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        number_embedded_in_string::serialize(u16::from(self), serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Analysis {
    /// Deserializes the MISP Analysis from a JSON string
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(number_embedded_in_string::deserialize::<u16, D>(deserializer)?.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::analysis::Analysis;
    #[test]
    #[cfg(feature = "serde")]
    pub fn json_to_analysis() {
        assert_eq!(Analysis::Initial, serde_json::from_str("\"0\"").unwrap());
        assert_eq!(Analysis::Ongoing, serde_json::from_str("\"1\"").unwrap());
        assert_eq!(Analysis::Complete, serde_json::from_str("\"2\"").unwrap());
        assert_eq!(
            Analysis::Custom(666),
            serde_json::from_str("\"666\"").unwrap()
        );
    }

    #[test]
    #[cfg(feature = "serde")]
    pub fn analysis_to_json() {
        assert_eq!("\"0\"", serde_json::to_string(&Analysis::Initial).unwrap());
        assert_eq!("\"1\"", serde_json::to_string(&Analysis::Ongoing).unwrap());
        assert_eq!("\"2\"", serde_json::to_string(&Analysis::Complete).unwrap());
        assert_eq!(
            "\"666\"",
            serde_json::to_string(&Analysis::Custom(666)).unwrap()
        );
    }
}
