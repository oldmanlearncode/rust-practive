# Câu trả lời — tuần 02-10-26

Trạng thái: **đã review lần 2 ngày 05/10/2026; thực hành hoàn thành, xem nhận xét câu trả lời bổ sung bên dưới**.
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

### Câu hỏi kiểm tra bổ sung — Đã trả lời, nhận xét ở review lần 2

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

## Review lần 2 — 05/10/2026, câu trả lời bổ sung

Giữ nguyên toàn bộ lời trả lời của người học ở trên.

| Phần | Đánh giá |
| --- | --- |
| Câu bổ sung 1 — slice rỗng | Đúng và đủ |
| Câu bổ sung 2 — last()/mode() | Đúng hướng về Copy, nhưng nhầm kiểu self.last |
| Câu bổ sung 3 — trim()/move | Đúng dự đoán lỗi, cần nhấn mạnh lần sử dụng cuối của borrow |
| Câu bổ sung 4 — pub | Đúng yêu cầu |
| Bài thử Invalid(_) trên giá trị sở hữu | Chưa minh họa đúng ngữ cảnh: code hiện tại match trên tham chiếu |

### 1. Slice rỗng

Giải thích đúng: for chạy 0 lần, result giữ None. Đọc lại measurement.rs thấy đã bỏ điều kiện is_empty() dư; hành vi này phù hợp giải thích. Lần này không chạy lại code; đang review câu trả lời và kiểm tra thay đổi source nhỏ bằng đọc code.

### 2. Copy của getter

Cần sửa đúng tên kiểu trong giải thích: self.last là Option<i32>, không phải i32.
Option<T> implement Copy khi T: Copy; vì i32 là Copy nên toàn bộ Option<i32> được copy, cả trường hợp Some lẫn None.
Mode hiện không implement Copy, nên không thể move self.mode ra khỏi &self. Compiler từ chối phép move; không có việc field bị lấy ra rồi mới phát hiện lỗi. Getter hiện trả &Mode để mượn.
Không phải mọi enum đều không Copy: Option<i32> cũng là enum. Hãy xét implement của chính type và điều kiện của nó.

### 3. trim(), borrow và move

Ownership vẫn thuộc s khi gọi trim(), nhưng riêng điều đó chưa đủ để cho phép move. Chủ sở hữu có thể đang bị mượn và chưa được phép move.
Trong code hiện tại, lần dùng cuối của new_name là new_name.is_empty(); borrow kết thúc trước self.name = s nên move hợp lệ.
Nếu thêm một lần dùng new_name sau phép gán, borrow phải còn hiệu lực qua phép gán; compiler từ chối move s ngay tại đó (E0505: cannot move out because it is borrowed).
Không cần hình dung String/heap bị drop hoặc tham chiếu đã hỏng rồi Rust mới báo lỗi: việc move String không tự drop heap; lỗi là move owner trong khi borrow còn được dùng.

### 4. Visibility

Đã giải thích đúng mục đích pub cho type/hàm/method dùng ngoài module và private cho fields để kiểm soát cập nhật qua method.
Dùng thuật ngữ “variant” của enum thay cho “thể hiện” sẽ chính xác hơn (thể hiện thường chỉ một giá trị/instance).
Phần giải thích bổ sung này đáp ứng yêu cầu lý do pub. Ghi chú phân biệt binary crate với crate root ở review lần 1 vẫn áp dụng.

### 5. Bài thử Invalid(_)

Bạn nhận xét đúng rằng bài hiện tại có Measurement::Invalid(_).
Tuy nhiên temperature() nhận &Measurement và đang match &measurement, nên không thể dùng code này để chứng minh underscore không move String trong một match trên giá trị sở hữu. Đó là hai ngữ cảnh khác nhau.

Bài thử nhỏ trên bản riêng, không đổi source/case đã nộp:
- Tạo Measurement::Invalid(String::from("lỗi")) và match trực tiếp biến sở hữu.
- Dùng arm Invalid(_) và các arm kết thúc bình thường; thử mượn lại enum sau match.
- Đổi thành Invalid(message), dùng message trong arm rồi thử mượn lại enum sau match.
- Trước khi chạy, dự đoán hai kết quả và giải thích khác biệt.

### Câu hỏi chốt — Chưa trả lời

1. Option<i32> và Option<String>: kiểu nào Copy, và vì sao?
2. Vì sao “s vẫn sở hữu String” chưa đủ để move s, nếu new_name còn được dùng sau đó?

Kết luận: phần thực hành tuần 02-10-26 đã hoàn thành; lý do pub đã bổ sung đúng. Hai điểm cần củng cố trong diễn đạt là Option<i32>: Copy và borrow kết thúc ở lần sử dụng cuối. Không cần viết lại mini project.

Tài liệu:
- [Option: impl Copy khi T: Copy](https://doc.rust-lang.org/std/option/enum.Option.html#impl-Copy-for-Option%3CT%3E)
- [References and Borrowing: phạm vi tham chiếu và lần sử dụng cuối](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
- [E0505: move khi đang được mượn](https://doc.rust-lang.org/error_codes/E0505.html)

## Hỏi đáp tiếp — 05/10/2026: Copy của Option

Câu hỏi: Option<i32> và Option<String> — kiểu nào là Copy, vì sao?

Trả lời nguyên văn của người học:

> `Option<i32>`  vì có implement Copy cho i32 theo mặc định còn String thì không .

Nhận xét: Đúng. Option<T> implement Copy khi T: Copy. Vì i32: Copy nên Option<i32>: Copy; String không Copy nên Option<String> không Copy. Đã chốt câu hỏi này; câu hỏi về borrow và bài thử match sở hữu ở review lần 2 vẫn là bài củng cố riêng.

## Hỏi đáp tiếp — 05/10/2026: Option<&str>

Câu hỏi: Nếu a: Option<&str>, phép gán let b = a copy hay move?

Trả lời nguyên văn của người học:

> Kiểu &str có implement Copy nên sẽ copy chứ ko move khi thực hiện gán Option<&str>

Nhận xét: Đúng. &str: Copy nên Option<&str>: Copy. Phép gán copy Option và tham chiếu bên trong nếu là Some; không sao chép nội dung chuỗi và không chuyển ownership của String được mượn. Hai tham chiếu vẫn chịu quy tắc borrowing/lifetime của dữ liệu gốc. Đã chốt phần Copy của Option.
