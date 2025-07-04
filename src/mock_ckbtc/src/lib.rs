use candid::{CandidType, Principal};
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

type Tokens = u128;

#[derive(CandidType, Deserialize, Serialize, Clone)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct TransferArg {
    pub from_subaccount: Option<Vec<u8>>,
    pub to: Account,
    pub amount: Tokens,
    pub fee: Option<Tokens>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, Tokens>> = RefCell::new(HashMap::new());
}

#[update]
fn icrc1_transfer(arg: TransferArg) -> Result<Tokens, String> {
    let from = ic_cdk::caller();
    let amount = arg.amount;

    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let sender_balance = balances.entry(from).or_insert(0);

        if *sender_balance < amount {
            return Err("Insufficient funds".into());
        }

        *sender_balance -= amount;
        let recipient_balance = balances.entry(arg.to.owner).or_insert(0);
        *recipient_balance += amount;

        Ok(amount)
    })
}

#[update]
fn icrc1_transfer_from(from: Principal, arg: TransferArg) -> Result<Tokens, String> {
    let amount = arg.amount;

    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let sender_balance = balances.entry(from).or_insert(0);

        if *sender_balance < amount {
            return Err("Insufficient funds".into());
        }

        *sender_balance -= amount;
        let recipient_balance = balances.entry(arg.to.owner).or_insert(0);
        *recipient_balance += amount;

        Ok(amount)
    })
}

#[query]
fn icrc1_balance_of(account: Account) -> Tokens {
    BALANCES.with(|b| {
        let balances = b.borrow();
        *balances.get(&account.owner).unwrap_or(&0)
    })
}

#[update]
fn mint(to: Principal, amount: Tokens) {
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let entry = balances.entry(to).or_insert(0);
        *entry += amount;
    });
}
