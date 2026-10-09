// Case của đề do trợ giảng chuẩn bị; chưa có lời giải của người học.
use rust_practice_week_09_10_26::{
    add_stock, label_at, take_label, unicode_prefix, word_counts, Board, TicketStatus,
};
use std::collections::HashMap;

fn main() {
    println!("Bài 1 — get và lấy ownership");
    let mut labels = vec![String::from("Đỏ"), String::from("Xanh"), String::from("Vàng")];
    let borrowed = label_at(&labels, 0);
    assert_eq!(borrowed, Some("Đỏ")); // Lần dùng cuối, trước khi mượn mutable.
    assert_eq!(label_at(&labels, 2), Some("Vàng"));
    assert_eq!(label_at(&labels, 3), None);
    assert_eq!(label_at(&labels, usize::MAX), None);
    assert_eq!(take_label(&mut labels, 1), Some(String::from("Xanh")));
    assert_eq!(labels, vec![String::from("Đỏ"), String::from("Vàng")]);
    assert_eq!(take_label(&mut labels, 2), None);
    assert_eq!(take_label(&mut labels, usize::MAX), None);
    assert_eq!(labels, vec![String::from("Đỏ"), String::from("Vàng")]);
    assert_eq!(take_label(&mut labels, 1), Some(String::from("Vàng")));
    assert_eq!(take_label(&mut labels, 0), Some(String::from("Đỏ")));
    assert!(labels.is_empty());
    assert_eq!(label_at(&labels, 0), None);
    assert_eq!(take_label(&mut labels, 0), None);
    labels.push(String::new());
    assert_eq!(label_at(&labels, 0), Some(""));
    assert_eq!(take_label(&mut labels, 0), Some(String::new()));
    assert!(labels.is_empty());

    println!("Bài 2 — Unicode");
    let text = String::from("Đỏ🦀a");
    assert_eq!(unicode_prefix(&text, 0), "");
    assert_eq!(unicode_prefix(&text, 1), "Đ");
    assert_eq!(unicode_prefix(&text, 2), "Đỏ");
    assert_eq!(unicode_prefix(&text, 3), "Đỏ🦀");
    assert_eq!(unicode_prefix(&text, 4), "Đỏ🦀a");
    assert_eq!(unicode_prefix(&text, usize::MAX), "Đỏ🦀a");
    assert_eq!(text, "Đỏ🦀a");
    assert_eq!(unicode_prefix("", 3), "");
    assert_eq!(unicode_prefix("abc", 2), "ab");
    assert_eq!(unicode_prefix("e\u{301}x", 1), "e");
    assert_eq!(unicode_prefix("e\u{301}x", 2), "e\u{301}");
    assert_eq!(unicode_prefix(" \n", 1), " ");

    println!("Bài 3 — entry đếm từ");
    assert!(word_counts("").is_empty());
    assert!(word_counts(" \t\n ").is_empty());
    let source = String::from("Rust rust Rust\n bánh\t bánh 🦀 🦀");
    let counts = word_counts(&source);
    assert_eq!(counts.len(), 4);
    assert_eq!(counts.get("Rust"), Some(&2));
    assert_eq!(counts.get("rust"), Some(&1));
    assert_eq!(counts.get("bánh"), Some(&2));
    assert_eq!(counts.get("🦀"), Some(&2));
    assert_eq!(counts.get("khác"), None);
    assert_eq!(source, "Rust rust Rust\n bánh\t bánh 🦀 🦀");
    let punctuation = word_counts("bánh bánh, bánh");
    assert_eq!(punctuation.len(), 2);
    assert_eq!(punctuation.get("bánh"), Some(&2));
    assert_eq!(punctuation.get("bánh,"), Some(&1));

    println!("Bài 4 — tồn kho giới hạn");
    let mut stock = HashMap::new();
    let invalid_items = [String::new(), String::from(" \t\n"), String::from("\u{2003}")];
    for item in invalid_items {
        assert!(!add_stock(&mut stock, item, 1));
        assert!(stock.is_empty());
    }
    for quantity in [0, 101, u32::MAX] {
        assert!(!add_stock(&mut stock, String::from("Bút"), quantity));
        assert!(stock.is_empty());
    }
    assert!(add_stock(&mut stock, String::from("Bút"), 40));
    assert!(add_stock(&mut stock, String::from("Bút"), 60));
    assert_eq!(stock.get("Bút"), Some(&100));
    for quantity in [0, 1, 101, u32::MAX] {
        assert!(!add_stock(&mut stock, String::from("Bút"), quantity));
        assert_eq!(stock.len(), 1);
        assert_eq!(stock.get("Bút"), Some(&100));
    }
    assert!(add_stock(&mut stock, String::from(" Vở "), 1));
    assert_eq!(stock.get(" Vở "), Some(&1));
    assert_eq!(stock.get("Vở"), None);
    assert_eq!(stock.get("Bút"), Some(&100));
    // Hàm nhận map bất kỳ: tồn kho đã vượt giới hạn cũng phải được từ chối an toàn.
    stock.insert(String::from("Cũ"), u32::MAX);
    assert!(!add_stock(&mut stock, String::from("Cũ"), 1));
    assert_eq!(stock.get("Cũ"), Some(&u32::MAX));
    assert_eq!(stock.len(), 3);
    stock.insert(String::from("Rỗng"), 0);
    assert!(add_stock(&mut stock, String::from("Rỗng"), 100));
    assert_eq!(stock.get("Rỗng"), Some(&100));
    assert_eq!(stock.get(" Vở "), Some(&1));
    assert_eq!(stock.get("Bút"), Some(&100));

    println!("Mini project — sổ yêu cầu hỗ trợ");
    let mut board = Board::new();
    assert_eq!(board.len(), 0);
    assert!(board.ticket(0).is_none());
    assert!(board.ticket(usize::MAX).is_none());
    assert!(!board.close(0));
    assert!(board.open_counts().is_empty());

    // Các cặp được sở hữu bởi Vec; for chuyển từng String vào submit.
    let submissions = vec![
        (String::from("Sửa API"), String::from("Backend")),
        (String::from("Sửa đăng nhập"), String::from("Backend")),
        (String::from("Đổi màu 🦀"), String::from("Frontend")),
        (String::from("  Kiểm tra CI  "), String::from(" Backend ")),
    ];
    for (title, team) in submissions {
        assert!(board.submit(title, team));
    }
    assert_eq!(board.len(), 4);
    let first = match board.ticket(0) { Some(ticket) => ticket, None => panic!("Phải có ticket đầu") };
    assert_eq!(first.title(), "Sửa API");
    assert_eq!(first.team(), "Backend");
    match first.status() {
        TicketStatus::Open => {}
        TicketStatus::Closed => panic!("Ticket mới phải Open"),
    }
    let padded = match board.ticket(3) { Some(ticket) => ticket, None => panic!("Phải có ticket 3") };
    assert_eq!(padded.title(), "  Kiểm tra CI  ");
    assert_eq!(padded.team(), " Backend ");
    let before = board.open_counts();
    assert_eq!(before.len(), 3);
    assert_eq!(before.get("Backend"), Some(&2));
    assert_eq!(before.get("Frontend"), Some(&1));
    assert_eq!(before.get(" Backend "), Some(&1));
    assert_eq!(before.get("Không có"), None);

    let rejected = [
        ("", "Backend"), (" \t\n", "Backend"),
        ("Hợp lệ", ""), ("Hợp lệ", "\u{2003}"),
    ];
    for (title, team) in rejected {
        assert!(!board.submit(String::from(title), String::from(team)));
        assert_eq!(board.len(), 4);
        assert_eq!(board.open_counts(), before);
        let ticket = match board.ticket(0) { Some(ticket) => ticket, None => panic!("Không được mất ticket") };
        assert_eq!(ticket.title(), "Sửa API");
        assert_eq!(ticket.team(), "Backend");
        match ticket.status() {
            TicketStatus::Open => {}
            TicketStatus::Closed => panic!("Submit lỗi không được đổi trạng thái"),
        }
    }
    for index in [4, usize::MAX] {
        assert!(!board.close(index));
        assert_eq!(board.len(), 4);
        assert_eq!(board.open_counts(), before);
    }
    assert!(board.close(0));
    let first = match board.ticket(0) { Some(ticket) => ticket, None => panic!("Đóng không được xóa ticket") };
    assert_eq!(first.title(), "Sửa API");
    assert_eq!(first.team(), "Backend");
    match first.status() {
        TicketStatus::Open => panic!("Phải chuyển sang Closed"),
        TicketStatus::Closed => {}
    }
    let after = board.open_counts();
    assert_eq!(after.get("Backend"), Some(&1));
    assert_eq!(after.get("Frontend"), Some(&1));
    assert_eq!(after.get(" Backend "), Some(&1));
    assert_eq!(before.get("Backend"), Some(&2)); // Thống kê cũ là map độc lập.
    assert!(!board.close(0));
    assert_eq!(board.open_counts(), after);
    assert_eq!(board.len(), 4);
    assert!(board.close(1));
    let remaining = board.open_counts();
    assert_eq!(remaining.get("Backend"), None); // Không giữ key có đếm 0.
    assert_eq!(remaining.len(), 2);
    assert!(board.close(2));
    assert!(board.close(3));
    assert!(board.open_counts().is_empty());
    assert_eq!(board.len(), 4);
    assert!(board.submit(String::from("Sửa API"), String::from("Backend")));
    assert_eq!(board.len(), 5); // Cho phép tiêu đề trùng.
    assert_eq!(board.open_counts().get("Backend"), Some(&1));
    assert!(board.ticket(5).is_none());

    println!("Tất cả case đã pass — có thể nộp bài để review.");
}
