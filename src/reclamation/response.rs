use super::soap::response::ReclamationSoapResponse;

#[derive(Debug, Clone)]
pub struct ReclamationWsResponse {
    pub id: i32,
}

impl From<ReclamationSoapResponse> for ReclamationWsResponse {
    fn from(value: ReclamationSoapResponse) -> Self {
        Self { id: value.id }
    }
}
