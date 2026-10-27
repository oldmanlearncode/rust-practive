pub(crate) fn non_blank(text: &str) -> bool {
    !text.trim().is_empty()
}

pub(crate) fn non_blank_value(text: &str) -> Option<&str> {
    let trimmed_text = text.trim();
    if trimmed_text.is_empty() {
        None
    } else {
        Some(trimmed_text)
    }
}
