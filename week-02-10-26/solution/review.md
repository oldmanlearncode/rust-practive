# Review — tuần 02-10-26

Trạng thái hiện tại: **thực hành hoàn thành; lý do pub đã đạt; đã review trả lời bổ sung lần 2, đã chốt Copy của Option; còn bài củng cố về borrow và match sở hữu**.

## Review lần 2 — 05/10/2026, trả lời bổ sung

- Đã đọc lại answers.md, measurement.rs và thermostat.rs hiện tại.
- Giữ nguyên lời người học, thêm nhận xét riêng vào answers.md.
- Câu slice rỗng đúng; source đã bỏ điều kiện is_empty() dư.
- Câu pub đáp ứng yêu cầu về visibility; phần module thực hành và giải thích pub đạt.
- Câu getter đúng hướng nhưng gọi nhầm self.last là i32: kiểu thực tế là Option<i32>, Copy vì i32: Copy.
- Câu trim() đúng dự đoán lỗi; cần nói rõ borrow kết thúc ở lần dùng cuối. Ownership chưa chuyển không đồng nghĩa chủ sở hữu luôn được phép move khi còn borrow.
- Bài thử Invalid(_) chưa minh họa match sở hữu vì code hiện tại match trên tham chiếu. Bài luyện bổ sung nằm trong answers.md.
- Thực hành tuần này hoàn thành. Không yêu cầu viết lại mini project; còn hai câu hỏi ngắn củng cố Copy/borrow.
- Lần này không sửa source và không chạy lại CI; bằng chứng CI lần 1 áp dụng source lúc đó, thay đổi bỏ kiểm tra slice rỗng được xem xét bằng đọc code.

## Review lần 1 — 05/10/2026

### Phạm vi và bằng chứng

