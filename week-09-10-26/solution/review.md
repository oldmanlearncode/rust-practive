# Review — tuần 09-10-26

Trạng thái: **đã nộp; review lần 1 ngày 11/10/2026. Case pass; chờ chỉnh ba ràng buộc luyện tập và bổ sung lý thuyết.** Các mục phát hành bên dưới là lịch sử trước lần nộp.

## Bối cảnh phát hành — 09/10/2026

- Đã hoàn thành chương 7 và cơ bản hoàn thành chương 8; phạm vi chương 1–8.
- Đã đọc cấu trúc repo, đề hai tuần trước, source/module và answers/review tuần 02-10-26 ở head `acc0b1690f42a92be6a8ba05aa0553241a8dad15`.
- Tuần trước thực hành hoàn thành, Copy của Option đã chốt. Hồ sơ đã giữ nguyên lời người học và các hỏi đáp; lần này không sửa hoặc ghi đè tuần cũ.
- Bộ mới tập trung collections cơ bản, an toàn chỉ số, Unicode, key sở hữu, entry và binary/library visibility; không lặp mini project cũ, không dùng chương 9 trở đi.

## Kiểm tra phát hành

- Chỉ có chữ ký, field, wiring module, todo và các assert; chưa có thuật toán lời giải.
- Case đối chiếu với đề, không phụ thuộc thứ tự HashMap hoặc format in tùy ý.
- Chưa chạy các assert đến cuối vì khung còn todo; chạy main phải dừng cho tới khi người học triển khai.
- Môi trường soạn bài không có Cargo/Rust. Kết quả biên dịch thực tế, nếu có, được ghi riêng sau khi xem GitHub Actions; không coi workflow xanh/PENDING là bài hoàn thành.

## Kết quả từng phần

| Phần | Trạng thái |
| --- | --- |
| Bài 1–4 | Chưa nộp |
| Câu hỏi 1–5 | Chưa trả lời |
| Mini project | Chưa nộp |
| Tổ chức module/visibility | Có wiring khung; chưa review bài người học |

## Lịch sử nộp, sửa và hỏi đáp

_Chưa có lần nộp. Khi review sẽ ghi commit được xem, kết quả case, thiết kế/ownership, vấn đề và từng lần sửa; giữ nguyên source người học, ghi rõ mọi chỉnh cú pháp của trợ giảng nếu có._

## Xác minh biên dịch qua GitHub Actions — 09/10/2026

