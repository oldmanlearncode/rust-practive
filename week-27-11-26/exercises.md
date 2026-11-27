# Rust tuần 27-11-26 — Phiên luyện bảng nhân

## Phạm vi
Chương 1–9. Dùng mảng, Vec, HashMap, String, Option, Result, ?, struct/method, enum chứa dữ liệu, match đầy đủ, for, module và re-export. Không trait/generic tự viết, lifetime tường minh, closure, iterator adapters, unsafe hoặc dependency. Không unwrap/expect/panic để xử lý input sai. Chỉ mô phỏng trong bộ nhớ; không gọi AI hay xây UI.

## Bài 1 — Một hàng bảng nhân
`times_table(factor: u32) -> Result<[u32; 10], PracticeError>`: factor phải trong 1..=10, nếu sai trả InvalidFactor. Thành công trả [factor*1,...,factor*10]. Ví dụ 3 → [3,6,...,30]; 0,11,u32::MAX là lỗi, không panic. Gợi ý: kiểm tra trước khi nhân; mảng cố định, vòng for và index.

## Bài 2 — Chấm một câu, mô tả lỗi bằng enum
`grade(expected: u32, answer: Option<u32>) -> Grade`:
- None → Unanswered.
- Some bằng expected → Correct.
- Some khác expected → Wrong { expected, received }.
Không hạn chế giá trị expected/received; 0 và u32::MAX đều hợp lệ. Không làm phép trừ để so sánh.
Ví dụ grade(6,Some(5)) chứa expected=6, received=5. Gợi ý: match Option rồi so sánh số.

## Bài 3 — Lấy nhãn cần ôn bằng ownership
`retry_labels(items: Vec<(String, bool)>) -> Vec<String>`: bool true nghĩa đã làm đúng, false nghĩa cần ôn. Trả các nhãn false theo thứ tự gốc, giữ nhãn trùng, rỗng và Unicode nguyên văn. Không clone String. [] → []; [(A,false),(B,true),(A,false)] → [A,A]. Gợi ý: for nhận ownership từng tuple, move nhãn vào kết quả.

## 4 câu hỏi hiểu bản chất
1. Vì sao times_table phải kiểm tra factor trước phép nhân? Vì sao sai đầu vào trả Result, còn Wrong ở grade là dữ liệu bình thường?
2. None và Some(0) khác nhau thế nào? Wrong chứa hai u32 có cần clone String hoặc lifetime không?
3. for trên items và &items khác nhau thế nào về kiểu từng nhãn? Khi retry_labels nhận Vec bằng giá trị, caller có còn dùng Vec cũ không?
4. ? có tự khôi phục trạng thái đã ghi trước đó không? Vì sao answer phải kiểm tra Missing/AlreadyAnswered trước khi ghi? Vẽ cây binary/library crate và giải thích re-export.

## Mini project — PracticeSession
Lấy cảm hứng từ module bảng nhân của game giáo dục; đây chỉ là bài Rust độc lập, không thay đổi dự án game.

Session chứa Vec<Question>; mỗi Question có label String, left/right u32 và answer Option<u32>. Fields private, API đã có trong khung.

| API | Hợp đồng |
| --- | --- |
| new(), len() | Session rỗng / số câu đã thêm |
| add(label,left,right) -> Result<usize,PracticeError> | label.trim() rỗng → BlankLabel; tiếp theo left hoặc right ngoài 1..=10 → InvalidFactor. Thành công move label nguyên văn vào câu chưa trả lời, trả index từ 0. Cho phép trùng nhãn. |
| question(index) -> Option<&Question> | Dùng get, index sai trả None, không panic |
| Question getters | label mượn &str; factors trả (u32,u32); answer trả Option<u32> |
| answer(index,value) -> Result<(),PracticeError> | index thiếu → Missing; đã có Some → AlreadyAnswered; còn lại ghi Some(value), kể cả 0 hay u32::MAX. Trả lời sai vẫn là Ok. Mọi Err giữ nguyên toàn bộ session. |
| result(index) -> Result<Grade,PracticeError> | Thiếu → Missing; còn lại gọi grade với tích hai thừa số và answer hiện tại. Không thay dữ liệu. |
| retry_labels() -> Vec<String> | Chỉ lấy nhãn câu đã trả lời sai, giữ thứ tự/trùng, loại câu chưa trả lời và câu đúng. Kết quả sở hữu độc lập. Không cần gọi bài 3 vì nguồn đang mượn; được tạo String mới từ label. |
| counts() -> HashMap<String,usize> | Đếm ba nhóm bằng key "unanswered", "correct", "wrong". Chỉ có nhóm đếm >0. Dùng for, match đủ Grade, entry/or_insert. Map mới độc lập; không yêu cầu thứ tự. |

Ví dụ thêm (Rừng🦀,2,3), (Làng,3,4). answer(0,5) thành công và result(0) là Wrong{6,5}. answer(0,6) bị AlreadyAnswered, vẫn giữ 5. Câu 1 chưa trả lời không vào retry_labels. add thất bại không tạo câu ngầm. Không cần xóa, sửa đáp án, đóng phiên hoặc giới hạn số câu.

Giữ basics/session private, public API re-export qua lib.rs. Không thêm derive/trait chỉ để so sánh enum; main đã dùng match. Không clone cả session để cập nhật; snapshot trong harness chỉ để kiểm tra. Gợi ý: đọc/xác thực đủ rồi dùng get_mut cho bước ghi; dùng grade ở result, có thể tái sử dụng result khi thống kê.

## Chạy và nộp
```bash
cd week-27-11-26/solution
cargo check --all-targets
cargo run
```
Thay todo, giữ API/assert gốc. Khung biên dịch nhưng chạy dừng ở todo. Khi hiện “Tất cả case đã pass”, có thể push để nộp review; review vẫn xem thiết kế, ownership, độ rõ và case ngoài main. CI PENDING chỉ xác nhận biên dịch khung, chưa phải đã pass assert.
