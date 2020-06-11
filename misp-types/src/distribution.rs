#[cfg(feature = "serde")]
use crate::serialization_helpers::number_embedded_in_string;

#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Distribution represents the basic distribution rules of the event.
/// The system must adhere to the distribution setting for access control and for dissemination
/// of the event.
/// [RFC](https://github.com/MISP/misp-rfc/blob/master/misp-core-format/raw.md#distribution)
// TODO: In the future, changing SharingGroup to SharingGroup(SharingGroupIdentifier) would
// guarantee having a sharing_group_id...
#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Distribution {
    YourOrganizationOnly,
    ThisCommunityOnly,
    ConnectedCommunities,
    AllCommunities,
    SharingGroup,
    Unsupported(u16),
}

impl From<u16> for Distribution {
    /// Creates a MISP Distribution type from a number
    fn from(distribution: u16) -> Distribution {
        match distribution {
            0 => Distribution::YourOrganizationOnly,
            1 => Distribution::ThisCommunityOnly,
            2 => Distribution::ConnectedCommunities,
            3 => Distribution::AllCommunities,
            4 => Distribution::SharingGroup,
            _ => Distribution::Unsupported(distribution),
        }
    }
}

impl From<&Distribution> for u16 {
    /// Converts a MISP Distribution type to a number.
    fn from(distribution: &Distribution) -> u16 {
        match distribution {
            Distribution::YourOrganizationOnly => 0,
            Distribution::ThisCommunityOnly => 1,
            Distribution::ConnectedCommunities => 2,
            Distribution::AllCommunities => 3,
            Distribution::SharingGroup => 4,
            Distribution::Unsupported(v) => *v,
        }
    }
}

#[cfg(feature = "serde")]
impl Serialize for Distribution {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        number_embedded_in_string::serialize(u16::from(self), serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Distribution {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(number_embedded_in_string::deserialize::<u16, D>(deserializer)?.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::distribution::Distribution;
    #[test]
    pub fn value_to_distribution() {
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
    pub fn distribution_to_value() {
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
