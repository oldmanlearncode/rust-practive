use std::collections::HashMap;
use rust_practice_week_23_10_26::{validate_title, take_document, insert_unique,
    InsertError, TakeError, Spool, JobState, QueueError};

// Panic trong harness chỉ báo kết quả sai; không dùng panic xử lý input trong implementation.
fn assert_spool(s: &Spool, titles: &[&str], pages: &[u32], states: &[u8], pending: u32) {
    assert_eq!(s.len(), titles.len());
    assert_eq!(s.pending_pages(), pending);
    for i in 0..titles.len() {
        let job = match s.job(i) { Some(j) => j, None => panic!("Thiếu job") };
        assert_eq!(job.title(), titles[i]);
        assert_eq!(job.pages(), pages[i]);
        let state = match job.state() {
            JobState::Queued => 0, JobState::Printed => 1, JobState::Cancelled => 2,
        };
        assert_eq!(state, states[i]);
    }
    assert!(s.job(titles.len()).is_none());
    assert!(s.job(usize::MAX).is_none());
}
fn main() {
    assert_eq!(validate_title("  Đỏ🦀  "), Ok(()));
    assert_eq!(validate_title("\u{2003}\n"), Err(String::from("blank")));
    assert_eq!(validate_title(""), Err(String::from("blank")));
    assert_eq!(validate_title("12345678901234567890"), Ok(()));
    assert_eq!(validate_title("123456789012345678901"), Err(String::from("too_long")));
    assert_eq!(validate_title(&"🦀".repeat(20)), Ok(()));
    assert_eq!(validate_title(&"🦀".repeat(21)), Err(String::from("too_long")));
    assert_eq!(validate_title(&"e\u{301}".repeat(10)), Ok(()));
    assert_eq!(validate_title(&"e\u{301}".repeat(11)), Err(String::from("too_long")));

    let mut docs = vec![String::from("A"), String::from(" \t"), String::from(" Đỏ🦀 ")];
    let before = docs.clone(); // Snapshot cho case, không phải gợi ý clone trong implementation.
    assert!(matches!(take_document(&mut docs, 1), Err(TakeError::Blank)));
    assert_eq!(docs, before);
    assert!(matches!(take_document(&mut docs, usize::MAX), Err(TakeError::Missing)));
    assert_eq!(docs, before);
    match take_document(&mut docs, 2) { Ok(s) => assert_eq!(s, " Đỏ🦀 "), Err(_) => panic!("Phải lấy được") }
    assert_eq!(docs, vec![String::from("A"), String::from(" \t")]);
    match take_document(&mut docs, 0) { Ok(s) => assert_eq!(s, "A"), Err(_) => panic!("Phải lấy được") }
    assert_eq!(docs, vec![String::from(" \t")]);
    let mut empty = Vec::new();
    assert!(matches!(take_document(&mut empty, 0), Err(TakeError::Missing)));
    assert!(empty.is_empty());

    let mut map = HashMap::new();
    assert!(insert_unique(&mut map, String::from(" A "), String::from("one")).is_ok());
    assert!(insert_unique(&mut map, String::from("A"), String::new()).is_ok());
    assert!(insert_unique(&mut map, String::from("🦀"), String::from("Việt")).is_ok());
    let before = map.clone();
    assert!(matches!(insert_unique(&mut map, String::from(" A "), String::from("replace")), Err(InsertError::Duplicate)));
    assert_eq!(map, before);
    assert!(matches!(insert_unique(&mut map, String::from(" \n"), String::from("x")), Err(InsertError::InvalidKey)));
    assert_eq!(map, before);
    assert_eq!(map.get("A"), Some(&String::new()));
    assert_eq!(map.get(" A "), Some(&String::from("one")));

    let mut zero = Spool::new(0);
    assert!(matches!(zero.submit(String::new(), 0), Err(QueueError::InvalidTitle)));
    assert!(matches!(zero.submit(String::from("A"), 0), Err(QueueError::InvalidPages)));
    assert!(matches!(zero.submit(String::from("A"), 1), Err(QueueError::Full)));
    assert!(matches!(zero.cancel(0), Err(QueueError::Missing)));
    assert_eq!(zero.print_next(), None);
    assert_spool(&zero, &[], &[], &[], 0);
    assert!(zero.queued_counts().is_empty());

    let mut s = Spool::new(3);
    match s.submit(String::from(" Đỏ🦀 "), 1) { Ok(i) => assert_eq!(i, 0), Err(_) => panic!("Submit 0") }
    match s.submit(String::from(" Đỏ🦀 "), 100) { Ok(i) => assert_eq!(i, 1), Err(_) => panic!("Submit 1") }
    match s.submit(String::from("B"), 2) { Ok(i) => assert_eq!(i, 2), Err(_) => panic!("Submit 2") }
    let snapshot = s.queued_counts();
    assert_eq!(snapshot.len(), 2);
    assert_eq!(snapshot.get(" Đỏ🦀 "), Some(&2));
    assert_eq!(snapshot.get("B"), Some(&1));
    for pages in [0, 101, u32::MAX] {
        assert!(matches!(s.submit(String::from("A"), pages), Err(QueueError::InvalidPages)));
        assert_spool(&s, &[" Đỏ🦀 ", " Đỏ🦀 ", "B"], &[1,100,2], &[0,0,0], 103);
        assert_eq!(s.queued_counts(), snapshot);
    }
    assert!(matches!(s.submit("🦀".repeat(21), 1), Err(QueueError::InvalidTitle)));
    assert!(matches!(s.submit(String::from("C"), 1), Err(QueueError::Full)));
    assert!(matches!(s.cancel(usize::MAX), Err(QueueError::Missing)));
    assert_spool(&s, &[" Đỏ🦀 ", " Đỏ🦀 ", "B"], &[1,100,2], &[0,0,0], 103);
    assert_eq!(s.queued_counts(), snapshot);
    assert!(s.cancel(0).is_ok());
    assert_spool(&s, &[" Đỏ🦀 ", " Đỏ🦀 ", "B"], &[1,100,2], &[2,0,0], 102);
    assert!(matches!(s.cancel(0), Err(QueueError::NotQueued)));
    assert_spool(&s, &[" Đỏ🦀 ", " Đỏ🦀 ", "B"], &[1,100,2], &[2,0,0], 102);
    assert_eq!(s.print_next(), Some(1));
    assert!(matches!(s.cancel(1), Err(QueueError::NotQueued)));
    assert_spool(&s, &[" Đỏ🦀 ", " Đỏ🦀 ", "B"], &[1,100,2], &[2,1,0], 2);
    let now = s.queued_counts();
    assert_eq!(now.len(), 1);
    assert_eq!(now.get("B"), Some(&1));
    assert_eq!(snapshot.get(" Đỏ🦀 "), Some(&2));
    assert_eq!(s.print_next(), Some(2));
    assert_eq!(s.print_next(), None);
    assert!(matches!(s.submit(String::from("C"), 1), Err(QueueError::Full)));
    assert_spool(&s, &[" Đỏ🦀 ", " Đỏ🦀 ", "B"], &[1,100,2], &[2,1,1], 0);
    assert!(s.queued_counts().is_empty());
    let mut capped = Spool::new(usize::MAX);
    for _ in 0..100 {
        assert!(capped.submit(String::from("A"), 100).is_ok());
    }
    assert_eq!(capped.pending_pages(), 10_000);
    assert!(matches!(capped.submit(String::from("B"), 1), Err(QueueError::Full)));
    assert_eq!(capped.len(), 100);
    assert_eq!(capped.pending_pages(), 10_000);
    println!("Tất cả case đã pass — có thể nộp bài để review.");
}
