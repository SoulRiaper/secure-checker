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
pub struct ExecuteFullTextQuery400Response {
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<Vec<String>>,
    #[serde(rename = "count", skip_serializing_if = "Option::is_none")]
    pub count: Option<i32>,
    #[serde(rename = "estimated", skip_serializing_if = "Option::is_none")]
    pub estimated: Option<i32>,
    #[serde(rename = "processed", skip_serializing_if = "Option::is_none")]
    pub processed: Option<i32>,
    #[serde(rename = "cursor", skip_serializing_if = "Option::is_none")]
    pub cursor: Option<i32>,
    #[serde(rename = "result_code", skip_serializing_if = "Option::is_none")]
    pub result_code: Option<i32>,
}

impl ExecuteFullTextQuery400Response {
    pub fn new() -> ExecuteFullTextQuery400Response {
        ExecuteFullTextQuery400Response {
            result: None,
            count: None,
            estimated: None,
            processed: None,
            cursor: None,
            result_code: None,
        }
    }
}

