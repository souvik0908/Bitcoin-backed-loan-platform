use ic_cdk::api::caller;
use ic_cdk_macros::*;
use ic_cdk::{trap, call};
use std::collections::HashMap;
use std::cell::RefCell;
use candid::Principal;

const MAX_LTV: f64 = 0.7;
const ORACLE_CANISTER_ID: &str = "ulvla-h7777-77774-qaacq-cai"; // Oracle Canister
const BTC_CANISTER_ID: &str = "uxrrr-q7777-77774-qaaaq-cai";   // BTC Token Canister
const IUSD_CANISTER_ID: &str = "uzt4z-lp777-77774-qaabq-cai";  // iUSD Token Canister

#[derive(Default)]
struct Loan {
    btc_locked: u64,       // in satoshis
    iusd_borrowed: u64,    // in iUSD smallest unit
}

thread_local! {
    static LOANS: RefCell<HashMap<Principal, Loan>> = RefCell::new(HashMap::new());
}

#[update]
async fn deposit_btc(sats: u64) {
    let user = caller();
    let loan_canister_id = ic_cdk::api::id();

    let (res,): (bool,) = call(
        Principal::from_text(BTC_CANISTER_ID).unwrap(),
        "transfer_from",
        (user, loan_canister_id, sats)
    ).await.unwrap();

    if !res {
        trap("BTC token transfer failed");
    }

    LOANS.with(|loans| {
        let mut loans = loans.borrow_mut();
        let entry = loans.entry(user).or_default();
        entry.btc_locked += sats;
    });
}

#[update]
async fn borrow_iusd() -> u64 {
    let user = caller();

    let (price,): (u64,) = call(
        Principal::from_text(ORACLE_CANISTER_ID).unwrap(),
        "get_price",
        ()
    ).await.unwrap();

    let borrow_amount = LOANS.with(|loans| {
        loans.borrow().get(&user).map_or(0, |loan| {
            let usd_value = (loan.btc_locked as u128 * price as u128) / 100_000_000;
            (usd_value as f64 * MAX_LTV) as u64
        })
    });

    LOANS.with(|loans| {
        let mut loans = loans.borrow_mut();
        let entry = loans.entry(user).or_default();
        entry.iusd_borrowed += borrow_amount;
    });

    // Mint iUSD to user
    let (_res,): (bool,) = call(
        Principal::from_text(IUSD_CANISTER_ID).unwrap(),
        "mint_to",
        (user, borrow_amount)
    ).await.unwrap();

    borrow_amount
}

#[update]
async fn repay_iusd(amount: u64) {
    let user = caller();

    // Burn iUSD from user
    let (burn_success,): (bool,) = call(
        Principal::from_text(IUSD_CANISTER_ID).unwrap(),
        "burn_from",
        (user, amount)
    ).await.unwrap();

    if !burn_success {
        trap("iUSD burn failed: insufficient balance or other error.");
    }

    LOANS.with(|loans| {
        let mut loans = loans.borrow_mut();
        if let Some(loan) = loans.get_mut(&user) {
            loan.iusd_borrowed = loan.iusd_borrowed.saturating_sub(amount);

            if loan.iusd_borrowed == 0 {
                let btc_locked = loan.btc_locked;
                loan.btc_locked = 0;

                // Return BTC collateral
                ic_cdk::spawn(async move {
                    let (_res,): (bool,) = call(
                        Principal::from_text(BTC_CANISTER_ID).unwrap(),
                        "transfer_from",
                        (ic_cdk::api::id(), user, btc_locked)
                    ).await.unwrap();
                });
            }
        }
    });
}

#[query]
fn get_loan() -> (u64, u64) {
    let user = caller();
    LOANS.with(|loans| {
        loans.borrow().get(&user).map_or((0, 0), |loan| (loan.btc_locked, loan.iusd_borrowed))
    })
}

#[update]
async fn get_max_borrowable_auto() -> u64 {
    let user = caller();

    // Fetch latest BTC price from the oracle canister
    let (price,): (u64,) = call(
        Principal::from_text(ORACLE_CANISTER_ID).unwrap(),
        "get_price",
        ()
    ).await.unwrap();

    // Calculate how much iUSD the user can borrow based on LTV
    LOANS.with(|loans| {
        loans.borrow().get(&user).map_or(0, |loan| {
            let usd_value = loan.btc_locked as u128 * price as u128 / 100_000_000;
            (usd_value as f64 * MAX_LTV) as u64
        })
    })
}






