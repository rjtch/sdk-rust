// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "environment",)]
    pub environment: Environment,
    #[serde(rename = "outcome",)]
    pub outcome: String,
    #[serde(rename = "reason", default, skip_serializing_if = "Option::is_none",)]
    pub reason: Option<String>,
    #[serde(rename = "severity", default, skip_serializing_if = "Option::is_none",)]
    pub severity: Option<String>,
    #[serde(rename = "testSuite", default, skip_serializing_if = "Option::is_none",)]
    pub test_suite: Option<TestSuite>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestSuite {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none",)]
    pub name: Option<String>,
    #[serde(rename = "uri", default, skip_serializing_if = "Option::is_none", with = "crate::serde::uri_optional",)]
    pub uri: Option<fluent_uri::Uri<String>>,
    #[serde(rename = "version", default, skip_serializing_if = "Option::is_none",)]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Environment {
    #[serde(rename = "id",)]
    pub id: String,
    #[serde(rename = "source", default, skip_serializing_if = "Option::is_none", with = "crate::serde::uri_reference_optional",)]
    pub source: Option<fluent_uri::Uri<String>>,
}

