use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, metadata, near_bindgen, AccountId, Balance, Promise};

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
            if near_sdk::env::attached_deposit() == 0 {
                near_sdk::env::panic(b"Deposit must be more than zero");
            }

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
    use near_sdk::test_utils::test_env::{alice, bob};
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    fn set_predecessor_and_deposit(predecessor: AccountId, deposit: Balance) {
        testing_env!(VMContextBuilder::new()
            .predecessor_account_id(predecessor)
            .attached_deposit(deposit)
            .build())
    }


    #[test]
    fn retrieve_empty_escrow() {
        set_predecessor_and_deposit(alice(), 20);

        let mut contract = Escrow::default();
        match contract.retrieve_escrow("test".to_string()) {
            None => {},
            _ => panic!(),
        };
    }

    #[test]
    fn submit_escrow() {
        set_predecessor_and_deposit(alice(), 20);

        let mut contract = Escrow::default();
        let c = contract.submit_escrow();
    }

    #[test]
    fn retrieve_empty_escrow_2() {
        set_predecessor_and_deposit(alice(), 20);

        let mut contract = Escrow::default();
        match contract.retrieve_escrow("test".to_string()) {
            None => {},
            _ => panic!(),
        };
    }
}
