use std::collections::HashSet;

use serde::{Deserialize, Serialize};

use super::soap::response::{PositionEventMarkerSoapResponse, PositionTag};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct PositionEvent {
    pub recepisse: String,
    pub event_id: Option<i32>,
    pub label: String,
}

pub struct PositionEventMarkerWsResponse {
    pub items: Vec<PositionEvent>,
}

impl PositionEventMarkerWsResponse {
    pub fn get_messages_vec(&self) -> Vec<String> {
        self.items.iter().map(|x| x.label.clone()).collect()
    }

    pub fn get_messages_string(&self) -> String {
        let mut labels = self
            .items
            .iter()
            .map(|x| x.label.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();

        labels.sort();

        labels.join("\n")
    }
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
            event_id: value.event.map(|x| x.id),
            label: value.label,
        }
    }
}
