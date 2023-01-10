use sfml::{
    graphics::{RenderWindow, Texture},
    system::Vector2f,
    SfBox,
};

use crate::tile::Tile;

pub struct Room {
    tiles: Vec<Vec<Tile>>,
}

fn get_image_count(i: i32, j: i32) -> Vector2f {
    let image_count: Vector2f;
    if i == 0 {
        if j == 0 {
            image_count = Vector2f { x: 0.0, y: 0.0 };
        } else if j == 14 {
            image_count = Vector2f { x: 0.0, y: 2.0 };
        } else {
            image_count = Vector2f { x: 0.0, y: 1.0 };
        }
    } else if i == 23 {
        if j == 0 {
            image_count = Vector2f { x: 2.0, y: 0.0 };
        } else if j == 14 {
            image_count = Vector2f { x: 2.0, y: 2.0 };
        } else {
            image_count = Vector2f { x: 2.0, y: 1.0 };
        }
    } else if j == 0 {
        image_count = Vector2f { x: 1.0, y: 0.0 };
    } else if j == 14 {
        image_count = Vector2f { x: 1.0, y: 2.0 };
    } else {
        image_count = Vector2f { x: 1.0, y: 1.0 };
    }
    image_count
}

impl Room {
    pub fn new() -> Room {
        let mut room = Room { tiles: Vec::new() };
        for _i in 0..15 {
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
                room.tiles[0].push(tile);
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
}
