use rinha::domain::{Account, Transaction, TransactionErr};

#[test]
fn acc_add_transaction_ok() {
    let acc = Account::new(10_000, 20_000);
    let tra = Transaction {
        value: 10_000,
        op_type: String::from("c"),
        ..Default::default()
    };

    let op = acc + tra;
    
    let r = Account::new(20_000, 20_000);

    assert_eq!(op.unwrap(), r);
}

#[test]
fn acc_add_transaction_err() {
    let acc = Account::new(10_000, 20_000);
    let tra = Transaction {
        value: 10_001,
        op_type: String::from("c"),
        ..Default::default()
    };

    let op = acc + tra;
    
    let r = TransactionErr::LimitExceeded;

    assert_eq!(op.unwrap_err(), r);
}

#[test]
fn acc_sub_transaction_ok() {
    let acc = Account::new(10_000, 20_000);
    let tra = Transaction {
        value: 30_000,
        op_type: String::from("d"),
        ..Default::default()
    };

    let op = acc - tra;
    
    let r = Account::new(-20_000, 20_000);

    assert_eq!(op.unwrap(), r);

}

#[test]
fn acc_sub_transaction_err() {
    let acc = Account::new(10_000, 20_000);
    let tra = Transaction {
        value: 30_001,
        op_type: String::from("d"),
        ..Default::default()
    };

    let op = acc.perform_op(tra);
    
    let r = TransactionErr::InsufficientBalance;

    assert_eq!(op.unwrap_err(), r);
}

#[test]
fn acc_perform_op_err() {
    let acc = Account::new(10_000, 20_000);
    let tra = Transaction {
        value: 30_001,
        op_type: String::from("9"),
        ..Default::default()
    };
    
    let op = acc.perform_op(tra);

    let r = TransactionErr::InvalidOperation;

    assert_eq!(op.unwrap_err(), r);
}
