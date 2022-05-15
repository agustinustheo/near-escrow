use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, metadata, near_bindgen, Balance, Promise};

use std::collections::HashMap;

use rand::{thread_rng, Rng};
use rand::distributions::{Alphanumeric};

metadata! {
    #[near_bindgen]
    #[derive(Default, BorshDeserialize, BorshSerialize)]
    pub struct Escrow {
        balances: HashMap<String, Balance>,
    }

    #[near_bindgen]
    impl Escrow {
        #[payable]
        pub fn submit_escrow(&mut self) -> String {
            let random_string: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(24)
                .map(char::from)
                .collect();

            self.balances.insert(random_string.clone(), env::attached_deposit());
            random_string
        }

        pub fn retrieve_escrow(&mut self, key: String) -> Option::<Promise> {
            if !self.balances.contains_key(&key) {
                return None;
            }

            let pr = Promise::new(env::signer_account_id())
                .transfer(*self.balances.get(&key).unwrap());
            self.balances.remove(&key);
                        
            Some(pr)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::{get_logs, VMContextBuilder};

    use std::convert::TryInto;

    fn get_context(is_view: bool) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id("bob_near".try_into().unwrap())
            .is_view(is_view)
            .build()
    }

    #[test]
    fn retrieve_empty_escrow() {
        let context = get_context(false);
        testing_env!(context);

        let mut contract = Escrow::default();
        match contract.retrieve_escrow("test".to_string()) {
            None => {},
            _ => panic!(),
        };
    }
}
