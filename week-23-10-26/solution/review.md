# Review — tuần 23-10-26

Trạng thái: **PENDING — chưa nộp**.

## Cơ sở ra đề — 23/10/2026

- Đọc cây repo tại main `3fdf0727f2b1b642e1c0341b3704f6dfb1ffdb34`, workflow, source/bài trả lời/review gần nhất và các trao đổi được cung cấp.
- Thực hành tuần 09-10-26 đã hoàn thành; câu phân biệt Option<&String>/Option<&str> đã có nguyên văn trong answers.md tuần đó. Không có câu trả lời mới cần chép lại.
- Tuần 16-10-26 vẫn là khung todo, answers/review chưa nộp. Không giả định có bài làm hoặc tạo nhận xét chấm bài giả định; không chỉnh source/hồ sơ tuần cũ.
- Đã xác nhận học hết chương 9; chưa xác nhận chương 10. Bài mới giữ mức cơ bản chương 1–9, tránh parse_amount/borrow_name/parse_batch/read_note/Wallet và sổ ticket. Tập trung ownership có điều kiện, insert không ghi đè, enum state và Unicode validation trong bối cảnh hàng đợi in.

## Kiểm tra phát hành

- Khung chỉ có chữ ký, fields, module wiring và todo!(); không có thuật toán lời giải.
- Main có assert cho thường/biên/lỗi, snapshot và toàn bộ trạng thái sau thất bại; không phụ thuộc thứ tự HashMap hay format in.
- Không có Rust/Cargo trong môi trường soạn; chưa xác nhận biên dịch local. Xem GitHub Actions để xác nhận biên dịch khung. PENDING không phải các assert đã pass.

| Phần | Trạng thái |
| --- | --- |
| Bài 1–3 | Chưa nộp |
| Câu 1–4 | Chưa trả lời |
| Mini project | Chưa nộp |

## Lịch sử review và sửa bài

_Chưa có bài nộp. Khi review ghi commit, kết quả chạy, thiết kế/ownership và từng lần sửa. Giữ nguyên source/lời trả lời người học; mọi chỉnh cú pháp của trợ giảng phải chú thích rõ._

## Review phần đã triển khai — 30/10/2026

Commit: f68147f03148a2e09d24779811fed56682c3473b. Bài đang làm, chưa hoàn thành; chưa chạy local. take_document và insert_unique đúng luồng đọc/xác thực rồi cập nhật. validate_title đang đếm text gốc thay vì trimmed_text: 20 scalar có khoảng trắng hai đầu bị từ chối sai; đổi vòng đếm sang chuỗi đã trim. Spool chưa giới hạn capacity tối đa 100; new(usize::MAX) có thể nhận job thứ 101 sai hợp đồng. submit cần gọi helper validation::valid_title theo đề, thay vì gọi hàm public trực tiếp. pending_pages và queued_counts còn todo. Getter/cancel/print_next đã có triển khai; chưa kết luận pass toàn bộ khi chưa hoàn thiện. answers.md vẫn chưa có câu trả lời; giữ nguyên. Source được giữ nguyên, không chỉnh cú pháp.
