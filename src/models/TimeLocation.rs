use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeLocation {
    pub begin_time: Option<String>,
    pub building: Option<String>,
    pub days: Option<String>,
    pub end_time: Option<String>,
    pub room: Option<String>,
    pub room_capacity: Option<u32>,
}
