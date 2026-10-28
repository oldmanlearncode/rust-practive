use std::collections::HashMap;

pub enum TakeError {
    Missing,
    Blank,
}
pub enum InsertError {
    InvalidKey,
    Duplicate,
}

pub fn validate_title(text: &str) -> Result<(), String> {
    let trimmed_text = text.trim();
    if trimmed_text.is_empty() {
        return Err("blank".to_string());
    }
    let mut count: u32 = 0;
    for _ in text.chars() {
        count += 1;
    }
    if count > 20 {
        return Err("too_long".to_string());
    }
    Ok(())
}

pub fn take_document(docs: &mut Vec<String>, index: usize) -> Result<String, TakeError> {
    match docs.get(index) {
        None => Err(TakeError::Missing),
        Some(doc) => {
            if doc.trim().is_empty() {
                Err(TakeError::Blank)
            } else {
                Ok(docs.remove(index))
            }
        }
    }
}

pub fn insert_unique(
    map: &mut HashMap<String, String>,
    key: String,
    value: String,
) -> Result<(), InsertError> {
    if key.trim().is_empty() {
        return Err(InsertError::InvalidKey);
    }
    if map.contains_key(&key) {
        return Err(InsertError::Duplicate);
    }
    map.insert(key, value);
    Ok(())
}
