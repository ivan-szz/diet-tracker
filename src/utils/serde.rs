use serde::de::{self, Visitor};
use std::fmt;

const INVALID_NUMBER_MESSAGE: &str = "Inserisci un numero di calorie valido";

fn i32_from_i64<E: de::Error>(value: i64) -> Result<i32, E> {
    i32::try_from(value).map_err(|_| E::custom(INVALID_NUMBER_MESSAGE))
}

fn i32_from_u64<E: de::Error>(value: u64) -> Result<i32, E> {
    i32::try_from(value).map_err(|_| E::custom(INVALID_NUMBER_MESSAGE))
}

fn i32_from_str<E: de::Error>(value: &str) -> Result<i32, E> {
    value
        .trim()
        .parse()
        .map_err(|_| E::custom(INVALID_NUMBER_MESSAGE))
}

struct NumberFromStringVisitor;

impl<'de> Visitor<'de> for NumberFromStringVisitor {
    type Value = i32;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a whole number or a numeric string")
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<i32, E> {
        i32_from_i64(value)
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<i32, E> {
        i32_from_u64(value)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<i32, E> {
        i32_from_str(value)
    }
}

/// Accepts a JSON number (server-to-server calls) or a numeric string (HTML
/// form fields, which always submit text), reporting a friendly message
/// instead of a raw type-mismatch error.
pub fn number_from_string<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: de::Deserializer<'de>,
{
    deserializer.deserialize_any(NumberFromStringVisitor)
}

struct OptionalNumberFromStringVisitor;

impl<'de> Visitor<'de> for OptionalNumberFromStringVisitor {
    type Value = Option<i32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a whole number, a numeric string, or nothing")
    }

    fn visit_i64<E: de::Error>(self, value: i64) -> Result<Self::Value, E> {
        i32_from_i64(value).map(Some)
    }

    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Self::Value, E> {
        i32_from_u64(value).map(Some)
    }

    fn visit_str<E: de::Error>(self, value: &str) -> Result<Self::Value, E> {
        if value.trim().is_empty() {
            return Ok(None);
        }
        i32_from_str(value).map(Some)
    }

    fn visit_none<E: de::Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_any(self)
    }
}

/// Same as [`number_from_string`], but a missing, null, or blank value
/// becomes `None` instead of an error.
pub fn optional_number_from_string<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: de::Deserializer<'de>,
{
    deserializer.deserialize_option(OptionalNumberFromStringVisitor)
}
