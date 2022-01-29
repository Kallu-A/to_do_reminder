extern crate serde_json;

use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;

const PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "data.json");

/// Struct to parse the data.json
#[derive(Debug, Deserialize, Serialize)]
pub struct Data {
    pub members: i32,
    pub to_do: i32,
    pub connexion: i32,
}

impl Data {
    /// Open the data.json file and parses him in the struct Data
    pub fn get_json() -> Data {
        let file = fs::read_to_string(PATH).expect("Unable to read file");
        serde_json::from_str(file.as_str()).expect("JSON was not well-formatted")
    }

    /// Update the value in the file to the one of the struct
    /// the value can't be set to < 0
    pub fn update_json(&mut self) {
        self.members = if self.members < 0 { 0 } else { self.members };
        self.connexion = if self.connexion < 0 {
            0
        } else {
            self.connexion
        };
        self.to_do = if self.to_do < 0 { 0 } else { self.to_do };

        ::serde_json::to_writer(&File::create(PATH).unwrap(), &self).unwrap();
    }
}

/// Increment the value of user
pub fn incr_members() {
    let mut data = Data::get_json();
    data.members += 1;
    data.update_json();
}

/// Increment the value of connexion
pub fn incr_connexion() {
    let mut data = Data::get_json();
    data.connexion += 1;
    data.update_json();
}

/// Decrement the value of connexion until it's value 0
pub fn decr_members() {
    let mut data = Data::get_json();
    if data.members > 0 {
        data.members -= 1;
        data.update_json();
    }
}



///  Increment the value of to-do
pub fn incr_to_do() {
    let mut data = Data::get_json();
    data.to_do += 1;
    data.update_json();
}

#[cfg(test)]
mod test {
    use crate::Data;

    #[test]
    fn test_data_json() {
        let mut data = Data::get_json();
        data.update_json();
    }
}
