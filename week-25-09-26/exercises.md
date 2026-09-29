# Bài tập Rust tuần 25-09-26 — tổng hợp chương 1–5

Phạm vi: biến, hàm, vòng lặp, ownership, borrowing, slices, struct và methods. Chưa yêu cầu `Vec`, `enum`, `match` hay lifetime tường minh.

> Lịch sử đề: Bản đầu có bài `add_unique_tags` dùng `Vec<String>` và mini project nhiều sách dùng `Vec<Book>`. Người học chỉ ra `Vec` thuộc chương 8, nên bài vector được bỏ và mini project được thay bằng đề bên dưới. Bài 2 và câu hỏi liên quan vector không thuộc phần phải nộp.

## Bài 1 — Xoay slice sang trái một vị trí

Viết `fn rotate_left_once(values: &mut [i32])`. Ví dụ `[10, 20, 30]` thành `[20, 30, 10]`. Slice rỗng và slice một phần tử giữ nguyên. Không tạo `Vec` mới.

Gợi ý: lưu phần tử đầu trước khi dịch các phần tử còn lại.

## Bài 3 — Struct ReadingGoal

Tạo struct có tên sách, mục tiêu số trang và số trang đã đọc. Viết `new(...)`, `read(&mut self, pages: u32) -> bool` và `remaining(&self) -> u32`. `read` chỉ thành công khi `pages > 0` và không vượt số trang còn lại; thất bại thì không đổi trạng thái.

Ví dụ mục tiêu 120 trang, đã đọc 90: `read(31)` trả `false`, `remaining()` vẫn là `30`; `read(30)` trả `true`, còn `0`. Tự quyết định và ghi rõ `new` xử lý mục tiêu 0 trang thế nào.

## Bài 4 — Mượn hay chuyển quyền sở hữu?

Với `ReadingGoal`, viết thêm `title(&self) -> &str` và một hàm độc lập `rename(goal: &mut ReadingGoal, new_title: String)`. Thử gọi `title()`, in kết quả, rồi `rename()`. Sau đó thử giữ tham chiếu cũ để dùng sau khi đổi tên và giải thích compiler phản đối ở đâu, vì sao. Phần thử lỗi để trong comment hoặc file thử riêng; bài chính phải biên dịch được.

## Câu hỏi hiểu bản chất

1. Vì sao bài 1 nhận `&mut [i32]` thay vì `&mut Vec<i32>`? Caller có thể truyền những loại dữ liệu nào? (Phần so sánh `Vec` có thể để sau chương 8.)
2. Câu hỏi ban đầu về `Vec<String>`: **bỏ qua**, vì vượt tiến độ.
3. Nếu `let name = goal.title();` rồi gọi `rename(&mut goal, ...)` và sau đó dùng `name`, hai lần mượn xung đột thế nào? Điều gì thay đổi nếu dùng `name` trước khi `rename`?
4. Với cú pháp struct update `let next = ReadingGoal { read_pages: 10, ..goal };`, field nào có thể bị move? Sau đó còn dùng `goal` theo những cách nào? (Đề ban đầu ghi `read_pages`; người học có một bản đặt tên field là `reading_pages`. Hãy dùng tên field đúng với struct đang viết.)

## Mini project thay thế — Nhật ký đọc sách trong 7 ngày

Dùng **một cuốn sách** và mảng cố định `[u32; 7]`. Tạo `WeeklyReading` gồm một `ReadingGoal` bắt đầu từ 0 trang và `daily_pages: [u32; 7]` bắt đầu bằng 0. Ngày 0 là thứ Hai, ngày 6 là Chủ nhật.

- `new(title: String, target_pages: u32) -> Self`: tạo nhật ký. Trong `main`, dùng mục tiêu lớn hơn 0.
- `record(&mut self, day: usize, pages: u32) -> bool`: ghi thêm số trang trong ngày. Chỉ thành công khi ngày nằm trong `0..7`, `pages > 0` và tổng không vượt mục tiêu. Nếu thất bại, cả hai nơi lưu tiến độ đều không đổi. Có thể ghi nhiều lần trong cùng ngày.
- `print_report(&self)`: in tên, số trang từng ngày, tổng đã đọc/mục tiêu, số trang còn lại và trạng thái hoàn thành.

Tự thử: ghi thứ Hai 20 rồi 5 trang (thứ Hai phải là 25); đọc vừa đủ mục tiêu; thử ngày 7, 0 trang và đọc vượt mục tiêu; thử tên tiếng Việt có dấu.

Gợi ý: kiểm tra chỉ số trước khi truy cập mảng; chỉ cập nhật mảng ngày khi `ReadingGoal::read` thành công.