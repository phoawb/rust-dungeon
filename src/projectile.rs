use std::f32::consts::PI;

use rust_dungeon::Body;
use sfml::{
    graphics::{RenderTarget, RenderWindow, Shape, Texture, Transformable},
    system::{Vector2f, Vector2i},
    SfBox,
};

use crate::{animation::Animation, texture_storage::TextureIdentifiers};

#[derive(Debug)]
pub struct Projectile {
    position: Vector2f,
    size: Vector2f,
    direction: Vector2f,
    origin: Vector2f,
    animation: Animation,
    speed: f32,
}

impl Projectile {
    pub fn new(position: Vector2f, size: Vector2f, direction: Vector2f) -> Projectile {
        let origin = size / 2.0;
        let image_count = Vector2i::new(17, 1);
        let switch_time = 0.2;
        let speed = 5.0;
        Projectile {
            position,
            size,
            direction,
            origin,
            animation: Animation::from(TextureIdentifiers::Projectile, image_count, switch_time),
            speed,
        }
    }
    pub fn update(&mut self) {
        self.position += self.direction * self.speed;
        let row = 0;
        let face_right = true;
        self.animation.update(row, face_right)
    }
    pub fn draw(&self, window: &mut RenderWindow, texture: &SfBox<Texture>) {
        let mut body = self.create_body(
            self.size,
            self.position,
            self.origin,
            self.animation.get_uv_rect(),
        );
        let angle: f32 = if self.direction.x < 0.0 {
            180.0 - (180.0 / PI) * ((-1.0 * self.direction.y) / self.direction.x).atan()
        } else {
            -1.0 * (180.0 / PI) * ((-1.0 * self.direction.y) / self.direction.x).atan()
        };
        body.set_rotation(angle);
        body.set_scale(Vector2f { x: 1.1, y: 1.1 });
        body.set_texture(texture, false);
        window.draw(&body);
    }
}

impl Body for Projectile {}
