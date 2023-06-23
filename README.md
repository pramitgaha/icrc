# icrc
Library for interacting with ICRC standard tokens on ICP

<h3> Adding as dependency </h3>
```json
[dependencies]
icrc = { git = "https://github.com/pramitgaha/icrc.git" }
```

<h3> Example </h3>

- Using Icrc1 standard token
```rust
use icrc::Icrc1::{Icrc1, Icrc1Token};

pub async fn balance_of(token: Principal, account: Account) -> Nat{
    let icrc1_token: Icrc1Token = Icrc1Token::new(token);
    icrc1_token.icrc1_balance_of(account).unwrap().0
}
```

<!-- - Implementing Trait on `Struct`
this will be useful if you know that you'll be interacting with a fix token canister upfront
```rs
use icrc::icrc1::{Icrc1, TokenPrincipalFetcher};

pub const CKBTC: Principal = Principal::from_slice(b"mxzaz-hqaaa-aaaar-qaada-cai");

pub struct User{
    pub account: Account,
}

impl TokenPrincipalFetcher for User{
    fn token_principal(&self) -> Principal{
        CKBTC
    }
}

impl Icrc1 for User{}
``` -->