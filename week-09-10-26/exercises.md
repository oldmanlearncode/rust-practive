# Rust tuần 09-10-26 — Collections, ownership và module

## Phạm vi và cách chạy

Bạn đã hoàn thành chương 7 và xác nhận cơ bản hoàn thành chương 8. Bộ này dùng chương 1–8: `Vec`, `String`/UTF-8, `HashMap`, struct, enum, `Option`, `match`, borrowing và module. Ưu tiên thao tác collections cơ bản; chưa dùng `Result`, xử lý lỗi chương 9, trait/generic tự viết, lifetime tường minh, closure hoặc iterator adapters như `map/filter/collect`. Dùng `for` trên collection, `chars()` và `split_whitespace()` được phép. Không cần dependency hoặc nhập bàn phím.

Tuần trước bạn đã làm đúng kiểm tra trước cập nhật, getter mượn dữ liệu và `Copy` của `Option`. Tuần này tiếp tục luyện kết thúc borrow trước khi sửa collection và chuyển ownership của `String`, nhưng đổi sang tình huống khác với bộ điều nhiệt/nhật ký đọc sách.

```bash
cd week-09-10-26/solution
cargo check --all-targets
cargo run
```

Thay từng `todo!()` bằng code của bạn; giữ chữ ký/API và các assert trong `src/main.rs`. Khung chưa có lời giải: kiểm tra biên dịch có thể thành công nhưng chạy sẽ dừng ở chỗ chưa làm. Khi hiện **Tất cả case đã pass**, có thể nộp để review. Review vẫn xem thiết kế, ownership, độ rõ ràng và trường hợp ngoài các case có sẵn. Không sửa assert để làm bài pass. GitHub Actions hiện có sẽ tự nhận diện project; `PENDING` nghĩa là chưa hoàn thành dù workflow xanh.

## Bài 1 — Mượn nhãn và lấy nhãn ra khỏi Vec

Viết hai hàm trong `src/basics.rs`:

- `label_at(labels: &[String], index: usize) -> Option<&str>`: dùng `get` để mượn nhãn tại chỉ số, không sửa dữ liệu; ngoài phạm vi trả `None`.
- `take_label(labels: &mut Vec<String>, index: usize) -> Option<String>`: nếu hợp lệ, bỏ nhãn khỏi vector và trả chính `String` đó bằng ownership, không clone. Các nhãn còn lại giữ thứ tự tương đối; nếu chỉ số sai, trả `None`, vector giữ nguyên.

Ví dụ `["Đỏ", "Xanh", "Vàng"]`: mượn 0 được `Some("Đỏ")`; lấy 1 được `Some("Xanh")`, vector còn `["Đỏ", "Vàng"]`. Nhãn rỗng là dữ liệu hợp lệ, khác với không có nhãn.

Biên: vector rỗng, phần tử đầu/cuối/duy nhất, chỉ số bằng len và `usize::MAX`. Gợi ý: `get` và `remove` có hành vi khác nhau khi chỉ số sai; kiểm tra trước thao tác có thể panic. Slice nhận được cả mảng và vector mà không cần sở hữu chúng.

## Bài 2 — Tiền tố Unicode an toàn

`unicode_prefix(text: &str, limit: usize) -> String` trả chuỗi mới chứa tối đa `limit` **Unicode scalar values** đầu tiên theo `chars()`. Nếu limit lớn hơn số phần tử thì trả toàn bộ nội dung; limit 0 hoặc chuỗi rỗng trả chuỗi rỗng. Không sửa chuỗi gốc, không cắt theo byte, không dùng adapters.

Ví dụ `"Đỏ🦀a"`: limit 2 → `"Đỏ"`, limit 3 → `"Đỏ🦀"`. Khoảng trắng cũng được tính. Đây không phải bài phân tách grapheme: `"e\u{301}x"` có `e` và dấu kết hợp là hai scalar values riêng; limit 1 → `"e"`, limit 2 → `"e\u{301}"`.

