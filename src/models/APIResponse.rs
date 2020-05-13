use serde::{Deserialize, Serialize};

#[path = "Course.rs"]
mod course;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct APIResponse {
    pub page_size: u32,
    pub page_number: u32,
    pub total: u32,
    pub classes: Vec<course::Course>,
}
