# Rust tuần 13-11-26 — Sổ log homelab

Phạm vi chương 1–9. Dùng vòng for, match đầy đủ, Option, Result, ?, String, Vec, HashMap và module. Không cần trait/generic tự viết, lifetime tường minh, closure, iterator adapters, unsafe hay dependency. Không dùng unwrap/expect/panic để xử lý đầu vào sai. Bài này mô phỏng trong bộ nhớ, không gọi OpenTelemetry hay đọc file.

## Bài 1 — Tách hai lát chuỗi mượn
`split_record(line: &str) -> Result<(&str, &str), LogError>` yêu cầu đúng một dấu `|`. Thiếu hoặc nhiều hơn một dấu → BadFormat (ưu tiên trước các lỗi khác). Trim hai phần; phần dịch vụ rỗng → BlankService; phần phải rỗng → BadStatus. Phần phải không rỗng được trả nguyên sau trim, chưa cần kiểm tra là ok/fail. Không cấp phát String mới. Ví dụ `"  Ảnh 🦀 | fail "` → `Ok(("Ảnh 🦀", "fail"))`. Gợi ý: được dùng split_once và contains; không tự suy vị trí byte từ số char.

## Bài 2 — Chuỗi lỗi dài nhất
`longest_failure_run(events: &[Event]) -> usize` trả độ dài đoạn Failed liên tiếp dài nhất; Healthy ngắt đoạn. Rỗng hoặc toàn Healthy → 0. [Failed, Healthy, Failed, Failed] → 2. Gợi ý: theo dõi đoạn hiện tại và kỷ lục, xét cả đoạn cuối.

## Bài 3 — Lập chỉ mục vị trí theo nhãn
`index_labels(labels: Vec<String>) -> HashMap<String, Vec<usize>>` nhận ownership của vector, gom index bắt đầu 0 theo nhãn. Giữ nguyên nhãn kể cả rỗng, khoảng trắng và Unicode; mỗi danh sách index tăng dần. Không clone String; dùng for, biến đếm và entry/or_insert. Ví dụ ["Ảnh","Tệp","Ảnh"] → Ảnh:[0,2], Tệp:[1]. Lô rỗng → map rỗng; không yêu cầu thứ tự key.

## 4 câu hỏi hiểu bản chất
1. Hai &str trả từ split_record mượn dữ liệu ở đâu? Nếu sửa String gốc rồi vẫn dùng hai lát thì sao?
2. Với Vec<String> đầu vào, for trên vector và for trên &vector khác nhau thế nào về kiểu từng phần tử và quyền move key?
3. entry có thể tạo key mới ở thời điểm nào? Vì sao không nên gọi or_insert trước khi biết trạng thái log hợp lệ?
4. Khi split_record trả Err, ? trong record thoát khỏi hàm nào? Nếu có thao tác ghi trước ?, thao tác đó có tự hoàn lại không? Vẽ cây module và chỉ ra API re-export mà main dùng.

## Mini project — Sổ log dịch vụ
`LogBook` lưu `HashMap<String, Vec<Event>>`, private. `Event` có Healthy và Failed. Mỗi dịch vụ lưu tối đa 4 event theo thứ tự nhận; không thay thế event cũ.

- new() tạo sổ rỗng. service_count() trả số key. events(service) trả Option<&[Event]>, tên tra cứu nguyên văn (không trim), không tạo key nếu thiếu.
- record(line) gọi split_record bằng ?, kiểm tra status đúng chữ thường `ok` hoặc `fail`, sai → BadStatus. Sau đó kiểm tra giới hạn của đúng dịch vụ: đầy → Full. Chỉ sau mọi xác thực mới thêm event, trả Ok(index trong lịch sử dịch vụ, bắt đầu 0). Tên key là tên đã trim. Mọi Err giữ nguyên toàn bộ sổ; kiểm tra trạng thái trước giới hạn.
- longest_run(service) trả None nếu không có dịch vụ, còn lại Some(kết quả bài 2 trên lịch sử).
- Giữ basics/book private và re-export public API từ lib.rs. Không cần derive hay implement trait cho enum. Gợi ý: tách đọc số event khỏi bước ghi để borrow kết thúc đúng lúc.

Ví dụ ghi "Ảnh | fail", "Tệp | ok", "Ảnh | fail" → index 0, 0, 1; longest_run("Ảnh") = Some(2). Ghi "Mới | broken" không tạo key.

## Chạy và nộp
```bash
cd week-13-11-26/solution
cargo check --all-targets
cargo run
```
Thay từng todo, giữ chữ ký và assert gốc. Khung biên dịch nhưng chạy dừng ở todo. Khi tất cả assert pass, push để nộp review; review còn xem thiết kế, ownership, độ rõ và trường hợp chưa có trong case. CI PENDING là khung biên dịch, chưa chạy assert.
