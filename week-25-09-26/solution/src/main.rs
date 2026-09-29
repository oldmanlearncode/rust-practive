// Tổng hợp phiên bản người học nộp. Sửa một lỗi cú pháp đã được review:
// format(...) -> format!(...). main chỉ là kịch bản chạy kiểm tra.
struct ReadingGoal {
    title: String,
    target_pages: u32,
    read_pages: u32,
}

impl ReadingGoal {
    // Nếu tiến độ đầu vào vượt mục tiêu, đặt lại về 0.
    // Mục tiêu 0 được chấp nhận và không thể đọc thêm.
    fn new(title: String, target_pages: u32, input_reading_pages: u32) -> Self {
        let mut read_pages = input_reading_pages;
        if read_pages > target_pages {
            read_pages = 0;
        }
        ReadingGoal {
            title,
            target_pages,
            read_pages,
        }
    }

    fn read(&mut self, pages: u32) -> bool {
        if pages == 0 || pages > self.remaining() {
            return false;
        }
        self.read_pages += pages;
        true
    }

    fn remaining(&self) -> u32 {
        self.target_pages - self.read_pages
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn reading_rate(&self) -> String {
        format!("{}/{}", self.read_pages, self.target_pages)
    }
}

fn rotate_left_once(values: &mut [i32]) {
    if values.len() <= 1 {
        return;
    }
    let first_value = values[0];
    for i in 0..(values.len() - 1) {
        values[i] = values[i + 1];
    }
    values[values.len() - 1] = first_value;
}

struct WeeklyReading {
    book: ReadingGoal,
    daily_pages: [u32; 7],
}

impl WeeklyReading {
    fn new(title: String, target_pages: u32) -> Self {
        WeeklyReading {
            book: ReadingGoal::new(title, target_pages, 0),
            daily_pages: [0; 7],
        }
    }

    fn record(&mut self, day: usize, pages: u32) -> bool {
        if day >= self.daily_pages.len() {
            return false;
        }
        let recorded = self.book.read(pages);
        if recorded {
            self.daily_pages[day] += pages;
        }
        recorded
    }

    fn print_report(&self) {
        println!("Weekly Reading");
        println!("-----------------------------");
        println!("Name: {}", self.book.title());
        println!("Weekly diary:");
        for i in 0..self.daily_pages.len() {
            println!("Day {}: read {} pages", i, self.daily_pages[i]);
        }
        println!("Target pages: {}", self.book.target_pages);
        println!("Reading rate: {}", self.book.reading_rate());
        println!("Remaining pages : {}", self.book.remaining());
        println!("Is finished: {}", self.book.remaining() == 0);
        println!("-----------------------------");
    }
}

fn main() {
    let mut numbers = [10, 20, 30];
    rotate_left_once(&mut numbers);
    assert_eq!(numbers, [20, 30, 10]);
    let mut empty: [i32; 0] = [];
    rotate_left_once(&mut empty);
    let mut one = [42];
    rotate_left_once(&mut one);
    assert_eq!(one, [42]);

    let mut weekly = WeeklyReading::new(String::from("Sách tiếng Việt"), 100);
    assert!(weekly.record(0, 20));
    assert!(weekly.record(0, 5));
    assert_eq!(weekly.daily_pages[0], 25);
    assert!(!weekly.record(7, 1));
    assert!(!weekly.record(0, 0));
    assert!(!weekly.record(1, 76));
    assert_eq!(weekly.book.reading_rate(), "25/100");
    assert!(weekly.record(1, 75));
    assert_eq!(weekly.book.remaining(), 0);
    assert!(!weekly.record(1, 1));
    weekly.print_report();

    let mut edge = ReadingGoal::new(String::from("Edge"), u32::MAX, u32::MAX - 1);
    assert!(edge.read(1));
    assert_eq!(edge.remaining(), 0);
}
