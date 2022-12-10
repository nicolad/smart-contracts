use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::Serialize;
use near_sdk::{AccountId, log, near_bindgen};
use near_sdk::collections::{LookupMap, UnorderedSet};

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[derive(Clone)]
pub struct Token {
    token_id: u128,
    owner_id: AccountId,
    name: String,
    description: String,
    media_uri: String,
    level: u128,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct NFTContract {
    owner_by_id: UnorderedMap<u128, AccountId>,
    token_id: u128,
    token_by_id: UnorderedMap<u128, Token>,
}

impl Default for NFTContract {
    fn default() -> Self {
        Self {        
            owner_by_id: UnorderedMap::new(b'm'),
            token_id: 0,
            token_by_id: UnorderedMap::new(b'n')
        }
    }
}

#[near_bindgen]
impl NFTContract {
    pub fn mint(&mut self, token_owner_id: AccountId, name: String, description: String, media_uri: String, level: u128) -> Token {
        self.owner_by_id.insert(&self.token_id, &token_owner_id);

        let token = Token {
            token_id: self.token_id,
            owner_id: token_owner_id,
            name: name,
            description: description,
            media_uri: media_uri,
            level: level,
        };

        self.token_by_id.insert(&self.token_id, &token);
        self.token_id += 1;
        return token;
    }

    pub fn get_total_tokens(&self) -> u128 {
        return self.token_id;
    }
 
    pub fn get_token_by_id(&self, token_id: u128) -> Token {
        if let None = self.token_by_id.get(&token_id) {
            return Token {
                token_id: token_id,      
                owner_id: "nicolad.testnet".parse().unwrap(),
                name: "None".to_string(),
                description: "None".to_string(),
                media_uri: "None".to_string(),
                level: 0,
            };
        } else {
            return self.token_by_id.get(&token_id).unwrap();
        }
    }
}