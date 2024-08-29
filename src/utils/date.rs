use chrono::{DateTime, Local};

pub fn format_to_teliway_ws_datetimez(date: DateTime<Local>) -> String {
    date.format("%Y-%m-%dT%H:%M:%S.0%z").to_string()
}
