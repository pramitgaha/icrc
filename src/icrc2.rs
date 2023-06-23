use candid::{Nat, CandidType, Principal};
use async_trait::async_trait;
use ic_cdk::api::call::CallResult;
use serde::Deserialize;
use crate::icrc1::{Account, Subaccount, TokenPrincipalFetcher, Icrc1};

#[derive(CandidType, Deserialize)]
pub struct ApproveArgs{
    pub from_subaccount : Option<Subaccount>,
    pub spender : Account,
    pub amount : Nat,
    pub expected_allowance : Option<Nat>,
    pub expires_at : Option<u64>,
    pub fee : Option<Nat>,
    pub memo : Option<Vec<u8>>,
    pub created_at_time : Option<u64>
}

#[derive(CandidType, Deserialize)]
pub enum ApproveError{
    BadFee{ expected_fee : Nat },
    InsufficientFunds{ balance : Nat },
    AllowanceChanged{ current_allowance : Nat },
    Expired{ ledger_time : u64 },
    TooOld,
    CreatedInFuture{ ledger_time : u64 },
    Duplicate{ duplicate_of : Nat },
    TemporarilyUnavailable,
    GenericError{ error_code : Nat, message : String },
}

#[derive(CandidType, Deserialize)]
pub struct TransferFromArgs{
    pub from : Account,
    pub to : Account,
    pub amount : Nat,
    pub fee : Option<Nat>,
    pub memo : Option<Vec<u8>>,
    pub created_at_time : Option<u64>,
}

impl TransferFromArgs{
    pub fn new(to: Account, fee: Nat, from: Account, amount: Nat) -> Self{
        Self{
            to,
            fee: Some(fee.clone()),
            memo: None,
            from,
            created_at_time: None,
            amount: amount - fee,
        }
    }
}

#[derive(CandidType, Deserialize)]
pub enum TransferFromError{
    BadFee{ expected_fee : Nat },
    BadBurn{ min_burn_amount : Nat },
    InsufficientFunds{ balance : Nat },
    InsufficientAllowance{ allowance : Nat },
    TooOld,
    CreatedInFuture{ ledger_time : u64 },
    Duplicate{ duplicate_of : Nat },
    TemporarilyUnavailable,
    GenericError{ error_code : Nat, message : String },
}

#[derive(CandidType, Deserialize)]
pub struct AllowanceArgs{
    account : Account,
    spender : Account,
}

#[derive(CandidType, Deserialize)]
pub struct Allowance{
    pub allowance: Nat,
    pub expires_at: Option<u64>,
}

#[async_trait]
pub trait Icrc2: Icrc1 + TokenPrincipalFetcher{
    async fn icrc2_approve(&self, args: ApproveArgs) -> CallResult<(Result<Nat, ApproveError>,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc2_approve", (args,)).await
    }

    async fn icrc2_transfer_from(&self, args: TransferFromArgs) -> CallResult<(Result<Nat, TransferFromError>,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc2_transfer_from", (args,)).await
    }

    async fn icrc2_allowance(&self, args: AllowanceArgs) -> CallResult<(Allowance,)>{
        let token = <Self as TokenPrincipalFetcher>::token_principal(&self);
        ic_cdk::call(token, "icrc2_allowance", (args,)).await
    }
}

pub struct Icrc2Token(Principal);

impl Icrc2Token{
    pub fn new(principal: Principal) -> Self{
        Self(principal)
    }
}

impl TokenPrincipalFetcher for Icrc2Token{
    fn token_principal(&self) -> Principal {
        self.0.clone()
    }
}

impl Icrc1 for Icrc2Token{}

impl Icrc2 for Icrc2Token{}