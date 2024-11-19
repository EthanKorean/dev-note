pub trait Character {
    fn attack(&self);
    fn get_role(&self) -> String;
    fn get_health(&self) -> u32;
    fn level_up(&mut self);
    fn get_level(&self) -> u32;
}
