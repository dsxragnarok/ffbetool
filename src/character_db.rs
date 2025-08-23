use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::error::{FfbeError, Result};

#[derive(Default, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterInfo {
    pub r#type: String,
    pub name: String,
    pub rarity: String,
}

#[derive(Deserialize)]
pub struct Db(HashMap<u32, CharacterInfo>);

impl Db {
    pub fn new() -> Self {
        Self(HashMap::<u32, CharacterInfo>::new())
    }
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = fs::read_to_string(path)?;
        let db: Db = serde_json::from_str(&contents)?;

        Ok(db)
    }
    pub fn insert(&mut self, key: u32, val: CharacterInfo) {
        self.0.insert(key, val);
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u32, CharacterInfo> {
        self.0.iter()
    }
    pub fn find_by_name(&self, name: &str) -> Result<CharacterInfo> {
        for (key, val) in self.iter() {
            if &val.name == name {
                println!("Found character {name}, ID = {key}");
                return Ok(val.clone());
            }
        }
        Err(FfbeError::CharacterNotFound(String::from(name)))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_find_character_by_name() {
        let mut db: Db = Db::new();
        let kain = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Kain"),
            rarity: String::from(""),
        };
        let rosa = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Rosa"),
            rarity: String::from(""),
        };
        let cecil = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Cecil"),
            rarity: String::from(""),
        };
        db.insert(204000203, kain.clone());
        db.insert(204000304, rosa.clone());
        db.insert(204000103, cecil.clone());

        assert!(
            db.find_by_name("Rosa")
                .is_ok_and(|character| character == rosa)
        );
        assert!(
            db.find_by_name("Cecil")
                .is_ok_and(|character| character == cecil)
        );
        assert!(
            db.find_by_name("Kain")
                .is_ok_and(|character| character == kain)
        )
    }

    #[test]
    fn test_from_file() {
        // Create a temporary JSON file with test data
        let json_content = r#"{
            "100000102": {"type": "story", "name": "Rain", "rarity": ""},
            "100000202": {"type": "story", "name": "Lasswell", "rarity": ""},
            "100000302": {"type": "story", "name": "Fina", "rarity": ""}
        }"#;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file.write_all(json_content.as_bytes()).expect("Failed to write to temp file");
        
        // Test loading from file
        let db = Db::from_file(temp_file.path().to_str().unwrap()).expect("Failed to load from file");
        
        // Verify the data was loaded correctly
        let rain = db.find_by_name("Rain").expect("Rain should be found");
        assert_eq!(rain.name, "Rain");
        assert_eq!(rain.r#type, "story");
        assert_eq!(rain.rarity, "");

        let lasswell = db.find_by_name("Lasswell").expect("Lasswell should be found");
        assert_eq!(lasswell.name, "Lasswell");
        assert_eq!(lasswell.r#type, "story");

        let fina = db.find_by_name("Fina").expect("Fina should be found");
        assert_eq!(fina.name, "Fina");
        assert_eq!(fina.r#type, "story");

        // Verify that a non-existent character returns an error
        assert!(db.find_by_name("NonExistent").is_err());
    }

    #[test]
    fn test_from_file_nonexistent() {
        // Test that loading from a non-existent file returns an error
        let result = Db::from_file("nonexistent_file.json");
        assert!(result.is_err());
    }
}
