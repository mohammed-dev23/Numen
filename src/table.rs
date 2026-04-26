use std::collections::HashMap;

use crate::chunk::Values;

// this struct basicly stores values by there strings
pub struct Table {
    // we use hashmap with string and values
    hash_table: HashMap<String, Values>,
}

impl Table {
    // sets a new hashmap
    pub fn new() -> Self {
        Self {
            hash_table: HashMap::new(),
        }
    }

    pub fn set_table(&mut self, key: String, value: Values) -> bool {
        // here we first check if the key already exsits or not if it does it
        // returns false if it doesn't it return true
        let is_new = !self.hash_table.contains_key(&key);
        // we insert the key and value to the hashmap
        self.hash_table.insert(key, value);
        // return this at last
        is_new
    }

    // this func used to copy all entries to new one
    pub fn add_all(&mut self, table: &Table) {
        // by going through the passed table and getting the key and value
        for (key, value) in &table.hash_table {
            // then we insert them into the new one
            self.hash_table.insert(key.clone(), value.clone());
        }
    }

    pub fn get_value(&self, key: String) -> Option<&Values> {
        // here we get the value by simply calling it if it ex
        // we get Some(v) if it's not we get None
        self.hash_table.get(&key)
    }

    pub fn delete(&mut self, key: String) -> bool {
        // if we got is_some or some that means the value got removed
        // if we get None that means the value did not get removed for some reson
        // but mostly it does not ex
        self.hash_table.remove(&key).is_some()
    }
}
