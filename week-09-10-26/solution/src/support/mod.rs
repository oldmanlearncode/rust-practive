use std::collections::HashMap;
// Khi triển khai, dùng crate::validation::non_blank để xác thực title/team.

pub enum TicketStatus {
    Open,
    Closed,
}

pub struct Ticket {
    title: String,
    team: String,
    status: TicketStatus,
}

impl Ticket {
    pub fn title(&self) -> &str {
        todo!("Mượn title")
    }

    pub fn team(&self) -> &str {
        todo!("Mượn team")
    }

    pub fn status(&self) -> &TicketStatus {
        todo!("Mượn status")
    }
}

pub struct Board {
    tickets: Vec<Ticket>,
}

impl Board {
    pub fn new() -> Self {
        todo!("Sổ rỗng")
    }

    pub fn len(&self) -> usize {
        todo!("Số ticket")
    }

    pub fn submit(&mut self, title: String, team: String) -> bool {
        todo!("Xác thực rồi move title/team vào ticket Open")
    }

    pub fn ticket(&self, index: usize) -> Option<&Ticket> {
        todo!("Truy cập an toàn bằng get")
    }

    pub fn close(&mut self, index: usize) -> bool {
        todo!("Đóng ticket Open; lỗi không đổi dữ liệu")
    }

    pub fn open_counts(&self) -> HashMap<String, usize> {
        todo!("Tạo thống kê mới bằng entry, không sửa ticket")
    }
}
