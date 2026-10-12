use std::collections::HashMap;

use crate::validation::non_blank;
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
        &self.title
    }

    pub fn team(&self) -> &str {
        &self.team
    }

    pub fn status(&self) -> &TicketStatus {
        &self.status
    }
}

pub struct Board {
    tickets: Vec<Ticket>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            tickets: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.tickets.len()
    }

    pub fn submit(&mut self, title: String, team: String) -> bool {
        let is_valid_string = non_blank(&title) && non_blank(&team);
        if !is_valid_string {
            return false;
        }
        self.tickets.push(Ticket {
            title,
            team,
            status: TicketStatus::Open,
        });
        true
    }

    pub fn ticket(&self, index: usize) -> Option<&Ticket> {
        self.tickets.get(index)
    }

    pub fn close(&mut self, index: usize) -> bool {
        if let Some(ticket) = self.tickets.get_mut(index) {
            if matches!(ticket.status, TicketStatus::Open) {
                ticket.status = TicketStatus::Closed;
                return true;
            }
        }
        false
    }

    pub fn open_counts(&self) -> HashMap<String, usize> {
        let mut result: HashMap<String, usize> = HashMap::new();
        for ticket in &self.tickets {
            match ticket.status() {
                TicketStatus::Open => {
                    let count = result.entry(ticket.team().to_string()).or_insert(0);
                    *count += 1;
                }
                TicketStatus::Closed => continue,
            }
        }
        result
    }
}
