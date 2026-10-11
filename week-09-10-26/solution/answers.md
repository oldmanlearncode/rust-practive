# Câu trả lời — tuần 09-10-26

Trạng thái: **đã nộp; review lần 1 ngày 11/10/2026, cần đính chính một số ý lý thuyết**.

## Câu 1 — Mượn phần tử và push
Vì biến first mượn trước đó lại được sử dụng sau khi thay đổi vec, 
Nếu dùng first lần cuối trước push thì vẫn được
Không, compiler vẫn không chấp nhận.

## Câu 2 — get, indexing và remove
Vì ở index 0 của vec thực sự chỉ lưu địa chỉ tham chiếu của string, nên viết let name = names[0] sẽ ko lấy ownership được
names.get(0) trả về Option<&String>, ownership vẫn thuộc về phần từ trong vector, trạng thái vector không đổi, chỉ số sai sẻ trả về None
names.remove(0) trả về kiểu String, ownership được move ra và sẽ thuộc về biến nhận giá trị của hàm, trạng thái vector: vecto sẽ dịch toàn bộ phần tử của nó sang trái, nếu chỉ số sai sẽ lỗi.

## Câu 3 — Ownership của key HashMap
Key không còn dùng được, 
Nếu dùng map.insert(&key, 1) thì kiểu key là &String thì vòng đời có phụ thuộc vô biến key, nếu key bị drop thì cũng không còn sử dụng được nữa.
Khi map ra hỏi hàm chọn key sở hữu vì nếu dùng mượn, khi ra khỏi hàm key cũng sẽ bị mất khi ownership của key bị drop 

## Câu 4 — Byte, Unicode scalar value và dấu kết hợp
Với `"Đỏ🦀"`, `len()` đo số lượng bytes, chars() duyệt số ký tự theo chuẩn UTF-8,
`&text[0..1]` không trả ra một ký tự, mà trả ra kết quả bytes, Rust ko cho phép điều này.
Vì một ký tự hiển thị trên màn hình có thể được kết hợp bởi nhiều bytes, và có cả những trường hợp, hiển thị được kết hợp bởi nhiều hơn 1 chars.

## Câu 5 — Package, crates, module tree và visibility
module tree
package:rust-practice-week-09-10-26"
|- library crate
	|-lib.rs
		|- basic 
			|-pub label_at()
			|-pub take_label()
			|-pub unicode_prefix()
			|-pub word_counts()
			|-pub add_stock()
		|- support
				|- pub Board
				|- pub Ticket
				|- pub TicketStatus
		|- validation		
				|- pub(crate) non_blank()
		|- re-export
				|- label_at()
				|- take_label()
				|- unicode_prefix()
				|- word_counts()
				|- add_stock()
				|- Board
				|- Ticket
				|- TicketStatus
|- binary crate
		|- main.rs

vì các hàm ở basic đã được re-export tại crate
vì pub(crate) chỉ public cho toàn bộ trong crate, không public cho crate khác. main() nằm ở crate khác nên không gọi được.
Đường dẫn tương đương tới helper bằng crate: cratre::validation::non_blank
Đường dẫn tương đương tới helper bằng super: super::validation::non_blank
self tùy thuộc vào impl mà nó trỏ tới item tương ứng, nếu là impl của Board, thì nó là Board, impl của Ticket, thif là Ticket.

## Tự đánh giá và case bổ sung
_Ghi chỗ còn vướng và các case bạn tự thêm, nếu có._

## Nhận xét của trợ giảng và hỏi đáp bổ sung
_Chưa review. Khi có bài nộp, giữ nguyên văn câu trả lời; thêm nhận xét riêng và ghi từng lần bổ sung._

## Review lần 1 — 11/10/2026

Commit được review: `8dfe24a1c4ea6cdb21f486b5a789b20717d4b2ba`. Giữ nguyên toàn bộ lời trả lời phía trên.

### Câu 1 — Đạt
Bạn dự đoán đúng cả ba tình huống. `get` mượn shared từ vector; `push` cần `&mut Vec`. Vì first còn được dùng sau push, hai borrow xung đột. Nếu lần dùng cuối của first ở trước push, borrow có thể kết thúc trước mutable borrow. Capacity lớn không thay đổi quy tắc này; khả năng reallocate là lý do minh họa cho nguy cơ, không phải điều kiện compiler dùng để cho phép ngoại lệ.

