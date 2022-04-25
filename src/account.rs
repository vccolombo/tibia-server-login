use std::ops::Deref;

use crate::{character::Character, server::Server};

pub struct Account {
    characters: Vec<Character>,
    pub premium_days: u16,
}

impl Account {
    pub fn get(account_number: u32, password: String) -> Option<Self> {
        // Fake data

        if account_number == 1 && password == "1" {
            let mut account = Vec::new();
            let char1 = Character::new("Player 1".to_string(), Server::default());
            let char2 = Character::new("Player 2".to_string(), Server::default());

            account.push(char1);
            account.push(char2);

            return Some(Self {
                characters: account,
                premium_days: 42,
            });
        }

        return None;
    }

    pub fn number_of_characters(&self) -> u8 {
        return self.characters.len() as u8;
    }
}

impl Deref for Account {
    type Target = Vec<Character>;

    fn deref(&self) -> &Self::Target {
        return &self.characters;
    }
}
