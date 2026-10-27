use std::io;

use crate::validation::non_blank_value;

pub fn parse_amount(text: &str) -> Result<u32, String> {
    let trimmed_text = text.trim();
    if trimmed_text.is_empty() {
        return Err("empty".to_string());
    }
    match trimmed_text.parse::<u32>() {
        Err(_) => Err("not_u32".to_string()),
        Ok(value) => {
            if (1..=10_000).contains(&value) {
                Ok(value)
            } else {
                Err("out_of_range".to_string())
            }
        }
    }
}

pub fn borrow_name(names: &[String], index: usize) -> Result<&str, String> {
    match names.get(index) {
        None => Err("missing".to_string()),
        Some(name) => match non_blank_value(name) {
            None => Err("blank".to_string()),
            Some(value) => Ok(value),
        },
    }
}

pub fn parse_batch(lines: &[String]) -> Result<Vec<u32>, String> {
    todo!("Bài 3: gọi parse_amount và truyền lỗi bằng ?")
}

pub fn read_note(path: &str) -> Result<String, io::Error> {
    todo!("Bài 4: đọc UTF-8, truyền io::Error bằng ?, trim nội dung")
}