### Câu 2 — Kết quả đúng, cần sửa mô hình lưu trữ
`Vec<String>` chứa chính các giá trị String trong buffer liên tiếp, không chứa các `&String`. Mỗi String quản lý buffer byte riêng thông qua pointer, length và capacity. Đừng nhầm pointer bên trong String với một tham chiếu Rust được lưu làm phần tử.
`names[0]` không cung cấp thao tác lấy một phần tử sở hữu ra và cập nhật vector. String không Copy nên `let name = names[0]` cố move từ nơi được truy cập qua indexing và bị từ chối; vector vẫn phải chứa phần tử hợp lệ.
Bạn mô tả `get`/`remove` đúng: `Option<&String>` so với `String`, borrow so với chuyển ownership, không thay đổi so với xóa phần tử. Remove chỉ dịch các phần tử nằm SAU index sang trái một vị trí, không phải toàn bộ phần tử; index sai gây panic.
Câu tự kiểm tra: cùng biểu thức indexing, vì sao `Vec<i32>` lại cho phép `let n = numbers[0]`?

### Câu 3 — Đúng hướng, cần tránh khẳng định tuyệt đối
Insert key String bằng giá trị move key vào map. Insert `&key` suy ra key `&String`, map mượn chuỗi gốc.
Rust không cho drop/move key trong khi map còn được dùng và còn phụ thuộc borrow; đây là lỗi compiler ngăn, không phải tham chiếu được phép hỏng rồi mới phát hiện.
Không phải mọi map có key mượn đều không trả được khỏi hàm: có thể mượn từ dữ liệu của caller còn sống đủ lâu. Bài 3 chọn key sở hữu để kết quả độc lập với nguồn và tránh trả tham chiếu tới String cục bộ. Chưa cần viết lifetime tường minh cho trường hợp tổng quát này.

### Câu 4 — Cần chính xác về UTF-8
`len()` đo byte; `chars()` trả từng char, tức Unicode scalar value, không phải byte và cũng không đảm bảo một đơn vị hiển thị.
`"Đỏ🦀"` có 9 byte, 3 scalar values: Đ = 2 byte, ỏ = 3 byte, 🦀 = 4 byte.
Slice chuỗi `&text[a..b]` dùng offset byte nhưng kết quả vẫn là `&str`. Rust cho cắt theo byte khi hai đầu đúng ranh giới UTF-8; `&text[0..1]` panic vì byte 1 nằm giữa mã hóa của Đ. Không phải mọi slice theo byte đều bị cấm; `&text[0..2]` hợp lệ, cho "Đ". Đoạn cố ý panic đừng đưa vào main của bài.
`"e\u{301}"` có 2 scalar values (e và combining acute accent), có thể hiển thị thành một grapheme. Số byte, scalar values và grapheme là ba cách đếm khác nhau.
Câu tự kiểm tra: dự đoán `len()` và `chars().count()` của `"e\u{301}"` trước khi chạy.

### Câu 5 — Crate/visibility đúng, nhầm self
Cây bạn vẽ thể hiện đúng hai crate và re-export. Tên module thật là `basics`, không phải `basic`; `Cargo.toml` định nghĩa package, main.rs và lib.rs là hai crate root. Re-export là API, không phải module riêng.
`mod basics` private chặn caller bên ngoài đi theo đường dẫn `crate_name::basics::...`. Các hàm bên trong là pub nên lib.rs có thể re-export chúng qua đường dẫn public tại root. Không được re-export item private ra public chỉ bằng cách viết pub use.
Bạn giải thích pub(crate) đúng: binary crate và library crate khác nhau dù chung package.
Helper từ support: `crate::validation::non_blank` (sửa typo cratre), hoặc `super::validation::non_blank`.
`self::` trong PATH là module hiện tại, không phải receiver `self` của method. Trong support có thể viết `self::Board`, `self::Ticket`, `self::TicketStatus`; vì helper đã được use vào scope, `self::non_blank` cũng trỏ tới binding import ấy. Receiver `self`/`&self` chỉ đối tượng hiện tại; `Self` là kiểu của impl.
Câu tự kiểm tra: bên trong support, `self::validation::non_blank` có hợp lệ không? Vì sao?

### Việc cần bổ sung
- Sửa/đính chính riêng các ý ở câu 2, 4, 5; giữ câu trả lời ban đầu làm lịch sử.
- Sửa ba điểm phạm vi trong code theo review.md, chạy lại main.
- Bài luyện tùy chọn: thêm case vào main so sánh toàn bộ map trước/sau một add_stock thất bại trên map đã có nhiều key. Giữ các case gốc.

### Nguồn chính thống
- [Vectors — Rust Book](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [Strings / UTF-8 — Rust Book](https://doc.rust-lang.org/book/ch08-02-strings.html)
- [Vec: guarantees và remove](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [Paths: self, super, crate — Rust Reference](https://doc.rust-lang.org/reference/paths.html)
- [Re-export bằng pub use — Rust Book](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)
