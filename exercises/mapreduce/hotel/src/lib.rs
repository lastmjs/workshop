use borsh::{BorshDeserialize, BorshSerialize};
use near_bindgen::near_bindgen;
use rand::rngs::StdRng;
use rand::{SeedableRng, Rng};
use serde::{Serialize, Deserialize};

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
pub struct User {
    is_business: bool,
    id: String,
}

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
pub struct Reservation {
    user: User,
    amount: u64,
    date: String,
    room_number: u64
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Hotel {
    past_reservations: Vec<Reservation>,
}

#[near_bindgen(init => new_random)]
impl Hotel {
    pub fn new_random(seed: u8, num_stays: u64) -> Self {
        let mut res = Self {
            past_reservations: Vec::with_capacity(num_stays as _)
        };
        let mut rng = StdRng::from_seed([seed; 32]);
        for _ in 0..num_stays {
            let user_id = rng.gen::<u64>() % 100;
            let user = User {
                is_business: user_id % 2 == 0,
                id: format!("User{}", user_id)
            };
            let reservation = Reservation {
                user,
                amount: rng.gen::<u64>() % 100, date: format!("2019-10-{}", rng.gen::<u64>() % 31),
                room_number: rng.gen::<u64>() % 200,
            };
            res.past_reservations.push(reservation);
        }
        res
    }

    pub fn get_reservations(&self) -> Vec<Reservation> {
        self.past_reservations.clone()
    }
}
