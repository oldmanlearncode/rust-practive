use std::io;

pub fn parse_amount(text: &str) -> Result<u32, String> {
    todo!("Bài 1: phân tích và xác thực số tiền")
}

pub fn borrow_name(names: &[String], index: usize) -> Result<&str, String> {
    todo!("Bài 2: mượn tên hợp lệ bằng get")
}

pub fn parse_batch(lines: &[String]) -> Result<Vec<u32>, String> {
    todo!("Bài 3: gọi parse_amount và truyền lỗi bằng ?")
}

pub fn read_note(path: &str) -> Result<String, io::Error> {
    todo!("Bài 4: đọc UTF-8, truyền io::Error bằng ?, trim nội dung")
}
