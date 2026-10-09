# Review — tuần 09-10-26

Trạng thái: **chưa nộp**. Source là khung đề và case do trợ giảng tạo, không phải code đã nộp hoặc lời giải giả định.

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
