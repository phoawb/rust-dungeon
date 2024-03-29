use crate::collider::Collider;
use crate::texture_storage::TextureIdentifiers;
use crate::{animation::Animation, projectile::Projectile};
use rust_dungeon::{Body, PLAYER_PROJECTILE_IMAGE_COUNT};
use sfml::{
    graphics::*,
    system::{Vector2f, Vector2i},
    SfBox,
};

use sfml::window::Key;

#[derive(Debug)]
pub struct Player {
    size: Vector2f,
    origin: Vector2f,
    row: i32,
    animation: Animation,
    speed: f32,
    //facing direction top, right, bottom, left
    direction: [bool; 4],
    collider: Collider,
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
    pub fn from(position: Vector2f) -> Player {
        let size = Vector2f { x: 70.0, y: 70.0 };
        let image_count = Vector2i { x: 4, y: 10 };
        Player {
            size,
            origin: size / 2.0,
            row: 0,
            animation: Animation::from(TextureIdentifiers::Player, image_count, 0.2),
            speed: 10.0, //TODO: CHANGE THE SPEED TO 5 IN PRODUCTION
            direction: [false, false, true, false],
            collider: Collider::new(size, position, None),
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
        match (movement.x, movement.y) {
            (x, y) if x > 0.0 => {
                self.direction = [false, true, false, false];
                match y {
                    y if y > 0.0 => {
                        self.row = 8;
                        self.direction[2] = true;
                    }
                    y if y < 0.0 => {
                        self.row = 6;
                        self.direction[0] = true;
                    }
                    _ => {
                        self.row = 7;
                    }
                }
            }
            (x, y) if x < 0.0 => {
                self.direction = [false, false, false, true];
                match y {
                    y if y > 0.0 => {
                        self.row = 8;
                        self.direction[2] = true;
                    }
                    y if y < 0.0 => {
                        self.row = 6;
                        self.direction[0] = true;
                    }
                    _ => {
                        self.row = 7;
                    }
                }
            }
            (_, y) if y > 0.0 => {
                self.row = 9;
                self.direction = [false, false, true, false];
            }
            (_, y) if y < 0.0 => {
                self.row = 5;
                self.direction = [true, false, false, false];
            }
            _ => { /* Do Nothing */ }
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

        self.set_position(self.get_position() + movement);
    }

    pub fn get_position(&self) -> Vector2f {
        self.collider.get_position()
    }

    pub fn set_position(&mut self, position: Vector2f) {
        self.collider.set_position(position);
    }

    pub fn shoot(&self, mouse_coords: Vector2f) -> Projectile {
        let direction = mouse_coords - self.get_position();
        let normalized_direction =
            direction / ((direction.x.powf(2.0) + direction.y.powf(2.0)).sqrt());
        //TODO: PUT THIS AS A VAR IN LIB
        let projectile_size = Vector2f::new(64.0, 64.0);
        Projectile::new(
            self.get_position(),
            projectile_size,
            normalized_direction,
            TextureIdentifiers::Projectile,
            PLAYER_PROJECTILE_IMAGE_COUNT,
        )
    }

    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        let mut body = self.create_body(
            self.size,
            self.collider.get_position(),
            self.origin,
            self.animation.get_uv_rect(),
        );
        body.set_scale(Vector2f { x: 1.1, y: 1.1 });
        body.set_texture(texture, false);
        body.set_outline_color(Color::CYAN);
        window.draw(&body);
    }

    pub fn get_collider(&mut self) -> &mut Collider {
        &mut self.collider
    }
}

impl Body for Player {}
