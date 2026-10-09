// Khung đề, chưa phải bài nộp. Giữ API để main chạy sau khi triển khai.
mod basics;
mod support;
mod validation;

pub use basics::{label_at, take_label, unicode_prefix, word_counts, add_stock};
pub use support::{Board, Ticket, TicketStatus};
