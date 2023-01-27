use crate::tile::Tile;
use rust_dungeon::{CardinalDirection, RoomColor, TILE_SIZE, VIEW_SIZE};
use sfml::{
    graphics::{RenderWindow, Texture},
    system::{Vector2f, Vector2i},
    SfBox,
};

#[derive(Debug)]
struct Doors {
    up: bool,
    down: bool,
    right: bool,
    left: bool,
}

#[derive(Debug)]
pub struct Room {
    tiles: Vec<Vec<Tile>>,
    doors: Doors,
    color: RoomColor,
}

fn get_image_count(i: i32, j: i32) -> Vector2i {
    match (i, j) {
        (0, 0) => Vector2i { x: 0, y: 0 },
        (0, 14) => Vector2i { x: 0, y: 2 },
        (0, _) => Vector2i { x: 0, y: 1 },
        (23, 0) => Vector2i { x: 2, y: 0 },
        (23, 14) => Vector2i { x: 2, y: 2 },
        (23, _) => Vector2i { x: 2, y: 1 },
        (_, 0) => Vector2i { x: 1, y: 0 },
        (_, 14) => Vector2i { x: 1, y: 2 },
        _ => Vector2i { x: 1, y: 1 },
    }
}

impl Room {
    pub fn new() -> Room {
        let mut room = Room {
            tiles: Vec::new(),
            doors: Doors {
                up: false,
                down: false,
                right: false,
                left: false,
            },
            color: RoomColor::Brown,
        };
        for _i in 0..24 {
            room.tiles.push(Vec::new());
        }
        room
    }

    //Spawn position is the coords for the upper left corner of the room
    pub fn from(spawn_position: Vector2f) -> Room {
        let tile_spawn_position: Vector2f = spawn_position + Vector2f { x: 16.0, y: 16.0 };
        let mut room: Room = Room::new();
        let amount_of_tiles_x_axis = VIEW_SIZE.x as i32 / TILE_SIZE.x as i32;
        let amount_of_tiles_y_axis = VIEW_SIZE.y as i32 / TILE_SIZE.y as i32;

        for i in 0..amount_of_tiles_x_axis {
            for j in 0..amount_of_tiles_y_axis {
                let image_count = get_image_count(i, j);
                let tile = Tile::from(
                    Vector2f {
                        x: tile_spawn_position.x + 32.0 * i as f32,
                        y: tile_spawn_position.y + 32.0 * j as f32,
                    },
                    image_count,
                );
                room.tiles[i as usize].push(tile);
            }
        }
        room
    }

    pub fn set_color(&mut self, color: RoomColor) {
        self.color = color;
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        for row in &self.tiles {
            for tile in row {
                tile.draw(window, texture, self.color);
            }
        }
    }
    pub fn set_door(&mut self, direction: CardinalDirection) {
        match direction {
            CardinalDirection::Up => {
                self.doors.up = true;
                self.tiles[11][0].set_texture_coordinates(Vector2i { x: 0, y: 3 });
                self.tiles[12][0].set_texture_coordinates(Vector2i { x: 0, y: 3 });
            }
            CardinalDirection::Down => {
                self.doors.down = true;
                self.tiles[11][14].set_texture_coordinates(Vector2i { x: 2, y: 3 });
                self.tiles[12][14].set_texture_coordinates(Vector2i { x: 2, y: 3 });
            }
            CardinalDirection::Right => {
                self.doors.right = true;
                self.tiles[23][6].set_texture_coordinates(Vector2i { x: 1, y: 3 });
                self.tiles[23][7].set_texture_coordinates(Vector2i { x: 1, y: 3 });
            }
            CardinalDirection::Left => {
                self.doors.left = true;
                self.tiles[0][6].set_texture_coordinates(Vector2i { x: 3, y: 3 });
                self.tiles[0][7].set_texture_coordinates(Vector2i { x: 3, y: 3 });
            }
        }
    }

    pub fn has_doors(&self) -> bool {
        self.doors.up || self.doors.down || self.doors.right || self.doors.left
    }

    pub fn get_door(&self, direction: CardinalDirection) -> bool {
        match direction {
            CardinalDirection::Up => self.doors.up,
            CardinalDirection::Down => self.doors.down,
            CardinalDirection::Left => self.doors.left,
            CardinalDirection::Right => self.doors.right,
        }
    }
}