Biên: ASCII, tiếng Việt có dấu, emoji, dấu kết hợp, limit `usize::MAX`. Gợi ý: duyệt `chars()` bằng vòng `for`, có thể thêm từng `char` vào `String` bằng `push` và dừng khi đủ; không cần tính limit nhân số byte.

## Bài 3 — Đếm từ bằng HashMap/entry

`word_counts(text: &str) -> HashMap<String, usize>` đếm token từ `split_whitespace()`. Dùng `entry(...).or_insert(...)` để cập nhật số đếm. Chuỗi rỗng/toàn khoảng trắng cho map rỗng. So sánh phân biệt hoa/thường; giữ nguyên dấu câu, không chuẩn hóa Unicode.

Ví dụ `"Rust rust Rust\n bánh\t bánh"` cho key `Rust` đếm 2, `rust` đếm 1, `bánh` đếm 2. `"bánh bánh,"` có hai key khác nhau. Chuỗi nguồn được mượn và vẫn dùng được sau hàm; map kết quả sở hữu các key `String`.

Không yêu cầu thứ tự duyệt `HashMap`. Gợi ý: một token là `&str`; xem kiểu key của map trước khi dùng `entry`. Các case kiểm tra từng key hoặc so sánh map, không kiểm tra thứ tự in.

## Bài 4 — Thêm tồn kho không làm thay đổi map khi thất bại

`add_stock(stock: &mut HashMap<String, u32>, item: String, quantity: u32) -> bool`:

- Tên hợp lệ khi `trim()` không rỗng; dùng helper `crate::validation::non_blank`. Giữ nguyên tên gốc làm key, kể cả khoảng trắng đầu/cuối; `" Vở "` khác `"Vở"`.
- Quantity phải trong 1..=100. Key chưa có có tồn kho hiện tại là 0; thành công thì thêm quantity. Key đã có thì cộng quantity, chỉ thành công nếu tổng mới không vượt 100.
- Hàm có thể nhận map do caller tự tạo: nếu tồn kho hiện tại đã lớn hơn 100, từ chối và giữ nguyên. Không được overflow/panic kể cả với `u32::MAX`.
- Thất bại trả false và giữ nguyên **toàn bộ map**, không vô tình tạo key có giá trị 0. Thành công chỉ đổi key tương ứng. Key `String` được nhận bằng giá trị; không cần clone nó.

Ví dụ thêm `"Bút"` 40 rồi 60 → tồn kho 100; thêm 1 nữa → false, vẫn 100. Tên mới với quantity 0 → false và không tạo key. Key hiện tại có giá trị 0 có thể nhận 100.

Gợi ý: xác thực trước khi chèn; kiểm tra số còn có thể thêm thay vì cộng rồi mới kiểm tra. Hàm không bắt buộc dùng entry; bài 3 và mini project sẽ luyện entry.

## 5 câu hỏi hiểu bản chất

Trả lời trong `solution/answers.md`, dự đoán trước khi thử. Code cố ý lỗi chỉ để trong code fence/comment.

1. Với `let mut names = vec![String::from("An")]; let first = names.get(0); names.push(String::from("Bình")); println!("{:?}", first);`, vì sao compiler từ chối? Nếu dùng `first` lần cuối trước push thì sao? Nếu vector có sẵn capacity lớn, quy tắc mượn có thay đổi không?
2. Vì sao không thể lấy ownership của String bằng `let name = names[0];`? So sánh `names.get(0)` với `names.remove(0)` về kiểu trả về, ownership, trạng thái vector và trường hợp chỉ số sai. Liên hệ hai hàm bài 1.
3. Sau `let key = String::from("Backend"); let mut map = HashMap::new(); map.insert(key, 1);`, key còn dùng được không? Nếu dùng `map.insert(&key, 1)` thì kiểu key và mối liên hệ với chuỗi gốc thay đổi thế nào? Khi trả map khỏi hàm, vì sao bài 3 chọn key sở hữu?
4. Với `"Đỏ🦀"`, `len()` đo gì, `chars()` duyệt gì? Vì sao `&text[0..1]` không hợp lệ? Giải thích trường hợp dấu kết hợp trong bài 2 và vì sao scalar values không luôn bằng số ký tự người đọc nhìn thấy.
5. Vẽ module tree của project và chỉ ra package, hai crate root `main.rs`/`lib.rs`, API re-export. Vì sao `mod basics` private vẫn có thể `pub use` hàm? Vì sao helper `pub(crate)` gọi được từ support nhưng không gọi được từ main, dù cùng package? Trong library, nêu đường dẫn tương đương tới helper bằng `crate` và `super`; `self` dùng được để trỏ item nào trong module support?

