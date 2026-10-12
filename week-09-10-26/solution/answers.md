# Câu trả lời — tuần 09-10-26

Trạng thái: **review lần 2 ngày 12/10/2026: thực hành hoàn thành; câu bổ sung đạt ý chính; còn đính chính câu 2**.

## Câu 1 — Mượn phần tử và push
Vì biến first mượn trước đó lại được sử dụng sau khi thay đổi vec, 
Nếu dùng first lần cuối trước push thì vẫn được
Không, compiler vẫn không chấp nhận.

## Câu 2 — get, indexing và remove
Vì ở index 0 của vec thực sự chỉ lưu địa chỉ tham chiếu của string, nên viết let name = names[0] sẽ ko lấy ownership được
names.get(0) trả về Option<String>, Indexing không cho move một String ra rồi để lại một giá trị không hợp lệ.
names.remove(0) trả về kiểu String, ownership được move ra và sẽ thuộc về biến nhận giá trị của hàm, trạng thái vector: vecto sẽ dịch toàn bộ phần tử của nó sang trái, nếu chỉ số sai sẽ lỗi.
remove thực hiện cả việc lấy ownership và cập nhật vector.

## Câu 3 — Ownership của key HashMap
Key không còn dùng được, 
Nếu dùng map.insert(&key, 1) thì kiểu key là &String thì vòng đời có phụ thuộc vô biến key, nếu key bị drop thì cũng không còn sử dụng được nữa.
Khi map ra hỏi hàm chọn key sở hữu vì nếu dùng mượn, khi ra khỏi hàm key cũng sẽ bị mất khi ownership của key bị drop 

## Câu 4 — Byte, Unicode scalar value và dấu kết hợp
Với `"Đỏ🦀"`, `len()` đo số lượng bytes, chars() duyệt số ký tự theo chuẩn UTF-8,
`&text[0..1]` panic vì bị ngắt giữa `"Đ"` không đủ để trả ra một char `&text[0..2]` hợp lệ
`chars()` duyệt Unicode scalar values, không luôn tương ứng từng ký tự nhìn thấy.

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
`self::` trong đường dẫn chỉ module hiện tại. Trong support, có thể dùng `self::Board` hoặc `self::Ticket`.

## Tự đánh giá và case bổ sung
_Ghi chỗ còn vướng và các case bạn tự thêm, nếu có._

## Nhận xét của trợ giảng và hỏi đáp bổ sung
Trả lời câu hỏi bổ sung:
Câu hỏi: Vì sao let n = `numbers[0]` hợp lệ với Vec<i32>, còn Vec<String> thì không?
Trả lời: vì với i32 có implement Copy nên n sẽ nhận giá trị Copy ở index 0, còn với String, không có implement Copy nên khi viết như vậy sẽ bị move ownership ra khỏi vector và để lại một gía trị không hợp lệ.
Để hợp lệ có thể viết n = `&text[0]` để n chỉ mượn giá trị chứ không lấy ownership.

Câu hỏi: Trong module support, self::validation::non_blank có hợp lệ không? Vì sao?
 Trả lời: không, được, vì self đang chỉ module hiện tại là support thì ko thể theo path self::validation được

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

## Review lần 2 — 12/10/2026

Commit được review: `e15777347f7722b380547843c57a5e2f16044078`. Giữ nguyên lời trả lời và bản sửa của người học; nhận xét dưới đây là phần riêng.

### Kết quả bản sửa
- Ba chỉnh sửa code đã đạt: label_at dùng match/get trả chuỗi mượn; unicode_prefix duyệt chars với biến đếm; open_counts match đầy đủ Open/Closed.
- [GitHub Actions 38223469887](https://github.com/oldmanlearncode/rust-practive/actions/runs/38223469887), job `114725126468`: đã đọc log biên dịch thành công và main chạy đến dòng tất cả case pass.
- Main vẫn có blob SHA `b2a06c8039f6863b8ebceb75318290f73a55755f`, không đổi các assert gốc.
- Phần thực hành hoàn thành. Chỉ còn đính chính nội dung lý thuyết bên dưới; không cần sửa lại mini project.

### Câu bổ sung 1 — Đúng ý chính về Copy
Bạn phân biệt đúng i32: Copy với String không Copy.
Chỉnh cách diễn đạt: `let n = names[0]` với Vec<String> CỐ move và compiler từ chối; không phải chương trình đã move rồi để lại phần tử hỏng.
`let n = &names[0]` mượn một `&String`, hợp lệ khi index tồn tại. Nếu dùng tên biến text thì text phải là Vec<String>; String/&str không cho indexing bằng một số nguyên.

### Câu bổ sung 2 — Đạt
Đúng với cây module hiện tại: self:: là support; support không có module/binding tên validation. validation nằm ở root, cùng cấp support. Dùng `crate::validation::non_blank` hoặc `super::validation::non_blank`.
Không phải self:: không được dùng cho module con nói chung; đường dẫn ấy chỉ không có đích tương ứng ở cây hiện tại.

### Câu 2 gốc — Còn hai điểm cần đính chính
1. Bản sửa đang ghi `names.get(0)` trả `Option<String>`. Kiểu đúng là `Option<&String>`: Some chứa tham chiếu mượn, không lấy ownership và không sửa vector. Phân biệt với label_at trả `Option<&str>` nhờ as_str().
2. Dòng “ở index 0 ... chỉ lưu địa chỉ tham chiếu của string” vẫn sai. Vec<String> chứa giá trị String; mỗi String quản lý buffer byte bằng pointer, length, capacity. Vec<&String> mới chứa shared references.
Remove chỉ dịch những phần tử SAU vị trí xóa sang trái một vị trí; index sai panic.

Câu tự kiểm tra ngắn: điền kiểu của `a` và `b`:
```rust
let names = vec![String::from("An")];
let a = names.get(0);
let b = label_at(&names, 0);
```
Đây cũng là bài thực hành nhỏ: thêm type annotations dự đoán vào đoạn trên rồi cargo check, không cần đổi assert cũ.

### Các ý khác
- Câu 4 đã sửa đúng ranh giới UTF-8 và scalar values. Slice hợp lệ trả &str, không phải char.
- Câu 5 đã sửa đúng self::. Các typo basic/basics và cratre/crate vẫn có thể chỉnh cho tài liệu chính xác.
- Câu 3 chưa đổi; giữ nhận xét lần 1: Rust ngăn drop/move owner khi borrow vẫn cần dùng, và map mượn từ nguồn caller có thể được trả về nếu lifetime phù hợp. Chưa cần triển khai ví dụ lifetime lúc này.

### Trạng thái
Thực hành: **hoàn thành**. Câu hỏi bổ sung: **đạt ý chính**. Hồ sơ lý thuyết: cần đính chính kiểu get và mô hình Vec<String>. Trợ giảng không sửa source hoặc thay lời trả lời người học.

## Hỏi đáp bổ sung — 12/10/2026: kiểu get và label_at

**Trả lời nguyên văn của người học:**
```text
a : Option<&String>
b: Option<%str>
```

**Nhận xét:** a đúng: Option<&String>. Với b, trợ giảng hiểu %str là lỗi gõ của &str; kiểu đúng là Option<&str>, vì label_at chuyển tham chiếu String thành string slice bằng as_str(). Cả hai đều mượn, không clone hoặc chuyển ownership. Phần phân biệt kiểu đã đúng nếu hiểu theo lỗi gõ này; hai đính chính trong câu 2 gốc vẫn cần cập nhật riêng để hồ sơ nhất quán.

**Thực hành nhỏ:** thêm type annotations Option<&String> và Option<&str> vào hai biến, rồi cargo check.
