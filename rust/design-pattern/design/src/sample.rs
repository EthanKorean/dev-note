use character::character::Character;
use character::role::Role;   
use common::example::Example;
use crate::factory::character_factory::CharacterFactory;
use crate::factory_method::character_factory::{CharacterFactoryByMethod, MageFactory, WarriorFactory};

/**
 * Factory 디자인으로 생성하는 예제
 */
pub fn init_character_by_factory_design() -> Example{
    Example::new("factory design",
        Box::new(||{
            // Warrior 캐릭터 생성
            init_character( Role::Warrior);
            // Mage 캐릭터 생성
            let mut mage = init_character(Role::Mage);
            level_up_by_dynamic_dispatch(&mut * mage);
    }))
}

/**
 * Factory Method 디자인으로 생성하는 예제
 */
pub fn init_character_by_factory_method_design()-> Example {
    Example::new("factory method design",
    Box::new(||{
        let _warrior_factory = WarriorFactory;
        let _warrior = _warrior_factory.generate_charactor();
        _warrior.attack(); 

        let _mage_factory = MageFactory;
        let _mage = _mage_factory.generate_charactor();
        _mage.attack(); 
    }))
}

fn init_character(role: Role) -> Box<dyn Character>{
    let character = CharacterFactory::create_character(role);
    println!("[Created a {}]!", character.get_role());
    character.attack();
    println!("Current level of Warrior: {}", character.get_level());
    println!();
    character
}


fn level_up_by_dynamic_dispatch(character: &mut dyn Character) {
    character.level_up();
}
