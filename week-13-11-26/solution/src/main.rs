use rust_week_13_11_26::{index_labels, longest_failure_run, split_record, Event, LogBook, LogError};

fn assert_history(book: &LogBook, service: &str, expected_failures: &[bool]) {
    let actual = match book.events(service) {
        Some(events) => events,
        None => panic!("Thiếu lịch sử dịch vụ"),
    };
    assert_eq!(actual.len(), expected_failures.len());
    for i in 0..actual.len() {
        let failed = match &actual[i] {
            Event::Healthy => false,
            Event::Failed => true,
        };
        assert_eq!(failed, expected_failures[i]);
    }
}

fn main() {
    assert!(matches!(split_record(" Ảnh 🦀 | fail "), Ok(("Ảnh 🦀", "fail"))));
    assert!(matches!(split_record("x | unknown"), Ok(("x", "unknown"))));
    assert!(matches!(split_record(""), Err(LogError::BadFormat)));
    assert!(matches!(split_record("|x|y"), Err(LogError::BadFormat)));
    assert!(matches!(split_record(" | "), Err(LogError::BlankService)));
    assert!(matches!(split_record("x| "), Err(LogError::BadStatus)));

    use Event::{Failed, Healthy};
    assert_eq!(longest_failure_run(&[]), 0);
    assert_eq!(longest_failure_run(&[Healthy]), 0);
    assert_eq!(longest_failure_run(&[Failed, Healthy, Failed, Failed]), 2);
    assert_eq!(longest_failure_run(&[Failed, Failed, Healthy, Failed]), 2);

    let index = index_labels(vec!["Ảnh".to_string(), "Tệp".to_string(),
        "Ảnh".to_string(), " Ảnh ".to_string(), String::new()]);
    assert_eq!(index.len(), 4);
    assert_eq!(index.get("Ảnh"), Some(&vec![0, 2]));
    assert_eq!(index.get("Tệp"), Some(&vec![1]));
    assert_eq!(index.get(" Ảnh "), Some(&vec![3]));
    assert_eq!(index.get(""), Some(&vec![4]));
    assert!(index_labels(vec![]).is_empty());

    let mut book = LogBook::new();
    assert_eq!(book.service_count(), 0);
    assert!(book.events("Ảnh").is_none());
    assert_eq!(book.longest_run("Ảnh"), None);
    assert!(matches!(book.record("Ảnh | fail"), Ok(0)));
    assert!(matches!(book.record("Tệp | ok"), Ok(0)));
    assert!(matches!(book.record(" Ảnh | fail "), Ok(1)));
    assert!(matches!(book.record("Ảnh | ok"), Ok(2)));
    assert!(matches!(book.record("Ảnh | fail"), Ok(3)));
    assert_history(&book, "Ảnh", &[true, true, false, true]);
    assert_eq!(book.longest_run("Ảnh"), Some(2));
    assert_eq!(book.longest_run("Tệp"), Some(0));
    assert!(book.events(" Ảnh ").is_none());

    assert!(matches!(book.record("Ảnh | FAIL"), Err(LogError::BadStatus)));
    assert!(matches!(book.record("Ảnh | ok"), Err(LogError::Full)));
    assert!(matches!(book.record("Mới | broken"), Err(LogError::BadStatus)));
    assert!(matches!(book.record("Mới | ok | fail"), Err(LogError::BadFormat)));
    assert!(matches!(book.record(" | fail"), Err(LogError::BlankService)));
    assert_eq!(book.service_count(), 2);
    assert!(book.events("Mới").is_none());
    assert_history(&book, "Ảnh", &[true, true, false, true]);
    assert_history(&book, "Tệp", &[false]);
    assert_eq!(book.longest_run("Ảnh"), Some(2));
    assert!(matches!(book.record("Tệp | fail"), Ok(1)));
    assert_history(&book, "Tệp", &[false, true]);
    println!("Tất cả case đã pass — có thể nộp review.");
}
