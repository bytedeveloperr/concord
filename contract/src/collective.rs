use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{AccountId, Balance};

pub type CollectiveId = String;
pub type CollectiveMetadataHash = String;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
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
    pub collective_metadata_hash: CollectiveMetadataHash,
    pub collective_contributors: UnorderedMap<AccountId, Balance>,
}

impl Collective {
    pub fn new(
        token_id: AccountId,
        collective_type: CollectiveType,
        collective_creator: AccountId,
        collective_metadata_hash: CollectiveMetadataHash,
    ) -> Self {
        Self {
            token_id,
            collective_type,
            collective_creator,
            collective_metadata_hash,
            collective_balance: 0,
            collective_contributors: UnorderedMap::new("dd".as_bytes()),
        }
    }
}
