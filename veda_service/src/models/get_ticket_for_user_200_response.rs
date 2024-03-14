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
pub struct GetTicketForUser200Response {
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<i32>,
    #[serde(rename = "user_uri", skip_serializing_if = "Option::is_none")]
    pub user_uri: Option<String>,
}

impl GetTicketForUser200Response {
    pub fn new() -> GetTicketForUser200Response {
        GetTicketForUser200Response {
            end_time: None,
            id: None,
            result: None,
            user_uri: None,
        }
    }
}