## Mini project — Sổ yêu cầu hỗ trợ

Trong `src/support/mod.rs`, triển khai `Board` giữ `Vec<Ticket>`. Ticket chứa `title: String`, `team: String`, `status: TicketStatus`; enum có `Open` và `Closed`. Field giữ private. Không xóa ticket: vị trí trong vector là chỉ số ổn định.

| API | Yêu cầu |
| --- | --- |
| `Board::new() -> Self` | Sổ rỗng |
| `len(&self) -> usize` | Tổng ticket, kể cả đã đóng |
| `submit(&mut self, title: String, team: String) -> bool` | Cả hai chuỗi qua non_blank; tạo ticket Open, move String vào field, không clone; giữ nguyên chuỗi gốc. Thất bại không thêm/sửa ticket nào. Cho phép title trùng. |
| `ticket(&self, index: usize) -> Option<&Ticket>` | Dùng get; ngoài phạm vi là None; không clone ticket |
| `close(&mut self, index: usize) -> bool` | Chỉ Open được đổi thành Closed; ngoài phạm vi hoặc đã Closed trả false và giữ nguyên toàn bộ sổ. Không sửa title/team, không xóa ticket. Có thể dùng get_mut. |
| `open_counts(&self) -> HashMap<String, usize>` | Tạo map mới, chỉ đếm ticket Open theo team, dùng entry/or_insert. Không có ticket Open của team thì không có key team ấy; không giữ key 0. Không sửa sổ. |
| `Ticket::title()/team()` | Trả &str mượn dữ liệu |
| `Ticket::status()` | Trả &TicketStatus |

Ví dụ: nộp hai yêu cầu cho `Backend`, một cho `Frontend`; thống kê lần đầu 2 và 1. Đóng ticket Backend đầu tiên → 1 và 1. Đóng lại cùng ticket → false, không đổi sổ. Đóng Backend cuối cùng → không có key Backend. Thống kê cũ vẫn giữ số cũ vì là map độc lập.

Tên team so sánh chính xác: `" Backend "` khác `"Backend"`. Không cần thứ tự thống kê. Title/team rỗng hoặc toàn khoảng trắng (kể cả khoảng trắng Unicode) bị từ chối; nội dung tiếng Việt/emoji hợp lệ.

`open_counts` mượn sổ nhưng trả map có key sở hữu: được tạo `String` mới từ team được mượn khi cần; đây không phải chuyển ownership của field ra khỏi ticket. Không cần lưu map đếm bên trong Board. Dùng vòng for và match đầy đủ hai variant, không dùng adapters.

### Tổ chức module là phần bắt buộc

Khung đã tách binary/library crate, basics, support và helper validation. Duy trì API ở crate root qua `pub use`; main import library theo tên crate. Triển khai helper `pub(crate) non_blank` và gọi nó từ bài 4 lẫn submit. Không đổi helper thành pub để gọi từ main. Không làm public field để né getter. Có thể tự chia support thành module con sau khi chạy pass, nhưng giữ API và case.

Các case có sẵn bao phủ sổ rỗng, index sai/cực đại, dữ liệu Unicode, tên có khoảng trắng, submit bị từ chối, đóng lặp, thống kê chỉ Open và map thống kê cũ độc lập. Bạn nên tự thêm case cho những bất biến chưa được kiểm tra toàn bộ; không thay các case cũ.
