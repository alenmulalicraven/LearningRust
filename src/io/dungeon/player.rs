use quicksilver::prelude::Color;
#[derive(Copy, Clone)]
pub struct Player {
    pub character: char,
    pub position_x: usize,
    pub position_y: usize,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defence: i32,
    pub alive: bool,
    pub color: Color,
}

impl Player {
    pub fn process_combat(self, mon_attack: i32) -> i32 {
        let hp_after = self.hp - (mon_attack / self.defence);
        hp_after
    }
}
