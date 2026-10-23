use std::collections::HashMap;

pub enum JobState { Queued, Printed, Cancelled }
pub enum QueueError { InvalidTitle, InvalidPages, Full, Missing, NotQueued }

pub struct Job {
    title: String,
    pages: u32,
    state: JobState,
}
impl Job {
    pub fn title(&self) -> &str { todo!("Getter mượn") }
    pub fn pages(&self) -> u32 { todo!("Getter Copy") }
    pub fn state(&self) -> &JobState { todo!("Getter mượn enum") }
}

pub struct Spool {
    capacity: usize,
    jobs: Vec<Job>,
}
impl Spool {
    pub fn new(capacity: usize) -> Self { todo!("Hàng đợi rỗng") }
    pub fn len(&self) -> usize { todo!("Số job đã lưu") }
    pub fn job(&self, index: usize) -> Option<&Job> { todo!("Truy cập an toàn") }
    pub fn submit(&mut self, title: String, pages: u32) -> Result<usize, QueueError> {
        todo!("Xác thực title qua helper, pages, capacity rồi move vào job")
    }
    pub fn cancel(&mut self, index: usize) -> Result<(), QueueError> {
        todo!("Chỉ Queued chuyển thành Cancelled")
    }
    pub fn print_next(&mut self) -> Option<usize> {
        todo!("Queued có index nhỏ nhất chuyển Printed")
    }
    pub fn pending_pages(&self) -> u32 { todo!("Tổng pages của Queued") }
    pub fn queued_counts(&self) -> HashMap<String, usize> {
        todo!("Map mới đếm title của Queued bằng entry/or_insert")
    }
}
