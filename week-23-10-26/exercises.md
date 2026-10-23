# Rust tuần 23-10-26 — Ownership và hàng đợi in homelab

## Phạm vi và cách nộp

Chương 1–9. Tuần 09-10-26 đã hoàn thành thực hành; tuần 16-10-26 hiện vẫn là khung chưa nộp trên repo. Bộ này tiếp tục mức cơ bản, không giả định đã học chương 10. Không lặp parse số, đọc file hay chuyển điểm: trọng tâm là lấy ownership có điều kiện, không ghi đè key và chuyển trạng thái enum.

Dùng `match`, `if let`, vòng `for`, `get/get_mut`, `Vec`, `HashMap`, `Result`, `Option`, module và re-export. Không dùng trait/generic tự viết, lifetime tường minh, closure, iterator adapters (`map/filter/collect/enumerate`), unsafe hoặc dependency. Được dùng `trim`, `chars`, `push`, `remove`, `contains_key`, `insert`, `entry/or_insert`; chỉ duyệt `chars()` bằng `for`, không cần `.count()`. Không dùng `unwrap/expect/panic!` để xử lý input sai trong implementation. Các macro assert/matches/panic và repeat/clone ở main là công cụ kiểm tra được cung cấp, không phải phần kiến thức phải triển khai.

```bash
cd week-23-10-26/solution
cargo check --all-targets
cargo run
```

Thay từng `todo!()`, giữ API, field private và assert gốc. Khung biên dịch được nhưng chạy dừng ở todo cho tới khi hoàn thành. Khi hiện **Tất cả case đã pass**, push để nộp review. Review vẫn kiểm tra thiết kế, ownership, độ rõ ràng và case còn thiếu; CI xanh với **PENDING** chỉ xác nhận khung biên dịch. Không cần làm CLI, file, driver hay kết nối máy in thật.

## Bài 1 — Xác thực tiêu đề Unicode

`validate_title(text: &str) -> Result<(), String>`:

- Xét nội dung sau `trim()`. Rỗng → `Err("blank".to_string())`.
- Quá **20 Unicode scalar values** → `Err("too_long".to_string())`.
- Còn lại → `Ok(())`. Không sửa chuỗi nguồn.

Ví dụ `" Đỏ🦀 "` hợp lệ; 20 emoji hợp lệ, 21 emoji quá dài. `"e\u{301}"` có 2 scalar values dù có thể nhìn như một chữ: lặp 10 lần hợp lệ, 11 lần quá dài. Không dùng số byte hoặc tự cắt chuỗi theo byte. Các mã lỗi là hợp đồng để kiểm tra, không cần viết thông báo tự do.

Gợi ý: vòng `for` trên `trim().chars()` với một biến đếm. Thành công ở đây không cần trả chuỗi mới.

## Bài 2 — Lấy tài liệu, nhưng không làm mất dữ liệu sai

`take_document(docs: &mut Vec<String>, index: usize) -> Result<String, TakeError>`:

1. Không có phần tử ở index → `Missing` (kể cả `usize::MAX`). Dùng `get` kiểm tra.
2. Có phần tử nhưng trắng sau trim → `Blank`.
3. Còn lại xóa đúng phần tử đó, trả String sở hữu **nguyên văn**, giữ thứ tự tương đối phần còn lại. Không clone phần tử.

Mọi Err giữ nguyên toàn vector. Không áp dụng giới hạn 20 scalar của bài 1 ở bài này. Ví dụ `["A", " ", " Đỏ🦀 "]`, lấy index 1 trả Blank và không xóa; lấy index 2 trả `" Đỏ🦀 "`, vector còn `["A", " "]`.

Gợi ý: xác thực trong thời gian mượn ngắn, sau đó mới thay đổi collection. Không lấy ra rồi mới kiểm tra.

## Bài 3 — Thêm key sở hữu nhưng không ghi đè

`insert_unique(map: &mut HashMap<String, String>, key: String, value: String) -> Result<(), InsertError>`:

- Key rỗng/trắng sau trim → `InvalidKey`.
- Key đã tồn tại → `Duplicate`.
- Còn lại move key và value vào map, trả `Ok(())`.

Key giữ nguyên văn, phân biệt hoa/thường, không chuẩn hóa Unicode. `" A "` và `"A"` khác nhau. Value được phép rỗng hoặc trắng. Không giới hạn độ dài key. Mọi Err giữ nguyên toàn map; không chèn giá trị mặc định, không thay value cũ. Không clone key/value hoặc map trong implementation.

Ví dụ map có `"A" -> "old"`, thêm `"A" -> "new"` trả Duplicate và vẫn giữ old. Thêm `"🦀" -> ""` thành công. Không yêu cầu thứ tự duyệt HashMap.

Gợi ý: phân biệt kiểm tra sự tồn tại với thao tác insert có thể thay thế giá trị cũ.

## 4 câu hỏi hiểu bản chất

Ghi trong `solution/answers.md`; giữ đoạn cố ý không compile trong code fence/comment.

