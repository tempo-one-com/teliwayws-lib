pub fn extract_xml_tag<'a>(tag: &str, value: &'a str) -> Option<&'a str> {
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
