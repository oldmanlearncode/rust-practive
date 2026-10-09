use std::collections::HashMap;
// Khi triển khai, dùng crate::validation::non_blank ở bài 4.

pub fn label_at(labels: &[String], index: usize) -> Option<&str> {
    todo!("Bài 1a: mượn nhãn qua get")
}

pub fn take_label(labels: &mut Vec<String>, index: usize) -> Option<String> {
    todo!("Bài 1b: lấy ownership của nhãn, không clone")
}

pub fn unicode_prefix(text: &str, limit: usize) -> String {
    todo!("Bài 2: lấy tối đa limit Unicode scalar values")
}

pub fn word_counts(text: &str) -> HashMap<String, usize> {
    todo!("Bài 3: đếm từ bằng entry/or_insert")
}

pub fn add_stock(stock: &mut HashMap<String, u32>, item: String, quantity: u32) -> bool {
    todo!("Bài 4: thêm tồn kho có giới hạn, lỗi không đổi map")
}
