use near_sdk::{AccountId, Balance};

pub struct Collective {
    pub token_id: AccountId,
    pub collective_type: String,
    pub collective_creator: AccountId,
}

impl Collective {
    pub fn new() {}

    pub fn get_collective(&self, _collective_id: String) {}

    pub fn get_collective_balance(&self, _collective_id: String) -> Balance {
        0
    }
}
