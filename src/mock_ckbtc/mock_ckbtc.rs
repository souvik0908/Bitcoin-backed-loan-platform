// This is an experimental feature to generate Rust binding from Candid.
// You may want to manually adjust some of the types.
#![allow(dead_code, unused_imports)]
use candid::{self, CandidType, Deserialize, Principal};
use ic_cdk::api::call::CallResult as Result;

#[derive(CandidType, Deserialize)]
pub struct Account {
  pub owner: Principal,
  pub subaccount: Option<serde_bytes::ByteBuf>,
}
#[derive(CandidType, Deserialize)]
pub struct TransferArg {
  pub to: Account,
  pub fee: Option<candid::Nat>,
  pub memo: Option<serde_bytes::ByteBuf>,
  pub from_subaccount: Option<serde_bytes::ByteBuf>,
  pub created_at_time: Option<u64>,
  pub amount: candid::Nat,
}
pub type TransferResult = std::result::Result<candid::Nat, String>;

pub struct Service(pub Principal);
impl Service {
  pub async fn icrc_1_balance_of(&self, arg0: &Account) -> Result<(candid::Nat,)> {
    ic_cdk::call(self.0, "icrc1_balance_of", (arg0,)).await
  }
  pub async fn icrc_1_transfer(&self, arg0: &TransferArg) -> Result<(TransferResult,)> {
    ic_cdk::call(self.0, "icrc1_transfer", (arg0,)).await
  }
  pub async fn icrc_1_transfer_from(&self, arg0: &Principal, arg1: &TransferArg) -> Result<(TransferResult,)> {
    ic_cdk::call(self.0, "icrc1_transfer_from", (arg0,arg1,)).await
  }
  pub async fn mint(&self, arg0: &Principal, arg1: &candid::Nat) -> Result<()> {
    ic_cdk::call(self.0, "mint", (arg0,arg1,)).await
  }
}

