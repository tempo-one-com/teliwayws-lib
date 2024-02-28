use chrono::{DateTime, Local};

pub mod auth;
pub mod error;
pub mod position;
pub mod soap_ws;
pub mod tiers;

fn format_to_teliway_ws_datetimez(date: DateTime<Local>) -> String {
    date.format("%Y-%m-%dT%H:%M:%S.0%z").to_string()
}

fn extract_xml_tag<'a>(tag: &str, value: &'a str) -> Option<&'a str> {
    let begin = value.find(&format!("<{tag}"));
    let end = value.find(&format!("</{tag}>"));

    if begin.is_none() || end.is_none() {
        None
    } else {
        let end = end.unwrap() + tag.len() + "</>".len();
        let part = &value[begin.unwrap()..end];

        Some(part)    
    }
}