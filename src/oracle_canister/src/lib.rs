use ic_cdk_macros::*;
use std::cell::RefCell;

thread_local! {
    static PRICE: RefCell<u64> = RefCell::new(107_000); // $107,000 per BTC
}

#[query]
fn get_price() -> u64 {
    PRICE.with(|p| *p.borrow())
}

#[update]
fn set_price(new_price: u64) {
    PRICE.with(|p| *p.borrow_mut() = new_price);
}
