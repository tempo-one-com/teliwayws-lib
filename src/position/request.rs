use chrono::{DateTime, Local};

#[derive(Debug, Clone, Default)]
pub struct PositionEventMarkerWsRequest {
    pub position_ids: Vec<i32>,
    pub event_code: String,
    pub datetime: DateTime<Local>,
    pub created_by: String,
    pub agence_code: String,
    pub date_rdv: Option<DateTime<Local>>,
    pub info_palette_rendu: Option<String>,
}
