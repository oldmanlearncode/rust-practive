# Câu trả lời — tuần 02-10-26

Trạng thái: chưa nộp. Không có câu trả lời hay nhận xét giả định.
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
_Chưa có._