- Đọc exercises.md, answers.md, main.rs, measurement.rs, thermostat.rs tại commit `a80ef7f18ae5e8ebc6af5191fee3cfb27febed9b`.
- GitHub Actions đã chạy cargo check --all-targets và cargo run cho tuần 02-10-26: thành công, log có dòng “Tất cả case đã pass — có thể nộp bài để review.”
- [Workflow run đã xác minh](https://github.com/oldmanlearncode/rust-practive/actions/runs/37324311372), job 111811011785.
- Toolchain trong log: rustc 1.99.0, cargo 1.99.0.
- Đây là bằng chứng biên dịch/chạy thực tế trên GitHub runner. Không chạy Rust trong container của cuộc trò chuyện; không cần chạy lại cùng source chỉ để review.
- Không phát hiện lỗi hành vi qua đối chiếu code với đề. Các case hiện có không phải chứng minh cho mọi chương trình Rust hoặc mọi thay đổi sau này.

### Kết quả từng phần

| Phần | Kết quả | Nhận xét |
| --- | --- | --- |
| Bài 1 — temperature | Đạt | Đúng miền [-40,125], dữ liệu lỗi trả None, không lấy ownership |
| Bài 2 — intensity | Đạt | Match đầy đủ variant, giới hạn 10, Beep(0) hợp lệ |
| Bài 3 — last_valid | Đạt | Giữ mẫu hợp lệ mới nhất, bỏ qua mẫu lỗi, slice rỗng trả None; O(n) thời gian, O(1) bộ nhớ phụ |
| Bài 4 — Calibration | Đạt | Kiểm tra trước cập nhật, Reset luôn thành công, command được mượn |
| Mini project — Thermostat | Đạt | Constructor/getter, các mode, biên và quy tắc bảo toàn trạng thái đều đúng |
| Câu 1–5 | Đúng ý chính | Làm rõ binding tham chiếu, partial move, điều kiện Copy và drop |
| Module tự chọn | Code đạt, mô tả cần bổ sung | Tách module chạy đúng, field private; còn thiếu lý do pub và phân biệt crate với crate root |

### Các điểm làm đúng trong mini project

- SetTarget từ chối ngoài [16,30] trước khi gán.
- Sample tái sử dụng temperature(); mẫu lỗi giữ nguyên last trước đó.
- Rename dùng trim() chỉ để kiểm tra, giữ nguyên chuỗi gốc theo đề và không clone.
- SetMode chỉ sửa mode; các command thành công chỉ cập nhật field liên quan.
- is_heating() tính từ trạng thái hiện tại: Off=false, Manual theo bool, Auto chỉ true khi có mẫu nhỏ hơn target.
- Getter name()/mode() mượn dữ liệu; target()/last() trả giá trị Copy.
- Không Vec, không collection mới, không iterator adapters; samples.iter() với for không vi phạm hạn chế adapters.

### Góp ý nhỏ — Không phải lỗi bắt buộc sửa

1. temperature() nhận &Measurement, nên match measurement đã đủ; match &measurement tạo thêm tầng tham chiếu.
2. last_valid(): check is_empty() dư vì vòng for trên slice rỗng không chạy và result vẫn None. Có thể viết for temp in samples để ngắn hơn for temp in samples.iter().
3. Các wildcard imports chạy đúng; import tường minh giúp theo dõi phụ thuộc. thermostat chỉ cần Measurement và temperature.
4. Header main.rs còn mô tả khung ban đầu; nên cập nhật khi tự chỉnh bài sau này để người đọc biết đã triển khai.
5. Tên “mesurement/Mesurement” trong câu trả lời khác tên measurement/Measurement thật; đã ghi nhận riêng, không sửa lời người học.

### Nhận xét lý thuyết và bước tiếp theo

Xem [answers.md](answers.md), phần “Review lần 1 — 05/10/2026”. Lời người học được giữ nguyên; nhận xét tách riêng.
Code đủ điều kiện hoàn thành phần thực hành tuần này. Để chốt phần lý thuyết/module, trả lời câu hỏi bổ sung về slice rỗng, Option<i32> so với Mode, thời điểm kết thúc borrow của trim(), và lý do pub. Không cần viết lại mini project.

### Lịch sử thay đổi

- Trước lần nộp: hồ sơ phát hành bên dưới lưu trạng thái khung và giới hạn xác minh ở thời điểm tạo đề.
- 05/10/2026 — Lần nộp 1: người học triển khai bốn bài, mini project, tách measurement/thermostat, trả lời năm câu và mô tả module.
- 05/10/2026 — Review lần 1: đối chiếu yêu cầu và log CI; code pass. Cập nhật review.md/answers.md; không sửa source, assert hay lời trả lời của người học.
- 05/10/2026 — Review lần 2: người học bổ sung câu trả lời và bỏ kiểm tra slice rỗng dư; nhận xét Copy/borrow được ghi riêng. Trợ giảng không sửa source.

## Hồ sơ phát hành ban đầu — Thông tin lịch sử

Phần trích dưới đây là hồ sơ trước khi nộp. Những dòng “chưa nộp/chưa làm/chưa xác nhận” chỉ mô tả thời điểm phát hành; kết quả hiện tại nằm ở trên.

Trạng thái: **chưa nộp**. src/main.rs là khung đề do trợ giảng tạo, không phải bài làm của người học.

## Bối cảnh
- Bài tuần 25/09 và các lần sửa đã có trong thư mục tuần tương ứng, không thay đổi.
- Đã hoàn thành chương 6; ngày 01/10 xác nhận đã đọc chương 7.
- Bộ này tập trung enum + struct + ownership (chương 1–6); phần module là tự chọn.
- Không tạo source từ bản tóm tắt các bài enum đã nộp ngoài cuộc trao đổi này: chưa có đủ nguyên văn để lưu trung thực.

## Kiểm tra trước khi phát hành
- Khung có todo! và các case assert trong main; chưa có thuật toán lời giải.
- Đã đối chiếu case với quy tắc đề: biên số, slice rỗng, enum dữ liệu, không đổi trạng thái khi thất bại.
- Môi trường tạo bài không có cargo/rustc, nên chưa xác nhận bằng cargo check hay chạy thực tế. Người học chạy cargo check/cargo run trên máy đã cài Rust.
- Các case chỉ là bộ kiểm tra hành vi; không thay thế review code.

## Nhận xét từng bài
| Phần | Trạng thái |
| --- | --- |
| Bài 1–4 | Chưa nộp |
| Câu hỏi 1–5 | Chưa trả lời |
| Mini project | Chưa nộp |
| Module tự chọn | Chưa làm |

## Lịch sử sửa và hỏi đáp
_Chưa có lần nộp hoặc sửa của người học._

Sau khi nộp: ghi từng lần review, vấn đề, chỉnh sửa của người học và kết quả xác minh. Giữ nguyên lời trả lời; mọi chỉnh cú pháp vào source phải được ghi rõ.


## Hỏi đáp tiếp — 05/10/2026

Người học trả lời đúng câu hỏi Option<i32> so với Option<String>: Option<T> là Copy khi T: Copy. Lời trả lời nguyên văn đã lưu vào answers.md. Phần thực hành đã hoàn thành; câu hỏi Copy này được chốt. Không sửa source hoặc chạy lại kiểm tra cho thay đổi ghi nhận câu trả lời.

- 05/10/2026 — Hỏi đáp Option<&str>: người học trả lời đúng; đã lưu nguyên văn và ghi rõ copy tham chiếu không copy nội dung chuỗi. Không thay đổi source.
