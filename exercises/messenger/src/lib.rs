use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{env, near_bindgen, ext_contract, Promise};
use std::collections::HashMap;
use serde_json::json;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Messenger {
    // sender_id -> left messages.
    unread_messages: HashMap<String, Vec<String>>,
}

/// Methods called by other contracts.
#[ext_contract]
pub trait ExtMessenger {
    fn leave_message(&mut self, message: String);
    fn get_unread_messages(&self, sender_id: String) -> Vec<String>;
}

#[near_bindgen]
impl Messenger {
    pub fn send(&self, receiver_id: String, message: String) -> Promise {
        let prepaid_gas = env::prepaid_gas();
        let this_account = env::current_account_id();
        ext_messenger::leave_message(message, &receiver_id, 0, prepaid_gas/3)
            .then(ext_messenger::get_unread_messages(this_account, &receiver_id, 0, prepaid_gas/3))
    }

    pub fn get_all_unread_messages(&self) -> HashMap<String, Vec<String>> {
        self.unread_messages.clone()
    }

    pub fn mark_all_as_read(&mut self) {
        self.unread_messages.clear();
    }
}

#[near_bindgen]
impl ExtMessenger for Messenger {
    fn leave_message(&mut self, message: String) {
        let sender_id = env::predecessor_account_id();
        self.unread_messages.entry(sender_id).or_insert_with(Vec::new).push(message);
    }

    fn get_unread_messages(&self, sender_id: String) -> Vec<String> {
        unimplemented!()
    }
}

