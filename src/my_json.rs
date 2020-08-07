use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Deserialize, Serialize, Debug)]
pub struct Person {
    name: String,
    age: u8,
    address: String,
}

impl Person {
    pub fn new() -> Person {
        Person {
            name: String::new(),
            age: 0,
            address: String::new(),
        }
    }

    pub fn from_json(json: &str) -> Result<Self> {
        let p: Person = serde_json::from_str(json)?;

        Ok(p)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn set_name(&mut self, name: &str) -> &mut Self {
        self.name = String::from(name);
        self
    }

    pub fn set_age(&mut self, age: u8) -> &mut Self {
        self.age = age;
        self
    }

    pub fn set_address(&mut self, address: &str) -> &mut Self {
        self.address = String::from(address);
        self
    }

    pub fn to_json(&self) -> Result<String> {
        let json = serde_json::to_string(self)?;

        Ok(json)
    }
}
