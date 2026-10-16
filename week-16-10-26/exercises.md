# Rust tuần 16-10-26 — Result, truyền lỗi và cập nhật dữ liệu an toàn

## Phạm vi và cách làm

Bạn đã hoàn thành thực hành tuần 09-10-26 và xác nhận đọc xong chương 9. Bộ này dùng chương 1–9: collections, struct/enum, ownership, module, `Option`, `Result`, `match` và `?`. Chưa yêu cầu trait/generic tự viết, lifetime tường minh, closure hoặc iterator adapters (`map`, `filter`, `collect`, `enumerate`...). Dùng vòng `for`, `get`, `get_mut`, `entry/or_insert` và các hàm thư viện được nêu trong đề. Không cần dependency.

Không dùng `unwrap`, `expect` hoặc `panic!` trong phần triển khai để xử lý đầu vào sai. Các `panic!` trong main chỉ báo một case hợp lệ bị trả lỗi. Khung giữ field private, API qua re-export ở lib.rs, main là binary crate riêng.

```bash
cd week-16-10-26/solution
cargo check --all-targets
cargo run
```

Thay từng `todo!()` bằng code của bạn; giữ API và các assert gốc. Khung có thể biên dịch nhưng chạy sẽ dừng ở todo cho đến khi làm xong. Khi hiện **Tất cả case đã pass**, bạn có thể nộp review; review vẫn xem thiết kế, ownership, độ rõ ràng và các biên ngoài case sẵn có. Workflow tự phát hiện tuần mới: **PENDING không có nghĩa đã hoàn thành**.

Gợi ý cách chia buổi: làm bài 1–2, tiếp bài 3–4, sau đó mini project. Không cần tự làm giao diện CLI hoặc nhập bàn phím.

## Bài 1 — Phân tích số tiền với lỗi rõ ràng

Trong `basics.rs`, triển khai `parse_amount(text: &str) -> Result<u32, String>`:

1. Bỏ khoảng trắng hai đầu bằng `trim`; nếu rỗng, trả `Err("empty".to_string())`.
2. Phân tích bằng `parse::<u32>()`; mọi lỗi parse trả `Err("not_u32".to_string())`.
3. Giá trị phải trong **1..=10_000**, ngoài khoảng trả `Err("out_of_range".to_string())`; hợp lệ trả `Ok(value)`.

Thứ tự trên là một phần hợp đồng. Dùng cú pháp số mà `u32::from_str`/`parse::<u32>()` chấp nhận, không tự đặt thêm quy tắc cấm số 0 đầu hoặc dấu `+`. Cú pháp `::<u32>` chỉ chọn kiểu cho hàm thư viện, không yêu cầu viết generic.

Ví dụ: `" 42\n"` → `Ok(42)`; `"0"` → out_of_range; `"-1"`, `"1.5"`, `"4 2"`, `"4294967296"` → not_u32. `"4294967295"` parse được nhưng ngoài giới hạn. Chuỗi rỗng và khoảng trắng Unicode → empty.

Gợi ý: dùng `match` để đổi lỗi parse sang lỗi nghiệp vụ. Các mã lỗi cố định chỉ nhằm giúp case so sánh, không cần định dạng câu thông báo.

## Bài 2 — Mượn tên nhưng giải thích được vì sao thất bại

`borrow_name(names: &[String], index: usize) -> Result<&str, String>`:

- Dùng `get`. Không có phần tử → lỗi `"missing"`.
- Có phần tử nhưng `trim()` rỗng → lỗi `"blank"`.
- Còn lại trả string slice đã trim; không clone tên và không sửa vector.

Ví dụ `names = ["  An🦀  ", " "]`: index 0 → `Ok("An🦀")`; index 1 → blank; index 2 hoặc `usize::MAX` → missing. Chuỗi nguồn vẫn giữ khoảng trắng. Không cắt theo byte và không cần lifetime tường minh.

Gợi ý: đối chiếu `Option<&String>` từ get với `Result<&str, String>` cần trả. Đây là bước tiếp từ getter tuần trước: caller phân biệt được thiếu phần tử và dữ liệu trắng.

## Bài 3 — Parse một lô và dừng ở lỗi đầu tiên

