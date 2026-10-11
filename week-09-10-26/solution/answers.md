# Câu trả lời — tuần 09-10-26

Trạng thái: **chưa nộp, chưa có câu trả lời của người học**.

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
