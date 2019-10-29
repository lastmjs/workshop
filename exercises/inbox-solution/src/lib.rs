use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{env, near_bindgen, ext_contract};
use std::collections::HashMap;
use serde_json::json;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Inbox {
    // sender_id -> left messages.
    unread_messages: HashMap<String, Vec<String>>,
}

/// Methods called by other contracts.
#[ext_contract]
pub trait ExtInbox {
    fn leave_message(&mut self, message: String);
}

#[near_bindgen]
impl Inbox {
    pub fn send(&self, receiver_id: String, message: String) {
        let prepaid_gas = env::prepaid_gas();
        ext_inbox::leave_message(message, &receiver_id, 0, prepaid_gas/2);
    }

    pub fn get_all_unread_messages(&self) -> HashMap<String, Vec<String>> {
        self.unread_messages.clone()
    }

    pub fn mark_all_as_read(&mut self) {
        self.unread_messages.clear();
    }
}

#[near_bindgen]
impl ExtInbox for Inbox {
    fn leave_message(&mut self, message: String) {
        let sender_id = env::predecessor_account_id();
        self.unread_messages.entry(sender_id).or_insert_with(Vec::new).push(message);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, Config, VMContext};

    macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
    );

    fn get_context(signer_account_id: &str, current_account_id: &str) -> VMContext {
        VMContext {
            current_account_id: current_account_id.to_string(),
            signer_account_id: signer_account_id.to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: signer_account_id.to_string(),
            input: vec![],
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(9),
            random_seed: vec![0, 1, 2],
            is_view: false,
            output_data_receivers: vec![],
        }
    }

    #[test]
    fn leave_message() {
        let mut contract = Inbox::default();
        // Alice leaves a message for Bob.
        testing_env!(get_context("alice_near", "bob_near"), Config::default());
        contract.leave_message("Hey!".to_string());

        // Bob gets message that he received from Alice.
        testing_env!(get_context("bob_near", "bob_near"), Config::default());
        assert_eq!(contract.get_all_unread_messages(), map! {
         "alice_near".to_string() => vec!["Hey!".to_string()]
        });
    }
}

