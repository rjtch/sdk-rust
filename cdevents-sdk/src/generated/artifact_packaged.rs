// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "change",)]
    pub change: Change,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Change {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none", with = "crate::serde::uri_reference_optional",)]
    pub source: Option<fluent_uri::Uri<String>>,
}

