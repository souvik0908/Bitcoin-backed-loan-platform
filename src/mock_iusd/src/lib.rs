use candid::{CandidType, Decode, Encode, Nat, Principal};
use ic_cdk::{api::caller, query, update};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

type Tokens = Nat;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<[u8; 32]>,
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
    pub from_subaccount: Option<[u8; 32]>,
    pub to: Account,
    pub amount: Tokens,
    pub fee: Option<Tokens>,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize)]
pub struct TransferFromArgs {
    pub spender_subaccount: Option<[u8; 32]>,
    pub from: Account,
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
fn icrc1_transfer(arg: TransferArg) -> Result<Nat, String> {
    let caller = caller();
    let amount = arg.amount.clone();

    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let sender_balance = balances.entry(caller).or_insert(Nat::from(0u8));

        if *sender_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        *sender_balance -= amount.clone();
        *balances.entry(arg.to.owner).or_insert(Nat::from(0u8)) += amount;
        Ok(Nat::from(0u8)) // fake block index
    })
}

#[update]
fn icrc1_transfer_from(arg: TransferFromArgs) -> Result<Nat, String> {
    let amount = arg.amount.clone();

    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        let from_balance = balances.entry(arg.from.owner).or_insert(Nat::from(0u8));

        if *from_balance < amount {
            return Err("Insufficient balance".to_string());
        }

        *from_balance -= amount.clone();
        *balances.entry(arg.to.owner).or_insert(Nat::from(0u8)) += amount;
        Ok(Nat::from(0u8)) // fake block index
    })
}

#[update]
fn mint(to: Principal, amount: Tokens) {
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        *balances.entry(to).or_insert(Nat::from(0u8)) += amount;
    });
}

#[query]
fn icrc1_balance_of(account: Account) -> Tokens {
    BALANCES.with(|b| b.borrow().get(&account.owner).cloned().unwrap_or_else(|| Nat::from(0u8)))
}

#[query]
fn icrc1_name() -> String {
    "Internet USD".to_string()
}

#[query]
fn icrc1_symbol() -> String {
    "IUSD".to_string()
}

#[query]
fn icrc1_decimals() -> u8 {
    8
}

#[query]
fn icrc1_fee() -> Tokens {
    Nat::from(0u8)
}

#[query]
fn icrc1_total_supply() -> Tokens {
    BALANCES.with(|b| {
        b.borrow()
            .values()
            .fold(Nat::from(0u8), |acc, v| acc + v.clone())
    })
}
