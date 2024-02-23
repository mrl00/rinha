use actix_web::cookie::time::OffsetDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq)]
pub enum TransactionErr {
    LimitExceeded,
    InsufficientBalance,
    InvalidOperation,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct Transaction {
    pub value: i64, 
    pub op_type: String,
    pub description: String,
    pub created_at: OffsetDateTime,
}

impl Transaction {
    pub fn new(value: i64, op_type: String, description: String, created_at: OffsetDateTime) -> Transaction {
        Transaction {
            value,
            op_type,
            description,
            created_at
        }
    }
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            value: 0,
            op_type: String::from(""),
            description: String::from(""),
            created_at: OffsetDateTime::now_utc(), 
        }
    }
}

#[derive(Debug,Deserialize,Serialize, Clone)]
pub struct TransactionDTO {
    #[serde(rename = "valor")]
    pub value: i64, 

    #[serde(rename = "tipo")]
    pub op_type: String,

    #[serde(rename = "descricao")]
    pub description: String,
}

