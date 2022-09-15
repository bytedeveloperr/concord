use near_sdk::{AccountId, Balance};

pub struct Account {
    pub account_id: AccountId,
    pub account_type: String,
    pub account_creator: AccountId,
}

impl Account {
    pub fn new() {}

    pub fn get_account(&self, _account_id: String) {}

    pub fn get_account_balance(&self, _account_id: String) -> Balance {
        0
    }
}
