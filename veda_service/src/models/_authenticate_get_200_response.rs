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
pub struct AuthenticateGet200Response {
    /// The end time of the authentication session.
    #[serde(rename = "end_time", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// The unique identifier for the authenticated session.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    /// The result code for the authentication process.
    #[serde(rename = "result", skip_serializing_if = "Option::is_none")]
    pub result: Option<i32>,
    /// The URI for the authenticated user.
    #[serde(rename = "user_uri", skip_serializing_if = "Option::is_none")]
    pub user_uri: Option<String>,
}

impl AuthenticateGet200Response {
    pub fn new() -> AuthenticateGet200Response {
        AuthenticateGet200Response {
            end_time: None,
            id: None,
            result: None,
            user_uri: None,
        }
    }
}

