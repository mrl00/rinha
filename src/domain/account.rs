use std::ops::{Add, Sub};

use super::{Transaction, TransactionErr};

#[derive(Debug, Default, PartialEq)]
pub struct Account {
    pub balance: i64,
    pub limit: i64,
}

impl Account {
    pub fn new(balance: i64, limit: i64) -> Account {
        Account {
            balance,
            limit
        }
    }

    pub fn perform_op(self, t: Transaction) ->  Result<Account, TransactionErr> {
        match t.op_type.as_str() {
            "c" => self + t,
            "d" => self - t,
            _ =>  Err(TransactionErr::InvalidOperation)
        }   
    }
}

impl Add<Transaction> for Account {
    type Output = Result<Account, TransactionErr>;

    fn add(self, rhs: Transaction) -> Self::Output {
        match self.balance + rhs.value > self.limit {
            true => Err(TransactionErr::LimitExceeded),
            _ => Ok(Account::new(self.balance + rhs.value, self.limit))
        }
    }
}

impl Sub<Transaction> for Account {
    type Output = Result<Account, TransactionErr>;

    fn sub(self, rhs: Transaction) -> Self::Output {
        match (self.balance - rhs.value).abs() > self.limit {
            true => Err(TransactionErr::InsufficientBalance),
            _ => Ok(Account::new(self.balance - rhs.value, self.limit))
        }
    }
}
