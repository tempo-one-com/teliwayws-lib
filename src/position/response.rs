use serde::{Deserialize, Serialize};

use super::soap::response::{PositionEventMarkerSoapResponse, PositionTag};

pub struct PositionEventMarkerWsResponse {
    pub items: Vec<PositionEvent>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PositionEvent {
    pub recepisse: String,
    pub event_id: i32,
}

impl From<PositionEventMarkerSoapResponse> for PositionEventMarkerWsResponse {
    fn from(value: PositionEventMarkerSoapResponse) -> Self {
        let items = value.items.into_iter().map(PositionEvent::from).collect();

        Self { items }
    }
}

impl From<PositionTag> for PositionEvent {
    fn from(value: PositionTag) -> Self {
        PositionEvent {
            recepisse: value.get_recepisse().unwrap_or_default(),
            event_id: value.event.id,
        }
    }
}
