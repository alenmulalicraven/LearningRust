#[derive(Copy, Clone)]
pub struct Monster {
    pub character: char,
    pub pos_x: usize,
    pub pos_y: usize,
    pub hp: i32,
    pub max_hp: i32,
}
