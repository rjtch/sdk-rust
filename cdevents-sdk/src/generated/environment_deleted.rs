// @generated
// by cdevents/sdk-rust/generator (subject.hbs)

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Content {
    #[serde(rename = "name", default, skip_serializing_if = "Option::is_none",)]
    pub name: Option<String>,
}

