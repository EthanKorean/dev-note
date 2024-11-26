use character::Warrior;
use character::Mage;
use character::Character;

pub trait CharacterFactoryByMethod{
    fn generate_charactor(&self) -> Box<dyn Character>;
}

pub struct WarriorFactory;
impl CharacterFactoryByMethod for WarriorFactory{
    fn generate_charactor(&self) -> Box<dyn Character>{
        Box::new(Warrior::new())
   }
}

pub struct MageFactory;
impl CharacterFactoryByMethod for MageFactory{
    fn generate_charactor(&self) -> Box<dyn Character>{
        Box::new(Mage::new())
   }
}

