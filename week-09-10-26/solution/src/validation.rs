// Helper dùng trong library crate, không công khai cho binary crate.
pub(crate) fn non_blank(text: &str) -> bool {
    !text.trim().is_empty()
}
