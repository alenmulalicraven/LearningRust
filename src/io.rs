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

#[derive(Clone, Debug, PartialEq)]
struct Entity {
    pos: Vector,
    glyph: char,
    color: Color,
    hp: i32,
    max_hp: i32,
}

fn generate_entities(d: Dungeon) -> Vec<Entity> {
    let mut vec: Vec<Entity> = Vec::new();
    for i in 0..d.monsters.len() {
        let ent = Entity {
            pos: Vector::new(d.monsters[i].pos_y as i32, d.monsters[i].pos_x as i32),
            glyph: d.monsters[i].character,
            color: Color::RED,
            hp: d.monsters[i].hp,
            max_hp: d.monsters[i].max_hp,
        };
        vec.push(ent);
    }

    vec
}

pub struct Game {
    title: Asset<Image>,
    inventory: Asset<Image>,
    map: Vec<Tile>,
    entities: Vec<Entity>,
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
        let mut entities = generate_entities(dungeon);

        // Generate Player
        entities.push(Entity {
            pos: Vector::new(
                dungeon.player.position_y as i32,
                dungeon.player.position_x as i32,
            ),
            glyph: '@',
            color: Color::BLUE,
            hp: 3,
            max_hp: 5,
        });

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
            entities,
            tileset,
            tile_size_px,
            dungeon,
        })
    }

    /// Process keyboard and mouse, update the game state
    fn update(&mut self, window: &mut Window) -> Result<()> {
        use ButtonState::*;

        let entities = &mut self.entities;
        let mut player_id = entities.len();
        player_id -= 1;
        if window.keyboard()[Key::Left] == Pressed {
            self.dungeon = dungeon::monster_map(self.dungeon);
            if self.dungeon.move_character('x', 1000) {
                entities[player_id].pos.x -= 1.0;
                self.dungeon.player.position_y -= 1;

                for i in 0..(entities.len() -1) {
                    let moves = self.dungeon.determine_monster_move(i);
                    self.dungeon.monsters[i].pos_x = moves.0;
                    self.dungeon.monsters[i].pos_y = moves.1;
                    entities[i].pos.x = moves.1 as f32;
                    entities[i].pos.y = moves.0 as f32;

                    self.dungeon = dungeon::monster_map(self.dungeon);
                }

                // let monster_moves = self.dungeon.determine_monster_move();

                // for i in 0..monster_moves.len() {
                //     self.dungeon.monsters[i].pos_x = monster_moves[i].1;
                //     self.dungeon.monsters[i].pos_y = monster_moves[i].0;
                //     entities[i].pos.x = monster_moves[i].0 as f32;
                //     entities[i].pos.y = monster_moves[i].1 as f32;
                // }

                self.dungeon = dungeon::calculate_distance_map(self.dungeon);
            }
        }
        if window.keyboard()[Key::Right] == Pressed {
            self.dungeon = dungeon::monster_map(self.dungeon);
            if self.dungeon.move_character('x', 1) {
                self.dungeon.player.position_y += 1;
                entities[player_id].pos.x += 1.0;

                for i in 0..(entities.len() -1) {
                    let moves = self.dungeon.determine_monster_move(i);
                    self.dungeon.monsters[i].pos_x = moves.0;
                    self.dungeon.monsters[i].pos_y = moves.1;
                    entities[i].pos.x = moves.1 as f32;
                    entities[i].pos.y = moves.0 as f32;

                    self.dungeon = dungeon::monster_map(self.dungeon);
                }

                

                self.dungeon = dungeon::calculate_distance_map(self.dungeon);
            }
        }
        if window.keyboard()[Key::Up] == Pressed {
            self.dungeon = dungeon::monster_map(self.dungeon);
            if self.dungeon.move_character('y', 1000) {
                self.dungeon.player.position_x -= 1;
                entities[player_id].pos.y -= 1.0;

                for i in 0..(entities.len() -1) {
                    let moves = self.dungeon.determine_monster_move(i);
                    self.dungeon.monsters[i].pos_x = moves.0;
                    self.dungeon.monsters[i].pos_y = moves.1;
                    entities[i].pos.x = moves.1 as f32;
                    entities[i].pos.y = moves.0 as f32;

                    self.dungeon = dungeon::monster_map(self.dungeon);
                }


                self.dungeon = dungeon::calculate_distance_map(self.dungeon);
            }
        }
        if window.keyboard()[Key::Down] == Pressed {
            self.dungeon = dungeon::monster_map(self.dungeon);
            if self.dungeon.move_character('y', 1) {
                self.dungeon.player.position_x += 1;
                entities[player_id].pos.y += 1.0;

                for i in 0..(entities.len() -1) {
                    let moves = self.dungeon.determine_monster_move(i);
                    self.dungeon.monsters[i].pos_x = moves.0;
                    self.dungeon.monsters[i].pos_y = moves.1;
                    entities[i].pos.x = moves.1 as f32;
                    entities[i].pos.y = moves.0 as f32;

                    self.dungeon = dungeon::monster_map(self.dungeon);
                }

                self.dungeon = dungeon::calculate_distance_map(self.dungeon);
            }
        }
        if window.keyboard()[Key::Escape].is_down() {
            window.close();
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

        // Draw entities
        let (tileset, entities) = (&mut self.tileset, &self.entities);
        tileset.execute(|tileset| {
            for entity in entities.iter() {
                if let Some(image) = tileset.get(&entity.glyph) {
                    let pos_px = offset_px + entity.pos.times(tile_size_px);
                    window.draw(
                        &Rectangle::new(pos_px, image.area().size()),
                        Blended(&image, entity.color),
                    );
                }
            }
            Ok(())
        })?;

        let player_id = self.entities.len() - 1;
        let player = &self.entities[player_id];
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
