pub mod date_to_mispdate {
    use chrono::{Date, NaiveDate, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Date<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Date<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(Date::<Utc>::from_utc(
            NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)?,
            Utc,
        ))
    }
}

pub mod option_date_to_mispdate {
    use chrono::{Date, NaiveDate, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &Option<Date<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => serializer.serialize_str(&format!("{}", date.format(FORMAT))),
            None => serializer.serialize_none(), //TODO check if this works
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Date<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp: Option<&str> = Option::deserialize(deserializer)?;
        match temp {
            Some(str) => Ok(Some(Date::<Utc>::from_utc(
                NaiveDate::parse_from_str(str, "%Y-%m-%d").map_err(serde::de::Error::custom)?,
                Utc,
            ))),
            None => Ok(None),
        }
    }
}

pub mod datetime_to_epoch {
    use super::number_embedded_in_string;
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        number_embedded_in_string::serialize(date.timestamp(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = number_embedded_in_string::deserialize::<i64, D>(deserializer)?;
        Ok(Utc.timestamp(v, 0))
    }
}

pub mod option_datetime_to_epoch {
    use super::number_embedded_in_string;
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(option: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match option {
            Some(date) => number_embedded_in_string::serialize(date.timestamp(), serializer),
            None => serializer.serialize_none(), //TODO check if this works
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let temp: Option<&str> = Option::deserialize(deserializer)?;
        match temp {
            Some(str) => {
                let v = str
                    .parse::<i64>()
                    .map_err(|_| serde::de::Error::custom("Wrong timestamp format in JSON"))?;
                Ok(Some(Utc.timestamp(v, 0)))
            }
            None => Ok(None),
        }
    }
}

pub mod number_embedded_in_string {
    use serde::{self, Deserialize, Deserializer, Serializer};
    use std::str::FromStr;

    pub fn serialize<S, T>(number: T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: ToString,
    {
        serializer.serialize_str(&number.to_string())
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: FromStr,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|_| serde::de::Error::custom("Expected json number embedded in string"))
    }
}
