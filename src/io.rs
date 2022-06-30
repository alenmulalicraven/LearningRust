use quicksilver::prelude::*;
use std::collections::HashMap;
pub mod dungeon;
use crate::dungeon::Dungeon;

#[derive(Clone, Debug, PartialEq)]
struct Tile {
    pos: Vector,
    glyph: char,
    color: Color,
}

fn generate_map(d: Dungeon) -> Vec<Tile> {
    let width = d.dungeon_y;
    let height = d.dungeon_x;
    let mut map = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            let mut tile = Tile {
                pos: Vector::new(x as f32, y as f32),
                glyph: d.char_map[y][x],
                color: Color::BLACK,
            };

            if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                tile.glyph = '%';
            };
            map.push(tile);
        }
    }
    map
}

pub struct Game {
    title: Asset<Image>,
    inventory: Asset<Image>,
    map: Vec<Tile>,
    tileset: Asset<HashMap<char, Image>>,
    tile_size_px: Vector,
    dungeon: Dungeon,
}

impl State for Game {
    /// Load the assets and initialise the game
    fn new() -> Result<Self> {
        // The Mononoki font: https://madmalik.github.io/mononoki/
        // License: SIL Open Font License 1.1
        let font_mononoki = "mononoki-Regular.ttf";

        let title =
            Asset::new(Font::load(font_mononoki).and_then(|font| {
                font.render("Learning Rust ", &FontStyle::new(72.0, Color::BLACK))
            }));

        let dungeon: Dungeon = dungeon::generate_dungeon();
        let inventory = Asset::new(Font::load(font_mononoki).and_then(move |font| {
            font.render(
                "Inventory:\n[A] Sword\n[B] Shield\n[C] Darts",
                &FontStyle::new(20.0, Color::BLACK),
            )
        }));
        let map = generate_map(dungeon);

        // The Square font: http://strlen.com/square/?s[]=font
        // License: CC BY 3.0 https://creativecommons.org/licenses/by/3.0/deed.en_US
        let font_square = "square.ttf";
        let game_glyphs = "#@g.%-";
        let tile_size_px = Vector::new(24, 24);
        let tileset = Asset::new(Font::load(font_square).and_then(move |text| {
            let tiles = text
                .render(game_glyphs, &FontStyle::new(tile_size_px.y, Color::WHITE))
                .expect("Could not render the font tileset.");
            let mut tileset = HashMap::new();
            for (index, glyph) in game_glyphs.chars().enumerate() {
                let pos = (index as i32 * tile_size_px.x as i32, 0);
                let tile = tiles.subimage(Rectangle::new(pos, tile_size_px));
                tileset.insert(glyph, tile);
            }
            Ok(tileset)
        }));

        Ok(Self {
            title,
            inventory,
            map,
            tileset,
            tile_size_px,
            dungeon,
        })
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;

        if window.keyboard()[Key::Left] == Pressed {
            self.dungeon = dungeon::monster_map(self.dungeon);
            if self.dungeon.move_character('x', 1000) {
                self.dungeon.player.position_y -= 1;

                self.dungeon = dungeon::process_monster_moves_attack(self.dungeon);
                self.dungeon = dungeon::calculate_distance_map(self.dungeon);
            } else {
                // Process Target Monster
                self.dungeon.player = dungeon::process_target_monster(self.dungeon.player, 
                    self.dungeon.player.position_x, 
                    self.dungeon.player.position_y - 1);
       
            }
        }
        if window.keyboard()[Key::Right] == Pressed {
            self.dungeon = dungeon::monster_map(self.dungeon);
            if self.dungeon.move_character('x', 1) {
                self.dungeon.player.position_y += 1;

                self.dungeon = dungeon::process_monster_moves_attack(self.dungeon);
                self.dungeon = dungeon::calculate_distance_map(self.dungeon);
            } else {
                // Process Target Monster
                self.dungeon.player = dungeon::process_target_monster(self.dungeon.player, 
                    self.dungeon.player.position_x, 
                    self.dungeon.player.position_y + 1);
                
            }
        }
        if window.keyboard()[Key::Up] == Pressed {
            self.dungeon = dungeon::monster_map(self.dungeon);
            if self.dungeon.move_character('y', 1000) {
                self.dungeon.player.position_x -= 1;

                self.dungeon = dungeon::process_monster_moves_attack(self.dungeon);
                self.dungeon = dungeon::calculate_distance_map(self.dungeon);
            } else {
                // Process Target Monster
                self.dungeon.player = dungeon::process_target_monster(self.dungeon.player, 
                    self.dungeon.player.position_x - 1, 
                    self.dungeon.player.position_y);
            }
        }
        if window.keyboard()[Key::Down] == Pressed {
            self.dungeon = dungeon::monster_map(self.dungeon);
            if self.dungeon.move_character('y', 1) {
                self.dungeon.player.position_x += 1;

                self.dungeon = dungeon::process_monster_moves_attack(self.dungeon);
                self.dungeon = dungeon::calculate_distance_map(self.dungeon);
            } else {
                // Process Target Monster
                self.dungeon.player = dungeon::process_target_monster(self.dungeon.player, 
                    self.dungeon.player.position_x + 1, 
                    self.dungeon.player.position_y);
            }
        }
        if window.keyboard()[Key::Escape].is_down() {
            window.close();
        }
        if window.keyboard()[Key::A].is_down() {
            let return_tup = dungeon::process_attack(self.dungeon);
            if return_tup.2 {
                self.dungeon.monsters[return_tup.1] = return_tup.0;
            }
        }
        Ok(())
    }

