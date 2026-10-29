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

pub enum UpdateType {
    Add,
    Subtract,
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
        self.balances.insert(name, balance).is_none()
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
        if !non_blank(&from) || !non_blank(&to) {
            println!("No from or to");
            return Err(TransferError::InvalidName);
        }
        if from.eq(&to) {
            println!("same account");
            return Err(TransferError::SameAccount);
        }
        if amount == 0 {
            println!("amount 0");
            return Err(TransferError::ZeroAmount);
        }

        if !self.balances.contains_key(&from) {
            return Err(TransferError::MissingSource);
        }

        if !self.balances.contains_key(&to) {
            return Err(TransferError::MissingTarget);
        }

        self.update_balance(&from, amount, UpdateType::Subtract)?;
        if let Err(TransferError::TargetLimit) = self.update_balance(&to, amount, UpdateType::Add) {
            self.update_balance(&from, amount, UpdateType::Add)?;
            return Err(TransferError::TargetLimit);
        }

        self.history.push(Transfer { from, to, amount });
        Ok(self.history.len() - 1)
    }

    fn update_balance(
        &mut self,
        account: &str,
        amount: u32,
        update_type: UpdateType,
    ) -> Result<(), TransferError> {
        let balance: &mut u32;
        if let Some(current) = self.balances.get_mut(account) {
            balance = current;
        } else {
            match update_type {
                UpdateType::Add => return Err(TransferError::MissingTarget),
                UpdateType::Subtract => return Err(TransferError::MissingSource),
            }
        }
        match update_type {
            UpdateType::Add => {
                if *balance + amount > 10_000 {
                    Err(TransferError::TargetLimit)
                } else {
                    *balance += amount;
                    Ok(())
                }
            }
            UpdateType::Subtract => {
                if *balance < amount {
                    Err(TransferError::InsufficientFunds)
                } else {
                    *balance -= amount;
                    Ok(())
                }
            }
        }
    }
    pub fn history(&self) -> &[Transfer] {
        &self.history
    }

    pub fn sent_counts(&self) -> HashMap<String, usize> {
        let mut result: HashMap<String, usize> = HashMap::new();
        for his in &self.history {
            let count = result.entry(String::from(&his.from)).or_insert(0);
            *count += 1;
        }
        result
    }
}
