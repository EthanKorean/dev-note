use character::Role;
use character::{Mage, Warrior, Character};

pub struct CharacterFactory;

impl CharacterFactory {
    pub fn create_character(role: Role) -> Box<dyn Character> {
        match role {
            Role::Warrior => Box::new(Warrior::new()),
            Role::Mage => Box::new(Mage::new()),
        }
    }
}

