use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::error::{FfbeError, Result};

#[derive(Default, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterInfo {
    pub r#type: String,
    pub name: String,
    pub rarity: Option<String>,
}

#[derive(Deserialize)]
pub struct Db(HashMap<u32, CharacterInfo>);

impl Db {
    pub fn new() -> Self {
        Self(HashMap::<u32, CharacterInfo>::new())
    }
    pub fn from_file(path: &str) -> Result<Self> {
        let contents = match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => return Err(err.into()),
        };
        let db: Db = match serde_json::from_str(&contents) {
            Ok(db) => db,
            Err(err) => return Err(err.into()),
        };

        Ok(db)
    }
    pub fn insert(&mut self, key: u32, val: CharacterInfo) {
        self.0.insert(key, val);
    }
    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, u32, CharacterInfo> {
        self.0.iter()
    }
    pub fn find_by_name(&self, name: &str) -> Result<u32> {
        for (key, val) in self.iter() {
            if val.name.to_lowercase() == name.to_lowercase() {
                println!("Found character {name}, ID = {key}");
                return Ok(*key);
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
            rarity: None,
        };
        let rosa = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Rosa"),
            rarity: None,
        };
        let cecil = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Cecil"),
            rarity: None,
        };
        db.insert(204000203, kain.clone());
        db.insert(204000304, rosa.clone());
        db.insert(204000103, cecil.clone());

        assert!(db.find_by_name("Rosa").is_ok_and(|uid| uid == 204000304));
        assert!(db.find_by_name("Cecil").is_ok_and(|uid| uid == 204000103));
        assert!(db.find_by_name("Kain").is_ok_and(|uid| uid == 204000203))
    }

    #[test]
    fn test_find_character_by_name_case_insensitve() {
        let mut db: Db = Db::new();
        let rain = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Rain"),
            rarity: None,
        };
        let cecil = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Cecil"),
            rarity: None,
        };
        db.insert(100000102, rain);
        db.insert(204000103, cecil);

        assert!(db.find_by_name("RAIN").is_ok_and(|uid| uid == 100000102));
        assert!(db.find_by_name("cEcIL").is_ok_and(|uid| uid == 204000103));
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
        temp_file
            .write_all(json_content.as_bytes())
            .expect("Failed to write to temp file");

        // Test loading from file
        let db =
            Db::from_file(temp_file.path().to_str().unwrap()).expect("Failed to load from file");

        // Verify the data was loaded correctly
        let rain = db.find_by_name("Rain").expect("Rain should be found");
        assert_eq!(rain, 100000102);

        let lasswell = db
            .find_by_name("Lasswell")
            .expect("Lasswell should be found");
        assert_eq!(lasswell, 100000202);

        let fina = db.find_by_name("Fina").expect("Fina should be found");
        assert_eq!(fina, 100000302);

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
