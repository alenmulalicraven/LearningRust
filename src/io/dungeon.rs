use rand::Rng;
use std::cmp;
pub mod monster;
pub mod player;
use crate::dungeon::monster::*;
use crate::dungeon::player::Player;
use colored::Colorize;

#[derive(Copy, Clone)]
pub struct Dungeon {
    pub dungeon_x: usize,
    pub dungeon_y: usize,
    pub min_width: usize,
    pub max_width: usize,
    pub min_length: usize,
    pub max_length: usize,
    pub rooms: usize,
    pub char_map: [[char; 80]; 30],
    pub player: Player,
    pub hardness_map: [[u8; 80]; 30],
    pub distance_map: [[u16; 80]; 30],
    pub mon_map: [[bool; 80]; 30],
    pub monsters: [Monster; 10],
}

impl Dungeon {
    pub fn print(self) {
        for i in 0..self.dungeon_x {
            for j in 0..self.dungeon_y {
                if self.char_map[i][j] == '.' {
                    print!("{}", self.char_map[i][j].to_string().green());
                } else if self.char_map[i][j] == '#' {
                    print!("{}", self.char_map[i][j].to_string().yellow());
                } else if self.char_map[i][j] == '@' {
                    print!("{}", self.char_map[i][j].to_string().red());
                } else {
                    print!("{}", self.char_map[i][j].to_string().white());
                }
            }
            println!("");
        }
    }

