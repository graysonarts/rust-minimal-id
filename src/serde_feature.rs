use std::fmt;

use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};

use crate::MinimalId;

struct MinimalIdVisitor;
impl<'de> Visitor<'de> for MinimalIdVisitor {
    type Value = MinimalId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("minimal id string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error,
              {
                  MinimalId::from_str(value).map_err(|_| E::custom(format!("unable to parse minimal id: {}", value)))
              }
}

impl Serialize for MinimalId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for MinimalId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>
    {
        deserializer.deserialize_str(MinimalIdVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXPECTED_ID : &'static str = "AAECAwQFBgcI";
    const EXPECTED_SERIALIZATION : &'static str = r#""AAECAwQFBgcI""#;

    #[test]
    fn can_deserialize() {
        let id: MinimalId = serde_json::from_str(EXPECTED_SERIALIZATION).unwrap();
        println!("{:#?}", id);
        assert_eq!(id.to_string(), EXPECTED_ID);
    }


    #[test]
    fn can_serialize() {
        let id = MinimalId::from_str(EXPECTED_ID).unwrap();
        let serialized = serde_json::to_string(&id).unwrap();

        assert_eq!(serialized, EXPECTED_SERIALIZATION);
    }
}
