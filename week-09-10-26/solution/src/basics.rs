use crate::validation::non_blank;
use std::collections::HashMap;
// Khi triển khai, dùng crate::validation::non_blank ở bài 4.

pub fn label_at(labels: &[String], index: usize) -> Option<&str> {
    labels.get(index).map(|label| label.as_str())
}

pub fn take_label(labels: &mut Vec<String>, index: usize) -> Option<String> {
    if index >= labels.len() {
        None
    } else {
        Some(labels.remove(index))
    }
}

pub fn unicode_prefix(text: &str, limit: usize) -> String {
    let mut result = "".to_string();
    for (index, letter) in text.chars().enumerate() {
        if index >= limit {
            break;
        }
        result.push(letter);
    }
    result
}

pub fn word_counts(text: &str) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();
    for word in text.split_whitespace() {
        let value = result.entry(word.to_string()).or_insert(0);
        *value += 1;
    }
    result
}

pub fn add_stock(stock: &mut HashMap<String, u32>, item: String, quantity: u32) -> bool {
    println!("quantity: {}", quantity);
    let is_valid_item = non_blank(&item);
    let is_valid_quantity = quantity > 0 && quantity <= 100;
    if !is_valid_item || !is_valid_quantity {
        return false;
    }
    let curent_quantity = stock.entry(item).or_insert(0);
    let new_quantity = quantity + *curent_quantity;
    if new_quantity > 100 {
        return false;
    }
    *curent_quantity += quantity;
    true
}