    /// Draw stuff on the screen
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;

        // Draw the game title
        self.title.execute(|image| {
            window.draw(
                &image
                    .area()
                    .with_center((window.screen_size().x as i32 / 2, 40)),
                Img(&image),
            );
            Ok(())
        })?;

        let tile_size_px = self.tile_size_px;
        let offset_px = Vector::new(50, 120);

        // Draw the map
        let (tileset, map) = (&mut self.tileset, &self.map);
        tileset.execute(|tileset| {
            for tile in map.iter() {
                if let Some(image) = tileset.get(&tile.glyph) {
                    let pos_px = tile.pos.times(tile_size_px);
                    window.draw(
                        &Rectangle::new(offset_px + pos_px, image.area().size()),
                        Blended(&image, tile.color),
                    );
                }
            }
            Ok(())
        })?;

        // Draw Monsters and PC 
        let (tileset, d) = (&mut self.tileset, &self.dungeon);
        tileset.execute(|tileset| {
            for i in 0..d.monsters.len() {
                if let Some(image) = tileset.get(&d.monsters[i].character) {
                    let mon_vector =
                        Vector::new(d.monsters[i].pos_y as i32, d.monsters[i].pos_x as i32);
                    let pos_px = offset_px + mon_vector.times(tile_size_px);
                    if d.monsters[i].alive {
                        window.draw(
                            &Rectangle::new(pos_px, image.area().size()),
                            Blended(&image, d.monsters[i].color),
                        );
                    }
                }
            }
            if d.player.alive {
                if let Some(image) = tileset.get(&d.player.character) {
                    let player_vector =
                        Vector::new(d.player.position_y as i32, d.player.position_x as i32);
                    let pos_px = offset_px + player_vector.times(tile_size_px);
                    window.draw(
                        &Rectangle::new(pos_px, image.area().size()),
                        Blended(&image, d.player.color),
                    );
                }
            }
            Ok(())
        })?;

        let player = &self.dungeon.player;
        let full_health_width_px = 100.0;
        let current_health_width_px =
            (player.hp as f32 / player.max_hp as f32) * full_health_width_px;

        let map_size = Vector::new(self.dungeon.dungeon_y as i32, self.dungeon.dungeon_x as i32);
        let map_size_px = map_size.times(tile_size_px);
        let health_bar_pos_px = offset_px + Vector::new(map_size_px.x, 0.0);

        // Full health
        window.draw(
            &Rectangle::new(health_bar_pos_px, (full_health_width_px, tile_size_px.y)),
            Col(Color::RED.with_alpha(0.5)),
        );

        // Current health
        window.draw(
            &Rectangle::new(health_bar_pos_px, (current_health_width_px, tile_size_px.y)),
            Col(Color::RED),
        );

        self.inventory.execute(|image| {
            window.draw(
                &image
                    .area()
                    .translate(health_bar_pos_px + Vector::new(0, tile_size_px.y)),
                Img(&image),
            );
            Ok(())
        })?;

        Ok(())
    }
}
