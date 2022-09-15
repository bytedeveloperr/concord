mod account;
mod collective;

use account::Account;
use collective::Collective;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{env, near_bindgen};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Concord {
    accounts: LookupMap<String, Account>,
    collectives: LookupMap<String, Collective>,
}

impl Default for Concord {
    fn default() -> Self {
        env::panic_str("Error: default not implemented")
    }
}

#[near_bindgen]
impl Concord {
    #[init]
    pub fn new() -> Self {
        Self {
            accounts: LookupMap::new(b"account".to_vec()),
            collectives: LookupMap::new(b"collective".to_vec()),
        }
    }
}
