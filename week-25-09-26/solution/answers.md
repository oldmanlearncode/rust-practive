# Câu trả lời và trao đổi — tuần 25-09-26

Các đoạn **Người học** dưới đây giữ nguyên lời đã gửi. Nhận xét của trợ giảng được tóm tắt. Code bài tập và lịch sử sửa nằm ở `src/main.rs` và `review.md`.

## Phạm vi học và bài 1

**Người học (phạm vi):**

> Tôi nhớ là tôi mới học tới chương 5 của the rust book [https://doc.rust-lang.org/stable/book/](https://doc.rust-lang.org/stable/book/) và vector thì thuộc chương 8 mà nhỉ ? Cho nên bài 2 và các câu hỏi của vector tôi sẽ không làm.

**Nhận xét:** Đúng. Bài `Vec` và câu hỏi của nó được bỏ; mini project được đổi thành một sách với mảng cố định.

**Câu hỏi bổ sung sau bài 1:** Nếu bỏ điều kiện `if values.len() <= 1`, chuyện gì xảy ra với slice rỗng và slice một phần tử?

**Trạng thái:** Chưa có câu trả lời trong cuộc trao đổi.

## Bài 4 và câu hỏi 3 — phạm vi mượn

**Người học (bài 4):**

> Nếu khai báo và gán giá trị trả về thử method title() và dùng biến đó để dùng lại sau khi đổi tên thì sẽ báo lỗi, vì khi gán cho biến mới, tức là biến mới đang mượn, lần mượn này khai báo trước rename, nhưng sử dụng lần cuối cùng sau rename thì ko hợp lệ.
> Tuy nhiên nếu ko gán cho biến mà trực tiếp in ra từ method title() thì vẫn compile được. do khi kết thúc method title ko được gán cho biến nào nên việc mượn kết thúc ở lần đầu tiên, lầu sau là một lần mượn mới.
> println!("book title: {}", my_book.title());

**Người học (câu hỏi 3):**

> tương tự như bài tập 4. Nếu dùng name trước khi rename thì sẽ hợp lệ vì chỉ cần lần dùng cuối cùng trước khi &mut là được

**Nhận xét:** Kết luận đúng: lần mượn `&str` còn được dùng sau `rename(&mut goal, ...)` sẽ xung đột; nếu dùng lần cuối trước khi đổi tên thì hợp lệ. `println!("{}", goal.title())` cũng tạo một tham chiếu tạm, được dùng xong ở lệnh in. Chưa có code hàm `rename` được nộp.

## Câu hỏi 4 — struct update

**Người học:**

> field title sẽ bị move, dùng goal ko thực hiện các thao tác liên quan đến title thì vẫn được.

**Nhận xét:** `title: String` bị move; các field `u32` còn có thể truy cập riêng. Không dùng `goal` nguyên vẹn cho method nhận `&self` sau khi field `title` đã bị move.

**Câu hỏi bổ sung:** Nếu gán `title: goal.title.clone()` khi struct update, `goal.remaining()` còn gọi được không?

**Trạng thái:** Chưa có câu trả lời trong cuộc trao đổi.

## Câu hỏi bổ sung về số học và nhật ký

**Câu hỏi:** Vì sao khi `pages <= remaining()` và `read_pages <= target_pages <= u32::MAX`, phép cộng `read_pages + pages` không thể tràn?

**Trạng thái:** Chưa có câu trả lời trực tiếp; người học đã sửa code để kiểm tra số trang còn lại trước khi cộng.

**Câu hỏi:** Nếu cập nhật `daily_pages[day]` rồi `ReadingGoal::read(pages)` trả `false`, dữ liệu sẽ sai như thế nào?

**Trạng thái:** Chưa có câu trả lời trực tiếp; code cập nhật mảng chỉ sau khi `read` thành công.

## `String`, `&str` và ownership — hỏi đáp sau mini project

**Câu hỏi:** Vì sao `reading_progress()` / `reading_rate()` trả về `String` thay vì `&str`?

**Người học:**

> method trả về String, vì method này tạo ra một string mới, ReadingGoal không chỉ sở hữu thông tin read_pages và target_pages chứ ko sỡ hữu string nào chưa thông tin trên. Nên tôi tạo ra một string mới, cho nên nếu trả về &str thì khi ra khỏi scope của method ownership của &str ở trong method đã bị drop, cho nên tôi trả về String để chuyển ownership cho caller.

**Nhận xét:** Ý chính đúng: chuỗi do `format!` tạo mới cần chuyển ownership cho caller. Hiệu chỉnh thuật ngữ: `&str` không sở hữu dữ liệu để bị drop; nếu trả tham chiếu vào `String` cục bộ, chính `String` đó bị drop khi method kết thúc.

**Câu hỏi:** `let progress = goal.reading_progress();` rồi `goal.read(5)` thì `progress` có tự thay đổi không?

**Người học:**

> Không, vì reading_progress() trả về một String mới, với giá trị mượn từ ReadingGoal lúc gọi method thôi, khi kết thúc method trả ra String thì ko có liên quan gì đến giá trị của ReadingGoal nữa. Nên các giá trị read_pages thay đổi thì biến progress cũng ko thay đổi.

**Nhận xét:** Đúng. `&self` chỉ mượn khi method chạy; hai field `u32` được đọc để tạo `String` độc lập.

**Câu hỏi:** `let next = progress;` rồi in `progress` có được không? Nếu `progress: &str` từ `goal.title()` thì sao?

**Người học:**

> nếu dùng let next= progress, sau đó in progress sẽ ko được, vì quyền ownership đã move sang next và progress lúc này ko giữ bất kỳ dữ liệu nào. nếu progress là &str thì lại được vì tất cả cũng mượn và trỏ tới chung địa chỉ, quyền owner vẫn còn ở goal.title

**Nhận xét:** Đúng về kết quả. Với `String`, `progress` không còn hợp lệ sau move. Với `&str`, tham chiếu được copy và cả hai cùng đọc dữ liệu mà `goal` sở hữu; `goal.title()` là method mượn, không phải chủ sở hữu.
