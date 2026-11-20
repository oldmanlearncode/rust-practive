# Rust tuần 20-11-26 — Sổ trang wiki trong bộ nhớ

## Phạm vi và cách làm
Chương 1–9; chưa có xác nhận chương 10. Dùng for, match, if let, Option, Result, ?, String, Vec, HashMap, struct và module. Không dùng trait/generic tự viết, lifetime tường minh, closure, iterator adapters, unsafe hoặc dependency. Không dùng unwrap/expect/panic để xử lý đầu vào sai trong implementation. Không cần đọc file, HTTP, XWiki hay Solr thật.

## Bài 1 — Mượn tiêu đề dài nhất
`longest_title(titles: &[String]) -> Option<&str>` chọn tiêu đề có nhiều Unicode scalar values nhất; hòa thì chọn phần tử đầu. Rỗng → None. Giữ nguyên cả khoảng trắng, không trim, không clone. ["abcd","🦀🦀"] → "abcd" dù emoji có nhiều byte hơn. ["",""] → Some(""). Gợi ý: for trên chars() với biến đếm; lưu tham chiếu tới ứng viên.

## Bài 2 — Ghép danh sách hiển thị
`join_titles(titles: &[String]) -> String` ghép nguyên văn các phần tử bằng đúng một ký tự xuống dòng giữa hai phần tử, không tự thêm newline cuối. Không bỏ phần tử rỗng, không trim. ["A","","B"] → "A\n\nB"; [] và [""] đều cho chuỗi rỗng; ["",""] cho "\n". Không sửa đầu vào. Dùng for và push_str/push; không dùng join hoặc iterator adapters. Gợi ý: phân biệt vị trí phần tử với việc chuỗi kết quả đang rỗng.

## Bài 3 — Đổi tên key, giữ nguyên nội dung
`rename_page(pages: &mut HashMap<String, String>, old: &str, new: String) -> Result<(), RenameError>`.
Kiểm tra theo thứ tự:
1. new.trim() rỗng → BlankName.
2. old không tồn tại → Missing.
3. old bằng nguyên văn new → Ok(()) không đổi gì.
4. new đã tồn tại → Conflict.
5. Còn lại đổi tên key, giữ nguyên value và số phần tử.
Mọi Err giữ nguyên toàn map. Không clone key/value hoặc cả map. Được dùng contains_key, remove, insert. new được lưu nguyên văn: "A" khác " A ", phân biệt hoa thường và không chuẩn hóa Unicode. Ví dụ A→"nội dung", B→"khác": đổi A sang B bị Conflict; đổi A sang "Trang 🦀" thành công.
Gợi ý: xác thực hết trước remove; remove trả ownership của value. Không remove trước rồi tìm cách rollback.

## 4 câu hỏi hiểu bản chất
1. Vì sao longest_title có thể trả &str mượn nhưng join_titles phải tạo String sở hữu? Chuỗi kết quả mượn được dùng sau titles.push có luôn hợp lệ không?
2. Vì sao dùng len() để so độ dài ở bài 1 có thể sai? Với "e\u{301}", số scalar và số ký tự nhìn thấy khác nhau thế nào?
3. remove trả kiểu gì và chuyển ownership của value ra sao? Nếu rename_page trả Err trước insert, caller có tự lấy lại biến new đã truyền không?
4. ? có tự hoàn lại thao tác remove trước đó không? Giải thích vì sao cần xác thực trước khi ghi. Vẽ cây module của khung và phân biệt pub(crate) với API re-export ở library root.

## Mini project — WikiNotes
WikiNotes chứa HashMap<String,String> private, mỗi key là tên trang, value là nội dung. Tên phải không trắng sau trim; **lưu tên nguyên văn**, nội dung được phép rỗng. Không giới hạn dung lượng và không tự chuẩn hóa Unicode.

| API | Hợp đồng |
| --- | --- |
| new() / len() | Sổ rỗng / số trang |
| create(name, body) -> Result<(), RenameError> | BlankName nếu tên trắng; Conflict nếu tên trùng; còn lại move cả hai String vào map. Không ghi đè. |
| body(name) -> Option<&str> | Mượn nội dung bằng get, không tạo key thiếu |
| rename(old, new) | Gọi bài 3 bằng ?, giữ nguyên semantics/lỗi |
| search(keyword) -> Vec<String> | Trim keyword; rỗng trả vector rỗng. Trả tên các trang mà **body** chứa keyword theo contains, phân biệt hoa thường. Không tìm trong tên. Mỗi trang tối đa một kết quả; thứ tự bất kỳ; kết quả sở hữu và độc lập với sổ. |
| length_counts() -> HashMap<usize,usize> | Map mới đếm số trang theo độ dài **tên** tính bằng scalar, không trim. Dùng for và entry/or_insert; không key đếm 0. Không đếm nội dung. |

Ví dụ "Ghi chú 🦀" chứa "Rust ownership", "Trang B" chứa "Rust": search(" Rust ") trả hai tên theo thứ tự bất kỳ; search("rust") rỗng. Đổi tên không thay nội dung, tổng trang hay kết quả tìm theo nội dung (chỉ đổi tên xuất hiện trong kết quả). Snapshot search/counts đã trả phải độc lập với các lần sửa sổ sau đó.

Giữ basics/notes private, public API re-export qua lib.rs. Field private như khung. Được tạo key String mới cho kết quả search mượn từ map; không clone toàn sổ để cập nhật. Không cần derive hoặc implement trait cho RenameError. Helper riêng nếu cần chỉ pub(crate), không mở thêm API để vượt kiểm tra.

## Chạy và nộp
```bash
cd week-20-11-26/solution
cargo check --all-targets
cargo run
```
Thay từng todo, giữ chữ ký/assert gốc. Khung biên dịch nhưng chạy dừng ở todo cho đến khi làm xong. Khi hiện “Tất cả case đã pass”, có thể push để nộp review. Review còn xét thiết kế, ownership, độ rõ và biên ngoài case sẵn có. CI PENDING chỉ xác nhận khung biên dịch, không phải đã pass assert.
