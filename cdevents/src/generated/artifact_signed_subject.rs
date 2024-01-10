// code generated by cdevents/sdk-rust/generator (subject.hbs)
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactSignedSubject {
    
    #[serde(rename = "content")]
    pub content: serde_json::Map<String, serde_json::Value>,
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "type")]
    pub tpe: String,
}
