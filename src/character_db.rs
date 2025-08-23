use serde::Deserialize;
use std::collections::HashMap;

use crate::error::{FfbeError, Result};

#[derive(Default, Deserialize, Debug, Clone, PartialEq)]
pub struct CharacterInfo {
    pub r#type: String,
    pub name: String,
    pub rarity: String,
}

pub struct Db(HashMap<u32, CharacterInfo>);

impl Db {
    pub fn new() -> Self {
        Self(HashMap::<u32, CharacterInfo>::new())
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
}
