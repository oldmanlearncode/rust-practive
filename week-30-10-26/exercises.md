# Rust tuần 30-10-26 — Phân loại dữ liệu và báo cáo nhập lô

Phạm vi chương 1–9; chưa dùng trait/generic tự viết, lifetime tường minh, closure hoặc iterator adapters. Dùng for, match, Option, Result, struct/enum và collections. Không dùng unwrap/expect/panic trong implementation. Không cần dependency.

## Bài 1 — Tìm vị trí ký tự
Viết find_scalar(text: &str, target: char) -> Option<usize>. Trả vị trí Unicode scalar đầu tiên (bắt đầu 0), không phải offset byte. Không thấy/rỗng trả None. Ví dụ "Đỏ🦀Đ", 'Đ' → Some(0), '🦀' → Some(2). Dùng for trên chars(), biến đếm; không enumerate.

## Bài 2 — Phân loại dòng cấu hình
classify(line: &str) -> Result<Option<String>, String>. Trim hai đầu; dòng rỗng hoặc bắt đầu bằng # trả Ok(None). Các dòng còn lại phải đúng một trong "plex", "immich", "nextcloud", phân biệt hoa thường; hợp lệ trả Ok(Some(tên đã trim sở hữu)), còn lại Err nguyên dòng gốc. Ví dụ " plex " thành tên plex, "Plex" trả Err("Plex"). Không parse số, không đọc file. Gợi ý: match trên &str, không cần split.

## Bài 3 — Tóm tắt kết quả độc lập
count_services(names: &[String]) -> HashMap<String, usize>. Đếm nguyên văn tất cả tên, kể cả rỗng; không trim. Dùng for và entry/or_insert. Map trả về sở hữu key, độc lập nguồn. Lô rỗng → map rỗng; không yêu cầu thứ tự.

## 4 câu hỏi
1. Chỉ số scalar khác offset byte thế nào với "Đỏ🦀"? Dấu kết hợp ảnh hưởng ra sao?
2. Result<Option<String>, String> có ba nhóm kết quả gì? Ai sở hữu String trong từng nhóm?
3. Nếu gọi classify bằng ? trong hàm xử lý lô, có tiếp tục sau dòng lỗi không? Vì sao mini project cần match?
4. Vì sao thống kê trả map key String thay vì tham chiếu tới String cục bộ? Vẽ cây module và giải thích pub use.

## Mini project — Báo cáo nhập danh sách dịch vụ
Report có accepted Vec<String>, rejected Vec<Rejected>, ignored usize. Rejected chứa line usize và text String. import(lines: &[String]) -> Report gọi classify đúng một lần mỗi dòng, duyệt theo thứ tự, tiếp tục sau lỗi. Dòng được chấp nhận thêm tên đã trim; dòng bỏ qua tăng ignored; dòng sai lưu số dòng bắt đầu 1 và nguyên văn. Không sửa đầu vào. Báo cáo phải sở hữu dữ liệu, dùng được sau khi nguồn ra khỏi scope. Lô rỗng cho ba nhóm rỗng/0. Giữ thứ tự accepted và rejected; tên trùng được chấp nhận nhiều lần.

Getter accepted() -> &[String], rejected() -> &[Rejected], ignored() -> usize; Rejected::line/text; Report::counts() gọi bài 3 trên accepted, trả map độc lập. Ví dụ [" plex ", "# hi", "BAD", "immich", ""] cho accepted [plex,immich], rejected [(3,BAD)], ignored=2. Không có cập nhật một phần vào trạng thái bên ngoài: input luôn giữ nguyên. Tổ chức basics/report thành module private và re-export ở lib.rs; fields private.

## Chạy và nộp
```bash
cd week-30-10-26/solution
cargo check --all-targets
cargo run
```
Thay todo, giữ API/assert. Khung chạy sẽ dừng tới khi triển khai xong. Khi tất cả assert main pass có thể push để review; review vẫn xem ownership, thiết kế, độ rõ và biên còn thiếu. PENDING của CI chỉ xác nhận khung biên dịch. Không có lời giải trong bộ này.
