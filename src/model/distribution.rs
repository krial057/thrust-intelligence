use crate::model::serialization_helpers::number_embedded_in_string;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Distribution represents the basic distribution rules of the event.
/// The system must adhere to the distribution setting for access control and for dissemination
/// of the event.
/// [RFC](https://github.com/MISP/misp-rfc/blob/master/misp-core-format/raw.md#distribution)
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Distribution {
    YourOrganizationOnly,
    ThisCommunityOnly,
    ConnectedCommunities,
    AllCommunities,
    SharingGroup,
}

impl Serialize for Distribution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let v: u16 = match *self {
            Distribution::YourOrganizationOnly => 0,
            Distribution::ThisCommunityOnly => 1,
            Distribution::ConnectedCommunities => 2,
            Distribution::AllCommunities => 3,
            Distribution::SharingGroup => 4,
        };
        number_embedded_in_string::serialize(v, serializer)
    }
}

impl<'de> Deserialize<'de> for Distribution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = number_embedded_in_string::deserialize::<u16, D>(deserializer)?;
        match v {
            0 => Ok(Distribution::YourOrganizationOnly),
            1 => Ok(Distribution::ThisCommunityOnly),
            2 => Ok(Distribution::ConnectedCommunities),
            3 => Ok(Distribution::AllCommunities),
            4 => Ok(Distribution::SharingGroup),
            _ => Err(serde::de::Error::custom(
                "Invalid distribution value. Distribution value must be between 0 and 4!",
            ))?,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::distribution::Distribution;
    #[test]
    pub fn value_to_thread_level() {
        assert_eq!(
            Distribution::YourOrganizationOnly,
            serde_json::from_str("\"0\"").unwrap()
        );
        assert_eq!(
            Distribution::ThisCommunityOnly,
            serde_json::from_str("\"1\"").unwrap()
        );
        assert_eq!(
            Distribution::ConnectedCommunities,
            serde_json::from_str("\"2\"").unwrap()
        );
        assert_eq!(
            Distribution::AllCommunities,
            serde_json::from_str("\"3\"").unwrap()
        );
        assert_eq!(
            Distribution::SharingGroup,
            serde_json::from_str("\"4\"").unwrap()
        );
    }

    #[test]
    pub fn thread_level_to_value() {
        assert_eq!(
            "\"0\"",
            serde_json::to_string(&Distribution::YourOrganizationOnly).unwrap()
        );
        assert_eq!(
            "\"1\"",
            serde_json::to_string(&Distribution::ThisCommunityOnly).unwrap()
        );
        assert_eq!(
            "\"2\"",
            serde_json::to_string(&Distribution::ConnectedCommunities).unwrap()
        );
        assert_eq!(
            "\"3\"",
            serde_json::to_string(&Distribution::AllCommunities).unwrap()
        );
        assert_eq!(
            "\"4\"",
            serde_json::to_string(&Distribution::SharingGroup).unwrap()
        );
    }
}
