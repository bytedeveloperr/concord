use crate::collective::CollectiveId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, Balance};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum AccountType {
    INDIVIDUAL,
    ORGANIZATION,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub account_id: AccountId,
    pub account_type: AccountType,
    pub collectives: UnorderedMap<CollectiveId, Balance>,
}

impl Account {
    pub fn new(account_id: AccountId, account_type: AccountType) -> Self {
        Self {
            account_type,
            account_id: account_id.clone(),
            collectives: UnorderedMap::new(account_id.as_bytes()),
        }
    }

    pub fn save_collective(&mut self, collective_id: CollectiveId, amount: Balance) {
        let new_amount = self
            .get_collective_balance(collective_id.clone())
            .checked_add(amount)
            .unwrap_or_else(|| env::panic_str("Error: amount overflow"));

        self.collectives.insert(&collective_id, &new_amount);
    }

    pub fn get_collectives(&self, from_index: usize, limit: usize) -> Vec<(CollectiveId, Balance)> {
        self.collectives
            .iter()
            .skip(from_index)
            .take(limit)
            .collect()
    }

    pub fn get_collective_balance(&self, collective_id: CollectiveId) -> Balance {
        self.collectives.get(&collective_id).unwrap_or(0)
    }
}
