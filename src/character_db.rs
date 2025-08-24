use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

use crate::error::Result;

#[derive(Default, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterInfo {
    pub r#type: String,
    pub name: String,
    pub rarity: Option<String>,
}

pub enum LookupResult {
    Found(u32),
    Multiple(Vec<(u32, CharacterInfo)>),
    NotFound,
}

#[derive(Default, Deserialize)]
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
    pub fn find_by_name(&self, name: &str) -> LookupResult {
        let mut exact_matches = Vec::new();
        let mut similar_matches = Vec::new();

        for (uid, char_info) in self.iter() {
            if char_info.name.eq_ignore_ascii_case(name) {
                exact_matches.push((*uid, char_info.clone()));
            } else if char_info
                .name
                .to_ascii_lowercase()
                .contains(&name.to_ascii_lowercase())
                || name
                    .to_ascii_lowercase()
                    .contains(&char_info.name.to_ascii_lowercase())
            {
                similar_matches.push((*uid, char_info.clone()));
            }
        }

        match (exact_matches.len(), similar_matches.len()) {
            (1, _) => LookupResult::Found(exact_matches[0].0),
            (0, 0) => LookupResult::NotFound,
            _ => {
                similar_matches.extend(exact_matches);
                LookupResult::Multiple(similar_matches)
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_find_character_by_name_exact_match() {
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

        // Test exact matches
        match db.find_by_name("Rosa") {
            LookupResult::Found(uid) => assert_eq!(uid, 204000304),
            _ => panic!("Expected Found result for Rosa"),
        }

        match db.find_by_name("Cecil") {
            LookupResult::Found(uid) => assert_eq!(uid, 204000103),
            _ => panic!("Expected Found result for Cecil"),
        }

        match db.find_by_name("Kain") {
            LookupResult::Found(uid) => assert_eq!(uid, 204000203),
            _ => panic!("Expected Found result for Kain"),
        }
    }

    #[test]
    fn test_find_character_by_name_case_insensitive() {
        let mut db: Db = Db::new();
        let rain = CharacterInfo {
            r#type: String::from("story"),
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

        // Test case-insensitive matches (should return Multiple since no exact match)
        match db.find_by_name("RAIN") {
            LookupResult::Found(uid) => {
                assert_eq!(uid, 100000102);
            }
            _ => panic!("Expected Found result for RAIN"),
        }

        match db.find_by_name("cEcIL") {
            LookupResult::Found(uid) => {
                assert_eq!(uid, 204000103);
            }
            _ => panic!("Expected Found result for cEcIL"),
        }
    }

    #[test]
    fn test_find_character_by_name_not_found() {
        let mut db: Db = Db::new();
        let rain = CharacterInfo {
            r#type: String::from("story"),
            name: String::from("Rain"),
            rarity: None,
        };
        db.insert(100000102, rain);

        match db.find_by_name("NonExistent") {
            LookupResult::NotFound => {} // Expected
            _ => panic!("Expected NotFound result for NonExistent"),
        }
    }

    #[test]
    fn test_find_character_by_name_multiple_matches() {
        let mut db: Db = Db::new();
        let lightning1 = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Lightning"),
            rarity: Some(String::from("5")),
        };
        let lightning2 = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Radiant Lightning"),
            rarity: Some(String::from("NV")),
        };
        let lightning3 = CharacterInfo {
            r#type: String::from("event"),
            name: String::from("Lightning (FFXIII-2)"),
            rarity: Some(String::from("NV")),
        };

        db.insert(213000105, lightning1);
        db.insert(213001005, lightning2);
        db.insert(250000205, lightning3);

        // Search for "Lightning" should return Found because there's an exact match
        match db.find_by_name("Lightning") {
            LookupResult::Found(uid) => {
                assert_eq!(uid, 213000105); // Exact match takes priority
            }
            _ => panic!("Expected Found result for exact match 'Lightning'"),
        }

        // Search for "Light" should return Multiple because no exact match
        match db.find_by_name("Light") {
            LookupResult::Multiple(matches) => {
                assert!(matches.len() >= 2);
                let uids: Vec<u32> = matches.iter().map(|(uid, _)| *uid).collect();
                assert!(uids.contains(&213000105)); // Lightning
                assert!(uids.contains(&213001005)); // Radiant Lightning
                assert!(uids.contains(&250000205)); // Lightning (FFXIII-2)
            }
            _ => panic!("Expected Multiple result for partial match 'Light'"),
        }
    }

    #[test]
    fn test_find_character_by_name_partial_match() {
        let mut db: Db = Db::new();
        let radiant_lightning = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Radiant Lightning"),
            rarity: Some(String::from("NV")),
        };
        let dark_knight_cecil = CharacterInfo {
            r#type: String::from("summon"),
            name: String::from("Dark Knight Cecil"),
            rarity: Some(String::from("4")),
        };

        db.insert(213001005, radiant_lightning);
        db.insert(204000403, dark_knight_cecil);

        // Search for "Radiant" should match "Radiant Lightning"
        match db.find_by_name("Radiant") {
            LookupResult::Multiple(matches) => {
                assert_eq!(matches.len(), 1);
                assert_eq!(matches[0].0, 213001005);
                assert_eq!(matches[0].1.name, "Radiant Lightning");
            }
            _ => panic!("Expected Multiple result for Radiant"),
        }

        // Search for "Knight" should match "Dark Knight Cecil"
        match db.find_by_name("Knight") {
            LookupResult::Multiple(matches) => {
                assert_eq!(matches.len(), 1);
                assert_eq!(matches[0].0, 204000403);
                assert_eq!(matches[0].1.name, "Dark Knight Cecil");
            }
            _ => panic!("Expected Multiple result for Knight"),
        }
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
        match db.find_by_name("Rain") {
            LookupResult::Found(uid) => assert_eq!(uid, 100000102),
            _ => panic!("Expected Found result for Rain"),
        }

        match db.find_by_name("Lasswell") {
            LookupResult::Found(uid) => assert_eq!(uid, 100000202),
            _ => panic!("Expected Found result for Lasswell"),
        }

        match db.find_by_name("Fina") {
            LookupResult::Found(uid) => assert_eq!(uid, 100000302),
            _ => panic!("Expected Found result for Fina"),
        }

        // Verify that a non-existent character returns NotFound
        match db.find_by_name("NonExistent") {
            LookupResult::NotFound => {} // Expected
            _ => panic!("Expected NotFound result for NonExistent"),
        }
    }

    #[test]
    fn test_from_file_nonexistent() {
        // Test that loading from a non-existent file returns an error
        let result = Db::from_file("nonexistent_file.json");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_file_with_optional_rarity() {
        // Test that the optional rarity field works correctly
        let json_content = r#"{
            "100000102": {"type": "story", "name": "Rain", "rarity": "5"},
            "100000202": {"type": "story", "name": "Lasswell"},
            "100000302": {"type": "story", "name": "Fina", "rarity": null}
        }"#;

        let mut temp_file = NamedTempFile::new().expect("Failed to create temp file");
        temp_file
            .write_all(json_content.as_bytes())
            .expect("Failed to write to temp file");

        let db =
            Db::from_file(temp_file.path().to_str().unwrap()).expect("Failed to load from file");

        // Check that characters with different rarity field configurations load correctly
        match db.find_by_name("Rain") {
            LookupResult::Found(uid) => {
                assert_eq!(uid, 100000102);
                // Verify we can access the character info
                let char_info = db.0.get(&uid).unwrap();
                assert_eq!(char_info.rarity, Some("5".to_string()));
            }
            _ => panic!("Expected Found result for Rain"),
        }

        match db.find_by_name("Lasswell") {
            LookupResult::Found(uid) => {
                assert_eq!(uid, 100000202);
                let char_info = db.0.get(&uid).unwrap();
                assert_eq!(char_info.rarity, None);
            }
            _ => panic!("Expected Found result for Lasswell"),
        }
    }
}