`parse_batch(lines: &[String]) -> Result<Vec<u32>, String>` gọi **parse_amount** cho từng dòng theo thứ tự, dùng **`?` ít nhất một lần** để truyền nguyên lỗi về caller.

- Tất cả hợp lệ: trả vector số theo đúng thứ tự; lô rỗng → `Ok(vec![])`.
- Gặp lỗi: trả lỗi đầu tiên, không trả vector kết quả một phần, không tiếp tục xử lý dòng sau.
- Không sửa hoặc lấy ownership của dữ liệu đầu vào.

Ví dụ `["2", "oops", ""]` → not_u32, không phải empty. `["0", "oops"]` → out_of_range. Không copy lại thuật toán parse bài 1.

Gợi ý: chữ ký của caller đã có Result với cùng kiểu lỗi. Không cần đính kèm số dòng, không cần closure hoặc collect.

## Bài 4 — Đọc ghi chú và truyền lỗi I/O

`read_note(path: &str) -> Result<String, std::io::Error>` đọc toàn bộ file UTF-8 bằng `std::fs::read_to_string`, dùng **`?`** để trả lỗi I/O nguyên kiểu về caller; thành công trả nội dung đã trim thành một String sở hữu.

File chỉ có khoảng trắng hoặc file rỗng là thành công với chuỗi rỗng. File không đọc được hoặc byte không phải UTF-8 là Err; không thay bằng Ok rỗng và không panic. Không sửa file.

Main cung cấp fixture tiếng Việt/emoji, fixture rỗng và đường dẫn chắc chắn không trỏ tới file con hợp lệ. Đường dẫn fixture được wiring sẵn, không cần học các macro đường dẫn để làm bài. Case I/O không so sánh chuỗi thông báo lỗi của hệ điều hành. Có thể tự bổ sung file chỉ chứa khoảng trắng hoặc byte UTF-8 sai.

Gợi ý: phân biệt lỗi nghiệp vụ String ở bài 1 với kiểu lỗi I/O có sẵn; tuần này không cần implement `Error`, `Display` hay chuyển đổi lỗi bằng trait.

## 5 câu hỏi hiểu bản chất

Trả lời trong `solution/answers.md`, dự đoán trước khi kiểm chứng. Code cố ý không compile chỉ đặt trong code fence/comment.

1. Vì sao `borrow_name` dùng Result có ích hơn Option trong đề này? Với chữ ký ấy, ai sở hữu chuỗi thành công, ai sở hữu String lỗi? So sánh với `balance` trả Option<u32>.
2. Khi `parse_batch` gặp Err tại dòng thứ hai, `?` làm gì với hàm hiện tại và vector cục bộ? `?` có tự rollback thay đổi trước đó trong HashMap bên ngoài không? Liên hệ thứ tự xác thực/cập nhật ở mini project.
3. Sau `let result = borrow_name(&names, 0);`, nếu `names.push(...)` rồi mới in result thì có compile không? Tại sao Err là String sở hữu không làm chữ ký trả về hết phụ thuộc borrow? Nếu dùng result lần cuối trước push thì sao?
4. Vì sao giữ hai kết quả `get_mut` từ cùng HashMap để cập nhật nguồn và đích thường bị compiler từ chối, dù hai tên khác nhau? Nêu cách tổ chức bước đọc/xác thực và bước ghi trong phạm vi đã học; không dùng unsafe hoặc API mới để lấy đồng thời hai mutable references.
5. Phân loại: tiền nhập sai, thiếu tài khoản, file không đọc được, assert thất bại trong main. Trường hợp nào nên trả lỗi cho caller, trường hợp nào dùng panic để báo case sai? Vẽ cây module và giải thích vì sao helper `pub(crate)` dùng được trong wallet nhưng binary main không gọi trực tiếp được dù chung package.

## Mini project — Ví điểm nội bộ cho homelab

Ví điểm mô phỏng có `HashMap<String, u32>` lưu số dư và `Vec<Transfer>` lưu chuyển điểm thành công. Đây chỉ là dữ liệu trong bộ nhớ, không dùng tiền thật, file lưu trữ hay đa luồng.

Mỗi số dư luôn trong **0..=10_000**. Tên hợp lệ khi không trắng sau trim, nhưng **giữ nguyên tên gốc**: `" A "` và `"A"` là hai tài khoản khác nhau; phân biệt hoa/thường, không chuẩn hóa Unicode.

