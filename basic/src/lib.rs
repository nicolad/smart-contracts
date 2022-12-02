use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{near_bindgen, env};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]

pub struct Contract {
    password_solution: String
}

#[near_bindgen]
impl Contract {
    #[init] 
    pub fn new(solution: String) -> Self {
        Self {
            password_solution: solution
        }
    }

    pub fn get_solution(&self) -> String {
        self.password_solution.clone()
    }

    pub fn guess_solution(&mut self, solution: String) -> bool {
        let hashed_input = env::sha256(solution.as_bytes());
        let hashed_input_hex = hex::encode(&hashed_input);
        if hashed_input_hex == self.password_solution {
            env::log_str("Correct!");
            true
        } else {
            env::log_str("Incorrect!");
            false
        }
    }
}

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::{get_logs, VMContextBuilder};
    use near_sdk::{testing_env, AccountId};

    // part of writing unit tests is setting up a mock context
    // provide a `predecessor` here, it'll modify the default context
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }

    #[test]
    fn debug_get_hash() {

        testing_env!(VMContextBuilder::new().build());

        let debug_solution = "tamago kake gohan";
        let debug_hash_bytes = env::sha256(debug_solution.as_bytes());
        let debug_hash_string = hex::encode(debug_hash_bytes);
        println!("Debug hash: {:?} ", debug_hash_string);
    }

    #[test]
    fn check_guess_solution() {
        let nicolad = AccountId::new_unchecked("nicolad.testnet".to_string());
        let context = get_context(nicolad.clone());
        testing_env!(context.build());

        let mut contract = Contract::new("8f968bdd5667276cc026dac352e423474564b3968f0d5e0a9023d8ef8f1f75c1".to_string(), );

        let mut guess_result = contract.guess_solution("tamago kake gohan".to_string());
        assert!(guess_result);
        assert_eq!(get_logs(), vec!["Correct!"]);

        guess_result = contract.guess_solution("wrong".to_string());
        assert_eq!(get_logs(), vec!["Correct!", "Incorrect!"]);
    }
}