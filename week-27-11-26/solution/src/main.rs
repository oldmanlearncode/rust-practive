use rust_week_27_11_26::*;

fn main() {
    match times_table(3) {
        Ok(row) => assert_eq!(row, [3,6,9,12,15,18,21,24,27,30]),
        Err(_) => panic!("Factor hợp lệ"),
    }
    match times_table(1) { Ok(row) => assert_eq!(row[9],10), Err(_) => panic!("Factor 1") }
    match times_table(10) { Ok(row) => assert_eq!(row[9],100), Err(_) => panic!("Factor 10") }
    for invalid in [0,11,u32::MAX] {
        assert!(matches!(times_table(invalid), Err(PracticeError::InvalidFactor)));
    }
    assert!(matches!(grade(6,None), Grade::Unanswered));
    assert!(matches!(grade(6,Some(6)), Grade::Correct));
    assert!(matches!(grade(6,Some(5)), Grade::Wrong { expected:6, received:5 }));
    assert!(matches!(grade(0,Some(0)), Grade::Correct));
    assert!(matches!(grade(u32::MAX,Some(0)), Grade::Wrong { expected:u32::MAX, received:0 }));
    assert!(retry_labels(vec![]).is_empty());
    assert_eq!(retry_labels(vec![("A".to_string(),false),("B".to_string(),true),
        ("A".to_string(),false),(String::new(),false),("🦀".to_string(),false)]),
        vec!["A".to_string(),"A".to_string(),String::new(),"🦀".to_string()]);

    let mut s=PracticeSession::new();
    assert_eq!(s.len(),0);
    assert!(s.question(0).is_none());
    assert!(s.counts().is_empty());
    assert!(s.retry_labels().is_empty());
    assert!(matches!(s.answer(usize::MAX,0),Err(PracticeError::Missing)));
    assert!(matches!(s.result(0),Err(PracticeError::Missing)));
    assert!(matches!(s.add(" \n".to_string(),0,11),Err(PracticeError::BlankLabel)));
    assert!(matches!(s.add("A".to_string(),0,1),Err(PracticeError::InvalidFactor)));
    assert!(matches!(s.add("A".to_string(),1,u32::MAX),Err(PracticeError::InvalidFactor)));
    assert_eq!(s.len(),0);
    assert!(matches!(s.add(" Rừng🦀 ".to_string(),2,3),Ok(0)));
    assert!(matches!(s.add("Làng".to_string(),3,4),Ok(1)));
    assert!(matches!(s.add(" Rừng🦀 ".to_string(),10,10),Ok(2)));
    let snapshot=s.counts();
    assert_eq!(snapshot.len(),1);
    assert_eq!(snapshot.get("unanswered"),Some(&3));
    assert!(matches!(s.result(0),Ok(Grade::Unanswered)));
    assert!(s.answer(0,5).is_ok());
    let retry=s.retry_labels();
    assert_eq!(retry,vec![" Rừng🦀 ".to_string()]);
    assert!(matches!(s.result(0),Ok(Grade::Wrong {expected:6,received:5})));
    assert!(s.answer(1,12).is_ok());
    assert!(matches!(s.result(1),Ok(Grade::Correct)));
    let counts=s.counts();
    assert_eq!(counts.len(),3);
    for key in ["unanswered","correct","wrong"] { assert_eq!(counts.get(key),Some(&1)); }
    assert!(matches!(s.answer(0,6),Err(PracticeError::AlreadyAnswered)));
    assert!(matches!(s.answer(usize::MAX,6),Err(PracticeError::Missing)));
    assert!(matches!(s.add(String::new(),2,3),Err(PracticeError::BlankLabel)));
    assert_eq!(s.len(),3);
    assert_eq!(s.counts(),counts);
    for i in 0..3 {
        let q=match s.question(i) { Some(q)=>q, None=>panic!("Thiếu câu") };
        match i {
            0 => { assert_eq!(q.label()," Rừng🦀 "); assert_eq!(q.factors(),(2,3)); assert_eq!(q.answer(),Some(5)); }
            1 => { assert_eq!(q.label(),"Làng"); assert_eq!(q.factors(),(3,4)); assert_eq!(q.answer(),Some(12)); }
            _ => { assert_eq!(q.label()," Rừng🦀 "); assert_eq!(q.factors(),(10,10)); assert_eq!(q.answer(),None); }
        }
    }
    assert!(s.answer(2,u32::MAX).is_ok());
    assert!(matches!(s.result(2),Ok(Grade::Wrong {expected:100,received:u32::MAX})));
    assert_eq!(s.retry_labels(),vec![" Rừng🦀 ".to_string()," Rừng🦀 ".to_string()]);
    assert_eq!(retry,vec![" Rừng🦀 ".to_string()]);
    assert_eq!(snapshot.get("unanswered"),Some(&3));
    assert_eq!(s.counts().len(),2);
    assert_eq!(s.counts().get("wrong"),Some(&2));
    assert!(!s.counts().contains_key("unanswered"));
    let mut zero=PracticeSession::new();
    assert!(zero.add("A".to_string(),1,1).is_ok());
    assert!(zero.answer(0,0).is_ok());
    assert!(matches!(zero.result(0),Ok(Grade::Wrong {expected:1,received:0})));
    println!("Tất cả case đã pass — có thể nộp review.");
}
