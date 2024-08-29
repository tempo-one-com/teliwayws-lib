use chrono::{DateTime, Local};

#[derive(Debug, Clone)]
pub struct PositionEventMarkerWsRequest {
    pub position_ids: Vec<i32>,
    pub event_code: String,
    pub date: DateTime<Local>,
    pub created_by: String,
    pub agence_code: String,
}
