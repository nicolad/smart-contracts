use crate::Contract;
use crate::ContractExt;

use near_sdk::{AccountId};
use near_sdk::json_types::U128;

pub struct Pledge {
    pub account_id: AccountId,
    pub total_amount: U128
}