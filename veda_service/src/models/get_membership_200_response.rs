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
pub struct GetMembership200Response {
    #[serde(rename = "@", skip_serializing_if = "Option::is_none")]
    pub at: Option<String>,
    #[serde(rename = "rdf:type", skip_serializing_if = "Option::is_none")]
    pub rdf_colon_type: Option<Vec<models::GetMembership200ResponseRdfTypeInner>>,
    #[serde(rename = "v-s:memberOf", skip_serializing_if = "Option::is_none")]
    pub v_s_colon_member_of: Option<Vec<models::GetMembership200ResponseRdfTypeInner>>,
    #[serde(rename = "v-s:resource", skip_serializing_if = "Option::is_none")]
    pub v_s_colon_resource: Option<Vec<models::GetMembership200ResponseRdfTypeInner>>,
}

impl GetMembership200Response {
    pub fn new() -> GetMembership200Response {
        GetMembership200Response {
            at: None,
            rdf_colon_type: None,
            v_s_colon_member_of: None,
            v_s_colon_resource: None,
        }
    }
}

