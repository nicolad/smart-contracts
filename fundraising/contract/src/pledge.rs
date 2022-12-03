use crate::Contract;
use crate::ContractExt;

use near_sdk::{AccountId, near_bindgen};
use near_sdk::json_types::U128;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Pledge {
    pub account_id: AccountId,
    pub total_amount: U128
}

#[near_bindgen]
impl Contract {
    pub fn get_number_of_pledges(&self) -> u64 {
        self.pledges.len()        
    }
}