use serde::Deserialize;
use std::fs;

/// Struct to parse the data.json
#[derive(Debug, Deserialize)]
pub struct Data {
    pub members: i32,
    pub to_do: i32,
    pub connexion: i32,
}

impl Data {
    /// Open the data.json file and parses him in the struct Data
    pub fn get_json() -> Data {
        let file = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/", "data.json"))
            .expect("Unable to read file");
        serde_json::from_str(file.as_str()).expect("JSON was not well-formatted")
    }

    /// Update the value in the file to the one of the struct
    pub fn update_json(&self) {

    }
}

#[cfg(test)]
mod test {
    use crate::Data;

    #[test]
    fn test_data_json() {
        Data::get_json();
    }
}
