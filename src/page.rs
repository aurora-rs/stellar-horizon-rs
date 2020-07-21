//! Pagination page.
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use serde::ser::Serialize;

#[derive(Debug, Clone)]
pub struct Page<T>
where
    T: DeserializeOwned + Serialize,
{
    pub records: Vec<T>,
}

impl<'de, T> Deserialize<'de> for Page<T>
where
    T: DeserializeOwned + Serialize,
{
    fn deserialize<D>(d: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner: Embedded<EmbeddedRecords<T>> = Embedded::deserialize(d)?;

        Ok(Page {
            records: inner.embedded.records,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Embedded<T> {
    #[serde(rename = "_embedded")]
    embedded: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddedRecords<T> {
    records: Vec<T>,
}
