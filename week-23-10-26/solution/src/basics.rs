use std::collections::HashMap;

pub enum TakeError { Missing, Blank }
pub enum InsertError { InvalidKey, Duplicate }

pub fn validate_title(text: &str) -> Result<(), String> {
    todo!("Bài 1: title không trắng, tối đa 20 Unicode scalar values")
}

pub fn take_document(docs: &mut Vec<String>, index: usize) -> Result<String, TakeError> {
    todo!("Bài 2: chỉ lấy ownership sau khi kiểm tra đủ")
}

pub fn insert_unique(map: &mut HashMap<String, String>, key: String, value: String)
    -> Result<(), InsertError> {
    todo!("Bài 3: key hợp lệ và chưa tồn tại; không ghi đè")
}
