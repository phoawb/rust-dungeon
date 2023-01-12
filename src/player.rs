use crate::animation::Animation;
use crate::texture_storage::TextureIdentifiers;
use sfml::{
    graphics::*,
    system::{Vector2f, Vector2i},
    SfBox,
};

use sfml::window::Key;

#[derive(Debug)]
pub struct Player {
    position: Vector2f,
    size: Vector2f,
    origin: Vector2f,
    row: i32,
    animation: Animation,
    speed: f32,
    //facing direction top, right, bottom, left
    direction: [bool; 4],
}

impl Player {
    //Empty constructor
    /*     pub fn new() -> Tile {
        // Size of one tile always set
        let size = Vector2f { x: 70.0, y: 70.0 };
        Tile {
            position: Vector2f { x: 0.0, y: 0.0 },
            uv_rect: IntRect::new(0, 0, 16, 16),
            size,
            origin: size / 2.0,
        }
    } */

    // Constructor
    pub fn from(position: Vector2f, image_count: Vector2i) -> Player {
        let size = Vector2f { x: 70.0, y: 70.0 };
        Player {
            position,
            size,
            origin: size / 2.0,
            row: 0,
            animation: Animation::from(TextureIdentifiers::Player, image_count, 0.2),
            speed: 5.0,
            direction: [false, false, true, false],
        }
    }

    fn set_idle_animation(&mut self) {
        if self.direction[0] && !self.direction[3] && !self.direction[1] {
            self.row = 0;
        }
        // straight up
        else if self.direction[0] && (self.direction[1] || self.direction[3]) {
            self.row = 1;
        }
        // up right or left
        else if self.direction[2] && !self.direction[3] && !self.direction[1] {
            self.row = 4;
        }
        // down
        else if self.direction[2] && (self.direction[1] || self.direction[3]) {
            self.row = 3;
        }
        // down right or left
        else {
            self.row = 2
        }; // höger eller vänster
    }

    fn set_animation(&mut self, movement: Vector2f) {
        if movement.x > 0.0 {
            self.direction[1] = true;
            self.direction[3] = false;
            if movement.y > 0.0 {
                self.row = 8;
                self.direction[0] = false;
                self.direction[2] = true;
            } else if movement.y < 0.0 {
                self.row = 6;
                self.direction[0] = true;
                self.direction[2] = false;
            } else if movement.y == 0.0 {
                self.row = 7;
                self.direction[0] = false;
                self.direction[2] = false;
            }
        } else if movement.x < 0.0 {
            self.direction[1] = false;
            self.direction[3] = true;
            if movement.y > 0.0 {
                self.row = 8;
                self.direction[0] = false;
                self.direction[2] = true;
            } else if movement.y < 0.0 {
                self.row = 6;
                self.direction[0] = true;
                self.direction[2] = false;
            } else if movement.y == 0.0 {
                self.row = 7;
                self.direction[0] = false;
                self.direction[2] = false;
            }
        } else if movement.x == 0.0 {
            if movement.y > 0.0 {
                // down
                self.row = 9;
                self.direction = [false, false, true, false];
            } else if movement.y < 0.0 {
                // up
                self.row = 5;
                self.direction = [true, false, false, false];
            }
        }
    }

    pub fn update(&mut self) {
        let mut movement = Vector2f { x: 0.0, y: 0.0 };
        if Key::Left.is_pressed() || Key::A.is_pressed() {
            movement.x = -1.0;
        } else if Key::Right.is_pressed() || Key::D.is_pressed() {
            movement.x = 1.0;
        }

        if Key::Up.is_pressed() || Key::W.is_pressed() {
            movement.y = -1.0;
        } else if Key::Down.is_pressed() || Key::S.is_pressed() {
            movement.y = 1.0
        }

        if !(movement.x == 0.0 && movement.y == 0.0) {
            movement.x = movement.x / (movement.x.powi(2) + movement.y.powi(2)).sqrt();
            movement.y = movement.y / (movement.x.powi(2) + movement.y.powi(2)).sqrt();
            movement *= self.speed;
        }

        if movement.x == 0.0 && movement.y == 0.0 {
            self.set_idle_animation();
        } else {
            self.set_animation(movement);
        }
        self.animation.update(self.row, self.direction[1]);

        self.position += movement;
    }

    fn create_body(&self) -> RectangleShape {
        let mut body = RectangleShape::new();
        body.set_size(self.size);
        body.set_position(self.position);
        body.set_origin(self.origin);
        body.set_texture_rect(&self.animation.get_uv_rect());
        body
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        let mut body = self.create_body();
        body.set_scale(Vector2f { x: 1.1, y: 1.1 });
        body.set_texture(texture, false);
        window.draw(&body);
    }
}
