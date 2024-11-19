use crate::factory::character::Character;

pub struct Warrior {
    health: u32,
    level: u32,
}

impl Warrior {
    pub fn new() -> Self {
        Warrior {
            health: 100,
            level: 1,
        }
    }
}

impl Character for Warrior {
    fn attack(&self) {
        println!("Warrior attacks with sword!");
    }

    fn get_role(&self) -> String {
        "Warrior".to_string()
    }

    fn get_health(&self) -> u32 {
        self.health
    }

    fn level_up(&mut self) {
        self.level += 1;
        self.health += 10;  // 레벨업 시 체력 증가
        println!("Warrior leveled up to level {}", self.level);
    }

    fn get_level(&self) -> u32 {
        self.level
    }
}
