/*
 * Veda Platform HTTP Interface
 *
 * API for interacting with the Veda platform services.
 *
 * The version of the OpenAPI document: 1.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMembership200ResponseRdfTypeInner {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl GetMembership200ResponseRdfTypeInner {
    pub fn new() -> GetMembership200ResponseRdfTypeInner {
        GetMembership200ResponseRdfTypeInner {
            data: None,
            r#type: None,
        }
    }
}
