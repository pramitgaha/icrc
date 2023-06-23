use candid::{Principal, Nat, CandidType,};
use ic_cdk::api::call::CallResult;
use async_trait::async_trait;
use serde::Deserialize;

pub type Subaccount = [u8; 32];

#[derive(CandidType, Deserialize, Clone, Copy)]
pub struct Account{
    pub owner: Principal,
    pub subaccount: Option<Subaccount>
}

impl Account{
    pub fn from_subaccount(subaccount: Option<Subaccount>) -> Self{
        Self { owner: ic_cdk::id(), subaccount }
    }

    pub fn default(principal: Principal) -> Self{
        Self { owner: principal, subaccount: Some([0u8; 32]) }
    }
}

#[derive(CandidType, Deserialize)]
pub struct TransferArg {
  pub to: Account,
  pub fee: Option<Nat>,
  pub memo: Option<Vec<u8>>,
  pub from_subaccount: Option<Subaccount>,
  pub created_at_time: Option<u64>,
  pub amount: Nat,
}

impl TransferArg{
    pub fn new(to: Account, fee: Nat, from_subaccount: Option<Subaccount>, amount: Nat) -> Self{
        Self{
            to,
            fee: Some(fee.clone()),
            memo: None,
            from_subaccount,
            created_at_time: None,
            amount: amount - fee,
        }
    }
}

#[derive(CandidType, Deserialize)]
pub enum TransferError {
  GenericError{ message: String, error_code: Nat },
  TemporarilyUnavailable,
  BadBurn{ min_burn_amount: Nat },
  Duplicate{ duplicate_of: Nat },
  BadFee{ expected_fee: Nat },
  CreatedInFuture{ ledger_time: u64 },
  TooOld,
  InsufficientFunds{ balance: Nat },
}


pub trait TokenPrincipalFetcher{
    fn token_principal(&self) -> Principal;
}

#[async_trait]
pub trait Icrc1: TokenPrincipalFetcher{
    async fn icrc1_name(&self) -> CallResult<(String,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc1_name", ()).await
    }

    async fn icrc1_symbol(&self) -> CallResult<(String,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc1_symbol", ()).await
    }

    async fn icrc1_decimals(&self) -> CallResult<(u8,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc1_decimals", ()).await
    }

    async fn icrc1_balance(&self, account: Account) -> CallResult<(Nat,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc1_balance_of", (account,)).await
    }

    async fn icrc1_fee(&self) -> CallResult<(Nat,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc1_fee", ()).await
    }

    async fn icrc1_total_supply(&self) -> CallResult<(Nat,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc1_total_supply", ()).await
    }

    async fn icrc1_transfer(&self, arg: TransferArg) -> CallResult<(Result<Nat, TransferError>,)>{
        let token = self.token_principal();
        ic_cdk::call(token, "icrc1_transfer", (arg,)).await
    }
}

pub struct Icrc1Token(Principal);

impl Icrc1Token{
    pub fn new(principal: Principal) -> Self{
        Self(principal)
    }
}

impl TokenPrincipalFetcher for Icrc1Token{
    fn token_principal(&self) -> Principal {
        self.0.clone()
    }
}

impl Icrc1 for Icrc1Token{}