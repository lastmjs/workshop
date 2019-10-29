use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{env, near_bindgen};
use std::collections::HashMap;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Chat {
    unread_messages: HashMap<String, Vec<(String, String)>>,
}

#[near_bindgen]
impl Chat {
    pub fn leave_message(&mut self, receiver_id: String, message: String) {
        let sender_id = env::signer_account_id();
        self.unread_messages.entry(receiver_id).or_insert_with(Vec::new).push((sender_id, message));
    }

    pub fn get_unread_messages(&self) -> Vec<(String, String)>{
        let receiver_id = env::signer_account_id();
        self.unread_messages.get(&receiver_id).cloned().unwrap_or_else(Vec::new)
    }

    pub fn mark_all_as_read(&mut self) {
        let receiver_id = env::signer_account_id();
        self.unread_messages.remove(&receiver_id);
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use super::*;
    use near_bindgen::MockedBlockchain;
    use near_bindgen::{testing_env, Config, VMContext};

    fn get_context(signer_account_id: &str) -> VMContext {
        VMContext {
            current_account_id: "chat.near".to_string(),
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
        let mut contract = Chat::default();
        // Alice sends message to Bob.
        testing_env!(get_context("alice_near"), Config::default());
        contract.leave_message("bob_near".to_string(), "Hey!".to_string());
        // Carol sends message to Bob.
        testing_env!(get_context("carol_near"), Config::default());
        contract.leave_message("bob_near".to_string(), "Hi!".to_string());

        // Bob gets message that he received from Alice and Carol.
        testing_env!(get_context("bob_near"), Config::default());
        assert_eq!(contract.get_unread_messages(), vec![
            ("alice_near".to_string(), "Hey!".to_string()),
            ("carol_near".to_string(), "Hi!".to_string()),
        ]);
    }

    #[test]
    fn no_messages() {
        let contract = Chat::default();
        // Alice has no messages.
        testing_env!(get_context("alice_near"), Config::default());
        assert_eq!(contract.get_unread_messages(), vec![]);
    }

    #[test]
    fn several_users() {
        let mut contract = Chat::default();
        // Alice sends message to Bob.
        testing_env!(get_context("alice_near"), Config::default());
        contract.leave_message("bob_near".to_string(), "Hey!".to_string());
        // Carol sends message to Alice.
        testing_env!(get_context("carol_near"), Config::default());
        contract.leave_message("alice_near".to_string(), "Hi!".to_string());

        // Bob gets message that he received from Alice.
        testing_env!(get_context("bob_near"), Config::default());
        assert_eq!(contract.get_unread_messages(), vec![
            ("alice_near".to_string(), "Hey!".to_string()),
        ]);

        // Alice gets message that she received from Carol.
        testing_env!(get_context("alice_near"), Config::default());
        assert_eq!(contract.get_unread_messages(), vec![
            ("carol_near".to_string(), "Hi!".to_string()),
        ]);
    }

    #[test]
    fn read_all() {
        let mut contract = Chat::default();
        // Alice sends message to Bob.
        testing_env!(get_context("alice_near"), Config::default());
        contract.leave_message("bob_near".to_string(), "Hey!".to_string());

        // Bob marks all messages as read and checks that there are no unread messages.
        testing_env!(get_context("bob_near"), Config::default());
        contract.mark_all_as_read();
        assert_eq!(contract.get_unread_messages(), vec![]);
    }
}
