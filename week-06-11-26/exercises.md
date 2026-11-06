# Rust tuần 06-11-26 — Playlist nhạc trong bộ nhớ

## Phạm vi
Chương 1–9. Bài gần nhất tuần 23 vẫn chưa hoàn tất; tuần 30 còn todo. Bộ này giữ mức cơ bản, không yêu cầu chương 10. Dùng for/match, Vec, HashMap, String, struct, enum chứa dữ liệu, Option, Result, module/re-export. Không dùng closure, iterator adapters, lifetime tường minh, trait/generic tự viết, unsafe hoặc dependency. Không unwrap/expect/panic khi xử lý input trong implementation.

## Bài 1 — Thay một scalar trong chuỗi
replace_scalar(text: &str, target: char, replacement: char) -> String thay mọi scalar bằng target, giữ nguyên phần còn lại. Không sửa nguồn; không cắt theo byte. Ví dụ "Đỏ🦀Đ", 'Đ', 'A' → "Aỏ🦀A". Chuỗi rỗng → rỗng; target không có → nội dung bằng nguồn. Với "e\u{301}", thay 'e' không xóa dấu kết hợp. Gợi ý: for trên chars(), push vào String mới.

## Bài 2 — Phát hiện tên trùng đầu tiên
first_duplicate(names: &[String]) -> Option<usize> trả index nhỏ nhất mà tên tại đó đã xuất hiện ở index trước. So sánh nguyên văn, kể cả khoảng trắng/rỗng; không chuẩn hóa Unicode. ["A","B","A","B"] → Some(2); ["A"," A "] → None. Không sửa đầu vào. Dùng HashMap<String, usize> làm bảng đã gặp; được tạo key sở hữu từ chuỗi mượn. Không yêu cầu thứ tự duyệt map. Gợi ý: duyệt theo thứ tự vector, kiểm tra key trước cập nhật.

## Bài 3 — Đổi chỗ an toàn
swap_checked(names: &mut [String], a: usize, b: usize) -> Result<(), String>. Nếu a ngoài slice trả Err("first"); nếu a hợp lệ nhưng b ngoài slice trả Err("second"); còn lại đổi chỗ, Ok(()). a=b hợp lệ là thành công không đổi. Mọi lỗi giữ nguyên toàn slice. Không clone String. Được dùng slice.swap(a,b) sau xác thực; đây là API thư viện được cho phép, không cần tự viết generic. Index usize::MAX phải trả lỗi không panic.

## 4 câu hỏi hiểu bản chất
1. Vì sao bài 1 không thể sửa byte tùy ý trong &str? Với dấu kết hợp, scalar khác ký tự nhìn thấy thế nào?
2. Bài 2 mượn String nguồn nhưng bảng tạm sở hữu key: vì sao cần tạo String mới? Trả index usize có giữ borrow vào nguồn không?
3. Vì sao names[a] = names[b] không phải cách hợp lệ để đổi chỗ hai String? swap chuyển giá trị thế nào về mặt ownership (mô tả, không cần nội bộ unsafe)?
4. State::Playing(usize) chứa dữ liệu gì? Vì sao current() trả Option<&str> còn state() trả &State? Giữ current rồi đổi playlist và dùng lại tham chiếu sẽ gặp vấn đề gì? Vẽ cây hai crate/module của khung.

## Mini project — Playlist cho thư viện nhạc Plex
Chỉ mô phỏng trong bộ nhớ. Player có tracks Vec<String> và state State::{Stopped, Playing(usize)}. Field private, API re-export ở lib.rs; module basics/player private. Không cần audio, file hay mạng.

- new(): playlist rỗng, Stopped.
- tracks() -> &[String], state() -> &State: getter mượn.
- add(title: String) -> Result<usize, String>: tên trim rỗng → "blank"; nếu đã có 50 bài → "full" (kiểm tra theo thứ tự này). Thành công move title nguyên văn vào cuối, trả index, không clone, không đổi state. Cho phép trùng tên; không giới hạn độ dài title.
- play(index) -> Result<(), String>: dùng get; thiếu → "missing", giữ nguyên toàn player. Có → Playing(index).
- stop(): chuyển Stopped, không sửa danh sách, gọi nhiều lần hợp lệ.
- current() -> Option<&str>: Stopped → None; Playing(i) → tên nguyên văn được mượn.
- advance() -> bool: nếu Playing(i) và có bài kế tiếp thì Playing(i+1), true; Stopped hoặc bài cuối → false và giữ nguyên toàn player. Không quay vòng.
- swap_tracks(a,b) -> Result<(), String>: gọi swap_checked bằng ?; nếu thành công đổi chỗ, và nếu đang phát một trong hai vị trí thì cập nhật index để **vẫn phát đúng bài trước thao tác**. Nếu a=b, giữ state. Mọi Err giữ nguyên tracks và state; không clone cả player.
- duplicate_index() -> Option<usize>: gọi bài 2 trên tracks.

Ví dụ [A,B,C], play(0), swap_tracks(0,2) → [C,B,A], state Playing(2), current A. advance lúc này false. Không phải chuyển sang phát C. Mọi tên Unicode/khoảng trắng được giữ nguyên.

Gợi ý: làm ba bài nhỏ, getter/new/add/play, rồi advance và swap. Không cần xóa bài, ID riêng hoặc lịch sử. Được match state bằng tham chiếu và copy usize; không cần derive Copy/Clone cho State.

## Chạy và nộp
```bash
cd week-06-11-26/solution
cargo check --all-targets
cargo run
```
Thay todo, giữ API và assert gốc. Khung biên dịch nhưng chạy dừng ở todo cho tới khi hoàn thành. Khi hiện “Tất cả case đã pass”, có thể push nộp review. Review vẫn xem thiết kế, ownership, độ rõ và case còn thiếu. CI PENDING chỉ xác nhận khung biên dịch, chưa phải assert đã pass.
