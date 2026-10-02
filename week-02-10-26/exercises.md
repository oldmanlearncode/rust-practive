# Rust tuần 02-10-26 — Enum, struct và ownership trong hệ cảm biến

## Phạm vi và cách làm

Phần bắt buộc dùng chương 1–6 của The Rust Book: hàm, vòng lặp, mảng/slice, ownership, borrowing, struct/method, enum chứa dữ liệu, `Option<T>`, `match` và `if let`. Không dùng `Vec`, `Result`, trait tự viết, iterator adapters hoặc lifetime tường minh. Bạn đã xác nhận đọc chương 7 ngày 01/10; phần module cuối đề là tự chọn.

Bài này tiếp nối những điểm đã luyện: không làm mất dữ liệu khi thao tác thất bại; mượn dữ liệu thay vì clone tùy tiện; phân biệt chuỗi sở hữu và tham chiếu. Không lặp bài ReadingGoal, Discount, LoginResult hay Order.

Khung type và chữ ký đã có trong `solution/src/main.rs`. Chỉ thay các `todo!()` bằng phần bạn viết; giữ các case trong `main`. Không có lời giải trong khung.

```bash
cd week-02-10-26/solution
cargo check
cargo run
```

Khung ban đầu được thiết kế để biên dịch; `cargo run` sẽ dừng ở `todo!()` cho tới khi bạn triển khai. Các case chạy tuần tự, nên mỗi lần hãy sửa điểm dừng rồi chạy lại. Khi xuất hiện dòng **Tất cả case đã pass**, có thể nộp để review. Pass các assert là mốc hoàn thành yêu cầu kiểm tra bằng code; review vẫn xem thiết kế, ownership, độ rõ ràng và biên còn thiếu. Không sửa assert để làm bài pass.

## Bài 1 — Đọc enum chứa dữ liệu qua tham chiếu

`Measurement` có ba variant: `Celsius(i32)`, `Offline`, `Invalid(String)`.
Viết `temperature(measurement: &Measurement) -> Option<i32>`:

- `Celsius(t)` chỉ hợp lệ khi `-40 <= t <= 125`: trả `Some(t)`.
- Nhiệt độ ngoài khoảng, `Offline`, `Invalid(...)`: trả `None`.
- Không lấy ownership hay sửa measurement. Chuỗi lỗi có thể rỗng hoặc có dấu.

Ví dụ: `Celsius(25)` → `Some(25)`; `Celsius(-41)` → `None`.
Biên: -40, 125, `i32::MIN`, `i32::MAX`.
Gợi ý: match trên tham chiếu; hãy nhìn kiểu của biến được bind trong arm trước khi dùng.

## Bài 2 — Match đầy đủ và giới hạn cường độ

`Alarm`: `Silent`, `Light`, `Beep(u8)`.
Viết `intensity(alarm: &Alarm) -> u8`: Silent → 0, Light → 1, Beep(n) → n nhưng giới hạn tối đa 10. Beep(0) hợp lệ và trả 0. Dùng `match` liệt kê đầy đủ ba variant, không dùng wildcard thay các variant đã biết.

Ví dụ Beep(7) → 7, Beep(255) → 10. Không sửa alarm.
Gợi ý: enum không tự động là Copy chỉ vì payload là số.

## Bài 3 — Tìm mẫu hợp lệ cuối cùng trong slice

Viết `last_valid(samples: &[Measurement]) -> Option<i32>`.
Trả nhiệt độ của phần tử hợp lệ có chỉ số lớn nhất theo quy tắc bài 1. Không có mẫu hợp lệ (kể cả slice rỗng) thì trả None. Không sửa slice, không tạo collection mới.

Ví dụ `[Celsius(18), Offline, Celsius(23), Invalid("lỗi")]` → Some(23).
Nếu phần tử cuối ngoài khoảng thì vẫn phải tìm mẫu hợp lệ phía trước.
Gợi ý: vòng lặp bình thường và một biến Option là đủ; có thể gọi hàm bài 1 và luyện `if let`.

## Bài 4 — Lệnh có thể bị từ chối

`Calibration { offset: i32 }`, ban đầu offset = 0.
`CalibrationCommand`: `Set(i32)`, `Reset`.

Triển khai `new() -> Self`, `offset(&self) -> i32`, `apply(&mut self, command: &CalibrationCommand) -> bool`:

- Set(v) hợp lệ trong [-10, 10], gán offset = v và trả true.
- Set ngoài khoảng trả false; offset giữ nguyên.
- Reset luôn trả true và đặt offset = 0, cả khi đang là 0.
- Command được mượn; caller có thể dùng lại cùng command.

