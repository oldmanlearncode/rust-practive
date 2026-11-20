use std::collections::HashMap;

pub enum RenameError {
    BlankName,
    Missing,
    Conflict,
}

pub fn longest_title(titles: &[String]) -> Option<&str> {
    todo!("Bài 1: mượn tiêu đề dài nhất theo scalar")
}

pub fn join_titles(titles: &[String]) -> String {
    todo!("Bài 2: ghép nguyên văn bằng newline")
}

pub fn rename_page(pages: &mut HashMap<String, String>, old: &str, new: String)
    -> Result<(), RenameError> {
    todo!("Bài 3: xác thực hết trước khi đổi key")
}
