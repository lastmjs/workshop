use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::{env, near_bindgen, Promise, ext_contract};
use serde_json::json;

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct HotelFactory {}

#[ext_contract]
pub trait ExtHotel {
    fn new_random(seed: u8, num_stays: u64);
}

#[near_bindgen]
impl HotelFactory {
    /// Asynchronously deploy several hotels.
    pub fn deploy_hotels(&self, num_hotels: u8, stays_per_hotel: u64) {
        let tokens_per_hotel = env::account_balance()/(num_hotels as u128 + 1); // Leave some for this account.
        let gas_per_hotel = env::prepaid_gas()/(num_hotels as u64 + 1); // Leave some for this execution.
        for i in 0..num_hotels {
            let account_id = format!("hotel{}", i);
            Promise::new(account_id.clone())
                .create_account()
                .transfer(tokens_per_hotel)
                .add_full_access_key(env::signer_account_pk())
                .deploy_contract(
                    include_bytes!("../../hotel/target/wasm32-unknown-unknown/release/hotel.wasm").to_vec(),
                ).then(ext_hotel::new_random(i, stays_per_hotel, &account_id, 0, gas_per_hotel));
        }
    }
}
