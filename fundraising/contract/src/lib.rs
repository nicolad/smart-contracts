use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId};


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub fundraiser: AccountId,
    pub pledges: UnorderedMap<AccountId, u128>
}

impl Default for Contract{
    fn default() -> Self{
        Self {
            fundraiser: "nicolad.testnet".parse().unwrap(),
            pledges: UnorderedMap::new(b"p")
        }
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    #[private]
    pub fn init(fundraiser: AccountId) -> Self {
        assert!(!env::state_exists(), "Contract already initialized");

        Self {
            fundraiser: fundraiser,
            pledges: UnorderedMap::new(b"p")
        }
    }

    pub fn get_fundraiser(&self) -> AccountId {
        self.fundraiser.clone()
    }
}
