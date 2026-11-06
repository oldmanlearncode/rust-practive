pub enum State { Stopped, Playing(usize) }
pub struct Player { tracks: Vec<String>, state: State }
impl Player {
 pub fn new() -> Self { todo!() }
 pub fn tracks(&self) -> &[String] { todo!() }
 pub fn state(&self) -> &State { todo!() }
 pub fn add(&mut self, title: String) -> Result<usize, String> { todo!() }
 pub fn play(&mut self, index: usize) -> Result<(), String> { todo!() }
 pub fn stop(&mut self) { todo!() }
 pub fn current(&self) -> Option<&str> { todo!() }
 pub fn advance(&mut self) -> bool { todo!() }
 pub fn swap_tracks(&mut self, a: usize, b: usize) -> Result<(), String> { todo!() }
 pub fn duplicate_index(&self) -> Option<usize> { todo!() }
}
