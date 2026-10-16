use rust_practice_week_16_10_26::{
    borrow_name, parse_amount, parse_batch, read_note, TransferError, Wallet,
};

fn check_basics() {
    assert_eq!(parse_amount(" 42\n"), Ok(42));
    assert_eq!(parse_amount("1"), Ok(1));
    assert_eq!(parse_amount("10000"), Ok(10_000));
    assert_eq!(parse_amount("\u{2003}7\u{2003}"), Ok(7));
    for text in ["", " \t\n", "\u{2003}"] {
        assert_eq!(parse_amount(text), Err(String::from("empty")));
    }
    for text in ["abc", "1.5", "-1", "4 2", "🦀", "4294967296"] {
        assert_eq!(parse_amount(text), Err(String::from("not_u32")));
    }
    for text in ["0", "10001", "4294967295"] {
        assert_eq!(parse_amount(text), Err(String::from("out_of_range")));
    }

    let names = vec![String::from("  An🦀  "), String::from("\u{2003}"), String::new()];
    let borrowed: Result<&str, String> = borrow_name(&names, 0);
    assert_eq!(borrowed, Ok("An🦀"));
    assert_eq!(names[0], "  An🦀  ");
    assert_eq!(borrow_name(&names, 1), Err(String::from("blank")));
    assert_eq!(borrow_name(&names, 2), Err(String::from("blank")));
    assert_eq!(borrow_name(&names, 3), Err(String::from("missing")));
    assert_eq!(borrow_name(&names, usize::MAX), Err(String::from("missing")));
    assert_eq!(borrow_name(&[], 0), Err(String::from("missing")));

    assert_eq!(parse_batch(&[]), Ok(Vec::<u32>::new()));
    let lines = vec![String::from(" 1 "), String::from("10000"), String::from("42")];
    assert_eq!(parse_batch(&lines), Ok(vec![1, 10_000, 42]));
    assert_eq!(lines[0], " 1 ");
    let bad = vec![String::from("2"), String::from("oops"), String::new()];
    assert_eq!(parse_batch(&bad), Err(String::from("not_u32")));
    assert_eq!(bad[1], "oops");
    assert_eq!(parse_batch(&[String::from("0"), String::from("oops")]),
        Err(String::from("out_of_range")));
    assert_eq!(parse_batch(&[String::new()]), Err(String::from("empty")));

    // Wiring cho case file: không cần tự viết các macro đường dẫn này.
    let note = read_note(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/note.txt"));
    match note {
        Ok(text) => assert_eq!(text, "Máy chủ 🦀 sẵn sàng."),
        Err(error) => panic!("Không đọc được fixture hợp lệ: {}", error),
    }
    match read_note(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/empty.txt")) {
        Ok(text) => assert_eq!(text, ""),
        Err(error) => panic!("Không đọc được fixture rỗng: {}", error),
    }
    // Một đường dẫn đi xuyên qua file thường không thể trỏ tới file con.
    assert!(read_note(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/note.txt/child")).is_err());
}

// Helper case, không phải lời giải. Kiểm tra toàn trạng thái quan sát được.
fn unchanged(wallet: &Wallet) {
    assert_eq!(wallet.account_count(), 4);
    assert_eq!(wallet.balance("An"), Some(60));
    assert_eq!(wallet.balance("Bình🦀"), Some(50));
    assert_eq!(wallet.balance("Đầy"), Some(10_000));
    assert_eq!(wallet.balance("Rỗng"), Some(0));
    assert_eq!(wallet.balance("Mất"), None);
    assert_eq!(wallet.balance(""), None);
    assert_eq!(wallet.history().len(), 1);
    assert_eq!(wallet.history()[0].from(), "An");
    assert_eq!(wallet.history()[0].to(), "Bình🦀");
    assert_eq!(wallet.history()[0].amount(), 40);
}

fn check_wallet() {
    let mut wallet = Wallet::new();
    assert_eq!(wallet.account_count(), 0);
    assert_eq!(wallet.balance("An"), None);
    assert!(wallet.history().is_empty());
    assert!(wallet.sent_counts().is_empty());
    assert!(!wallet.open(String::from("\u{2003}"), 0));
    assert!(!wallet.open(String::from("Quá"), u32::MAX));
    assert_eq!(wallet.account_count(), 0);
    assert!(wallet.open(String::from("An"), 100));
    assert!(wallet.open(String::from("Bình🦀"), 10));
    assert!(wallet.open(String::from("Đầy"), 10_000));
    assert!(wallet.open(String::from("Rỗng"), 0));
    assert!(!wallet.open(String::from("An"), 900));
    assert_eq!(wallet.balance("An"), Some(100));
    assert!(wallet.history().is_empty());
    match wallet.transfer(String::from("An"), String::from("Bình🦀"), 40) {
        Ok(index) => assert_eq!(index, 0),
        Err(_) => panic!("Chuyển hợp lệ phải thành công"),
    }
    unchanged(&wallet);

    assert!(matches!(wallet.transfer(String::new(), String::new(), 0), Err(TransferError::InvalidName)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("An"), String::from("\u{2003}"), 1), Err(TransferError::InvalidName)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("Mất"), String::from("Mất"), 0), Err(TransferError::SameAccount)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("Mất"), String::from("An"), 0), Err(TransferError::ZeroAmount)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("Mất"), String::from("Khác"), 1), Err(TransferError::MissingSource)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("An"), String::from("Mất"), 61), Err(TransferError::MissingTarget)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("An"), String::from("Đầy"), 61), Err(TransferError::InsufficientFunds)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("Rỗng"), String::from("An"), 1), Err(TransferError::InsufficientFunds)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("An"), String::from("Đầy"), 1), Err(TransferError::TargetLimit)));
    unchanged(&wallet);
    assert!(matches!(wallet.transfer(String::from("An"), String::from("Bình🦀"), u32::MAX), Err(TransferError::InsufficientFunds)));
    unchanged(&wallet);

    let old_counts = wallet.sent_counts();
    assert_eq!(old_counts.len(), 1);
    assert_eq!(old_counts.get("An"), Some(&1));
    match wallet.transfer(String::from("An"), String::from("Rỗng"), 60) {
        Ok(index) => assert_eq!(index, 1),
        Err(_) => panic!("Được chuyển hết số dư"),
    }
    assert_eq!(wallet.balance("An"), Some(0));
    assert_eq!(wallet.balance("Rỗng"), Some(60));
    assert_eq!(wallet.history()[1].from(), "An");
    assert_eq!(wallet.history()[1].to(), "Rỗng");
    assert_eq!(wallet.history()[1].amount(), 60);
    assert_eq!(old_counts.get("An"), Some(&1));
    assert_eq!(wallet.sent_counts().get("An"), Some(&2));

    let mut boundary = Wallet::new();
    assert!(boundary.open(String::from(" A "), 1));
    assert!(boundary.open(String::from("A"), 9_999));
    match boundary.transfer(String::from(" A "), String::from("A"), 1) {
        Ok(index) => assert_eq!(index, 0),
        Err(_) => panic!("Hai tên khác nhau; đích được đạt đúng giới hạn"),
    }
    assert_eq!(boundary.balance(" A "), Some(0));
    assert_eq!(boundary.balance("A"), Some(10_000));
    match boundary.transfer(String::from("A"), String::from(" A "), 10_000) {
        Ok(index) => assert_eq!(index, 1),
        Err(_) => panic!("Chuyển ngược hợp lệ"),
    }
    let counts = boundary.sent_counts();
    assert_eq!(counts.len(), 2);
    assert_eq!(counts.get(" A "), Some(&1));
    assert_eq!(counts.get("A"), Some(&1));
    assert_eq!(boundary.account_count(), 2);
    assert_eq!(boundary.history().len(), 2);
    assert_eq!(boundary.balance("A"), Some(0));
    assert_eq!(boundary.balance(" A "), Some(10_000));
}

fn main() {
    check_basics();
    check_wallet();
    println!("Tất cả case đã pass — có thể nộp bài để review.");
}
