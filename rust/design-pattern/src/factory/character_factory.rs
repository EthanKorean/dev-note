use crate::factory::warrior::Warrior;
use crate::factory::mage::Mage;
use crate::factory::role::Role;
use crate::factory::character::Character;
pub struct CharacterFactory;

impl CharacterFactory {
    pub fn create_character(role: Role) -> Box<dyn Character> {
        match role {
            Role::Warrior => Box::new(Warrior::new()),  // Warrior 타입 반환
            Role::Mage => Box::new(Mage::new()),        // Mage 타입 반환
            _ => panic!("Unknown character type!"),
        }
    }
}