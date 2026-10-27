use std::collections::HashMap;

use crate::validation::non_blank;

pub enum TransferError {
    InvalidName,
    SameAccount,
    ZeroAmount,
    MissingSource,
    MissingTarget,
    InsufficientFunds,
    TargetLimit,
}

pub struct Transfer {
    from: String,
    to: String,
    amount: u32,
}

impl Transfer {
    pub fn from(&self) -> &str {
        &self.from
    }
    pub fn to(&self) -> &str {
        &self.to
    }
    pub fn amount(&self) -> u32 {
        self.amount
    }
}

pub struct Wallet {
    balances: HashMap<String, u32>,
    history: Vec<Transfer>,
}

impl Wallet {
    pub fn new() -> Self {
        Wallet {
            balances: HashMap::new(),
            history: vec![],
        }
    }

    pub fn open(&mut self, name: String, balance: u32) -> bool {
        if !non_blank(&name) || balance > 10_000 || self.balances.contains_key(&name) {
            return false;
        }
        !self.balances.insert(name, balance).is_none()
    }

    pub fn balance(&self, name: &str) -> Option<u32> {
        let value = self.balances.get(name)?;
        Some(*value)
    }

    pub fn account_count(&self) -> usize {
        self.balances.len()
    }

    pub fn transfer(
        &mut self,
        from: String,
        to: String,
        amount: u32,
    ) -> Result<usize, TransferError> {
        todo!("Xác thực đủ trước khi cập nhật cả hai tài khoản và lịch sử")
    }

    pub fn history(&self) -> &[Transfer] {
        todo!("Mượn lịch sử")
    }

    pub fn sent_counts(&self) -> HashMap<String, usize> {
        todo!("Thống kê số lần gửi bằng entry/or_insert")
    }
}
