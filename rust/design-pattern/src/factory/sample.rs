use crate::factory::character_factory::CharacterFactory;
use super::{character::Character, mage::Mage};

pub fn sample() {
    // Warrior 캐릭터 생성
    init_character( "Warrior");

    // Mage 캐릭터 생성
    let mut mage = init_character("Mage");
    level_up_by_dynamic_dispatch(&mut * mage); 
    
    
}

fn init_character(role: &str) -> Box<dyn Character>{
    let character = CharacterFactory::create_character(role);
    println!("Created a {}=====================================!", character.get_role());
    character.attack();
    println!("Current level of Warrior: {}", character.get_level());
    return character;
}


fn level_up_by_dynamic_dispatch(character: &mut dyn Character) {
    character.level_up();
}
