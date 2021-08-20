#![allow(
    clippy::too_many_arguments,
    clippy::new_without_default,
    non_camel_case_types,
    unused_imports
)]
/*
 * Mayastor RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::IntoVec;

/// WatchCallback : Watch Callbacks

/// Watch Callbacks
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum WatchCallback {
    #[serde(rename = "uri")]
    uri(String),
}