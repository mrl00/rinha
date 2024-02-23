use std::{collections::HashMap, sync::Mutex};

use once_cell::sync::Lazy;

use crate::domain::Account;

static GLOBAL_DATA: Lazy<Mutex<HashMap<u32, Account>>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(1, Account::new(100_000,0));
    m.insert(2, Account::new(80_000,0));
    m.insert(3, Account::new(1_000_000,0));
    m.insert(4, Account::new(10_000_000,0));
    m.insert(5, Account::new(500_000,0));
    Mutex::new(m)
});


pub fn id_exists(id: u32) -> bool {
    GLOBAL_DATA.lock().unwrap().contains_key(&id)
}