| API | Hợp đồng |
| --- | --- |
| `Wallet::new()` | Ví rỗng, lịch sử rỗng |
| `open(name: String, balance: u32) -> bool` | Tên qua helper non_blank, số dư <=10_000, tên chưa tồn tại. Thành công move tên vào map; thất bại không thay bất cứ dữ liệu nào, không ghi đè số dư. Không thêm lịch sử. |
| `balance(&self, name: &str) -> Option<u32>` | Key tồn tại trả giá trị Copy, không có trả None; không tạo key 0 |
| `account_count(&self) -> usize` | Số tài khoản, gồm tài khoản có số dư 0 |
| `transfer(from: String, to: String, amount: u32) -> Result<usize, TransferError>` | Xác thực theo bảng dưới, thành công trừ nguồn/cộng đích, append đúng một Transfer, trả index mới trong history (bắt đầu 0). Move hai String vào lịch sử, không clone chúng. |
| `history(&self) -> &[Transfer]` | Slice mượn theo thứ tự chuyển thành công; không clone |
| `Transfer::from()/to()/amount()` | Hai getter &str mượn, getter u32 Copy |
| `sent_counts(&self) -> HashMap<String, usize>` | Map mới đếm số lần gửi thành công theo from; dùng for và entry/or_insert. Key sở hữu, không key 0; map cũ độc lập khi ví thay đổi. |

### Lỗi chuyển điểm và ưu tiên

Nếu nhiều điều kiện sai, trả lỗi đầu tiên theo đúng bảng. Enum đã có trong khung, chưa cần derive hoặc implement trait.

| Thứ tự | Điều kiện | Variant |
| --- | --- | --- |
| 1 | Một trong hai tên trắng/rỗng | InvalidName |
| 2 | Hai tên giống hệt nhau | SameAccount |
| 3 | amount = 0 | ZeroAmount |
| 4 | Nguồn không tồn tại | MissingSource |
| 5 | Đích không tồn tại | MissingTarget |
| 6 | Nguồn không đủ số dư | InsufficientFunds |
| 7 | Đích nhận xong sẽ vượt 10_000 | TargetLimit |

Chỉ sau khi qua hết kiểm tra mới sửa hai số dư và lịch sử. **Mọi Err giữ nguyên toàn bộ ví**; không tạo tài khoản ngầm, không thêm lịch sử thất bại. Không overflow/panic với `u32::MAX`. Có thể chuyển hết số dư nguồn về 0 hoặc đưa số dư đích lên đúng 10_000. Chuyển không làm đổi tổng số điểm hoặc số tài khoản.

Ví dụ: An=100, Bình🦀=10; chuyển 40 → An=60, Bình🦀=50, Ok(0). Chuyển tiếp 61 → InsufficientFunds, giữ nguyên cả hai và lịch sử. Đích đã 10_000 mà nhận 1 từ nguồn đủ tiền → TargetLimit. Nguồn chưa có, đích chưa có và amount=0 → ZeroAmount do ưu tiên.

### Ownership và module

- Giữ basics, wallet, validation là module private; re-export API public ở lib.rs. Field private như khung.
- Triển khai `validation::non_blank` với `pub(crate)` và gọi từ open/transfer. Không đổi helper thành pub.
- Đọc số dư bằng borrow ngắn rồi lưu bản sao u32 để xác thực; bước ghi không cần giữ đồng thời hai mutable references. Không bắt buộc dùng entry trong transfer; entry bắt buộc ở sent_counts.
- Được tạo key String mới từ `&str` khi thống kê. Không clone from/to của transfer hoặc clone toàn map chỉ để cập nhật.
- Không yêu cầu thứ tự duyệt HashMap hoặc format in. Không cần rollback bằng code nếu chưa thay đổi dữ liệu trước khi phát hiện lỗi.

Case có sẵn kiểm tra cả tên Unicode/khoảng trắng, trùng tên, giới hạn, ưu tiên lỗi, không đổi dữ liệu và snapshot thống kê độc lập. Bạn có thể thêm case nhưng không thay assert gốc.

## Tài liệu đối chiếu

- [The Rust Book — Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Recoverable Errors with Result](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
- [To panic! or Not to panic!](https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html)
