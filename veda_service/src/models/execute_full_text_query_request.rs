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
pub struct ExecuteFullTextQueryRequest {
    /// The unique identifier of the user's ticket.
    #[serde(rename = "ticket")]
    pub ticket: String,
    /// The full text query string.
    #[serde(rename = "query")]
    pub query: String,
    /// Optional parameter to specify sorting.
    #[serde(rename = "sort", skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    /// Optional parameter to specify databases.
    #[serde(rename = "databases", skip_serializing_if = "Option::is_none")]
    pub databases: Option<String>,
    /// Optional flag to reopen the query.
    #[serde(rename = "reopen", skip_serializing_if = "Option::is_none")]
    pub reopen: Option<bool>,
    /// Optional parameter to specify the starting point.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<i32>,
    /// Optional parameter to specify the top limit.
    #[serde(rename = "top", skip_serializing_if = "Option::is_none")]
    pub top: Option<i32>,
    /// Optional parameter to specify the limit on results.
    #[serde(rename = "limit", skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    /// Optional flag to enable tracing.
    #[serde(rename = "trace", skip_serializing_if = "Option::is_none")]
    pub trace: Option<bool>,
}

impl ExecuteFullTextQueryRequest {
    pub fn new(ticket: String, query: String) -> ExecuteFullTextQueryRequest {
        ExecuteFullTextQueryRequest {
            ticket,
            query,
            sort: None,
            databases: None,
            reopen: None,
            from: None,
            top: None,
            limit: None,
            trace: None,
        }
    }
}

