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

/// NodeTopology : node topology

/// node topology
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct NodeTopology {
    /// exclusive labels
    #[serde(rename = "exclusion")]
    pub exclusion: Vec<String>,
    /// inclusive labels
    #[serde(rename = "inclusion")]
    pub inclusion: Vec<String>,
}

impl NodeTopology {
    /// NodeTopology using only the required fields
    pub fn new(exclusion: impl IntoVec<String>, inclusion: impl IntoVec<String>) -> NodeTopology {
        NodeTopology {
            exclusion: exclusion.into_vec(),
            inclusion: inclusion.into_vec(),
        }
    }
    /// NodeTopology using all fields
    pub fn new_all(
        exclusion: impl IntoVec<String>,
        inclusion: impl IntoVec<String>,
    ) -> NodeTopology {
        NodeTopology {
            exclusion: exclusion.into_vec(),
            inclusion: inclusion.into_vec(),
        }
    }
}