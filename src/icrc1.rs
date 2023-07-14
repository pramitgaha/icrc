use candid::{Principal, Nat};
use ic_cdk::api::call::CallResult;
use async_trait::async_trait;
use icrc_ledger_types::icrc1::{account::Account, transfer::{TransferArg, TransferError}};

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

    async fn icrc1_balance_of(&self, account: Account) -> CallResult<(Nat,)>{
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