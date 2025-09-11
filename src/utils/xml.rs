pub fn extract_xml_tag<'a>(tag: &str, value: &'a str) -> Option<&'a str> {
    let begin = value.find(&format!("<{tag}"));
    let end = value.find(&format!("</{tag}>"));

    if let Some(begin) = begin && let Some(end) = end {
        let end = end + tag.len() + "</>".len();
        let part = &value[begin..end];

        Some(part)
    } else {
        None
    }
}
