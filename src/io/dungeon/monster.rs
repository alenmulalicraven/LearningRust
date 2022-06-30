use quicksilver::prelude::Color;

#[derive(Copy, Clone)]
pub struct Monster {
    pub character: char,
    pub pos_x: usize,
    pub pos_y: usize,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defence: i32,
    pub alive: bool,
    pub color: Color,
}

impl Monster {
    pub fn process_combat(self, player_attack: i32) -> i32 {
        let hp_after = self.hp - (player_attack - self.defence);
        hp_after
    }
}
