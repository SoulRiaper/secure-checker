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
pub struct RemoveIndividual200Response {
    #[serde(rename = "op_id", skip_serializing_if = "Option::is_none")]
    pub op_id: Option<i32>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<i32>,
}

impl RemoveIndividual200Response {
    pub fn new() -> RemoveIndividual200Response {
        RemoveIndividual200Response {
            op_id: None,
            result: None,
        }
    }
}
