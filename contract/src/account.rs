use near_sdk::AccountId;

pub enum AccountType {
    INDIVIDUAL,
    ORGANIZATION,
}

pub struct Account {
    pub account_id: AccountId,
    pub account_type: AccountType,
}

impl Account {
    pub fn new(&mut self, account_id: AccountId, account_type: AccountType) -> Self {
        Self {
            account_id,
            account_type,
        }
    }
}
