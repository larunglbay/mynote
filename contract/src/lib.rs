/*
 * This is an example of a Rust smart contract with two simple, symmetric functions:
 *
 * 1. set_greeting: accepts a greeting, such as "howdy", and records it for the user (account_id)
 *    who sent the request
 * 2. get_greeting: accepts an account_id and returns the greeting saved for it, defaulting to
 *    "Hello"
 *
 * Learn more about writing NEAR smart contracts with Rust:
 * https://github.com/near/near-sdk-rs
 *
 */


// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{AccountId, env, near_bindgen, setup_alloc};
use near_sdk::serde::{Serialize,Deserialize};
// use near_sdk::collections::HashMap;
use std::collections::HashMap;
setup_alloc!();


#[derive(Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct NoteItem {
    title: String,
    content: String
}

// Structs in Rust are similar to other languages, and may include impl keyword as shown below
// Note: the names of the structs are not important when calling the smart contract, but the function names are
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct MyNote {
    notes: HashMap<String, HashMap<String, NoteItem>>,
}

impl Default for MyNote {
    fn default() -> Self {
        Self {
            notes : HashMap::new()
        }
    }
}


#[near_bindgen]
impl MyNote {

    #[init]
    pub fn new() -> Self {
        Self {
            notes : HashMap::new()
        }
    }


    pub fn insert_note(&mut self, name: &String, _content: &String) {
        let account_id = env::signer_account_id();

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("Saving accountId {} : title '{:?}' and content '{:?}'", account_id, name, _content).as_bytes());

        let note_account_opt = self.notes.get_mut(&account_id);

        match note_account_opt {
            Some(not_account) =>
                if !not_account.contains_key(name) {
                    not_account.insert(name.to_string(), NoteItem{ title: name.to_string(), content: _content.to_string() });
                } else {
                    not_account.remove(name);
                    not_account.insert(name.to_string(), NoteItem{ title: name.to_string(), content: _content.to_string() });
                },
            None => {
                let mut note = HashMap::<String, NoteItem>::new();
                note.insert(name.to_string(), NoteItem{ title: name.to_string(), content: _content.to_string()});
                self.notes.insert(account_id, note);
            }
        }
    }


    pub fn edit_note(&mut self, name: &String, _content: &String) {
        let account_id = env::signer_account_id();

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("Saving accountId {} : title '{:?}' and content '{:?}'", account_id, name, _content).as_bytes());

        let note = self.notes.get_mut(&account_id);

        match note {
            Some(n) =>
                if n.contains_key(name) {
                    let note_item = n.get_mut(name).unwrap();
                    note_item.content = _content.to_string();
                } else {
                    println!("{}",format!("Not found note has name {:?}",name ));
                }
            None => {
                println!("{}",format!("Not found note has name {:?}",name ));
            }
        }
    }


    pub fn get_notes(&self) -> HashMap<String, NoteItem> {
        let account_id = env::signer_account_id();
        env::log(format!("Account '{:?}'", account_id).as_bytes());


        let notes = self.notes.get(&account_id);

        match notes {
            Some(n) =>  n.clone(),
            None => HashMap::<String, NoteItem>::new()
        }
    }
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 *
 * To run from contract directory:
 * cargo test -- --nocapture
 *
 * From project root, to run in combination with frontend tests:
 * yarn test
 *
 */
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn set_then_get_notes() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = MyNote::default();
        contract.insert_note(&"hello".to_string(), &"Hi".to_string());
        assert_eq!(
            true,
            contract.get_notes().contains_key("hello")
        );
    }

}