    pub fn print_distance_map(self) {
        for i in 0..self.dungeon_x {
            for j in 0..self.dungeon_y {
                if self.distance_map[i][j] != 1055 {
                    print!("{}", self.distance_map[i][j] % 10);
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }

    pub fn move_character(self, direction: char, value: usize) -> bool {
        if direction == 'x' {
            if value == 1000 {
                if self.hardness_map[self.player.position_x][self.player.position_y - 1] > 2 || 
                    self.mon_map[self.player.position_x][self.player.position_y - 1]{
                    return false;
                }
            } else {
                if self.hardness_map[self.player.position_x][self.player.position_y + 1] > 2 ||
                self.mon_map[self.player.position_x][self.player.position_y + 1]{
                    return false;
                }
            }
        } else {
            if value == 1000 {
                if self.hardness_map[self.player.position_x - 1][self.player.position_y] > 2 ||
                self.mon_map[self.player.position_x -1][self.player.position_y]{
                    return false;
                }
            } else {
                if self.hardness_map[self.player.position_x + 1][self.player.position_y] > 2 ||
                self.mon_map[self.player.position_x + 1][self.player.position_y]{
                    return false;
                }
            }
        }

        true
    }
    pub fn determine_monster_move(self, monster : usize) -> (usize, usize) {
        let posx = self.monsters[monster].pos_x;
        let posy = self.monsters[monster].pos_y;
        let mut x_min : usize = 0; 
        let mut y_min :usize = 0;
        let mut min = 10000;
        for x in (posx-1)..(posx+2) {
            for y in (posy -1)..(posy + 2) {
                if !self.mon_map[x][y] {
                    if self.distance_map[x][y] < min {
                        min = self.distance_map[x][y];
                        x_min = x;
                        y_min = y;
                    }
                }
            } 
        }
        (x_min, y_min)
    }
}

pub fn generate_dungeon() -> Dungeon {
    let mut dungeon = Dungeon {
        dungeon_x: 30,
        dungeon_y: 80,
        min_width: 4,
        max_width: 9,
        min_length: 4,
        max_length: 12,
        rooms: 7,
        char_map: [[' '; 80]; 30],
        player: Player {
            character: '@',
            position_x: 1000,
            position_y: 1000,
        },
        hardness_map: [[255; 80]; 30],
        distance_map: [[0; 80]; 30],
        mon_map: [[false; 80]; 30],
        monsters: [Monster {
            character: 'g',
            pos_x: 100,
            pos_y: 100,
            hp: 10,
            max_hp: 10,
        }; 10],
    };

    let mut rooms = 0;
    let mut room_list: Vec<(usize, usize, usize, usize)> = Vec::new();

    while rooms < dungeon.rooms {
        let length = rand::thread_rng().gen_range(dungeon.min_length..dungeon.max_length);
        let width = rand::thread_rng().gen_range(dungeon.min_width..dungeon.max_width);

        let max_width = dungeon.dungeon_x - width;
        let max_length = dungeon.dungeon_y - length;

        let x = rand::thread_rng().gen_range(1..max_width);
        let y = rand::thread_rng().gen_range(1..max_length);

        if is_valid_room(x, y, width, length, &dungeon) {
            dungeon = add_room(x, y, width, length, dungeon);
            rooms += 1;
            room_list.push((x, y, length, width));
        }
    }
    dungeon = add_hallways(dungeon, room_list);

    let mut set_player: bool = false;
    while !set_player {
        let x = rand::thread_rng().gen_range(1..dungeon.dungeon_x);
        let y = rand::thread_rng().gen_range(1..dungeon.dungeon_y);

        if dungeon.char_map[x][y] == '.' {
            dungeon.player.position_x = x;
            dungeon.player.position_y = y;
            set_player = true;
        }
    }
    dungeon = calculate_distance_map(dungeon);
    dungeon = place_monsters(dungeon);
    dungeon
}

fn is_valid_room(x: usize, y: usize, width: usize, length: usize, d: &Dungeon) -> bool {
    for i in x..(x + width) {
        for j in y..(y + length) {
            if d.char_map[i][j] != ' ' {
                return false;
            }
        }
    }
    true
}

fn add_room(x: usize, y: usize, width: usize, length: usize, mut d: Dungeon) -> Dungeon {
    for i in x..(x + width) {
        for j in y..(y + length) {
            d.char_map[i][j] = '.';
            d.hardness_map[i][j] = 0;
        }
    }
    d
}

fn add_hallways(mut d: Dungeon, mut rooms: Vec<(usize, usize, usize, usize)>) -> Dungeon {
    rooms.sort_by_key(|k| k.1);
    for i in 0..(rooms.len() - 1) {
        let mut overlap_x: Vec<(usize, usize)> = Vec::new();
        let mut overlap_y: Vec<(usize, usize)> = Vec::new();
        overlap_x.push((rooms[i].0, (rooms[i].0 + rooms[i].3)));
        overlap_x.push((rooms[i + 1].0, (rooms[i + 1].0 + rooms[i + 1].3 - 1)));

        overlap_y.push((rooms[i].1, (rooms[i].1 + rooms[i].2)));
        overlap_y.push((rooms[i + 1].1, (rooms[i + 1].1 + rooms[i + 1].2)));

        let mid_x = determine_overlap(overlap_x);
        let mid_y = determine_overlap(overlap_y);

        if mid_x < 1000 {
            for j in (rooms[i].1 + rooms[i].2)..rooms[i + 1].1 {
                d.char_map[mid_x][j] = '#';
                d.hardness_map[mid_x][j] = 0;
            }
        } else if mid_y < 1000 {
            if (rooms[i].0 + rooms[i].3) > rooms[i + 1].0 {
                for j in rooms[i + 1].0..(rooms[i].0 + rooms[i].3) {
                    if d.char_map[j][mid_y] != '.' {
                        d.char_map[j][mid_y] = '#';
                        d.hardness_map[j][mid_y] = 0;
                    }
                }
            } else {
                for j in rooms[i].0..rooms[i + 1].0 {
                    if d.char_map[j][mid_y] != '.' {
                        d.char_map[j][mid_y] = '#';
                        d.hardness_map[j][mid_y] = 0;
                    }
                }
            }
        } else {
            if rooms[i].0 > rooms[i + 1].0 {
                for j in rooms[i + 1].0..(rooms[i].0 + rooms[i].3) {
                    if d.char_map[j][rooms[i].1] != '.' {
                        d.char_map[j][rooms[i].1] = '#';
                        d.hardness_map[j][rooms[i].1] = 0;
                    }
                }

                for j in rooms[i].1..rooms[i + 1].1 {
                    if d.char_map[rooms[i + 1].0][j] != '.' {
                        d.char_map[rooms[i + 1].0][j] = '#';
                        d.hardness_map[rooms[i + 1].0][j] = 0;
                    }
                }
            } else {
                for j in (rooms[i].0)..(rooms[i + 1].0) {
                    if d.char_map[j][rooms[i + 1].1] != '.' {
                        d.char_map[j][rooms[i + 1].1] = '#';
                        d.hardness_map[j][rooms[i + 1].1] = 0;
                    }
                }

                for j in rooms[i].1..rooms[i + 1].1 {
                    if d.char_map[rooms[i].0][j] != '.' {
                        d.char_map[rooms[i].0][j] = '#';
                        d.hardness_map[rooms[i].0][j] = 0;
                    }
                }
            }
        }
    }
    d
}

fn determine_overlap(mut overlap: Vec<(usize, usize)>) -> usize {
    overlap.sort_by_key(|k| k.1);
    let mut max = 1000;
    let mut min = 1000;
    if overlap[1].0 <= overlap[0].1 {
        max = cmp::max(overlap[1].0, overlap[0].0);
        min = cmp::min(overlap[1].1, overlap[0].1);
    }
    if max == 1000 {
        return max;
    }
    let mid = (max + min) / 2;
    mid
}

pub fn calculate_distance_map(mut d: Dungeon) -> Dungeon {
    let px = d.player.position_x as i16;
    let py = d.player.position_y as i16;

    for x in 0..d.dungeon_x {
        for y in 0..d.dungeon_y {
            if d.hardness_map[x][y] > 2 {
                d.distance_map[x][y] = 1055;
            } else {
                let xz = x as i16;
                let yz = y as i16;
                let manhatan_distance = (px - xz).abs() + (py - yz).abs();
                d.distance_map[x][y] = manhatan_distance as u16;
            }
        }
    }
    d
}

fn place_monsters(mut d: Dungeon) -> Dungeon {
    let mut mon = 0;

    while mon < d.monsters.len() {
        let x = rand::thread_rng().gen_range(1..d.dungeon_x);
        let y = rand::thread_rng().gen_range(1..d.dungeon_y);

        if d.hardness_map[x][y] < 2 {
            d.monsters[mon].pos_x = x;
            d.monsters[mon].pos_y = y;
            mon += 1;
        }
    }
    d
}

pub fn monster_map(mut d: Dungeon) -> Dungeon {
    for x in 0..d.dungeon_x {
        for y in 0..d.dungeon_y {
            d.mon_map[x][y] = false;
        }
    }

    for mon in 0..d.monsters.len() {
        d.mon_map[d.monsters[mon].pos_x][d.monsters[mon].pos_y] = true;
    }
    d.mon_map[d.player.position_x][d.player.position_y] = true;
    d
}
