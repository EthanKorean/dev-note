use crate::factory::warrior::Warrior;
use crate::factory::mage::Mage;
use crate::factory::character::Character;

pub struct CharacterFactory;

impl CharacterFactory {
    pub fn create_character(role: &str) -> Box<dyn Character> {
        match role {
            "Warrior" => Box::new(Warrior::new()),  // Warrior 타입 반환
            "Mage" => Box::new(Mage::new()),        // Mage 타입 반환
            _ => panic!("Unknown character type!"),
        }
    }
}