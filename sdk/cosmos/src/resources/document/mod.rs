//! Utilities for interacting with [`Document`]s.

mod document_attributes;
mod indexing_directive;
mod query;

pub use document_attributes::DocumentAttributes;
pub use indexing_directive::IndexingDirective;
pub use query::Query;

use super::Resource;
use crate::CosmosError;

use http::header::HeaderMap;
use serde::de::DeserializeOwned;

/// User-defined content in JSON format.
///
/// You can learn more about Documents [here](https://docs.microsoft.com/en-us/rest/api/cosmos-db/documents).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Document<T> {
    #[serde(flatten)]
    pub document_attributes: DocumentAttributes,
    #[serde(flatten)]
    pub document: T, // raw, id not included
}

impl<T> Document<T> {
    pub fn new(document: T) -> Self {
        let document_attributes = DocumentAttributes::default();

        Self {
            document_attributes,
            document,
        }
    }
}

impl<T> std::convert::TryFrom<(&HeaderMap, &[u8])> for Document<T>
where
    T: DeserializeOwned,
{
    type Error = CosmosError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let _headers = value.0;
        let body = value.1;

        Ok(serde_json::from_slice(body)?)
    }
}

impl<T> Resource for Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}

impl<T> Resource for &Document<T> {
    fn uri(&self) -> &str {
        self.document_attributes._self()
    }
}