//! Pagination page.
use crate::link::Link;
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use serde::ser::{Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Page<T>
where
    T: DeserializeOwned + Serialize + Clone,
{
    pub links: Option<PageLinks>,
    pub records: Vec<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageLinks {
    #[serde(rename = "self")]
    pub self_: Link,
    pub next: Link,
    #[serde(rename = "prev")]
    pub previous: Link,
}

impl<'de, T> Deserialize<'de> for Page<T>
where
    T: DeserializeOwned + Serialize + Clone,
{
    fn deserialize<D>(d: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner: Embedded<EmbeddedRecords<T>> = Embedded::deserialize(d)?;

        Ok(Page {
            links: inner.links,
            records: inner.embedded.records,
        })
    }
}

impl<T> Serialize for Page<T>
where
    T: DeserializeOwned + Serialize + Clone,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let inner = Embedded {
            links: self.links.clone(),
            embedded: EmbeddedRecords {
                records: self.records.clone(),
            },
        };

        inner.serialize(serializer)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Embedded<T> {
    #[serde(rename = "_links")]
    links: Option<PageLinks>,
    #[serde(rename = "_embedded")]
    embedded: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct EmbeddedRecords<T> {
    records: Vec<T>,
}
