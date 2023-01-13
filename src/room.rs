use crate::tile::Tile;
use rust_dungeon::CardinalDirections;
use sfml::{
    graphics::{IntRect, RenderWindow, Texture},
    system::{Vector2f, Vector2i},
    SfBox,
};

#[derive(Debug)]
pub struct Room {
    tiles: Vec<Vec<Tile>>,
}

fn get_image_count(i: i32, j: i32) -> Vector2i {
    let image_count: Vector2i;
    if i == 0 {
        if j == 0 {
            image_count = Vector2i { x: 0, y: 0 };
        } else if j == 14 {
            image_count = Vector2i { x: 0, y: 2 };
        } else {
            image_count = Vector2i { x: 0, y: 1 };
        }
    } else if i == 23 {
        if j == 0 {
            image_count = Vector2i { x: 2, y: 0 };
        } else if j == 14 {
            image_count = Vector2i { x: 2, y: 2 };
        } else {
            image_count = Vector2i { x: 2, y: 1 };
        }
    } else if j == 0 {
        image_count = Vector2i { x: 1, y: 0 };
    } else if j == 14 {
        image_count = Vector2i { x: 1, y: 2 };
    } else {
        image_count = Vector2i { x: 1, y: 1 };
    }
    image_count
}

impl Room {
    pub fn new() -> Room {
        let mut room = Room { tiles: Vec::new() };
        for _i in 0..24 {
            room.tiles.push(Vec::new());
        }
        room
    }

    //Spawn position is the coords for the upper left corner of the room
    pub fn from(spawn_position: Vector2f) -> Room {
        let tile_spawn_position: Vector2f = spawn_position + Vector2f { x: 16.0, y: 16.0 };
        let mut room: Room = Room::new();

        for i in 0..24 {
            for j in 0..15 {
                let image_count = get_image_count(i, j);
                let tile = Tile::from(
                    Vector2f {
                        x: tile_spawn_position.x + 32.0 * i as f32,
                        y: tile_spawn_position.y + 32.0 * j as f32,
                    },
                    image_count,
                );
                println!("{}", j);
                room.tiles[i as usize].push(tile);
            }
        }
        room
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        for row in &self.tiles {
            for tile in row {
                tile.draw(window, texture);
            }
        }
    }
    pub fn set_door(&mut self, direction: CardinalDirections) {
        match direction {
            CardinalDirections::Up => {
                self.tiles[11][0].set_texture_coordinates(Vector2i { x: 0, y: 3 });
                self.tiles[12][0].set_texture_coordinates(Vector2i { x: 0, y: 3 });
            }
            CardinalDirections::Down => {
                self.tiles[11][14].set_texture_coordinates(Vector2i { x: 2, y: 3 });
                self.tiles[12][14].set_texture_coordinates(Vector2i { x: 2, y: 3 });
            }
            CardinalDirections::Right => {
                self.tiles[23][6].set_texture_coordinates(Vector2i { x: 1, y: 3 });
                self.tiles[23][7].set_texture_coordinates(Vector2i { x: 1, y: 3 });
            }
            CardinalDirections::Left => {
                self.tiles[0][6].set_texture_coordinates(Vector2i { x: 3, y: 3 });
                self.tiles[0][7].set_texture_coordinates(Vector2i { x: 3, y: 3 });
            }
        }
    }
}
