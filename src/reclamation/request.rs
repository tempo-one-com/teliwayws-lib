#[derive(Debug, Clone)]
pub struct ReclamationCreateWsRequest {
    pub position_id: i32,
    pub number: i32,
    pub r#type: String,
    pub motif: String,
    pub author: String,
}
