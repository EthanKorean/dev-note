use character::character::Character;
use character::role::Role;   
use common::example::Example;
use crate::factory::character_factory::CharacterFactory;


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
