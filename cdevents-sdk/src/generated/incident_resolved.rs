// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "artifactId", default, skip_serializing_if = "Option::is_none",)]
    pub artifact_id: Option<String>,
    #[serde(rename = "description", default, skip_serializing_if = "Option::is_none",)]
    pub description: Option<String>,
    #[serde(rename = "environment",)]
    pub environment: Environment,
    #[serde(rename = "service", default, skip_serializing_if = "Option::is_none",)]
    pub service: Option<Service>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Service {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none", with = "crate::serde::uri_reference_optional",)]
    pub source: Option<fluent_uri::Uri<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none", with = "crate::serde::uri_reference_optional",)]
    pub source: Option<fluent_uri::Uri<String>>,
}