1. Trong bài 2, `get(index)` trả kiểu gì? Vì sao không thể lấy String bằng `let item = docs[index]`? Sau khi dùng kết quả get lần cuối, vì sao có thể gọi remove? So sánh ownership của `Ok(String)` với getter `Job::title() -> &str`.
2. Bài 3 nhận key và value bằng giá trị. Khi trả Err trước insert, caller có tự lấy lại được hai biến đã truyền không? Map không đổi có đồng nghĩa ownership của tham số được hoàn lại không? Nếu cần trả lại key/value, chữ ký lỗi cần mang thêm dữ liệu gì (chỉ mô tả, không đổi API)?
3. `validate_title` trả `Result<(), String>`. `()` biểu diễn điều gì? Nếu helper có chữ ký `-> bool`, có thể viết trực tiếp `validate_title(text)?;` bên trong helper đó không? Nếu không, tại sao và xử lý bằng match thế nào (mô tả, chưa cần lời giải code)?
4. `Job::state()` trả `&JobState`. Vì sao không yêu cầu enum phải Copy/Clone? Nếu giữ `let title = spool.job(0)...title()` rồi gọi `print_next()` trước khi in title, borrow có xung đột không? Vẽ cây module và giải thích `pub(crate)` của helper so với API re-export mà binary main dùng.

## Mini project — Hàng đợi in homelab trong bộ nhớ

Lấy cảm hứng từ PrintBridge, chỉ mô phỏng công việc in. Job có title String, pages u32 và `JobState::{Queued, Printed, Cancelled}`. Spool lưu Vec<Job>; index là ID ổn định, không xóa hoặc sắp xếp lại job. Title được phép trùng. Giữ nguyên mọi khoảng trắng trong title được chấp nhận.

Dung lượng hiệu lực bằng capacity caller truyền nhưng **không quá 100 job**. `new(0)` hợp lệ, không nhận job nào; `new(usize::MAX)` chỉ nhận tối đa 100. Có thể lưu capacity đã giới hạn hoặc kiểm tra giới hạn khi submit, không ép chi tiết triển khai.

| API | Yêu cầu |
| --- | --- |
| `Spool::new(capacity)` | Tạo spool rỗng |
| `len()` | Tổng job đã lưu, gồm cả Printed/Cancelled |
| `job(index)` | `Option<&Job>`, dùng get, index sai trả None |
| `Job::title/pages/state` | Title &str mượn nguyên văn; pages u32 Copy; state &JobState mượn |
| `submit(title, pages)` | Thành công move title vào job Queued, append, trả index mới từ 0; không clone title |
| `cancel(index)` | Queued → Cancelled và Ok(()); không xóa job |
| `print_next()` | Chọn Queued có index nhỏ nhất, chuyển Printed và trả Some(index); không có Queued trả None và không đổi dữ liệu |
| `pending_pages()` | Tổng pages của các Queued, không tính hai trạng thái còn lại |
| `queued_counts()` | Map mới `HashMap<String, usize>` đếm job Queued theo title nguyên văn, dùng for + match đủ ba state + entry/or_insert. Key sở hữu; không có key đếm 0; không yêu cầu thứ tự |

### Lỗi và tính nhất quán

Submit kiểm tra theo đúng thứ tự (nếu nhiều lỗi cùng lúc, trả lỗi đầu tiên):

1. Title không qua bài 1 → `QueueError::InvalidTitle`.
2. Pages không trong 1..=100 → `InvalidPages`, kể cả u32::MAX.
3. Tổng job lưu đã đạt dung lượng hiệu lực → `Full`.

Cancel: index không tồn tại → `Missing`; tồn tại nhưng Printed/Cancelled → `NotQueued`; chỉ Queued mới thành công.

**Mọi Err giữ nguyên toàn bộ spool**: số job, title, pages, state, pending total và thống kê. Printed/Cancelled vẫn chiếm slot; không nhận job mới chỉ vì đã in/hủy. Tổng pending tối đa 10_000 nên không cần số lớn hoặc checked arithmetic mới. Không lưu tổng pending riêng nếu không cần thiết.

Ví dụ capacity=3: submit `" Đỏ🦀 "` 1 trang, cùng title 100 trang, `"B"` 2 trang → index 0,1,2 và pending=103. Hủy 0 → pending=102; in tiếp trả Some(1) → pending=2; in tiếp trả Some(2) → pending=0. In nữa trả None. Thêm job lúc này vẫn Full. Snapshot thống kê đã trả trước đó vẫn giữ dữ liệu cũ, không phụ thuộc spool.

### Module và ownership

Khung có library crate (`lib.rs`) và binary crate (`main.rs`). basics/spool/validation là module private; public API được re-export ở root. Giữ helper `validation::valid_title` là `pub(crate)`, triển khai bằng cách gọi hàm bài 1 và phân loại kết quả; submit phải dùng helper này. Không tự viết lại quy tắc đếm Unicode ở spool. Không cần implement bất kỳ trait nào hoặc thêm derive để case chạy.

Không clone Job/state, không clone cả spool để cập nhật. Được tạo String mới từ title mượn khi làm key thống kê. Main dùng clone cho snapshot vector/map để so sánh trước/sau, điều đó không cho phép clone trong implementation. Không thay API hoặc assert để tránh borrow checker.

Gợi ý: hoàn thành bài nhỏ trước, sau đó getter, submit/cancel, cuối cùng print_next và thống kê. Đọc trạng thái bằng match; cập nhật khi borrow phù hợp. Có thể dùng vòng lặp index để tìm job kế tiếp, không cần iterator adapters.