Biên: -10, 10, -11, 11 và cực trị i32.
Gợi ý: kiểm tra đầu vào trước khi thay đổi field.

## 5 câu hỏi hiểu bản chất

Trả lời trong `solution/answers.md`; đoạn không biên dịch chỉ để trong code fence/comment.

1. Trong `match measurement` khi measurement có kiểu `&Measurement`, biến `t` trong arm `Measurement::Celsius(t)` có kiểu gì? Vì sao trả nhiệt độ bằng giá trị không khiến measurement bị move?
2. Hai đoạn match `Measurement::Invalid(message)` trên giá trị sở hữu và trên `&Measurement` khác nhau thế nào? Khi nào caller còn dùng enum sau match? Minh họa ngắn.
3. Vì sao `last_valid` trả None thay vì dùng 0 làm giá trị báo không có dữ liệu? Với slice có mẫu Celsius(0), hai cách khác nhau thế nào?
4. Vì sao việc chỉ có field i32 chưa đủ để enum/struct tự động là Copy? Giải thích chuyện xảy ra với `let b = a;` khi a là Alarm trong khung hiện tại.
5. Mini project nhận `Command` bằng giá trị, nhưng getter `name()` trả &str. Ai sở hữu String sau Rename thành công? Nếu Rename bị từ chối, dữ liệu của thermostat và String đầu vào có số phận khác nhau thế nào?

## Mini project — Bộ điều nhiệt một phòng

Không kết nối phần cứng, không nhập bàn phím. Dùng các lượt gọi hard-code trong main.

Types:
- `Mode`: Off, Auto, Manual(bool).
- `Command`: SetTarget(i32), Sample(Measurement), SetMode(Mode), Rename(String).
- `Thermostat`: name: String, target: i32, last: Option<i32>, mode: Mode.

### Constructor và getter

`new(name: String) -> Self` nhận bất kỳ tên nào (kể cả rỗng); giữ nguyên tên, target = 22, last = None, mode = Off.
Triển khai `name(&self) -> &str`, `target(&self) -> i32`, `last(&self) -> Option<i32>`, `mode(&self) -> &Mode`.

### Xử lý lệnh

`apply(&mut self, command: Command) -> bool` tiêu thụ command:

| Lệnh | Thành công | Thất bại |
| --- | --- | --- |
| SetTarget(t) | 16 <= t <= 30; cập nhật target | Ngoài khoảng |
| Sample(m) | m hợp lệ theo bài 1; cập nhật last | Offline, Invalid hoặc nhiệt độ ngoài khoảng |
| SetMode(m) | Luôn thành công; thay mode | Không có |
| Rename(s) | s.trim() không rỗng; chuyển String vào name, giữ nguyên chuỗi gốc | Chuỗi rỗng hoặc chỉ khoảng trắng |

**Mọi lệnh thất bại phải giữ nguyên toàn bộ name, target, last và mode.** Lệnh thành công chỉ sửa field tương ứng. Sample lỗi không xóa mẫu tốt trước đó. Đổi mode không xóa target/last. Đổi tên không dùng clone để giữ lại bản sao chuỗi đầu vào. Không cần lưu lại thông báo lỗi.

### Quyết định bật sưởi

`is_heating(&self) -> bool` tính theo trạng thái hiện tại, không lưu thêm field:
- Off: false.
- Manual(on): trả on, không phụ thuộc mẫu hay target.
- Auto: chưa có mẫu → false; có mẫu → true chỉ khi mẫu < target.
- Bằng target thì false. Đây là mô hình bài tập, không có hysteresis.

Ví dụ mặc định → không sưởi; đổi Auto nhưng chưa có mẫu → không sưởi; nhận 18°C với target 22 → sưởi; nhận 22°C → dừng. Sau mẫu 18°C, Sample(Offline) thất bại và vẫn sưởi.

Cases trong main bao phủ các mode, ngưỡng mục tiêu, cực trị nhiệt độ, tên Unicode, tên trống, lỗi mẫu và trạng thái giữ nguyên. Gợi ý: tách xác thực đầu vào khỏi thay đổi trạng thái; mượn self trong getter và is_heating, chuyển ownership ở Rename.

## Tự chọn — Ôn chương 7

Sau khi pass phần bắt buộc, có thể chuyển bài cảm biến vào `src/measurement.rs` và bộ điều nhiệt vào `src/thermostat.rs`, khai báo `mod`, `pub`/ `use` vừa đủ để main vẫn chạy. Có thể giữ các bài còn lại ở main.rs. Không thêm dependency; không sửa hành vi hay case. Trong answers.md mô tả package, binary crate, module tree và lý do từng mục cần public. Không bắt buộc cho lần nộp này.
