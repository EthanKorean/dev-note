use crate::factory::character::Character;

pub struct Mage {
    health: u32,
    level: u32,
}

impl Mage {
    pub fn new() -> Self {
        Mage {
            health: 80,
            level: 1,
        }
    }
}

impl Character for Mage {
    fn attack(&self) {
        println!("Mage casts a spell!");
    }

    fn get_role(&self) -> String {
        "Mage".to_string()
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.health += 5;  // 레벨업 시 체력 증가
        println!("Mage leveled up to level {}", self.level);
    }

    fn get_level(&self) -> u32 {
        self.level
    }
}
