use crate::account::Account;
use crate::collective::{Collective, CollectiveId};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::{self, env, near_bindgen, AccountId};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Concord {
    pub accounts: LookupMap<AccountId, Account>,
    pub collectives: LookupMap<CollectiveId, Collective>,
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

    pub fn get_collective(&self, collective_id: CollectiveId) -> Collective {
        self.internal_get_collective(collective_id)
    }

    #[private]
    fn internal_get_collective(&self, collective_id: CollectiveId) -> Collective {
        match self.collectives.get(&collective_id) {
            Some(collective) => collective,
            _ => env::panic_str("Error: collective does not exist"),
        }
    }
}
