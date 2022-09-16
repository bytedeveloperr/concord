use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, Balance};

pub type CollectiveId = String;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum CollectiveType {
    USER,
    FUND,
    EVENT,
    PROJECT,
    COLLECTIVE,
    ORGANIZATION,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Collective {
    pub token_id: AccountId,
    pub collective_balance: Balance,
    pub collective_type: CollectiveType,
    pub collective_creator: AccountId,
    pub metadata_hash: String,
}

impl Collective {
    pub fn new(
        token_id: AccountId,
        collective_type: CollectiveType,
        collective_creator: AccountId,
        metadata_hash: String,
    ) -> Self {
        Self {
            token_id,
            collective_type,
            collective_creator,
            metadata_hash,
            collective_balance: 0,
        }
    }
}
