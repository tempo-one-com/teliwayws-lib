use super::soap::response::ReclamationSoapResponse;

pub struct ReclamationWsResponse {
    pub id: i32,
}

impl From<ReclamationSoapResponse> for ReclamationWsResponse {
    fn from(value: ReclamationSoapResponse) -> Self {
        Self { id: value.id }
    }
}
