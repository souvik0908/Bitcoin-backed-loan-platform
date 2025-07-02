use ic_cdk_macros::*;
use ic_cdk::api::caller;
use std::collections::HashMap;
use std::cell::RefCell;
use candid::Principal;

thread_local! {
    static BALANCES: RefCell<HashMap<Principal, u64>> = RefCell::new(HashMap::new());
}

#[update]
fn mint_to(to: Principal, amount: u64) -> bool {
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        *balances.entry(to).or_insert(0) += amount;
    });
    true
}

#[update]
fn burn_from(from: Principal, amount: u64) -> bool {
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        if let Some(balance) = balances.get_mut(&from) {
            if *balance >= amount {
                *balance -= amount;
                return true;
            }
        }
        false
    })
}

#[update]
fn transfer_from(from: Principal, to: Principal, amount: u64) -> bool {
    BALANCES.with(|b| {
        let mut balances = b.borrow_mut();
        if let Some(from_balance) = balances.get_mut(&from) {
            if *from_balance >= amount {
                *from_balance -= amount;
                *balances.entry(to).or_insert(0) += amount;
                return true;
            }
        }
        false
    })
}

#[query]
fn balance_of(user: Principal) -> u64 {
    BALANCES.with(|b| *b.borrow().get(&user).unwrap_or(&0))
}
