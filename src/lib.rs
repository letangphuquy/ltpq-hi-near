#![allow(dead_code)]
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::{near_bindgen, AccountId, Balance, Timestamp, env};
//Not LeetCode :)
// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
  pub owner: AccountId,
  pub problems_count: u32
}

#[derive(BorshDeserialize, BorshSerialize, Serialize)]
#[serde (crate = "near_sdk::serde")]
pub struct Exercise {
  pub problem_id: String,
  pub content: String, //markdown file
  pub solution_cost: Balance, //(premium) value for one problem is based on various factors, like difficulty, practicability, ... and is determined by admin. 
  //captivation for contributors like problem author, testcase setter, ... ?
  pub creation_time: Timestamp
}

#[near_bindgen]
impl Contract {
  #[init]
  pub fn init() -> Self {
    //call-self
    Self { owner: env::signer_account_id(), problems_count: 0 }
  }

  pub fn create_exercise(problem_id: String, content: String) -> Exercise {
    Exercise { problem_id, content, solution_cost: 1, creation_time: env::block_timestamp_ms() }
  }
}
 

