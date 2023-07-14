use icrc_ledger_types::icrc1::account::Account;

/// function is used for transforming an Account with subaccount None to an Account with default value for subaccount
pub fn account_transformer(account: Account) -> Account{
    if let Some(_) = account.subaccount{
        return account
    }
    Account { owner: account.owner, subaccount: Some([0; 32]) }
}