- Commit phát hành: `a49405975f58b0713267a45bf3849dd4bcfc4656`.
- [Workflow run 37869799312](https://github.com/oldmanlearncode/rust-practive/actions/runs/37869799312), job `113625012609`: thành công.
- Đã đọc log thực tế: `cargo check --all-targets` cho tuần 09-10-26 thành công với rustc/cargo 1.99.0; các cảnh báo unused/dead_code do khung chưa triển khai.
- Workflow nhận diện todo trong basics.rs, support/mod.rs và validation.rs; tuần mới là **PENDING**, không chạy main, không tuyên bố assert đã pass.
- Source, case và yêu cầu không đổi sau xác minh; lần cập nhật này chỉ thêm bằng chứng vào review.md.

## Review lần 1 — 11/10/2026

### Bằng chứng chạy thực tế
- Review source/câu trả lời tại commit `8dfe24a1c4ea6cdb21f486b5a789b20717d4b2ba`.
- [Workflow 38143199596](https://github.com/oldmanlearncode/rust-practive/actions/runs/38143199596), job `114481092466`: success.
- Đã đọc log: rustc/cargo 1.99.0; `cargo check --all-targets` thành công; `cargo run` tuần 09-10-26 chạy đến “Tất cả case đã pass — có thể nộp bài để review.”
- Máy soạn review hiện không có Rust/Cargo; không tuyên bố đã chạy local hoặc chạy case bổ sung.
- Đã đối chiếu code với toàn bộ yêu cầu và case main. Chưa thấy lỗi hành vi; có ba điểm không đúng ràng buộc phương pháp của đề. PASS case chưa đồng nghĩa hoàn thành các ràng buộc ấy.

### Kết quả
| Phần | Kết luận |
| --- | --- |
| label_at / take_label | Hành vi đúng; label_at dùng closure ngoài phạm vi |
| unicode_prefix | Kết quả/biên đúng; enumerate là adapter ngoài phạm vi |
| word_counts | Đạt: key sở hữu, entry/or_insert, phân biệt token đúng |
| add_stock | Đạt: xác thực, giới hạn, không overflow, dữ liệu không đổi khi false |
| Board/Ticket/helper | Hành vi và visibility đúng |
| open_counts | Kết quả đúng; cần match đầy đủ hai variant theo yêu cầu |
| Lý thuyết | Câu 1 đạt; câu 2/4/5 cần đính chính; câu 3 cần diễn đạt chính xác hơn |

### Ba chỉnh sửa bắt buộc để đúng mục tiêu luyện tập
1. `basics.rs::label_at`: `labels.get(index).map(|label| label.as_str())` là code Rust hợp lệ và gọn. map ở đây là `Option::map`, không phải iterator adapter, nhưng closure `|label| ...` chưa được phép trong đề. Tự viết lại bằng match Some/None, vẫn dùng get và vẫn trả chuỗi mượn; không clone.
2. `basics.rs::unicode_prefix`: `chars().enumerate()` đúng hành vi nhưng enumerate là iterator adapter; đề yêu cầu vòng for cơ bản và chưa dùng adapters. Duyệt trực tiếp chars(), quản lý số scalar đã thêm bằng biến đếm. Kiểm tra limit 0 và usize::MAX bằng case cũ.
3. `support/mod.rs::open_counts`: đề nói “match đầy đủ hai variant”. if let hiện tại cho kết quả đúng nhưng bỏ qua việc luyện exhaustiveness. Dùng match Open/Closed; Closed không cập nhật map. Giữ vòng for, entry/or_insert và key sở hữu.

Không viết thay lời giải hoặc sửa source; người học tự chỉnh ba chỗ, giữ API và assert.

### Phân tích add_stock — không có lỗi “entry tạo key khi thất bại” trong bản này
Entry/or_insert có khả năng chèn key, nên cần kiểm tra luồng cụ thể:
- Tên/quantity sai bị trả false trước entry.
- Key mới có current = 0 và quantity đã trong 1..=100, nên mọi đường đi sau đó đều thành công.
- Nếu current > 100, key ấy phải đã tồn tại; return false không đổi giá trị.
- Nếu current <= 100, quantity <= 100 nên phép cộng <= 200, không tràn u32. Nếu vượt 100 thì key đã tồn tại và giá trị chưa được gán.
Do đó các lần false hiện tại giữ nguyên map. Không cần bắt sửa thuật toán này. Có thể bỏ biến new_quantity hoặc cải thiện tên `curent_quantity` thành `current_quantity` khi muốn làm sạch code, nhưng không phải điều kiện hoàn thành.

### Ownership và thiết kế
- take_label dùng remove sau bounds check, lấy đúng String và giữ thứ tự tương đối.
- submit xác thực trước push, move cả hai String vào Ticket, giữ khoảng trắng gốc.
- close dùng get_mut; pattern matches chỉ kiểm tra variant không bind field sở hữu, không move status ra khỏi ticket. matches! ở close không phải điểm vi phạm match đầy đủ trong open_counts.
- open_counts mượn tickets/team và tạo String mới làm key; không move field và không giữ borrow trong map trả về. Bản thống kê cũ độc lập.
- to_string mỗi ticket Open có thể tạo key tạm ngay cả team trùng. Chấp nhận ở phạm vi bài; chưa yêu cầu tối ưu allocation.
- Helper pub(crate), module private và API re-export giữ đúng. Các field vẫn private.

### Lý thuyết và bước tiếp
Nhận xét chi tiết, nguồn và câu tự kiểm tra nằm ở answers.md; lời trả lời ban đầu được giữ nguyên.
Ưu tiên đính chính: Vec<String> chứa giá trị String; byte boundary khi slice &str; self:: là module path khác receiver self.
Bài luyện tùy chọn: snapshot stock bằng clone trong main trước thao tác thất bại, assert_eq! toàn map sau thao tác. Clone để kiểm tra độc lập trong case không thay yêu cầu không clone item của hàm.

### Lịch sử sửa
11/10/2026: trợ giảng chỉ cập nhật answers.md và review.md. Không sửa source, API, đề hoặc assert. Trạng thái: hành vi pass; chờ ba chỉnh sửa ràng buộc và bổ sung lý thuyết.
