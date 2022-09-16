use crate::account::Account;
use crate::collective::{Collective, CollectiveId, CollectiveMetadataHash, CollectiveType};
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

    pub fn create_collective(
        &mut self,
        token_id: AccountId,
        collective_id: CollectiveId,
        collective_type: CollectiveType,
        collective_metadata_hash: CollectiveMetadataHash,
    ) {
        let collective_creator = env::predecessor_account_id();

        self.internal_create_collective(
            token_id,
            collective_id,
            collective_creator,
            collective_type,
            collective_metadata_hash,
        )
    }

    #[private]
    fn internal_get_collective(&self, collective_id: CollectiveId) -> Collective {
        self.collectives
            .get(&collective_id)
            .unwrap_or_else(|| env::panic_str("Error: collective does not exist"))
    }

    #[private]
    fn internal_create_collective(
        &mut self,
        token_id: AccountId,
        collective_id: CollectiveId,
        collective_creator: AccountId,
        collective_type: CollectiveType,
        collective_metadata_hash: CollectiveMetadataHash,
    ) {
        match self.collectives.contains_key(&collective_id) {
            true => env::panic_str("Error: collective id already exist"),
            false => {
                let collective = Collective::new(
                    token_id,
                    collective_type,
                    collective_creator,
                    collective_metadata_hash,
                );

                self.collectives.insert(&collective_id, &collective);
            }
        }
    }
}
