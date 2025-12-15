use chrono::{DateTime, Local, NaiveDate};

pub fn format_to_teliway_ws_datetimez(date: DateTime<Local>) -> String {
    date.format("%Y-%m-%dT%H:%M:%S.0%z").to_string()
}

pub fn format_to_teliway_ws_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}
