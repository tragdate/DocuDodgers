use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Root {
    pub created: Option<String>,
    #[serde(rename = "executionPlan")]
    pub execution_plan: Value,
    #[serde(rename = "executionTime")]
    pub execution_time: i64,
    #[serde(rename = "firstRun")]
    pub first_run: String,
    #[serde(rename = "fromCache")]
    pub from_cache: bool,
    #[serde(rename = "maxResults")]
    pub max_results: i64,
    pub messages: String,
    #[serde(rename = "parentId")]
    pub parent_id: Value,
    #[serde(rename = "queryId")]
    pub query_id: i64,
    #[serde(rename = "querySetId")]
    pub query_set_id: i64,
    #[serde(rename = "resultSets")]
    pub result_sets: Vec<ResultSet>,
    #[serde(rename = "revisionId")]
    pub revision_id: i64,
    #[serde(rename = "siteId")]
    pub site_id: i64,
    #[serde(rename = "siteName")]
    pub site_name: String,
    pub slug: Option<String>,
    #[serde(rename = "targetSites")]
    pub target_sites: i64,
    #[serde(rename = "textOnly")]
    pub text_only: bool,
    #[serde(rename = "totalResults")]
    pub total_results: i64,
    pub truncated: bool,
    pub url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResultSet {
    pub columns: Vec<Column>,
    #[serde(rename = "messagePosition")]
    pub message_position: i64,
    pub rows: Vec<(i64, i64, String, i64)>,
    pub truncated: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}
