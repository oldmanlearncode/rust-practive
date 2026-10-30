use std::collections::HashMap;
pub struct Rejected { line: usize, text: String }
impl Rejected {
 pub fn line(&self) -> usize { todo!() }
 pub fn text(&self) -> &str { todo!() }
}
pub struct Report { accepted: Vec<String>, rejected: Vec<Rejected>, ignored: usize }
impl Report {
 pub fn accepted(&self) -> &[String] { todo!() }
 pub fn rejected(&self) -> &[Rejected] { todo!() }
 pub fn ignored(&self) -> usize { todo!() }
 pub fn counts(&self) -> HashMap<String, usize> { todo!() }
}
pub fn import(lines: &[String]) -> Report { todo!() }
