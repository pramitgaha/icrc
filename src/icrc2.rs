use candid::{Nat, Principal};
use async_trait::async_trait;
use ic_cdk::api::call::CallResult;
use crate::icrc1::{TokenPrincipalFetcher, Icrc1};
use icrc_ledger_types::icrc2::{allowance::*, approve::*, transfer_from::*};

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

    async fn icrc2_allowance(&self, args: AllowanceArgs) -> CallResult<(AllowanceArgs,)>{
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