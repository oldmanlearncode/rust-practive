use std::collections::HashMap;

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
    pub fn from(&self) -> &str { todo!("Getter mượn") }
    pub fn to(&self) -> &str { todo!("Getter mượn") }
    pub fn amount(&self) -> u32 { todo!("Getter Copy") }
}

pub struct Wallet {
    balances: HashMap<String, u32>,
    history: Vec<Transfer>,
}

impl Wallet {
    pub fn new() -> Self { todo!("Ví rỗng") }
    pub fn open(&mut self, name: String, balance: u32) -> bool {
        todo!("Xác thực rồi mở tài khoản; không ghi đè")
    }
    pub fn balance(&self, name: &str) -> Option<u32> {
        todo!("Truy cập an toàn, trả giá trị Copy")
    }
    pub fn account_count(&self) -> usize { todo!("Số tài khoản") }
    pub fn transfer(&mut self, from: String, to: String, amount: u32)
        -> Result<usize, TransferError> {
        todo!("Xác thực đủ trước khi cập nhật cả hai tài khoản và lịch sử")
    }
    pub fn history(&self) -> &[Transfer] { todo!("Mượn lịch sử") }
    pub fn sent_counts(&self) -> HashMap<String, usize> {
        todo!("Thống kê số lần gửi bằng entry/or_insert")
    }
}
