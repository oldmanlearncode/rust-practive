# Nhận xét bài tập và lịch sử sửa — tuần 25-09-26

## Phạm vi đề

- Người học đã học The Rust Book đến chương 5. Đề đầu đưa `Vec<String>` ở bài 2 và `Vec<Book>` trong mini project, vượt tiến độ (vector ở chương 8).
- Đã bỏ bài vector; thay mini project bằng `WeeklyReading` dùng `[u32; 7]`. Chưa yêu cầu `enum`, `match` hoặc lifetime tường minh.

## Bài 1 — `rotate_left_once`

**Bản nộp đầu:** Đúng. Lưu phần tử đầu, dịch trái rồi đưa phần tử đầu về cuối; `len() <= 1` xử lý mảng rỗng/một phần tử và bảo vệ `len() - 1`. `i32` là `Copy`. Thời gian O(n), bộ nhớ phụ O(1). Code trong `src/main.rs`.

## Bài 3 — `ReadingGoal`

| Lần nộp | Điểm cần sửa / kết quả |
| --- | --- |
| Đầu | `read()` dùng `pages + self.reading_pages` có nguy cơ tràn; `self.reading_pages = pages` ghi đè tiến độ thay vì cộng. `new()` nhận tiến độ vượt mục tiêu khiến `remaining()` không còn an toàn. |
| Sửa 1 | `new()` đặt tiến độ vượt mục tiêu về 0; `read()` dùng `saturating_add()`, rồi từ chối khi kết quả bằng `u32::MAX`. Điều này từ chối nhầm kết quả hợp lệ đúng bằng `u32::MAX`. |
| Sửa 2 | Kiểm tra `pages == 0 || pages > self.remaining()` trước `self.read_pages += pages`. Đúng cả biên `u32::MAX`, và khi thất bại trạng thái không đổi. |
| Cuối | Thêm `title(&self) -> &str` và `reading_rate(&self) -> String`. Khi nộp, gọi nhầm `format(...)`; bản code tổng hợp đã sửa thành `format!(...)` để biên dịch. |

**Quy tắc constructor do người học chọn:** Tiến độ đầu vào vượt mục tiêu được đặt về 0. Mục tiêu 0 được chấp nhận và không thể đọc thêm. Quy tắc này cần được hiểu bởi người gọi.

**Tên method:** `reading_rate()` hiện trả dạng `25/100` (tiến độ), không phải tỷ lệ phần trăm; `reading_progress()` sát nghĩa hơn nhưng không bắt buộc.

## Bài 4 và câu hỏi struct update

- Đã giải thích đúng xung đột `&str` với `&mut ReadingGoal` nếu tham chiếu cũ được dùng sau `rename`. Việc in `title()` trực tiếp tạo tham chiếu tạm, kết thúc sau lệnh in.
- Trả lời đúng `title: String` bị move bởi struct update, còn field `u32` có thể dùng riêng; không thể gọi method mượn toàn bộ struct sau partial move.
- Chưa nộp code `rename(goal: &mut ReadingGoal, new_title: String)`; vì thế phần triển khai bài 4 chưa được xác nhận.

## Mini project — `WeeklyReading`

| Lần nộp | Điểm cần sửa / kết quả |
| --- | --- |
| Đầu | Kiểm tra `day > 6` đúng; chỉ cập nhật mảng sau `book.read()` thành công, nên lượt bị từ chối không đổi trạng thái. Tuy nhiên `daily_pages[day] = pages` làm mất các lượt đọc trước trong cùng ngày. Báo cáo thiếu tổng đã đọc/mục tiêu. |
| Sửa 1 | Đổi thành `day >= self.daily_pages.len()` và `daily_pages[day] += pages`; sửa tên biến `recorded`, nhãn `Weekly diary`. Vẫn thiếu tổng đã đọc/mục tiêu. |
| Sửa 2 | Thêm `ReadingGoal::reading_rate()` và dòng `Reading rate` vào báo cáo; chỉ còn lỗi cú pháp `format` thiếu dấu `!`, đã sửa trong code tổng hợp. |

**Bất biến:** Khi bắt đầu từ 0 và chỉ ghi qua `record()`, tổng `daily_pages` bằng `book.read_pages`. Sau khi `book.read(pages)` trả `true`, cộng `pages` vào ngày cũng không tràn vì tổng ngày đó không vượt tổng mới của sách, mà tổng mới không vượt `target_pages <= u32::MAX`.

**Kiểm tra:** `main` trong `src/main.rs` là kịch bản minh họa được tổng hợp thêm, bao gồm ghi 20+5 thứ Hai, đầu vào bị từ chối, đọc đủ 100 trang và biên `u32::MAX`. Các câu hỏi và trao đổi ownership ở `answers.md`.

**Tài liệu:** [The Rust Book — Ownership](https://doc.rust-lang.org/stable/book/ch04-01-what-is-ownership.html), [Slices](https://doc.rust-lang.org/stable/book/ch04-03-slices.html), [Structs](https://doc.rust-lang.org/stable/book/ch05-01-defining-structs.html), [Methods](https://doc.rust-lang.org/stable/book/ch05-03-method-syntax.html).
