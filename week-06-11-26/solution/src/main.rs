use rust_week_06_11_26::*;
fn position(p: &Player) -> Option<usize> {
 match p.state() { State::Stopped => None, State::Playing(i) => Some(*i) }
}
fn main() {
 assert_eq!(replace_scalar("Đỏ🦀Đ", 'Đ', 'A'), "Aỏ🦀A");
 assert_eq!(replace_scalar("", 'a', 'b'), "");
 assert_eq!(replace_scalar("🦀", 'x', 'y'), "🦀");
 assert_eq!(replace_scalar("e\u{301}", 'e', 'a'), "a\u{301}");
 assert_eq!(first_duplicate(&[]), None);
 let mut names = vec![String::from("A"), String::from("B"), String::from("A")];
 assert_eq!(first_duplicate(&names), Some(2));
 let snapshot = names.clone();
 assert_eq!(swap_checked(&mut names, usize::MAX, 0), Err(String::from("first")));
 assert_eq!(names, snapshot);
 assert_eq!(swap_checked(&mut names, 0, usize::MAX), Err(String::from("second")));
 assert_eq!(names, snapshot);
 assert_eq!(swap_checked(&mut names, 1, 1), Ok(()));
 assert_eq!(names, snapshot);
 assert_eq!(swap_checked(&mut names, 0, 1), Ok(()));
 assert_eq!(names, vec![String::from("B"),String::from("A"),String::from("A")]);
 assert_eq!(first_duplicate(&[String::new(),String::new()]), Some(1));
 assert_eq!(first_duplicate(&[String::from("A"),String::from(" A ")]), None);
 let mut empty: Vec<String> = vec![];
 assert_eq!(swap_checked(&mut empty,0,0), Err(String::from("first")));
 let mut p = Player::new();
 assert!(p.tracks().is_empty());
 assert_eq!(position(&p),None);
 assert_eq!(p.current(),None);
 assert!(!p.advance());
 assert_eq!(p.play(0),Err(String::from("missing")));
 assert_eq!(p.add(String::from(" \n")),Err(String::from("blank")));
 assert!(p.tracks().is_empty());
 for name in [" A🦀 ", "B", "C"] { assert!(p.add(String::from(name)).is_ok()); }
 assert_eq!(p.play(0),Ok(()));
 assert_eq!(p.current(),Some(" A🦀 "));
 assert_eq!(p.swap_tracks(0,2),Ok(()));
 assert_eq!(p.tracks(), &[String::from("C"),String::from("B"),String::from(" A🦀 ")]);
 assert_eq!(position(&p),Some(2));
 assert_eq!(p.current(),Some(" A🦀 "));
 assert!(!p.advance());
 let before=p.tracks().to_vec();
 assert_eq!(p.swap_tracks(0,usize::MAX),Err(String::from("second")));
 assert_eq!(p.tracks(),before.as_slice());
 assert_eq!(position(&p),Some(2));
 assert_eq!(p.play(usize::MAX),Err(String::from("missing")));
 assert_eq!(position(&p),Some(2));
 assert_eq!(p.swap_tracks(2,2),Ok(()));
 assert_eq!(position(&p),Some(2));
 assert_eq!(p.swap_tracks(1,2),Ok(()));
 assert_eq!(position(&p),Some(1));
 assert_eq!(p.current(),Some(" A🦀 "));
 assert!(p.advance());
 assert_eq!(p.current(),Some("B"));
 p.stop(); p.stop();
 assert_eq!(position(&p),None);
 assert!(!p.advance());
 assert_eq!(p.swap_tracks(0,1),Ok(()));
 assert_eq!(position(&p),None);
 assert_eq!(p.duplicate_index(),None);
 assert_eq!(p.add(String::from("B")),Ok(3));
 assert_eq!(p.duplicate_index(),Some(3));
 let mut full=Player::new();
 for i in 0..50 { assert_eq!(full.add(String::from("🦀")),Ok(i)); }
 assert_eq!(full.play(49),Ok(()));
 let before=full.tracks().to_vec();
 assert_eq!(full.add(String::new()),Err(String::from("blank")));
 assert_eq!(full.add(String::from("extra")),Err(String::from("full")));
 assert_eq!(full.tracks(),before.as_slice());
 assert_eq!(position(&full),Some(49));
 assert_eq!(full.current(),Some("🦀"));
 println!("Tất cả case đã pass — có thể nộp review.");
}
