use rust_week_30_10_26::*;
fn main() {
 assert_eq!(find_scalar("Đỏ🦀Đ", '🦀'), Some(2));
 assert_eq!(find_scalar("Đỏ🦀Đ", 'Đ'), Some(0));
 assert_eq!(find_scalar("", 'a'), None);
 assert_eq!(find_scalar("abc", 'z'), None);
 assert_eq!(find_scalar("e\u{301}", '\u{301}'), Some(1));
 assert_eq!(classify(" plex "), Ok(Some(String::from("plex"))));
 assert_eq!(classify("immich"), Ok(Some(String::from("immich"))));
 assert_eq!(classify("nextcloud"), Ok(Some(String::from("nextcloud"))));
 assert_eq!(classify(" \n"), Ok(None));
 assert_eq!(classify(" # hello"), Ok(None));
 assert_eq!(classify(" BAD "), Err(String::from(" BAD ")));
 assert_eq!(classify("Plex"), Err(String::from("Plex")));
 assert!(count_services(&[]).is_empty());
 let names = vec![String::new(), String::from("A"), String::from("A")];
 let counts = count_services(&names);
 assert_eq!(counts.len(), 2);
 assert_eq!(counts.get(""), Some(&1));
 assert_eq!(counts.get("A"), Some(&2));
 let report = {
  let lines = vec![String::from(" plex "), String::from("# hi"), String::from(" BAD "), String::from("immich"), String::new(), String::from("plex"), String::from("🦀")];
  let before = lines.clone();
  let r = import(&lines);
  assert_eq!(lines, before);
  r
 };
 assert_eq!(report.accepted(), &[String::from("plex"), String::from("immich"), String::from("plex")]);
 assert_eq!(report.ignored(), 2);
 assert_eq!(report.rejected().len(), 2);
 assert_eq!(report.rejected()[0].line(), 3);
 assert_eq!(report.rejected()[0].text(), " BAD ");
 assert_eq!(report.rejected()[1].line(), 7);
 assert_eq!(report.rejected()[1].text(), "🦀");
 let counts = report.counts();
 assert_eq!(counts.len(), 2);
 assert_eq!(counts.get("plex"), Some(&2));
 assert_eq!(counts.get("immich"), Some(&1));
 let empty = import(&[]);
 assert!(empty.accepted().is_empty());
 assert!(empty.rejected().is_empty());
 assert!(empty.counts().is_empty());
 assert_eq!(empty.ignored(), 0);
 let bad = import(&[String::from("bad"), String::from("BAD")]);
 assert!(bad.accepted().is_empty());
 assert_eq!(bad.rejected().len(), 2);
 assert_eq!(bad.ignored(), 0);
 println!("Tất cả case đã pass — có thể nộp review.");
}
