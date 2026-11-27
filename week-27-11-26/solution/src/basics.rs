pub enum PracticeError {
    BlankLabel,
    InvalidFactor,
    Missing,
    AlreadyAnswered,
}

pub enum Grade {
    Unanswered,
    Correct,
    Wrong { expected: u32, received: u32 },
}

pub fn times_table(factor: u32) -> Result<[u32; 10], PracticeError> {
    todo!("Bài 1")
}

pub fn grade(expected: u32, answer: Option<u32>) -> Grade {
    todo!("Bài 2")
}

pub fn retry_labels(items: Vec<(String, bool)>) -> Vec<String> {
    todo!("Bài 3")
}
