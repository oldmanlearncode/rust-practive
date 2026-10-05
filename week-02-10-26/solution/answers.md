# Câu trả lời — tuần 02-10-26

Trạng thái: **đã nộp, đã review lần 1 ngày 05/10/2026**.
Điền câu trả lời dưới từng mục. Khi review, lời người học được giữ nguyên; nhận xét và hỏi đáp bổ sung được ghi riêng.

## Câu 1 — Binding khi match tham chiếu
biến t trong arm Mesurement::Celsius(t) có kiểu &i32. Vì kiểu i32 có implement Copy nên sẽ không bị move ownership mà copy value sang

## Câu 2 — Match trên giá trị sở hữu và giá trị mượn
khi dùng Mesurement::Invalid(message) thì String message sẽ bị move, còn trên &Mesurement sẽ chỉ mượn giá trị để đọc. Cho nên sau &Mesurement vẫn còn có thể sử dụng enum sau match, còn khi dùng Mesurement::Invalid(message) thì không thể được.
VD: 
`match mesurement {
    Mesurement::Celsius(t) => todo!(),
    Mesurement::Offline => todo!(),
    Mesurement::Invalid(message) => todo!(),
    }
    `
    thì sau đó mesurement sẽ không còn sử dụng được nữa.

## Câu 3 — None và nhiệt độ 0
Trả về None sẽ rõ ràng về mặt ý nghĩa rằng không có giá trị hoặc không hợp lệ, nếu có giá trị Celsius(0) thì sẽ hợp lệ nhưng nếu ta chọn trả ra 0 để báo ko có dữ liệu thì sẽ không phân biệt được trường hợp này là giá trị bằng 0 hay là không có dữ liệu

## Câu 4 — Move của Alarm
Để struct/enum có tự động Copy khi dùng phép gán = thì cần phải thêm derive Copy, Clone. Bởi vì Rust cần khai báo tường minh để compiler biết chắc chắn là người dùng muốn có tự động Copy, đồng thời cũng sẽ tránh lỗi khi vô tình thêm các kiểu không có implement Copy
Khi viết `let b = a;` thì sau đó ownership của a sẽ được move sang b, và không còn sử dụng a được nữa

## Câu 5 — Ownership của Rename
 Sở hữu cuối cùng của String sau Rename thành công là Thermostat.name. Nếu bị lỗi khi Rename, thì thermostat vẫn giũ giá trị String hiện có của nó, còn String đầu vào sẽ bị mất, do đã bị move vào khi xử lý Rename.

## Chương 7 tự chọn — Module tree
package: rust-practice-week-02-10-26 
binary crate: main.rs
module:
mesurement
|-Mesurement
|-temperature()
|-last_valid()

thermostat
|-Mode
|-Command
|-Thermostat
|-Thermostat::new
|-Thermostat.apply()
|-Thermostat.is_heating()

## Hỏi đáp bổ sung và nhận xét
Nhận xét lần 1 và câu hỏi bổ sung nằm bên dưới.

## Review lần 1 — 05/10/2026

Lời trả lời phía trên được giữ nguyên. Phần dưới là nhận xét của trợ giảng.
Code tham chiếu: commit `a80ef7f18ae5e8ebc6af5191fee3cfb27febed9b`.
Kết quả: phần code đạt yêu cầu; các câu trả lời đúng ý chính, cần làm rõ một số thuật ngữ.

### Câu 1 — Đúng, bổ sung cơ chế binding

`t: &i32` vì match trên `&Measurement`: Rust tự điều chỉnh binding sang mượn khi pattern khớp với giá trị tham chiếu (match ergonomics). Không phải vì `i32: Copy` nên `t` có kiểu tham chiếu.
Ở `Some(*t)`, dereference cho phép đọc giá trị i32; i32 là Copy nên giá trị được copy vào Some. Measurement vẫn được mượn, không bị move.
Code hiện tại viết `match &measurement` nên đầu vào match là `&&Measurement`; vẫn chạy đúng nhờ tự dereference, nhưng `match measurement` đã đủ.

### Câu 2 — Đúng ý chính, tránh khái quát mọi match sở hữu đều move

Trên giá trị sở hữu, binding `Invalid(message)` lấy String bằng giá trị, nên move field vào message. Trên `&Measurement`, binding message là `&String`, không lấy ownership.
Sau một match có nhánh move field String, không thể dùng lại biến như một Measurement nguyên vẹn. Tuy nhiên bản thân việc match trên giá trị sở hữu không mặc nhiên move mọi field: pattern `Invalid(_)` không bind/move String, hoặc binding `ref message` mượn nó.
Đoạn minh họa hiện có dùng todo!() ở mọi arm, nên luồng không quay lại câu lệnh phía sau nếu chạy; hãy dùng thân arm kết thúc bình thường khi thử chứng minh lỗi dùng sau move.

### Câu 3 — Đúng và đủ

None thể hiện không có mẫu hợp lệ; Some(0) là dữ liệu hợp lệ bằng 0. Không dùng một giá trị hợp lệ làm sentinel báo thiếu dữ liệu.

### Câu 4 — Đúng với khung hiện tại, bổ sung điều kiện Copy

Alarm hiện không implement Copy nên `let b = a;` move ownership, a không còn hợp lệ để dùng.
`#[derive(Copy, Clone)]` là cách thông dụng, không phải cách duy nhất (có thể implement thủ công khi đủ điều kiện). Copy yêu cầu Clone và mọi field phải Copy; kiểu có Drop không thể Copy.
Nếu sau này thêm String vào enum đã derive Copy, compiler từ chối derive, chứ không tự chuyển enum về kiểu move. Ở phạm vi tuần này chỉ cần hiểu khai báo Copy là lựa chọn tường minh.

### Câu 5 — Đúng, dùng từ drop thay cho “bị mất”

Rename thành công: s được move vào self.name; String tên cũ bị drop khi thay thế. s.trim() chỉ tạo &str để kiểm tra, không sửa chuỗi gốc. Lần mượn kết thúc trước lệnh move s nhờ lần sử dụng cuối.
Rename thất bại: thermostat giữ nguyên tên cũ; String đầu vào đã thuộc command/binding s và bị drop khi hàm trả về. Caller không lấy lại được nó với chữ ký trả bool.
Getter name() chỉ trả &str mượn từ self.name, không chuyển ownership.

### Chương 7 tự chọn — Code đạt; mô tả thiếu lý do pub

Package: rust-practice-week-02-10-26, được định nghĩa bởi Cargo.toml.
Binary crate có tên tương ứng package trong cấu hình hiện tại; src/main.rs là crate root, không phải tên crate.
Hai module con trực tiếp của crate root: measurement và thermostat. Phần mô tả phía trên viết “mesurement”/“Mesurement”; tên thực tế là measurement/Measurement.
Các method thuộc impl Thermostat, không phải các module con; mô tả còn thiếu name(), target(), last(), mode().

Lý do visibility:
- `mod measurement;` và `mod thermostat;` khai báo hai module; không cần pub mod vì sử dụng bên trong binary crate.
- Measurement, temperature, last_valid cần pub để main hoặc module thermostat truy cập item của module measurement.
- Thermostat, Mode, Command cần pub vì main dùng các type/variant đó.
- Các constructor/getter/apply/is_heating cần pub để main gọi từ ngoài module thermostat.
- Field của Thermostat giữ private để main không cập nhật trực tiếp trạng thái.
- Variant của pub enum tự có visibility của enum; không thêm pub riêng vào từng variant.
- `use crate::measurement::*;` chạy đúng. Có thể dùng import tường minh để nhìn rõ module phụ thuộc type/hàm nào; đây là góp ý, không phải lỗi.

### Câu hỏi kiểm tra bổ sung — Chưa trả lời

1. Nếu bỏ `if samples.is_empty()` trong last_valid(), kết quả với slice rỗng có đổi không? Giải thích theo vòng for và giá trị result ban đầu.
 Trả lời: không đổi, vì nếu samples rỗng thì vòng for sẽ không chạy cho nên vẫn lấy giá trị result ban đầu là None.
2. Vì sao last() có thể trả `self.last` từ `&self`, còn mode() hiện trả `&Mode`? Cả hai type có đặc điểm Copy gì?
 Trả lời: self.last có kiểu i32, còn self.mode có kiểu enum Mode, i32 có triển khai Copy theo mặc định còn Mode thì mặc định là không, cho nên khi trả trực tiếp self.last giá trị sẽ được Copy mà ko bị move ra khỏi last, ngược lại Mode chưa có triển khai Copy cho nên giá trị cần dùng con trỏ để mượn giá trị, nếu ko giá trị sẽ bị move khởi mode
3. Trong Rename, sau `let new_name = s.trim();`, vì sao vẫn move được s vào self.name? Nếu dùng new_name sau phép gán đó thì chuyện gì xảy ra?
 Trả lời: vì method trim() chỉ nhận tham chiếu &self(String) và trả về một &str, nên sau khi mượn xong thì Ownership vẫn ở s nên có thể move vào self.name được.
 Nếu dùng new_name sau phép gán sẽ bị lỗi vì giá trị của s đã bị move, cho nên &str từ s.trim() không còn hợp lệ nữa. 
4. Bổ sung lý do pub theo lời của bạn cho phần module tree.
 Trả lời: 
 measurement: cần public enum Measurement(chỉ cần pub cho enum, không cần cho các thể hiện của enum, vì sẽ có cùng pub với enum), fn temperature, fn last_valid vì enum và các function này được dùng ở nơi khác ngoài module,
 thermostat: cần public Thermostat, Mode, Command, vì main cần dùng; các method của struct Thermostat, còn các fields được giữ private, để đảm bảo các thông tin được cập nhật qua method với validate hoặc xử lý trước ...


Bài luyện ngắn: trên một bản thử riêng, thay `Invalid(message)` bằng `Invalid(_)` trong một match sở hữu có các arm kết thúc bình thường; thử dùng lại measurement sau match và giải thích kết quả. Giữ nguyên case của bài nộp.
 Trả lời: thực tế tôi đang dùng Measurement::Invalid(_) trong case bài tập của tôi. 

### Tài liệu chính thống

- [Patterns: binding modes và match ergonomics](https://doc.rust-lang.org/reference/patterns.html#binding-modes)
- [Copy](https://doc.rust-lang.org/std/marker.Copy.html)
- [Borrowing và lần sử dụng cuối](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [Visibility và đường dẫn module](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)
